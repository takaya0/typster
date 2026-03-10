; Based on https://github.com/WeetHet/typst.zed and
; https://github.com/uben0/tree-sitter-typst/blob/main/editors/helix/queries/highlights.scm

; CONTROL KEYWORDS
(let "let" @keyword.storage.type)
(branch ["if" "else"] @keyword.control.conditional)
(while "while" @keyword.control.repeat)
(for ["for" "in"] @keyword.control.repeat)
(import "import" @keyword.control.import)
(as "as" @keyword.operator)
(include "include" @keyword.control.import)
(show "show" @keyword.control)
(set "set" @keyword.control)
(return "return" @keyword.control)
(flow ["break" "continue"] @keyword.control)

; OPERATORS
(in ["in" "not"] @keyword.operator)
(context "context" @keyword.control)
(and "and" @keyword.operator)
(or "or" @keyword.operator)
(not "not" @keyword.operator)
(sign ["+" "-"] @operator)
(add "+" @operator)
(sub "-" @operator)
(mul "*" @operator)
(div "/" @operator)
(cmp ["==" "<=" ">=" "!=" "<" ">"] @operator)
(fraction "/" @operator)
(fac "!" @operator)
(attach ["^" "_"] @operator)
(wildcard) @operator

; VALUES
(ident) @variable
(number) @number
(string) @string
(bool) @boolean
(none) @constant.builtin
(auto) @constant.builtin

; RAW BLOCKS
(raw_blck
  "```" @punctuation.delimiter
  (blob) @text.literal
)
(raw_blck lang: (ident) @tag)
(raw_span
  "`" @punctuation.delimiter
  (blob) @text.literal
)

; LABELS AND REFERENCES
(label) @tag
(ref) @tag

; CONTENT
(content ["[" "]"] @operator)

; FUNCTIONS
(formula (ident) @function.method)
(attach (ident) @function.method)
(formula (field (ident) @function.method))

(tagged field: (ident) @tag)
(field field: (ident) @tag)

(call item: (ident) @function)
(call item: (field field: (ident) @function.method))

; MARKUP
(item "-" @punctuation.list_marker)
(term ["/" ":"] @punctuation.list_marker)
(heading) @title
(url) @tag
(emph) @emphasis
(strong) @emphasis.strong
(symbol) @operator
(shorthand) @constant.builtin
(quote) @markup.quote
(align) @operator
(linebreak) @constant.builtin

; MATH
(math "$" @operator)
"#" @operator
"end" @operator

; PUNCTUATION
(escape) @constant.character.escape
["(" ")" "{" "}"] @punctuation.bracket
["," ";" ".." ":" "sep"] @punctuation.delimiter
"assign" @punctuation
(field "." @punctuation)

; COMMENTS
(comment) @comment
