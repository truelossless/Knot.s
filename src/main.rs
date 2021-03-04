mod builder;
mod knots_objects;
mod parser;
mod prism_autoloader;
mod transpiler;
mod utils;

use std::{ffi::OsStr, fs, path::Path, process};

use gumdrop::Options;
use headless_chrome::{
    browser::default_executable, protocol::page::PrintToPdfOptions, Browser, FetcherOptions,
    LaunchOptionsBuilder,
};

#[derive(Debug, Options)]
struct MyOptions {
    #[options(free, help = "the Knots file to convert")]
    input: Vec<String>,

    #[options(
        no_long,
        help = "the output file (will only output a .pdf or a .html depending on the extension)"
    )]
    output: Option<String>,

    #[options(no_short, help = "don't create a summary")]
    no_summary: bool,

    #[options(
        no_short,
        help = "allow the download of a chrome copy to convert html to pdf, if no installation is found"
    )]
    allow_chrome_download: bool,

    #[options(help = "show Knots version")]
    version: bool,

    #[options(help = "show help message")]
    help: bool,
}

pub fn main() {
    let opts = MyOptions::parse_args_default_or_exit();

    if opts.version {
        println!(env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    if opts.input.len() != 1 {
        eprintln!("No file input");
        process::exit(1);
    }

    let pdf_output;
    let html_output;

    if let Some(output) = opts.output {
        let ext = Path::new(&output)
            .extension()
            .unwrap_or_else(|| OsStr::new("html"));

        // output a pdf
        if ext == "pdf" {
            pdf_output = Some(output);
            html_output = None;
        // output html
        } else {
            html_output = Some(output);
            pdf_output = None;
        }

    // no extension specified, generate an html file and a pdf file
    } else {
        let file_name = Path::new(&opts.input[0])
            .file_stem()
            .unwrap_or_else(|| {
                eprintln!("The input file has no name");
                process::exit(1);
            })
            .to_string_lossy();

        html_output = Some(format!("{}.html", file_name));
        pdf_output = Some(format!("{}.pdf", file_name));
    }

    let parse_result = parser::parse(&opts.input[0]).unwrap_or_else(|_| {
        eprintln!("Unable to read input file");
        process::exit(1);
    });

    let user_opts = transpiler::KnotsOptions {
        summary: !opts.no_summary,
    };

    let result = transpiler::transpile(parse_result, user_opts);

    if let Some(html) = &html_output {
        fs::write(html, &result).unwrap_or_else(|_| {
            eprintln!("Unable to write to html file");
            process::exit(1);
        });
    }

    let pdf = match pdf_output {
        Some(pdf) => pdf,
        _ => return,
    };

    // try to get an installed chrome executable
    let default_exe = default_executable().ok();

    let options = LaunchOptionsBuilder::default()
        .path(default_exe)
        .fetcher_options(FetcherOptions::default().with_allow_download(opts.allow_chrome_download))
        .build()
        .unwrap();

    let browser = Browser::new(options).unwrap_or_else(|_| {
            eprintln!("Couldn't find a chrome / chromium installation on this system. This is needed for the HTML to PDF conversion. To automatically install one, use the --allow-chrome-download command line argument.");
            process::exit(1);
        });
    let tab = browser.wait_for_initial_tab().unwrap();

    let is_tmp_html_required = html_output.is_none();
    let tmp_html = html_output.unwrap_or_else(|| {
        fs::write("tmp.html", &result).unwrap_or_else(|_| {
            eprintln!("Unable to write to html temp file");
            process::exit(1);
        });
        "tmp.html".to_owned()
    });
    let full_html_path = Path::new(&tmp_html).canonicalize().unwrap();

    tab.navigate_to(&format!("file://{}", full_html_path.to_string_lossy()))
        .unwrap()
        .wait_until_navigated()
        .unwrap();

    let pdf_content = tab
        .print_to_pdf(Some(PrintToPdfOptions {
            display_header_footer: Some(false),
            print_background: Some(true),
            margin_left: Some(0.),
            margin_right: Some(0.),
            margin_top: None,
            margin_bottom: None,
            landscape: None,
            scale: None,
            paper_width: None,
            paper_height: None,
            page_ranges: None,
            ignore_invalid_page_ranges: None,
            header_template: None,
            footer_template: None,
            prefer_css_page_size: None,
        }))
        .unwrap();

    fs::write(pdf, pdf_content).unwrap_or_else(|_| eprintln!("Failed to save the pdf"));

    if is_tmp_html_required {
        fs::remove_file(full_html_path)
            .unwrap_or_else(|_| eprintln!("Failed to remove the temporary html file"));
    }
}
