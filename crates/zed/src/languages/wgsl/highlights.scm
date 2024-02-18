; https://www.w3.org/TR/WGSL/#keyword-summary
[
    ; "alias"
    "break"
    "case"
    ; "const"
    ; "const_assert"
    "continue"
    "continuing"
    "default"
    ; "diagnostic"
    "discard"
    "else"
    "enable"
    "false"
    "fn"
    "for"
    "if"
    "let"
    "loop"
    "override"
    ; "requires"
    "return"
    "struct"
    "switch"
    "true"
    "var"
    "while"
] @keyword

(line_comment) @comment
(block_comment) @comment

(int_literal) @number
(float_literal) @number
(bool_literal) @constant

(type_declaration) @type.builtin

[
    "||"
    "&&"
    "|"
    "^"
    "&"
    "=="
    "!="
    "<="
    ">="
    "<<"
    ">>"
    "+"
    "-"
    "*"
    "/"
    "%"
] @operator

(attribute) @property
