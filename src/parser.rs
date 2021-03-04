use std::{fs::read_to_string, process};

use crate::knots_objects;

use super::knots_objects::KnotsObject;
use nom::{
    branch::alt,
    bytes::complete::is_a,
    bytes::complete::is_not,
    bytes::complete::tag,
    bytes::complete::take_until,
    character::complete::alpha1,
    character::complete::alphanumeric1,
    character::complete::line_ending,
    character::complete::multispace0,
    character::complete::not_line_ending,
    character::complete::space0,
    character::complete::{none_of, space1},
    combinator::{eof, peek},
    combinator::{opt, recognize},
    error::ParseError,
    multi::many0,
    multi::{count, many1},
    sequence::delimited,
    sequence::{pair, preceded, separated_pair, terminated},
    AsChar, IResult, InputTakeAtPosition, Parser,
};

pub struct ParseResult {
    pub root_object: Box<dyn KnotsObject>,
    pub document_title: String,
    pub document_authors: Vec<String>,
    pub document_license: Option<String>,
}

/// Parses a .knots file
pub fn parse(file_name: &str) -> Result<ParseResult, String> {
    // parse the file
    let input =
        read_to_string(file_name).map_err(|_| format!("Failed to open file {}", file_name))?;

    // start by getting all the variables
    let (other, variables) = many0(var_pair)(&input).unwrap();

    let mut document_title = None;
    let mut document_license = None;
    let mut document_authors = Vec::new();

    for (var_name, var_content) in variables {
        match var_name {
            "title" => document_title = Some(var_content.to_owned()),
            "author" => document_authors.push(var_content.to_owned()),
            "license" => document_license = Some(var_content.to_owned()),
            _ => eprintln!("unknown metadata: {}", var_name),
        }
    }

    let document_title = document_title.unwrap_or_else(|| file_name.to_owned());

    let (other, contents) = delimited(multispace0, many0(any_object), multispace0)(other).unwrap();

    let root_object = Box::new(knots_objects::Root { contents });

    if !other.is_empty() {
        let first_errored_line = other.lines().next().unwrap();

        eprintln!(
            "Parser failed! Incorrect syntax on this line:\n{}",
            first_errored_line
        );

        process::exit(1);
    }

    Ok(ParseResult {
        root_object,
        document_title,
        document_authors,
        document_license,
    })
}

/// Parses a raw string
fn basic(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = many1(alt((
        recognize(pair(tag("$"), is_a(" ?!.,;"))),
        recognize(is_not("`*\r\n#_[$|")),
    )))(input)?;
    let raw = Box::new(knots_objects::BasicText {
        contents: contents.into_iter().fold(String::new(), |acc, x| acc + x),
    });

    Ok((other, raw))
}

/// Parses an italic string using `*`
fn italic1(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("*"), many1(any_text_modifier), tag("*"))(input)?;
    let italic_obj = Box::new(knots_objects::Italic { contents });
    Ok((other, italic_obj))
}

/// Parses an italic string using `_`
fn italic2(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("_"), many1(any_text_modifier), tag("_"))(input)?;
    let italic_obj = Box::new(knots_objects::Italic { contents });
    Ok((other, italic_obj))
}

/// Parses a bold string using `**`
fn bold1(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("**"), many1(any_text_modifier), tag("**"))(input)?;
    let bold_obj = Box::new(knots_objects::Bold { contents });
    Ok((other, bold_obj))
}

/// Parses a bold string using `__`
fn bold2(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("__"), many1(any_text_modifier), tag("__"))(input)?;
    let bold_obj = Box::new(knots_objects::Bold { contents });
    Ok((other, bold_obj))
}

/// Parses a link
fn link(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, name) = delimited(tag("["), take_until("]"), tag("]"))(input)?;
    let (other, link) = delimited(tag("("), take_until(")"), tag(")"))(other)?;

    let link_obj = Box::new(knots_objects::Link {
        name: name.to_owned(),
        link: link.to_owned(),
    });

    Ok((other, link_obj))
}

