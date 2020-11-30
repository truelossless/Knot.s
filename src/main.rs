mod builder;
mod knots_objects;
mod parser;
mod transpiler;
mod utils;

use std::{ffi::OsStr, fs::File, io::Write, path::Path};

use gumdrop::Options;
use wkhtmltopdf::{PdfApplication, Size};

#[derive(Debug, Options)]
struct MyOptions {
    #[options(free, help = "the Knot.s file to convert")]
    input: Vec<String>,

    #[options(
        no_long,
        help = "the output file (will only output a .pdf or a .html depending on the extension)"
    )]
    output: Option<String>,

    #[options(no_short, help = "don't create a summary")]
    no_summary: bool,

    #[options(help = "show knots version")]
    version: bool,

    #[options(help = "show help message")]
    help: bool,
}

pub fn main() {
    let opts = MyOptions::parse_args_default_or_exit();

    if opts.input.len() != 1 {
        eprintln!("No file input");
        std::process::exit(1);
    }

    if opts.version {
        println!(env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    let pdf_output;
    let html_output;

    if let Some(output) = opts.output {
        let ext = Path::new(&output)
            .extension()
            .unwrap_or_else(|| OsStr::new("html"));

        if ext == "pdf" {
            pdf_output = Some(output);
            html_output = None;
        } else {
            html_output = Some(output);
            pdf_output = None;
        }
    } else {
        let file_name = Path::new(&opts.input[0])
            .file_stem()
            .expect("The input file has no name")
            .to_string_lossy();

        html_output = Some(format!("{}.html", file_name));
        pdf_output = Some(format!("{}.pdf", file_name));
    }

    let parse_result = parser::parse(&opts.input[0]).expect("Unable to read input file");
    let doc_title = parse_result.document_title.clone();

    let user_opts = transpiler::KnotsOptions {
        summary: !opts.no_summary,
    };

    let result = transpiler::transpile(parse_result, user_opts);

    if let Some(html) = html_output {
        let mut file = File::create(html).expect("Unable to create html file");
        file.write_all(result.as_bytes())
            .expect("Unable to write to html file");
    }

    if let Some(pdf) = pdf_output {
        let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");
        let mut pdfout = pdf_app
            .builder()
            .margin(Size::Millimeters(0))
            .title(&doc_title)
            .build_from_html(&result)
            .expect("failed to build pdf");

        pdfout.save(pdf).expect("failed to save pdf");
    }
}
