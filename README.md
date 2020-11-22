# Knot.s

Knot.s is a superset of Markdown allowing you to take notes efficently.  
It translates a Knot.s Markdown file to a self-contained HTML file, or to a PDF.

## Features

- Write Markdown and render to beautiful HTML / PDF
- LaTeX support
- HTML support
- Syntax highlighting for code blocks
- Works 100% offline
- Light and dark theme, respects your browser preference
- Responsive Design
- No third-party dependency besides the .dll file included

### Not supported yet (but easy to add :D)

- Links
- Images
- Tables
- Titles after `##`

### Planned

- Implement all of the above
- Colored titles

## Installation

### Windows

[Download here the latest build !](https://github.com/truelossless/Knots/releases/latest)

### Other OS

You'll have to download Wkhtmltopdf before, and I'm not sure about the linking process.  
Although if you're interested I can certainly make it work and bundle Ubuntu and MacOS executables as well.

## Usage

Create a new document with a `.md` extension in your favorite text editor. This way, you'll have syntax highlighting for Markdown. See below for syntax examples. Once you're finished, open your document with knots.exe to automatically create `yourfile.html` and `yourfile.pdf`. Prefer using the html file as the rendering is better.

Knot.s also has a command line, run `knots.exe --help` in a command prompt for more options.

## Syntax

See `examples/` for examples !

### Text modifiers

- Surround text with `*` for _italic_:
- Surround text with `**` for **bold**
- Surround text with `` ` `` to write `inline code`
- Surround text with `$` to write inline LaTeX

### Titles

Start a new title with `#`.
Titles will automatically be numerotated with roman numerals.

Subtitles are started with `##`.
Subtitles will automatically be numerotated with alphabetic numerals.

### Maths

Start a LaTeX block with `$$`. Example:

```
$$
\sum_{k=0}^{n}
$$
```

### Code

Start a code block with ` ``` `. You can also specify the desired language right after. Example:

````
```js
function helloWorld() {
    console.log('Hello world !');
}
```⠀
````

### HTML

You can also write HTML/JS/CSS everywhere.

```
Some text
<script>
    alert('hello world');
</script>
Other text
```

### Metadata

At the **start** of a document, you can include some informations.

```
%title Hello everyone !
```

will set the document title to "Hello everyone !"

```
%author truelossless
%author anonymous
```

will set the authors to "truelossless" and "anonymous"

```
%license MIT
```

will set the license to MIT

## Under the hood

- Nom to parse the Markdown-ish syntax
- Wkhtmltopdf-rs to generate a pdf from html
- Normalize.css for cross-browser consistency
- PrismJS for code highlighting
- Katex for LaTeX rendering

## Why ?

I used to do something similar with Pandoc, but it had several limitations. LaTeX would not render correctly, you have to pass certain flags to have Markdown line breaks, using css isn't easy, there are issues with some versions of Wkhtmltopdf and so on.

## Known issues

- Line breaks can cause the parser to fail
- Font in code blocks is broken if you use LaTeX, in PDFs

## Contributing

I'm not a good CSS designer, so if you want to contribute to make the CSS more awesome or add other themes, I'll be very thanksfull !

Any other help regarding the code is also greatly appreciated.