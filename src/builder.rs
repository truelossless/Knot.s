use super::knots_objects::KnotsObject;
use super::utils::escape_latex;

/// A Builder used to generate HTML tags from Knot.s objects.
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
    /// the number of lv1 titles
    pub lv1_titles: usize,
    /// the number of lv2 titles since the last lv1 title
    pub lv2_titles: usize,
    /// the number of maths blocks
    pub maths_blocks: usize,
    /// We need to populate katex blocks after the script inclusion
    katex_buf: String,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            indentation: 0,
            tags_queue: Vec::new(),
            should_include_prism: false,
            should_include_katex: false,
            lv1_titles: 0,
            lv2_titles: 0,
            maths_blocks: 0,
            katex_buf: String::new(),
        }
    }

    /// Prints the resulting buffer to stdout
    pub fn into_result(self) -> String {
        if !self.tags_queue.is_empty() {
            panic!("Unclosed tags !!");
        }

        self.buf
    }

    /// Starts an orphan tag
    pub fn start_orphan_tag(&mut self, tag_name: &str, attributes: &[(&str, &str)]) {
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
            // Chromium had a long-standing bug with text rendering when elements have small em values.
            // This is why we have to increase minRuleThickness to have the proper output in wkhtmltopdf.
            "katex.render('{}', document.getElementById('{}'), {{ throwOnError: false, minRuleThickness: 0.06 }});",
            escape_latex(content),
            el_id
        );
        self.katex_buf.push('\n');
    }

    /// Returns the katex buffer referencing all html elements and their latex contents
    pub fn get_katex_content(&self) -> String {
        self.katex_buf.clone()
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