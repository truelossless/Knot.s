use base64::encode;

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
        builder.start_tag("div", &[("class", next_container)]);
        builder.current_container = next_container.to_owned()
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

pub struct BlockQuote {
    pub contents: Vec<Box<dyn KnotsObject>>,
}

impl KnotsObject for BlockQuote {
    fn write_html(&self, builder: &mut Builder) {
        builder.start_tag("blockquote", &[]);
        builder.write_knots_objects(&self.contents);
        builder.end_tag(); // </blockquote>
    }
}

const INFO_SVG: &str = include_str!("../icons/info.svg");
pub struct InfoBox {
    pub contents: Vec<Box<dyn KnotsObject>>,
}

impl KnotsObject for InfoBox {
    fn write_html(&self, builder: &mut Builder) {
        builder.start_tag("div", &[("class", "infobox")]);
        builder.write_content(INFO_SVG);
        builder.start_tag("p", &[]);
        builder.write_knots_objects(&self.contents);
        builder.end_tag(); // </p>
        builder.end_tag(); // </div>
    }
}

const WARNING_SVG: &str = include_str!("../icons/danger.svg");

pub struct WarningBox {
    pub contents: Vec<Box<dyn KnotsObject>>,
}

impl KnotsObject for WarningBox {
    fn write_html(&self, builder: &mut Builder) {
        builder.start_tag("div", &[("class", "warningbox")]);
        builder.write_content(WARNING_SVG);
        builder.start_tag("p", &[]);
        builder.write_knots_objects(&self.contents);
        builder.end_tag(); // </p>
        builder.end_tag(); // </div>
    }
}

const ERROR_SVG: &str = include_str!("../icons/close-o.svg");
pub struct ErrorBox {
    pub contents: Vec<Box<dyn KnotsObject>>,
}

impl KnotsObject for ErrorBox {
    fn write_html(&self, builder: &mut Builder) {
        builder.start_tag("div", &[("class", "errorbox")]);
        builder.write_content(ERROR_SVG);
        builder.start_tag("p", &[]);
        builder.write_knots_objects(&self.contents);
        builder.end_tag(); // </p>
        builder.end_tag(); // </div>
    }
}

pub struct List {
    pub contents: Vec<Vec<Box<dyn KnotsObject>>>,
}

impl KnotsObject for List {
    fn write_html(&self, builder: &mut Builder) {
        builder.start_tag("ul", &[]);

        for list_item in &self.contents {
            builder.start_tag("li", &[]);
            builder.write_knots_objects(&list_item);
            builder.end_tag(); // </li>
        }

        builder.end_tag(); // </ul>
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

const LINK_SVG: &str = include_str!("../icons/link.svg");

pub struct Link {
    pub name: String,
    pub link: String,
}

impl KnotsObject for Link {
    fn write_html(&self, builder: &mut Builder) {
        builder.inline_tag("a", &[("href", &self.link), ("class", "link")], &self.name);
    }
}

pub struct Image {
    pub alt: String,
    pub link: String,
}

impl KnotsObject for Image {
    fn write_html(&self, builder: &mut Builder) {
        builder.end_tag(); // </div>
        builder.start_tag("div", &[("class", "container-lg")]);

        if self.link.starts_with("http://") || self.link.starts_with("https://") {
            // include directly the link if it's from internet
            builder.orphan_tag("img", &[("alt", &self.alt), ("src", &self.link)]);
        } else {
            // else if it's from the disk load it as base64
            let file = std::fs::read(&self.link).expect("Unable to open image");
            let base64_img = format!("data:application/octet-stream;base64,{}", encode(file));
            builder.orphan_tag("img", &[("alt", &self.alt), ("src", &base64_img)]);
        };

        builder.end_tag();

        let current_container = builder.current_container.clone();
        builder.start_tag("div", &[("class", &current_container)]);
    }
}

pub struct HorizontalRule {}

impl KnotsObject for HorizontalRule {
    fn write_html(&self, builder: &mut Builder) {
        builder.orphan_tag("hr", &[]);
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
        builder.inline_tag("span", &[("id", &el_id)], "");
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
        let current_container = builder.current_container.clone();
        builder.start_tag("div", &[("class", &current_container)]);
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