/// Parses inline code
fn inline_code(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("`"), is_not("`"), tag("`"))(input)?;
    let code_obj = Box::new(knots_objects::InlineCode {
        contents: contents.to_owned(),
    });
    Ok((other, code_obj))
}

/// Parses inline maths
fn inline_maths(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    // do not match sequences with the first character after the dollar sign being punctuation or space.
    // This is to avoid false positives when using text like "this costs 300$ at this store"
    let (other, contents) = delimited(
        pair(tag("$"), peek(none_of(" ?!.,;"))),
        is_not("$"),
        tag("$"),
    )(input)?;
    let maths_obj = Box::new(knots_objects::InlineMaths {
        contents: contents.to_owned(),
    });
    Ok((other, maths_obj))
}

/// Parses as a bold, italic or raw string
fn any_text_modifier(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    alt((
        link,
        bold1,
        bold2,
        italic1,
        italic2,
        inline_maths,
        inline_code,
        basic,
    ))(input)
}

/// Matches if we're at the end of a line or of the file
fn eolf(input: &str) -> IResult<&str, &str> {
    alt((line_ending, eof))(input)
}

/// Strips whitespaces
fn ws<I, O, E>(input: impl Parser<I, O, E>) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    E: ParseError<I>,
{
    delimited(space0, input, space0)
}

/// Parses a Knots variable name
fn variable(input: &str) -> IResult<&str, &str> {
    preceded(tag("%"), alpha1)(input)
}

/// Parses a variable and its contents like %title Hello world
/// to a pair (var_name, var_contents), in this case ("title", "Hello World")
fn var_pair(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(variable, space1, terminated(not_line_ending, eolf))(input)
}

/// Parses a paragraph of text
fn paragraph(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = terminated(many1(any_text_modifier), eolf)(input)?;
    let paragraph_obj = Box::new(knots_objects::Paragraph { contents });
    Ok((other, paragraph_obj))
}

/// Parses a Blockquote
fn block_quote(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag(">"), many1(any_text_modifier), eolf)(input)?;
    let quote_obj = Box::new(knots_objects::BlockQuote { contents });

    Ok((other, quote_obj))
}

/// Parses an info box
fn info_box(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("?>"), many1(any_text_modifier), eolf)(input)?;
    let box_obj = Box::new(knots_objects::InfoBox { contents });

    Ok((other, box_obj))
}

/// Parses a warning box
fn warning_box(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("!>"), many1(any_text_modifier), eolf)(input)?;
    let box_obj = Box::new(knots_objects::WarningBox { contents });

    Ok((other, box_obj))
}

/// Parses an error box
fn error_box(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("x>"), many1(any_text_modifier), eolf)(input)?;
    let box_obj = Box::new(knots_objects::ErrorBox { contents });

    Ok((other, box_obj))
}

/// Parses an horizontal ruler
fn horizontal_ruler(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, _) = delimited(
        alt((tag("***"), tag("---"), tag("___"))),
        many0(is_a("*_-")),
        eolf,
    )(input)?;
    let hr_obj = Box::new(knots_objects::HorizontalRule {});
    Ok((other, hr_obj))
}

/// Parses a level 1 title
fn lvl1_title(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("#"), ws(not_line_ending), eolf)(input)?;
    let title_obj = Box::new(knots_objects::Title {
        contents: contents.to_owned(),
        level: 1,
    });
    Ok((other, title_obj))
}

/// Parses a level 2 title
fn lvl2_title(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("##"), ws(not_line_ending), eolf)(input)?;
    let title_obj = Box::new(knots_objects::Title {
        contents: contents.to_owned(),
        level: 2,
    });
    Ok((other, title_obj))
}

/// Parses a level 3 title
fn lvl3_title(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("###"), ws(not_line_ending), eolf)(input)?;
    let title_obj = Box::new(knots_objects::Title {
        contents: contents.to_owned(),
        level: 3,
    });
    Ok((other, title_obj))
}

