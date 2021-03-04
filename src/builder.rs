use crate::prism_autoloader;
use std::collections::HashSet;

use super::knots_objects::KnotsObject;
use super::utils::{get_alpha_numeral, get_roman_numeral};

#[derive(Clone)]
pub struct Title {
    pub level: u8,
    pub name: String,
    pub anchor: String,
}

/// A Builder used to generate HTML tags from Knots objects.
#[derive(Default)]
pub struct Builder {
    /// the text buffer where all tags are stored once finished
    buf: String,
    /// the current indentation
    indentation: usize,
    /// the tags waiting to be closed
    tags_queue: Vec<String>,
    /// should we include prism ?
    pub should_include_prism: bool,
    /// should we include katex ?
    pub should_include_katex: bool,
    /// should we include mermaid ?
    pub should_include_mermaid: bool,
    /// keep track of the current container class
    pub current_container: String,
    /// the number of lv1 titles
    lv1_titles: usize,
    /// the number of lv2 titles since the last lv1 title
    lv2_titles: usize,
    /// an array to keep track of the summary
    titles: Vec<Title>,
    /// the number of maths blocks
    pub maths_blocks: usize,
    /// we need to populate katex blocks after the script inclusion
    katex_buf: String,
    /// the different programming languages used in the document
    pub languages: HashSet<String>,
}

impl Builder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Prints the resulting buffer to stdout
    pub fn into_result(self) -> String {
        if !self.tags_queue.is_empty() {
            panic!("Unclosed tags !!");
        }

        self.buf
    }

    /// Starts an orphan tag
    pub fn orphan_tag(&mut self, tag_name: &str, attributes: &[(&str, &str)]) {
        self.buf += &self.format_start_tag(tag_name, attributes);
    }

    /// Writes to an inline tag
    pub fn inline_tag(&mut self, tag_name: &str, attributes: &[(&str, &str)], contents: &str) {
        self.buf += &format!(
            "{blanks}<{tag}{attributes}>{contents}</{tag}>\n",
            blanks = self.blanks(),
            tag = tag_name,
            attributes = self.attributes(attributes),
            contents = contents
        );
    }

    /// Starts a new tag
    pub fn start_tag(&mut self, tag_name: &str, attributes: &[(&str, &str)]) {
        self.tags_queue.push(tag_name.to_owned());
        self.buf += &self.format_start_tag(tag_name, attributes);
        self.indentation += 1;
    }

    /// Writes content inside a tag
    pub fn write_content(&mut self, content: &str) {
        // indent the content
        let newline_blanks = format!("\n{}", self.blanks());
        let mut formatted_content = content.to_owned();

        // normalize line endings
        formatted_content = formatted_content.replace("\r\n", "\n");
        formatted_content = formatted_content.replace("\n", &newline_blanks);

        self.buf += &self.blanks();
        self.buf += &formatted_content;
        self.buf.push('\n');
    }

    /// Links a div with its katex content
    pub fn write_katex_content(&mut self, content: &str, el_id: &str) {
        self.katex_buf += &format!(
            "katex.render(String.raw`{}`, document.getElementById('{}'), {{ throwOnError: false }});",
            content, el_id
        );
        self.katex_buf.push('\n');
    }

    /// Returns the katex buffer referencing all html elements and their latex contents
    pub fn get_katex_content(&self) -> String {
        self.katex_buf.clone()
    }

    /// Returns the different prism plugins to be included
    pub fn get_prism_plugins(&mut self) -> Vec<&'static str> {
        prism_autoloader::find_plugins(&self.languages.drain().collect::<Vec<_>>())
    }

    /// Writes a Knots object
    #[inline]
    pub fn write_knots_object(&mut self, object: Box<dyn KnotsObject>) {
        object.write_html(self)
    }

    /// Writes multiple Knots objects
    #[inline]
    pub fn write_knots_objects(&mut self, objects: &[Box<dyn KnotsObject>]) {
        for object in objects {
            object.write_html(self);
        }
    }

    /// Ends a tag
    pub fn end_tag(&mut self) {
        self.indentation -= 1;
        self.buf += &format!("{}</{}>\n", self.blanks(), self.tags_queue.pop().unwrap());
    }

    /// Returns the summary
    pub fn get_summary(&self) -> &[Title] {
        &self.titles
    }

    /// Adds a title to the summary
    pub fn add_title(&mut self, level: u8, name: &str) -> Title {
        let num;

        match level {
            1 => {
                self.lv1_titles += 1;
                // reset the count on lv2 titles since we're starting a new section
                self.lv2_titles = 0;
                num = get_roman_numeral(self.lv1_titles);
            }

            2 => {
                self.lv2_titles += 1;
                num = get_alpha_numeral(self.lv2_titles);
            }
            3 => num = String::new(),
            _ => unreachable!(),
        }

        let mut anchor = String::new();

        if level >= 1 {
            anchor += &format!("{}-", self.lv1_titles);
        }

        if level >= 2 {
            anchor += &format!("{}-", self.lv2_titles);
        }

        if level >= 3 {
            anchor += "part-";
        }

        let escaped_name: String = name
            .replace(" ", "-")
            .chars()
            .into_iter()
            .filter(|&c| c.is_ascii_alphanumeric() || "_-!?".contains(c))
            .collect();

        anchor.push_str(&escaped_name);

        let name = if !num.is_empty() {
            format!("{} - {}", num, name)
        } else {
            name.to_owned()
        };

        let title = Title {
            anchor,
            level,
            name,
        };

        self.titles.push(title.clone());
        title
    }

    /// Returns the number of tabs corresponding to the indentation
    fn blanks(&self) -> String {
        let mut blanks = String::new();

        // indentation
        for _ in 0..self.indentation {
            blanks.push('\t');
        }

        blanks
    }

    /// Builds the HTML representation of a list of attributes
    fn attributes(&self, attributes: &[(&str, &str)]) -> String {
        let mut attributes_string = String::new();
        for attribute in attributes {
            attributes_string =
                format!("{} {}=\"{}\"", attributes_string, attribute.0, attribute.1);
        }
        attributes_string
    }

    /// Formats a start tag with his attributes
    #[inline]
    fn format_start_tag(&self, tag_name: &str, attributes: &[(&str, &str)]) -> String {
        format!(
            "{}<{}{}>\n",
            self.blanks(),
            tag_name,
            self.attributes(attributes)
        )
    }
}
