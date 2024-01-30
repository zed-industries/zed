[
    ";"
    ","
    "."
] @delimeter

[
    "after"
    "begin"
    "case"
    "catch"
    "end"
    "fun"
    "if"
    "maybe"
    "of"
    "receive"
    "try"
    "when"
] @keyword

; Term Comparisons
[
    "=="
    "/="
    "=<"
    "<"
    ">="
    ">"
    "=:="
    "=/="
] @operator

; Arithmetic Expressions
[
    "+"
    "-"
    "*"
    "/"
    "bnot"
    "div"
    "rem"
    "band"
    "bor"
    "bxor"
    "bsl"
    "bsr"
] @operator

; Boolean Expressions
[
    "and"
    "or"
    "not"
    "xor"
] @operator

; Short-Circuit Expressions
[
    "andalso"
    "orelse"
] @operator

; List Operations
[
    "++"
    "--"
] @operator

; Send Expressions
"!" @operator

[
    (atom)
] @string.special.symbol
(string) @string

[
    (integer)
    (float)
] @number

(var) @variable

(function_clause
    name: _ @function)

[
    (module_attribute "(" @open ")" @close)
    (pp_define "(" @open ")" @close)
    (spec)
] @keyword

(expr_args "(" @open ")" @close)

(external_fun
    module: (module name: (atom) @type)
    fun: (atom) @function)

(remote
    module: (remote_module module: (atom) @type)
    fun: (atom) @function)

(comment) @comment
