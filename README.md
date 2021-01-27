# Knot.s

Knot.s is a superset of Markdown allowing you to take notes efficently.  
It translates a Knot.s Markdown file to a self-contained HTML file, or to a PDF.

## Features

- Write Markdown and render to beautiful HTML / PDF
- Automatic summary generated from your titles
- LaTeX support
- HTML support
- Syntax highlighting for code blocks
- Works 100% offline
- Light and dark theme, respects your browser preference
- Responsive Design
- No third-party dependency besides the .dll file included

### Not supported yet (but easy to add :D)

- Tables

### Planned

- Implement all of the above
- Mermaid graphs
- smart syntax highlighting for code blocks

## Installation

### Windows

[Download here the latest build !](https://github.com/truelossless/Knot.s/releases/latest)

### Other OS

You'll have to download Wkhtmltopdf before, and I'm not sure about the linking process.  
Although if you're interested I can certainly make it work and bundle Ubuntu and MacOS executables as well.

## Usage

Create a new document with a `.md` extension in your favorite text editor. This way, you'll have syntax highlighting for Markdown. See below for syntax examples. Once you're finished, open your document with knots.exe to automatically create `yourfile.html` and `yourfile.pdf`. Prefer using the html file as the rendering is better.

Knot.s also has a command line, run `knots.exe --help` in a command prompt for more options.

## Syntax

See `examples/` for examples !

### Text modifiers

- Surround text with `*` or `_` for _italic_:
- Surround text with `**` or `__` for **bold**
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

### Block Quotes

Start a quote with `>`. Example:

```
Mandela once said:
> Hello World !
```

### Lists

Start a list with `-`. Example:

```
- Some item
    Description of the item
- Another item
    Description of this other item
- Last item
    - Nested list about this last item
    - another element in the nested list
```

To add a text or a nested list inside an item, ident with 4 spaces or 1 tab. 

### Boxes

Inform `?>`, warn `!>` or scare `x>` your readers with boxes. Example:

```
?> This is an info box
!> This is a warning box
x> This is an error box
```

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
- css.gg for the sexy icons

## Why ?

I used to do something similar with Pandoc, but it had several limitations. LaTeX would not render correctly, you have to pass certain flags to have Markdown line breaks, using css isn't easy, there are issues with some versions of Wkhtmltopdf and so on.

## Known issues

- The summary is at the end of the PDF
- Font in code blocks is broken if you use LaTeX, in PDFs

## Contributing

I'm not a good CSS designer, so if you want to contribute to make the CSS more awesome or add other themes, I'll be very thankful !

Any other help regarding the code is also greatly appreciated.
