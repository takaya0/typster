; function.inside/around: content blocks [...] and code blocks {...}
(content
  "[" @function.inside.start
  "]" @function.inside.end) @function.around

(code
  "{" @function.inside.start
  "}" @function.inside.end) @function.around

; class.inside/around: heading sections
(heading) @class.around
(heading (text) @class.inside)

; comment.inside/around: line comments
(comment) @comment.around
(comment) @comment.inside

; math.inside/around: formula blocks $...$
(formula
  "$" @math.inside.start
  "$" @math.inside.end) @math.around

; argument.inside/around: function call arguments (...)
(args
  "(" @argument.inside.start
  ")" @argument.inside.end) @argument.around
