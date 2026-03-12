#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use super::{ArgumentCompletion, CommandOutput, OutputSection};

pub(crate) struct ParamDoc {
    pub name: &'static str,
    pub type_hint: &'static str,
    pub description: &'static str,
    pub default: Option<&'static str>,
}

pub(crate) struct FunctionDoc {
    pub name: &'static str,
    pub category: &'static str,
    pub signature: &'static str,
    pub description: &'static str,
    pub parameters: &'static [ParamDoc],
    pub example: &'static str,
}

static DOCS: &[FunctionDoc] = &[
    // ── Text ─────────────────────────────────────────────────────────────────
    FunctionDoc {
        name: "text",
        category: "text",
        signature: "#text(font, size, weight, style, stretch, fill, tracking, spacing, baseline, overhang, ..body) -> content",
        description: "Customizes the look and layout of text.",
        parameters: &[
            ParamDoc { name: "font", type_hint: "str | array", description: "A font family name or priority list.", default: Some("\"Linux Libertine\"") },
            ParamDoc { name: "size", type_hint: "length", description: "The size of the glyphs.", default: Some("11pt") },
            ParamDoc { name: "weight", type_hint: "int | str", description: "The font weight (e.g. \"bold\", 700).", default: Some("\"regular\"") },
            ParamDoc { name: "fill", type_hint: "color | gradient", description: "The text color.", default: Some("black") },
            ParamDoc { name: "body", type_hint: "content", description: "The text content.", default: None },
        ],
        example: "#text(font: \"New Computer Modern\", size: 12pt, fill: blue)[Hello, Typst!]",
    },
    FunctionDoc {
        name: "emph",
        category: "text",
        signature: "#emph(body) -> content",
        description: "Emphasizes content by italicizing it.",
        parameters: &[
            ParamDoc { name: "body", type_hint: "content", description: "The content to emphasize.", default: None },
        ],
        example: "This is _emphasized_ or #emph[emphasized].",
    },
    FunctionDoc {
        name: "strong",
        category: "text",
        signature: "#strong(delta, body) -> content",
        description: "Strongly emphasizes content by increasing font weight.",
        parameters: &[
            ParamDoc { name: "delta", type_hint: "int", description: "The delta to apply to the current font weight.", default: Some("300") },
            ParamDoc { name: "body", type_hint: "content", description: "The content to strongly emphasize.", default: None },
        ],
        example: "This is *bold* or #strong[bold].",
    },
    FunctionDoc {
        name: "underline",
        category: "text",
        signature: "#underline(stroke, offset, evade, background, body) -> content",
        description: "Underlines text.",
        parameters: &[
            ParamDoc { name: "stroke", type_hint: "length | stroke", description: "How to stroke the line.", default: Some("auto") },
            ParamDoc { name: "offset", type_hint: "length", description: "Position of the line relative to baseline.", default: Some("auto") },
            ParamDoc { name: "body", type_hint: "content", description: "The content to underline.", default: None },
        ],
        example: "#underline[Underlined text]",
    },
    FunctionDoc {
        name: "strike",
        category: "text",
        signature: "#strike(stroke, offset, extent, background, body) -> content",
        description: "Strikes through text.",
        parameters: &[
            ParamDoc { name: "stroke", type_hint: "length | stroke", description: "How to stroke the line.", default: Some("auto") },
            ParamDoc { name: "body", type_hint: "content", description: "The content to strike.", default: None },
        ],
        example: "#strike[Struck through]",
    },
    FunctionDoc {
        name: "overline",
        category: "text",
        signature: "#overline(stroke, offset, evade, background, body) -> content",
        description: "Adds a line over text.",
        parameters: &[
            ParamDoc { name: "stroke", type_hint: "length | stroke", description: "How to stroke the line.", default: Some("auto") },
            ParamDoc { name: "body", type_hint: "content", description: "The content to overline.", default: None },
        ],
        example: "#overline[Overlined text]",
    },
    FunctionDoc {
        name: "highlight",
        category: "text",
        signature: "#highlight(fill, top-edge, bottom-edge, extent, radius, body) -> content",
        description: "Highlights text with a background color.",
        parameters: &[
            ParamDoc { name: "fill", type_hint: "color", description: "The color to highlight with.", default: Some("yellow") },
            ParamDoc { name: "body", type_hint: "content", description: "The content to highlight.", default: None },
        ],
        example: "#highlight(fill: yellow)[Highlighted text]",
    },
    FunctionDoc {
        name: "raw",
        category: "text",
        signature: "#raw(text, lang, block, tab-size, align) -> content",
        description: "Displays text verbatim in a monospace font. Use backtick syntax for inline or fenced code blocks.",
        parameters: &[
            ParamDoc { name: "text", type_hint: "str", description: "The raw text to display.", default: None },
            ParamDoc { name: "lang", type_hint: "str | none", description: "The language for syntax highlighting.", default: Some("none") },
            ParamDoc { name: "block", type_hint: "bool", description: "Whether to display as a block.", default: Some("false") },
            ParamDoc { name: "tab-size", type_hint: "int", description: "Spaces per tab.", default: Some("2") },
        ],
        example: "`inline code` or ```rust\nfn main() {}\n```",
    },
    FunctionDoc {
        name: "smallcaps",
        category: "text",
        signature: "#smallcaps(body) -> content",
        description: "Displays text in small capitals.",
        parameters: &[
            ParamDoc { name: "body", type_hint: "content", description: "The content to display in small caps.", default: None },
        ],
        example: "#smallcaps[Small Capitals]",
    },
    FunctionDoc {
        name: "sub",
        category: "text",
        signature: "#sub(typographic, baseline, size, body) -> content",
        description: "Renders text in subscript.",
        parameters: &[
            ParamDoc { name: "body", type_hint: "content", description: "The subscript content.", default: None },
        ],
        example: "H#sub[2]O",
    },
    FunctionDoc {
        name: "super",
        category: "text",
        signature: "#super(typographic, baseline, size, body) -> content",
        description: "Renders text in superscript.",
        parameters: &[
            ParamDoc { name: "body", type_hint: "content", description: "The superscript content.", default: None },
        ],
        example: "x#super[2] + y#super[2]",
    },
    // ── Layout ────────────────────────────────────────────────────────────────
    FunctionDoc {
        name: "page",
        category: "layout",
        signature: "#page(paper, width, height, flipped, margin, columns, fill, numbering, header, footer, ..body) -> content",
        description: "Customizes the page settings. Used with #set to apply globally.",
        parameters: &[
            ParamDoc { name: "paper", type_hint: "str", description: "Paper format (e.g. \"a4\", \"us-letter\").", default: Some("\"a4\"") },
            ParamDoc { name: "width", type_hint: "length", description: "Page width.", default: Some("auto") },
            ParamDoc { name: "height", type_hint: "length | auto", description: "Page height.", default: Some("auto") },
            ParamDoc { name: "margin", type_hint: "length | dict", description: "Page margins.", default: Some("auto") },
            ParamDoc { name: "columns", type_hint: "int", description: "Number of columns.", default: Some("1") },
            ParamDoc { name: "fill", type_hint: "color | none", description: "Page background fill.", default: Some("none") },
            ParamDoc { name: "numbering", type_hint: "str | function | none", description: "Page number format.", default: Some("none") },
        ],
        example: "#set page(paper: \"a4\", margin: (x: 2cm, y: 3cm), numbering: \"1\")",
    },
    FunctionDoc {
        name: "pagebreak",
        category: "layout",
        signature: "#pagebreak(weak, to) -> content",
        description: "Forces a page break.",
        parameters: &[
            ParamDoc { name: "weak", type_hint: "bool", description: "If true, skipped if page is already empty.", default: Some("false") },
            ParamDoc { name: "to", type_hint: "str | none", description: "\"odd\" or \"even\" to break to a specific page parity.", default: Some("none") },
        ],
        example: "#pagebreak()",
    },
    FunctionDoc {
        name: "par",
        category: "layout",
        signature: "#par(leading, justify, linebreaks, first-line-indent, hanging-indent, body) -> content",
        description: "Configures paragraph settings. Also sets the layout of implicit paragraphs.",
        parameters: &[
            ParamDoc { name: "leading", type_hint: "length", description: "Space between lines.", default: Some("0.65em") },
            ParamDoc { name: "justify", type_hint: "bool", description: "Whether to justify text.", default: Some("false") },
            ParamDoc { name: "first-line-indent", type_hint: "length", description: "Indent for first line.", default: Some("0pt") },
            ParamDoc { name: "body", type_hint: "content", description: "Paragraph content.", default: None },
        ],
        example: "#set par(justify: true, leading: 0.8em)",
    },
    FunctionDoc {
        name: "block",
        category: "layout",
        signature: "#block(width, height, breakable, fill, stroke, radius, inset, outset, spacing, above, below, clip, sticky, body) -> content",
        description: "A block-level container with optional visual styling.",
        parameters: &[
            ParamDoc { name: "width", type_hint: "length | auto | fraction", description: "Block width.", default: Some("auto") },
            ParamDoc { name: "height", type_hint: "length | auto | fraction", description: "Block height.", default: Some("auto") },
            ParamDoc { name: "fill", type_hint: "color | none", description: "Background fill.", default: Some("none") },
            ParamDoc { name: "stroke", type_hint: "stroke | none | dict", description: "Border stroke.", default: Some("none") },
            ParamDoc { name: "radius", type_hint: "length | dict", description: "Border radius.", default: Some("0pt") },
            ParamDoc { name: "inset", type_hint: "length | dict", description: "Inner padding.", default: Some("0pt") },
            ParamDoc { name: "body", type_hint: "content | none", description: "Block content.", default: None },
        ],
        example: "#block(fill: luma(230), inset: 8pt, radius: 4pt)[Content in a box]",
    },
    FunctionDoc {
        name: "box",
        category: "layout",
        signature: "#box(width, height, baseline, fill, stroke, radius, inset, outset, clip, body) -> content",
        description: "An inline-level container.",
        parameters: &[
            ParamDoc { name: "width", type_hint: "length | auto | fraction", description: "Box width.", default: Some("auto") },
            ParamDoc { name: "fill", type_hint: "color | none", description: "Background fill.", default: Some("none") },
            ParamDoc { name: "inset", type_hint: "length | dict", description: "Inner padding.", default: Some("0pt") },
            ParamDoc { name: "body", type_hint: "content | none", description: "Box content.", default: None },
        ],
        example: "#box(fill: aqua, inset: 2pt)[inline box]",
    },
    FunctionDoc {
        name: "stack",
        category: "layout",
        signature: "#stack(dir, spacing, ..children) -> content",
        description: "Arranges content and spacing along an axis.",
        parameters: &[
            ParamDoc { name: "dir", type_hint: "direction", description: "Stacking direction.", default: Some("ttb") },
            ParamDoc { name: "spacing", type_hint: "length | fraction | none", description: "Default spacing between children.", default: Some("none") },
            ParamDoc { name: "children", type_hint: "content | length | fraction", description: "Content or spacing to stack.", default: None },
        ],
        example: "#stack(dir: ltr, spacing: 1em)[A][B][C]",
    },
    FunctionDoc {
        name: "grid",
        category: "layout",
        signature: "#grid(columns, rows, gutter, column-gutter, row-gutter, fill, align, stroke, inset, ..children) -> content",
        description: "Arranges content in a grid.",
        parameters: &[
            ParamDoc { name: "columns", type_hint: "int | array | auto", description: "Column widths or count.", default: Some("auto") },
            ParamDoc { name: "rows", type_hint: "int | array | auto", description: "Row heights or count.", default: Some("auto") },
            ParamDoc { name: "gutter", type_hint: "length | array", description: "Spacing between rows and columns.", default: Some("0pt") },
            ParamDoc { name: "fill", type_hint: "color | none | function", description: "Cell background fill.", default: Some("none") },
            ParamDoc { name: "children", type_hint: "content", description: "Grid cells.", default: None },
        ],
        example: "#grid(columns: (1fr, 1fr), gutter: 1em)[Left][Right]",
    },
    FunctionDoc {
        name: "columns",
        category: "layout",
        signature: "#columns(count, gutter, body) -> content",
        description: "Separates a region into multiple columns.",
        parameters: &[
            ParamDoc { name: "count", type_hint: "int", description: "Number of columns.", default: Some("2") },
            ParamDoc { name: "gutter", type_hint: "length", description: "Space between columns.", default: Some("4%") },
            ParamDoc { name: "body", type_hint: "content", description: "Content to lay out in columns.", default: None },
        ],
        example: "#columns(2, gutter: 2em)[Column content here...]",
    },
    FunctionDoc {
        name: "align",
        category: "layout",
        signature: "#align(alignment, body) -> content",
        description: "Aligns content horizontally and vertically.",
        parameters: &[
            ParamDoc { name: "alignment", type_hint: "alignment", description: "Alignment (left, right, center, top, bottom, horizon, etc.).", default: Some("start + top") },
            ParamDoc { name: "body", type_hint: "content", description: "Content to align.", default: None },
        ],
        example: "#align(center)[Centered text]\n#align(right)[Right-aligned]",
    },
    FunctionDoc {
        name: "pad",
        category: "layout",
        signature: "#pad(x, y, top, right, bottom, left, rest, body) -> content",
        description: "Adds padding around content.",
        parameters: &[
            ParamDoc { name: "x", type_hint: "length | relative | fraction", description: "Horizontal padding shorthand.", default: Some("0pt") },
            ParamDoc { name: "y", type_hint: "length | relative | fraction", description: "Vertical padding shorthand.", default: Some("0pt") },
            ParamDoc { name: "body", type_hint: "content", description: "The padded content.", default: None },
        ],
        example: "#pad(x: 2em, y: 1em)[Padded content]",
    },
    FunctionDoc {
        name: "place",
        category: "layout",
        signature: "#place(alignment, dx, dy, float, clearance, body) -> content",
        description: "Places content at an absolute position.",
        parameters: &[
            ParamDoc { name: "alignment", type_hint: "alignment | auto", description: "Where to place relative to.", default: Some("start + top") },
            ParamDoc { name: "dx", type_hint: "length | relative", description: "Horizontal offset.", default: Some("0%") },
            ParamDoc { name: "dy", type_hint: "length | relative", description: "Vertical offset.", default: Some("0%") },
            ParamDoc { name: "float", type_hint: "bool", description: "Whether to float to top or bottom of page.", default: Some("false") },
            ParamDoc { name: "body", type_hint: "content", description: "Content to place.", default: None },
        ],
        example: "#place(top + right, dx: -1cm)[Watermark]",
    },
    FunctionDoc {
        name: "colbreak",
        category: "layout",
        signature: "#colbreak(weak) -> content",
        description: "Forces a column break.",
        parameters: &[
            ParamDoc { name: "weak", type_hint: "bool", description: "If true, skipped when at the start of a column.", default: Some("false") },
        ],
        example: "#colbreak()",
    },
    FunctionDoc {
        name: "v",
        category: "layout",
        signature: "#v(amount, weak) -> content",
        description: "Inserts vertical spacing.",
        parameters: &[
            ParamDoc { name: "amount", type_hint: "length | fraction", description: "How much vertical space to add.", default: None },
            ParamDoc { name: "weak", type_hint: "bool", description: "Whether to collapse adjacent spaces.", default: Some("false") },
        ],
        example: "#v(1em) // Insert 1em vertical space",
    },
    FunctionDoc {
        name: "h",
        category: "layout",
        signature: "#h(amount, weak) -> content",
        description: "Inserts horizontal spacing.",
        parameters: &[
            ParamDoc { name: "amount", type_hint: "length | fraction", description: "How much horizontal space to add.", default: None },
            ParamDoc { name: "weak", type_hint: "bool", description: "Whether to collapse adjacent spaces.", default: Some("false") },
        ],
        example: "#h(1em) // Insert 1em horizontal space",
    },
    // ── Structure ─────────────────────────────────────────────────────────────
    FunctionDoc {
        name: "heading",
        category: "structure",
        signature: "#heading(level, depth, offset, numbering, supplement, outlined, bookmarked, hanging-indent, body) -> content",
        description: "A section heading. Can also be written with = syntax.",
        parameters: &[
            ParamDoc { name: "level", type_hint: "int", description: "Heading level (1-6).", default: Some("1") },
            ParamDoc { name: "numbering", type_hint: "str | function | none", description: "Numbering format.", default: Some("none") },
            ParamDoc { name: "outlined", type_hint: "bool", description: "Whether to include in outline.", default: Some("true") },
            ParamDoc { name: "body", type_hint: "content", description: "Heading text.", default: None },
        ],
        example: "= Top-level Heading\n== Second-level\n#heading(level: 3)[Third]",
    },
    FunctionDoc {
        name: "list",
        category: "structure",
        signature: "#list(tight, marker, indent, body-indent, spacing, ..children) -> content",
        description: "A bullet list. Items can also be written with - syntax.",
        parameters: &[
            ParamDoc { name: "tight", type_hint: "bool", description: "Reduce spacing between items.", default: Some("true") },
            ParamDoc { name: "marker", type_hint: "content | array | function", description: "Bullet marker.", default: Some("•") },
            ParamDoc { name: "children", type_hint: "content", description: "List items.", default: None },
        ],
        example: "- First item\n- Second item\n#list[Item A][Item B]",
    },
    FunctionDoc {
        name: "enum",
        category: "structure",
        signature: "#enum(tight, numbering, start, full, indent, body-indent, spacing, ..children) -> content",
        description: "A numbered list. Items can also be written with + syntax.",
        parameters: &[
            ParamDoc { name: "tight", type_hint: "bool", description: "Reduce spacing between items.", default: Some("true") },
            ParamDoc { name: "numbering", type_hint: "str | function", description: "Number format.", default: Some("\"1.\"") },
            ParamDoc { name: "start", type_hint: "int", description: "Starting number.", default: Some("1") },
            ParamDoc { name: "children", type_hint: "content", description: "Numbered list items.", default: None },
        ],
        example: "+ First\n+ Second\n#enum(start: 3)[Third][Fourth]",
    },
    FunctionDoc {
        name: "terms",
        category: "structure",
        signature: "#terms(tight, separator, indent, hanging-indent, spacing, ..children) -> content",
        description: "A term list. Items can also be written with / term: description syntax.",
        parameters: &[
            ParamDoc { name: "tight", type_hint: "bool", description: "Reduce spacing.", default: Some("true") },
            ParamDoc { name: "separator", type_hint: "content", description: "Separator between term and description.", default: Some("\": \"") },
            ParamDoc { name: "children", type_hint: "content", description: "Term list items.", default: None },
        ],
        example: "/ Term: Description\n/ Other: Another definition",
    },
    FunctionDoc {
        name: "table",
        category: "structure",
        signature: "#table(columns, rows, gutter, fill, align, stroke, inset, ..children) -> content",
        description: "A table with optional borders and fills.",
        parameters: &[
            ParamDoc { name: "columns", type_hint: "int | array | auto", description: "Column widths or count.", default: Some("auto") },
            ParamDoc { name: "rows", type_hint: "int | array | auto", description: "Row heights or count.", default: Some("auto") },
            ParamDoc { name: "fill", type_hint: "color | none | function", description: "Cell background.", default: Some("none") },
            ParamDoc { name: "stroke", type_hint: "stroke | none | dict | function", description: "Cell border.", default: Some("1pt") },
            ParamDoc { name: "inset", type_hint: "length | dict | function", description: "Cell padding.", default: Some("0.5em") },
            ParamDoc { name: "children", type_hint: "content | table.cell | table.header", description: "Table cells.", default: None },
        ],
        example: "#table(\n  columns: 2,\n  [Name], [Value],\n  [Alpha], [1],\n  [Beta], [2],\n)",
    },
    FunctionDoc {
        name: "figure",
        category: "structure",
        signature: "#figure(caption, kind, supplement, numbering, gap, outlined, body) -> content",
        description: "A figure with an optional caption and numbering.",
        parameters: &[
            ParamDoc { name: "body", type_hint: "content", description: "The figure content.", default: None },
            ParamDoc { name: "caption", type_hint: "content | none", description: "The figure caption.", default: Some("none") },
            ParamDoc { name: "kind", type_hint: "str | function | auto", description: "Type of figure (image, table, etc.).", default: Some("auto") },
            ParamDoc { name: "supplement", type_hint: "content | function | auto | none", description: "Supplement like \"Figure\".", default: Some("auto") },
            ParamDoc { name: "numbering", type_hint: "str | function | none", description: "Number format.", default: Some("\"1\"") },
        ],
        example: "#figure(\n  image(\"plot.png\"),\n  caption: [Result of experiment]\n)",
    },
    FunctionDoc {
        name: "outline",
        category: "structure",
        signature: "#outline(title, target, depth, indent, fill) -> content",
        description: "A table of contents or outline.",
        parameters: &[
            ParamDoc { name: "title", type_hint: "content | auto | none", description: "Outline title.", default: Some("auto") },
            ParamDoc { name: "target", type_hint: "label | selector | location | function", description: "Elements to include.", default: Some("heading") },
            ParamDoc { name: "depth", type_hint: "int | none", description: "Maximum heading depth.", default: Some("none") },
            ParamDoc { name: "indent", type_hint: "bool | length | function | auto", description: "Indentation for nested entries.", default: Some("none") },
        ],
        example: "#outline(title: \"Contents\", depth: 3)",
    },
    FunctionDoc {
        name: "footnote",
        category: "structure",
        signature: "#footnote(numbering, body) -> content",
        description: "A footnote.",
        parameters: &[
            ParamDoc { name: "numbering", type_hint: "str | function", description: "Number format.", default: Some("\"1\"") },
            ParamDoc { name: "body", type_hint: "content | label", description: "Footnote content or reference.", default: None },
        ],
        example: "Text with a footnote.#footnote[This is the footnote.]",
    },
    FunctionDoc {
        name: "quote",
        category: "structure",
        signature: "#quote(block, quotes, attribution, body) -> content",
        description: "Displays a quotation.",
        parameters: &[
            ParamDoc { name: "block", type_hint: "bool", description: "Display as block quote.", default: Some("false") },
            ParamDoc { name: "quotes", type_hint: "bool | auto", description: "Whether to add quotation marks.", default: Some("auto") },
            ParamDoc { name: "attribution", type_hint: "content | label | none", description: "Attribution source.", default: Some("none") },
            ParamDoc { name: "body", type_hint: "content", description: "The quoted text.", default: None },
        ],
        example: "#quote(block: true, attribution: [Author])[Quoted text here.]",
    },
    FunctionDoc {
        name: "bibliography",
        category: "structure",
        signature: "#bibliography(path, title, full, style) -> content",
        description: "Shows a bibliography from a BibTeX or Hayagriva file.",
        parameters: &[
            ParamDoc { name: "path", type_hint: "str | array", description: "Path to bibliography file(s).", default: None },
            ParamDoc { name: "title", type_hint: "content | auto | none", description: "Bibliography title.", default: Some("auto") },
            ParamDoc { name: "full", type_hint: "bool", description: "Include all entries, even uncited.", default: Some("false") },
            ParamDoc { name: "style", type_hint: "str", description: "Citation style (e.g. \"ieee\", \"apa\").", default: Some("\"ieee\"") },
        ],
        example: "#bibliography(\"refs.bib\", style: \"apa\")",
    },
    FunctionDoc {
        name: "ref",
        category: "structure",
        signature: "#ref(target, supplement) -> content",
        description: "References a labeled element and generates a link.",
        parameters: &[
            ParamDoc { name: "target", type_hint: "label", description: "The label to reference.", default: None },
            ParamDoc { name: "supplement", type_hint: "content | function | auto | none", description: "Supplement (e.g. \"Figure\").", default: Some("auto") },
        ],
        example: "As shown in @fig:plot...\n#figure(...) <fig:plot>",
    },
    // ── Media ─────────────────────────────────────────────────────────────────
    FunctionDoc {
        name: "image",
        category: "media",
        signature: "#image(path, format, width, height, alt, fit) -> content",
        description: "Displays an image file (PNG, JPEG, GIF, SVG, or WASM).",
        parameters: &[
            ParamDoc { name: "path", type_hint: "str", description: "Path to the image file.", default: None },
            ParamDoc { name: "width", type_hint: "length | auto", description: "Display width.", default: Some("auto") },
            ParamDoc { name: "height", type_hint: "length | auto", description: "Display height.", default: Some("auto") },
            ParamDoc { name: "alt", type_hint: "str | none", description: "Alt text for accessibility.", default: Some("none") },
            ParamDoc { name: "fit", type_hint: "str", description: "Fit mode: \"cover\", \"contain\", \"stretch\".", default: Some("\"cover\"") },
        ],
        example: "#image(\"logo.png\", width: 50%)",
    },
    FunctionDoc {
        name: "line",
        category: "media",
        signature: "#line(start, end, length, angle, stroke) -> content",
        description: "Draws a line.",
        parameters: &[
            ParamDoc { name: "start", type_hint: "array", description: "Start point (x, y).", default: Some("(0%, 0%)") },
            ParamDoc { name: "end", type_hint: "array | none", description: "End point (x, y).", default: Some("none") },
            ParamDoc { name: "length", type_hint: "length", description: "Line length when end is unset.", default: Some("0pt") },
            ParamDoc { name: "angle", type_hint: "angle", description: "Line angle when end is unset.", default: Some("0deg") },
            ParamDoc { name: "stroke", type_hint: "length | color | stroke", description: "Line style.", default: Some("1pt") },
        ],
        example: "#line(length: 100%, stroke: 1pt + gray)",
    },
    FunctionDoc {
        name: "rect",
        category: "media",
        signature: "#rect(width, height, fill, stroke, radius, inset, outset, body) -> content",
        description: "Draws a rectangle.",
        parameters: &[
            ParamDoc { name: "width", type_hint: "length | auto", description: "Rectangle width.", default: Some("auto") },
            ParamDoc { name: "height", type_hint: "length | auto", description: "Rectangle height.", default: Some("auto") },
            ParamDoc { name: "fill", type_hint: "color | none", description: "Fill color.", default: Some("none") },
            ParamDoc { name: "stroke", type_hint: "stroke | none | dict", description: "Border stroke.", default: Some("auto") },
            ParamDoc { name: "radius", type_hint: "length | dict", description: "Corner radius.", default: Some("0pt") },
            ParamDoc { name: "body", type_hint: "content | none", description: "Content inside.", default: Some("none") },
        ],
        example: "#rect(width: 100%, fill: blue.lighten(90%), radius: 4pt)[Inside rect]",
    },
    FunctionDoc {
        name: "circle",
        category: "media",
        signature: "#circle(radius, width, height, fill, stroke, inset, outset, body) -> content",
        description: "Draws a circle.",
        parameters: &[
            ParamDoc { name: "radius", type_hint: "length", description: "Circle radius.", default: Some("auto") },
            ParamDoc { name: "fill", type_hint: "color | none", description: "Fill color.", default: Some("none") },
            ParamDoc { name: "stroke", type_hint: "stroke | none", description: "Border stroke.", default: Some("auto") },
            ParamDoc { name: "body", type_hint: "content | none", description: "Content inside.", default: Some("none") },
        ],
        example: "#circle(radius: 1cm, fill: red)",
    },
    FunctionDoc {
        name: "ellipse",
        category: "media",
        signature: "#ellipse(width, height, fill, stroke, inset, outset, body) -> content",
        description: "Draws an ellipse.",
        parameters: &[
            ParamDoc { name: "width", type_hint: "length | auto", description: "Ellipse width.", default: Some("auto") },
            ParamDoc { name: "height", type_hint: "length | auto", description: "Ellipse height.", default: Some("auto") },
            ParamDoc { name: "fill", type_hint: "color | none", description: "Fill color.", default: Some("none") },
            ParamDoc { name: "stroke", type_hint: "stroke | none", description: "Border stroke.", default: Some("auto") },
            ParamDoc { name: "body", type_hint: "content | none", description: "Content inside.", default: Some("none") },
        ],
        example: "#ellipse(width: 3cm, height: 1cm, fill: green)",
    },
    FunctionDoc {
        name: "polygon",
        category: "media",
        signature: "#polygon(fill, stroke, ..vertices) -> content",
        description: "Draws a polygon through a list of vertices.",
        parameters: &[
            ParamDoc { name: "fill", type_hint: "color | none", description: "Fill color.", default: Some("none") },
            ParamDoc { name: "stroke", type_hint: "stroke | none", description: "Border stroke.", default: Some("auto") },
            ParamDoc { name: "vertices", type_hint: "array", description: "List of (x, y) points.", default: None },
        ],
        example: "#polygon(fill: blue, (0pt, 0pt), (30pt, 0pt), (15pt, 26pt))",
    },
    // ── Math ──────────────────────────────────────────────────────────────────
    FunctionDoc {
        name: "equation",
        category: "math",
        signature: "#equation(block, numbering, supplement, body) -> content",
        description: "A math equation. Use $...$ for inline, $ ... $ (with spaces) for block.",
        parameters: &[
            ParamDoc { name: "block", type_hint: "bool", description: "Display as block (centered) equation.", default: Some("false") },
            ParamDoc { name: "numbering", type_hint: "str | function | none", description: "Equation number format.", default: Some("none") },
            ParamDoc { name: "body", type_hint: "content", description: "The math content.", default: None },
        ],
        example: "$E = m c^2$ (inline)\n$ F = m a $ (block)",
    },
    // ── Meta ──────────────────────────────────────────────────────────────────
    FunctionDoc {
        name: "link",
        category: "meta",
        signature: "#link(dest, body) -> content",
        description: "Links to a URL or labeled element.",
        parameters: &[
            ParamDoc { name: "dest", type_hint: "str | label | location", description: "URL or target label.", default: None },
            ParamDoc { name: "body", type_hint: "content", description: "Link text (defaults to destination).", default: None },
        ],
        example: "#link(\"https://typst.app\")[Typst website]",
    },
    FunctionDoc {
        name: "cite",
        category: "meta",
        signature: "#cite(key, supplement, form, style) -> content",
        description: "Cites a bibliography entry.",
        parameters: &[
            ParamDoc { name: "key", type_hint: "label", description: "Bibliography key.", default: None },
            ParamDoc { name: "supplement", type_hint: "content | none", description: "Additional info (page, etc.).", default: Some("none") },
            ParamDoc { name: "form", type_hint: "str | none", description: "Citation form: \"normal\", \"prose\", \"full\", \"author\", \"year\".", default: Some("\"normal\"") },
        ],
        example: "@knuth1984 or #cite(<knuth1984>, supplement: [p. 42])",
    },
];