/// Parses a code block
fn code_block(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, _) = tag("```")(input)?;

    // try to read the language annotation if it exists
    let (other, lang) = opt(alphanumeric1)(other)?;
    let (other, _) = line_ending(other)?;
    let (other, contents) = terminated(take_until("```"), tag("```"))(other)?;

    let lang = lang.unwrap_or_default().to_lowercase();

    // if the language annotation is mermaid, render as a mermaid diagram
    if lang == "mermaid" {
        let mermaid_obj = Box::new(knots_objects::Mermaid {
            contents: contents.to_owned(),
        });
        Ok((other, mermaid_obj))

    // else it's a prism code block
    } else {
        let code_obj = Box::new(knots_objects::CodeBlock {
            contents: contents.to_owned(),
            lang,
        });
        Ok((other, code_obj))
    }
}

/// Parses a maths block
fn maths_block(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("$$"), take_until("$$"), tag("$$"))(input)?;
    let maths_obj = Box::new(knots_objects::MathsBlock {
        contents: contents.to_owned(),
    });

    Ok((other, maths_obj))
}

/// Parses a list bullet
fn list_item(input: &str, level: u8) -> IResult<&str, Vec<Box<dyn KnotsObject>>> {
    // this item belongs to the list only if it has the right tabulation
    let indent_level = alt((
        count(tag(" "), (4 * level) as usize),
        count(tag("\t"), level as usize),
    ));

    let (other, first_contents) = preceded(pair(indent_level, tag("-")), paragraph)(input)?;

    // similarly, the next line belongs to this list item only if it has the right tabulation
    let next_indent_level = alt((
        count(tag(" "), 4 * (level + 1) as usize),
        count(tag("\t"), (level + 1) as usize),
    ));

    let (other, mut next_contents) = many0(alt((
        |input| list(input, level + 1),
        preceded(next_indent_level, paragraph),
    )))(other)?;

    next_contents.insert(0, first_contents);

    Ok((other, next_contents))
}

// Parses a list
fn list(input: &str, level: u8) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = many1(|input| list_item(input, level))(input)?;
    let list_obj = Box::new(knots_objects::List { contents });
    Ok((other, list_obj))
}

/// Parses the delimiter after the table header e.g |---|---|
fn table_delimiter(input: &str) -> IResult<&str, ()> {
    let (other, _) = many1(pair(tag("|"), many1(tag("-"))))(input)?;
    let (other, _) = pair(tag("|"), ws(eolf))(other)?;
    Ok((other, ()))
}

/// Parses a table row
fn table_row(input: &str) -> IResult<&str, Vec<Vec<Box<dyn KnotsObject>>>> {
    // match the field name
    let (other, fields) = many1(delimited(
        tag("|"),
        ws(many0(any_text_modifier)),
        peek(tag("|")),
    ))(input)?;

    // match the last closing pipe
    let (other, _) = pair(tag("|"), eolf)(other)?;
    Ok((other, fields))
}

/// Parses a table
fn table(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, header) = table_row(input)?;
    let (other, _) = table_delimiter(other)?;
    let (other, rows) = many1(table_row)(other)?;

    let table_obj = Box::new(knots_objects::Table { header, rows });
    Ok((other, table_obj))
}

/// Parses an image
fn image(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, _) = tag("!")(input)?;
    let (other, name) = delimited(tag("["), take_until("]"), tag("]"))(other)?;
    let (other, link) = delimited(tag("("), take_until(")"), tag(")"))(other)?;

    let img_obj = Box::new(knots_objects::Image {
        alt: name.to_owned(),
        link: link.to_owned(),
    });

    Ok((other, img_obj))
}

/// Parses an object contained on one line
fn any_object(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    delimited(
        multispace0,
        alt((
            horizontal_ruler,
            lvl3_title,
            lvl2_title,
            lvl1_title,
            |input| list(input, 0),
            table,
            code_block,
            maths_block,
            image,
            info_box,
            warning_box,
            error_box,
            block_quote,
            paragraph,
        )),
        multispace0,
    )(input)
}
