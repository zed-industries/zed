[
    ";"
    ","
    "."
] @delimeter

;; NOTE: `cond` and `let` are not covered by tree-sitter-erlang.
[
    "after"
    "and"
    "andalso"
    "band"
    "begin"
    "bnot"
    "bor"
    "bsl"
    "bsr"
    "bxor"
    "case"
    "catch"
    "div"
    "end"
    "fun"
    "if"
    "maybe"
    "not"
    "of"
    "or"
    "orelse"
    "receive"
    "rem"
    "try"
    "when"
    "xor"
] @keyword

[
    (atom)
] @string.special.symbol
(string) @string

[
    (integer)
    (float)
] @number

(var) @variable

(comment) @comment