pub(crate) fn find_function(name: &str) -> Option<&'static FunctionDoc> {
    DOCS.iter().find(|d| d.name.eq_ignore_ascii_case(name))
}

pub(crate) fn search_functions(query: &str) -> Vec<&'static FunctionDoc> {
    let q = query.to_ascii_lowercase();
    DOCS.iter()
        .filter(|d| d.name.starts_with(q.as_str()) || d.name.contains(q.as_str()))
        .collect()
}

pub(crate) fn list_functions() -> &'static [FunctionDoc] {
    DOCS
}

pub(crate) fn complete_docs(args: &[String]) -> Vec<ArgumentCompletion> {
    let query = args.first().map(String::as_str).unwrap_or("");
    let funcs: Vec<&FunctionDoc> = if query.is_empty() {
        DOCS.iter().collect()
    } else {
        search_functions(query)
    };
    funcs
        .into_iter()
        .map(|f| ArgumentCompletion {
            label: f.name.to_string(),
            new_text: f.name.to_string(),
            run_command: true,
        })
        .collect()
}

pub(crate) fn run_docs(args: &[String]) -> Result<CommandOutput, String> {
    let name = args
        .first()
        .map(String::as_str)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "Usage: /typst-docs <function-name>".to_string())?;
    let doc = find_function(name)
        .ok_or_else(|| format!("Function '{}' not found. Use /typst-docs with no argument to see available functions.", name))?;
    Ok(format_function_doc(doc))
}

