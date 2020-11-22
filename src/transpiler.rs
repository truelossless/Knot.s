use super::builder::Builder;
use super::parser::ParseResult;

/// Transpiles to an HTML page our Knot.s objects
pub fn transpile(parse_result: ParseResult) -> String {
    let mut builder = Builder::new();

    builder.start_orphan_tag("!DOCTYPE html", &[]);
    builder.start_tag("html", &[]);

    builder.start_tag("head", &[]);
    builder.start_orphan_tag("meta", &[("charset", "utf-8")]);

    builder.start_tag("title", &[]);
    builder.write_content(&parse_result.document_title);
    builder.end_tag(); // </title>

    // normalize css
    builder.start_tag("style", &[]);
    builder.write_content(include_str!("../css/normalize.css"));
    builder.end_tag();

    // our own css
    builder.start_tag("style", &[]);
    builder.write_content(include_str!("../css/style.css"));
    builder.end_tag(); // </style>

    builder.end_tag(); // </head>

    builder.start_tag("body", &[]);
    builder.start_tag("header", &[]);
    // document title
    builder.start_tag("h1", &[("id", "doctitle")]);
    builder.write_content(&parse_result.document_title);
    builder.end_tag();

    // document authors
    if !parse_result.document_authors.is_empty() {
        builder.start_tag("div", &[("class", "docinfo")]);

        // svg author icon
        builder.write_content(include_str!("../icons/profile.svg"));

        let mut authors_buf = parse_result.document_authors[0].to_owned();
        for author in parse_result.document_authors.iter().skip(1) {
            authors_buf = format!("{}, {}", authors_buf, author);
        }
        builder.write_content(&authors_buf);
        builder.end_tag(); // </div>
    }
    builder.end_tag(); // </header>

    // put everything in a container
    builder.start_tag("div", &[("class", "container")]);
    builder.write_knots_object(parse_result.root_object);
    builder.end_tag(); // </div>

    // NOTE: for some reason including katex breaks the font on code blocks in PDFs.
    // I have no idea why, this behavior is not repoducible on any browser outside the one
    // used by Wkhtmltopdf.
    // I've tried several workarounds without success.
    if builder.should_include_katex {
        builder.start_tag("style", &[]);
        builder.write_content(include_str!("../css/katex.css"));
        builder.end_tag(); // </style>

        builder.start_tag("script", &[]);
        builder.write_content(include_str!("../js/katex.js"));

        builder.write_content(&builder.get_katex_content());
        builder.end_tag(); // </script>
    }

    // if we have code blocks then we should include prism.
    if builder.should_include_prism {
        // style elements are usually added in the head section,
        // but we need to call `builder.write_knots_object()` before,
        // which will determine `builder.should_include_prism`.
        builder.start_tag("style", &[]);
        builder.write_content(include_str!("../css/prism.css"));
        builder.end_tag(); // </style>

        builder.start_tag("script", &[]);
        builder.write_content(include_str!("../js/prism.js"));
        builder.end_tag(); // </script>
    }

    // document license
    if let Some(license) = parse_result.document_license {
        builder.start_tag("div", &[("class", "docinfo discreet"), ("id", "license")]);
        builder.start_orphan_tag("hr", &[]);
        builder.write_content(include_str!("../icons/ereader.svg"));
        builder.write_content(&format!(
            "This work is available under the {} license",
            license
        ));
        builder.end_tag() // </div>
    }

    builder.end_tag(); // </body>
    builder.end_tag(); // </html>

    builder.into_result()
}