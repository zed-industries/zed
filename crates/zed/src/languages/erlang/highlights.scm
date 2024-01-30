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

    "+"
    ; "-"
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

    "and"
    "or"
    "not"
    "xor"

    "andalso"
    "orelse"

    "++"
    "--"

    "!"
    "|"
    "->"
] @operator

(unary_op_expr) @operator
(binary_op_expr) @operator

(atom) @string.special.symbol
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
    (record_decl "(" @open ")" @close)
] @keyword

(spec fun: (atom) @function)

(expr_args "(" @open ")" @close)

(external_fun
    module: (module name: (atom) @type)
    fun: (atom) @function)

(remote
    module: (remote_module module: (atom) @type)
    fun: (atom) @function)

(call expr: (atom) @function)

(comment) @comment