fn format_function_doc(doc: &FunctionDoc) -> CommandOutput {
    let mut text = String::new();

    text.push_str(&format!("# {}\n\n", doc.name));
    text.push_str(&format!("**Category:** {}\n\n", doc.category));
    text.push_str(&format!("## Description\n{}\n\n", doc.description));
    text.push_str(&format!("## Signature\n```typst\n{}\n```\n\n", doc.signature));

    if !doc.parameters.is_empty() {
        text.push_str("## Parameters\n\n");
        text.push_str("| Parameter | Type | Default | Description |\n");
        text.push_str("|-----------|------|---------|-------------|\n");
        for p in doc.parameters {
            let default = p.default.unwrap_or("-");
            text.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} |\n",
                p.name, p.type_hint, default, p.description
            ));
        }
        text.push('\n');
    }

    if !doc.example.is_empty() {
        text.push_str(&format!("## Example\n```typst\n{}\n```\n", doc.example));
    }

    let len = text.len();
    CommandOutput {
        sections: vec![OutputSection {
            range: 0..len,
            label: format!("typst-docs: {}", doc.name),
        }],
        text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- find_function ---

    #[test]
    fn find_function_returns_doc_when_name_matches_exactly() {
        let doc = find_function("text").unwrap();
        assert_eq!(doc.name, "text");
    }

    #[test]
    fn find_function_returns_doc_when_name_differs_in_case() {
        let doc = find_function("TEXT").unwrap();
        assert_eq!(doc.name, "text");
    }

    #[test]
    fn find_function_returns_none_when_name_not_found() {
        assert!(find_function("nonexistent_xyz").is_none());
    }

    // --- search_functions ---

    #[test]
    fn search_functions_returns_matches_when_prefix_matches() {
        let results = search_functions("he");
        assert!(results.iter().any(|d| d.name == "heading"));
    }

    #[test]
    fn search_functions_returns_empty_when_no_match() {
        let results = search_functions("zzz_no_match");
        assert!(results.is_empty());
    }

    // --- complete_docs ---

    #[test]
    fn complete_docs_returns_all_functions_when_no_args() {
        let completions = complete_docs(&[]);
        assert_eq!(completions.len(), list_functions().len());
    }

    #[test]
    fn complete_docs_returns_filtered_results_when_partial_arg() {
        let completions = complete_docs(&["im".to_string()]);
        assert!(completions.iter().any(|c| c.label == "image"));
    }

    #[test]
    fn complete_docs_sets_run_command_true() {
        let completions = complete_docs(&["text".to_string()]);
        assert!(completions.iter().all(|c| c.run_command));
    }

    // --- run_docs ---

    #[test]
    fn run_docs_returns_output_when_valid_function() {
        let output = run_docs(&["text".to_string()]).unwrap();
        assert!(output.text.contains("# text"));
        assert!(output.text.contains("## Signature"));
    }

    #[test]
    fn run_docs_returns_error_when_no_args() {
        assert!(run_docs(&[]).is_err());
    }

    #[test]
    fn run_docs_returns_error_when_function_not_found() {
        assert!(run_docs(&["nonexistent".to_string()]).is_err());
    }

    // --- format_function_doc ---

    #[test]
    fn format_function_doc_section_range_spans_entire_text() {
        let doc = find_function("image").unwrap();
        let output = format_function_doc(doc);
        assert_eq!(output.sections.len(), 1);
        assert_eq!(output.sections[0].range.end, output.text.len());
    }

    #[test]
    fn format_function_doc_includes_parameters_table_when_params_exist() {
        let doc = find_function("table").unwrap();
        let output = format_function_doc(doc);
        assert!(output.text.contains("## Parameters"));
        assert!(output.text.contains("| Parameter |"));
    }

    #[test]
    fn format_function_doc_includes_example_when_present() {
        let doc = find_function("image").unwrap();
        let output = format_function_doc(doc);
        assert!(output.text.contains("## Example"));
    }
}
