use super::builder::Builder;
use super::utils::escape_html;

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

const LINK_SVG: &str = include_str!("../icons/link.svg");

pub struct Title {
    pub contents: String,
    pub level: u8,
}

impl KnotsObject for Title {
    fn write_html(&self, builder: &mut Builder) {
        let title_container;
        let next_container;

        let title = builder.add_title(self.level, &self.contents);

        match self.level {
            1 => {
                title_container = "container-lvl1";
                next_container = "container-lvl2";
            }

            2 => {
                title_container = "container-lvl2";
                next_container = "container";
            }
            3 => {
                title_container = "container";
                next_container = "container";
            }
            _ => unreachable!(),
        }

        let tag = format!("h{}", self.level);
        let level_class = format!("lvl{}", self.level);

        builder.end_tag(); // </div>

        // switch to the larger container
        builder.start_tag("div", &[("class", &title_container)]);
        builder.start_tag(&tag, &[("class", &level_class), ("id", &title.anchor)]);
        builder.start_tag("a", &[("href", &format!("#{}", &title.anchor))]);
        builder.write_content(&title.name);
        builder.write_content(LINK_SVG);
        builder.end_tag();
        builder.end_tag(); // </h2>
        builder.end_tag(); // </div>
        builder.start_tag("div", &[("class", &next_container)]);
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
            &[("class", "inline-code")],
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
        builder.start_tag("div", &[("id", &el_id), ("class", "mathsblock")]);
        builder.end_tag(); // </div>
        builder.write_katex_content(&self.contents, &el_id);
    }
}
