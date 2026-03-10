; Code blocks
(code
  "{" @fold
  "}" @fold.end)

; Content blocks
(content
  "[" @fold
  "]" @fold.end)

; Function arguments
(args
  "(" @fold
  ")" @fold.end)

; Conditional blocks
(branch
  body: (_) @fold)

; While loop body
(while
  body: (_) @fold)

; For loop body
(for
  body: (_) @fold)

; Let binding with closure/block value
(let
  init: (code) @fold)

; Math blocks
(formula
  "$" @fold
  "$" @fold.end)
