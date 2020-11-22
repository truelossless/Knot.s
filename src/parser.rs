use anyhow::{Context, Result};
use std::fs::read_to_string;

use crate::knots_objects;

use super::knots_objects::KnotsObject;
use nom::{
    branch::alt,
    bytes::complete::is_not,
    bytes::complete::tag,
    bytes::complete::take_until,
    character::complete::alpha1,
    character::complete::alphanumeric1,
    character::complete::line_ending,
    character::complete::multispace0,
    character::complete::not_line_ending,
    character::complete::space0,
    character::complete::space1,
    combinator::eof,
    combinator::opt,
    error::ParseError,
    multi::many0,
    multi::many1,
    sequence::delimited,
    sequence::{preceded, separated_pair, terminated},
    AsChar, IResult, InputTakeAtPosition, Parser,
};

pub struct ParseResult {
    pub root_object: Box<dyn KnotsObject>,
    pub document_title: String,
    pub document_authors: Vec<String>,
    pub document_license: Option<String>,
}

/// Parses a .knots file
pub fn parse(file_name: &str) -> Result<ParseResult> {
    // parse the file
    let input =
        read_to_string(file_name).with_context(|| format!("Failed to open file {}", file_name))?;

    // start by getting all the variables
    let (other, variables) = many0(var_pair)(&input).unwrap();

    let mut document_title = None;
    let mut document_license = None;
    let mut document_authors = Vec::new();

    for variable in variables {
        match variable.0 {
            "title" => document_title = Some(variable.1.to_owned()),
            "author" => document_authors.push(variable.1.to_owned()),
            "license" => document_license = Some(variable.1.to_owned()),
            _ => eprintln!("unknown metadata: {}", variable.0),
        }
    }

    let document_title = document_title.unwrap_or_else(|| file_name.to_owned());

    let (other, contents) = many0(any_object)(other).unwrap();

    let root_object = Box::new(knots_objects::Root { contents });

    if !other.is_empty() {
        eprintln!("parser did not finish correctly ! '{}' remains", other);
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
    let (other, contents) = is_not("`*\n\r#$")(input)?;
    let raw = Box::new(knots_objects::BasicText {
        contents: contents.to_owned(),
    });

    Ok((other, raw))
}

/// Parses an italic string
fn italic(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("*"), many1(any_text_modifier), tag("*"))(input)?;
    let italic_obj = Box::new(knots_objects::Italic { contents });
    Ok((other, italic_obj))
}

/// Parses a bold string
fn bold(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("**"), many1(any_text_modifier), tag("**"))(input)?;
    let bold_obj = Box::new(knots_objects::Bold { contents });
    Ok((other, bold_obj))
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
    let (other, contents) = delimited(tag("$"), is_not("$"), tag("$"))(input)?;
    let maths_obj = Box::new(knots_objects::InlineMaths {
        contents: contents.to_owned(),
    });

    Ok((other, maths_obj))
}

/// Parses as a bold, italic or raw string
fn any_text_modifier(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    alt((bold, italic, inline_maths, inline_code, basic))(input)
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

/// Parses a Knot.s variable name and contents
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

/// Parses a level 1 title
fn lvl1_title(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("#"), ws(many1(any_text_modifier)), eolf)(input)?;
    let title_obj = Box::new(knots_objects::Lv1Title { contents });
    Ok((other, title_obj))
}

/// Parses a level 2 title
fn lvl2_title(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("##"), ws(many1(any_text_modifier)), eolf)(input)?;
    let title_obj = Box::new(knots_objects::Lv2Title { contents });
    Ok((other, title_obj))
}

fn code_block(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, _) = tag("```")(input)?;

    // try to read the language annotation if it exists
    let (other, lang) = opt(alphanumeric1)(other)?;
    let (other, _) = line_ending(other)?;
    let (other, contents) = terminated(take_until("```"), tag("```"))(other)?;

    let code_obj = Box::new(knots_objects::CodeBlock {
        contents: contents.to_owned(),
        lang: lang.map(String::from),
    });

    Ok((other, code_obj))
}

fn maths_block(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, contents) = delimited(tag("$$"), take_until("$$"), tag("$$"))(input)?;
    let maths_obj = Box::new(knots_objects::MathsBlock {
        contents: contents.to_owned(),
    });

    Ok((other, maths_obj))
}

/// Parses an object contained on one line
fn any_object(input: &str) -> IResult<&str, Box<dyn KnotsObject>> {
    let (other, _) = multispace0(input)?;
    alt((lvl2_title, lvl1_title, code_block, maths_block, paragraph))(other)
}
