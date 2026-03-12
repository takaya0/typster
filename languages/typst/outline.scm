(heading
    [
    "="
    "=="
    "==="
    "===="
    "====="
    "======"
    ] @context

    (text) @name
) @item

(let
  name: (ident) @name
  (#set! @name context "function")) @item
