use super::builder::Builder;
use super::utils::{escape_html, get_alpha_numeral, get_roman_numeral};

/// Trait representing any Knot.s Object.
pub trait KnotsObject {
    fn write_html(&self, _builder: &mut Builder) {
        unimplemented!();
    }
}

pub struct Root {
    pub contents: Vec<Box<dyn KnotsObject>>,
}

impl KnotsObject for Root {
    fn write_html(&self, builder: &mut Builder) {
        builder.write_knots_objects(&self.contents);
    }
}

pub struct Lv1Title {
    pub contents: Vec<Box<dyn KnotsObject>>,
}

impl KnotsObject for Lv1Title {
    fn write_html(&self, builder: &mut Builder) {
        // reset the count on lv2 titles since we're starting a new section
        builder.lv2_titles = 0;
        builder.lv1_titles += 1;
        let num = get_roman_numeral(builder.lv1_titles);
        builder.start_tag("h2", &[]);
        builder.write_content(&format!("{} - ", num));
        builder.write_knots_objects(&self.contents);
        builder.end_tag(); // </h2>
    }
}

pub struct Lv2Title {
    pub contents: Vec<Box<dyn KnotsObject>>,
}

impl KnotsObject for Lv2Title {
    fn write_html(&self, builder: &mut Builder) {
        builder.lv2_titles += 1;
        let num = get_alpha_numeral(builder.lv2_titles);
        builder.start_tag("h3", &[]);
        builder.write_content(&format!("{} - ", num));
        builder.write_knots_objects(&self.contents);
        builder.end_tag(); // </h3>
    }
}

pub struct Paragraph {
    pub contents: Vec<Box<dyn KnotsObject>>,
}
impl KnotsObject for Paragraph {
    fn write_html(&self, builder: &mut Builder) {
        builder.start_tag("p", &[]);
        builder.write_knots_objects(&self.contents);
        builder.end_tag() // </p>
    }
}

pub struct LineBreak {}

impl KnotsObject for LineBreak {
    fn write_html(&self, builder: &mut Builder) {
        builder.start_orphan_tag("br", &[]);
    }
}

pub struct BasicText {
    pub contents: String,
}

impl KnotsObject for BasicText {
    fn write_html(&self, builder: &mut Builder) {
        builder.write_content(&self.contents);
    }
}

pub struct Italic {
    pub contents: Vec<Box<dyn KnotsObject>>,
}

impl KnotsObject for Italic {
    fn write_html(&self, builder: &mut Builder) {
        builder.start_tag("i", &[]);
        builder.write_knots_objects(&self.contents);
        builder.end_tag() // </i>
    }
}

pub struct Bold {
    pub contents: Vec<Box<dyn KnotsObject>>,
}

impl KnotsObject for Bold {
    fn write_html(&self, builder: &mut Builder) {
        builder.start_tag("b", &[]);
        builder.write_knots_objects(&self.contents);
        builder.end_tag() // </b>
    }
}

pub struct InlineCode {
    pub contents: String,
}

impl KnotsObject for InlineCode {
    fn write_html(&self, builder: &mut Builder) {
        builder.inline_tag(
            "code",
            &[("class", "codeinline")],
            &escape_html(&self.contents),
        );
    }
}

pub struct InlineMaths {
    pub contents: String,
}

impl KnotsObject for InlineMaths {
    fn write_html(&self, builder: &mut Builder) {
        builder.should_include_katex = true;
        builder.maths_blocks += 1;
        let el_id = format!("maths{}", builder.maths_blocks);
        builder.start_tag("span", &[("id", &el_id)]);
        builder.end_tag(); // </span>
        builder.write_katex_content(&self.contents, &el_id);
    }
}

pub struct CodeBlock {
    pub contents: String,
    pub lang: Option<String>,
}

impl KnotsObject for CodeBlock {
    fn write_html(&self, builder: &mut Builder) {
        builder.should_include_prism = true;

        // switch to a container-lg div to have a wider code block
        builder.end_tag(); // </div>
        builder.start_tag("div", &[("class", "container-lg")]);

        builder.start_tag("pre", &[("class", "codeblock")]);
        if let Some(lang) = &self.lang {
            let lang_class = format!("language-{}", lang);
            builder.start_tag("code", &[("class", &lang_class)]);
        } else {
            builder.start_tag("code", &[]);
        };
        builder.write_content(&escape_html(&self.contents));
        builder.end_tag(); // </pre>
        builder.end_tag(); // </code>

        // open another regular container after that
        builder.end_tag(); // </div>
        builder.start_tag("div", &[("class", "container")]);
    }
}

pub struct MathsBlock {
    pub contents: String,
}

impl KnotsObject for MathsBlock {
    fn write_html(&self, builder: &mut Builder) {
        builder.should_include_katex = true;
        builder.maths_blocks += 1;
        let el_id = format!("maths{}", builder.maths_blocks);
        builder.start_tag("div", &[("id", &el_id)]);
        builder.end_tag(); // </div>
        builder.write_katex_content(&self.contents, &el_id);
    }
}
