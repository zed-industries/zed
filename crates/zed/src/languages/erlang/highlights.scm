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
    ":"
    "::"
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

[
    (module_attribute)
    (pp_define)
    (pp_include)
    (spec)
    (record_decl)
    (export_attribute)
    (compile_options_attribute)
] @keyword

(wild_attribute
    name: (attr_name
            "-" @keyword
            name: (atom) @keyword))

[
    "("
    ")"
    "{"
    "}"
    "["
    "]"
] @punctuation.bracket

(function_clause name: _ @function)
(spec fun: (atom) @function)
(fa fun: (atom) @function)
(call expr: (atom) @function)
(external_fun
    module: (module name: (atom) @type)
    fun: (atom) @function)
(remote
    module: (remote_module module: (atom) @type)
    fun: (atom) @function)

(record_name name: (atom) @type)

(comment) @comment
