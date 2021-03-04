# Knots

Knots is a superset of Markdown allowing you to take notes efficently.  
It translates a Knots Markdown file to a self-contained HTML file, or to a PDF.

![Knots image](https://i.imgur.com/8D29YAN.png)

## Features

- Write Markdown and render to beautiful HTML / PDF
- Automatic summary generated from your titles
- LaTeX support
- HTML support
- Mermaid diagrams support
- Syntax highlighting for code blocks
- Works 100% offline
- Light and dark theme according to your browser preference
- Responsive design
- No third-party dependencies

## Installation

[Download here the latest build !](https://github.com/truelossless/Knots/releases/latest)

## Usage

Create a new document with a `.md` extension in your favorite text editor. This way, you'll have syntax highlighting for Markdown. See below for syntax examples. Once you're finished, open your document with knots.exe to automatically create `yourfile.html` and `yourfile.pdf`. Prefer using the html file as the rendering is better.

Knots also has a command line, run `knots.exe --help` in a command prompt for more options.

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

### Diagrams

Start a Mermaid diagram like you would start a code block of the mermaid language, with ` ```mermaid `. You can find the full diagram reference at https://mermaid-js.github.io.

### Block Quotes

Start a quote with `>`. Example:

```
Mandela once said:
> Hello World !
```

### Boxes

Inform `?>`, warn `!>` or scare `x>` your readers with boxes. Example:

```
?> This is an info box
!> This is a warning box
x> This is an error box
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
- An headless chrome to generate a pdf from the html
- Normalize.css for cross-browser consistency
- PrismJS for code highlighting
- Katex for LaTeX rendering
- css.gg for the sexy icons
- Mermaid for the diagrams

## Why ?

I used to do something similar with Pandoc, but it had several limitations. LaTeX would not render correctly, you have to pass certain flags to have Markdown line breaks, using css isn't easy, there are issues with some versions of Wkhtmltopdf and so on.

## Contributing

I'm not a good CSS designer, so if you want to contribute to make the CSS more awesome or add other themes, I'll be very thankful !

Any other help regarding the code is also greatly appreciated.
