; Copied from https://github.com/nickel-lang/tree-sitter-nickel/
; Removed `@spell` annotations as they broke comment syntax highlighting.
; Replaced `@conditional` -> `@keyword`
; Replaced `@include` -> `@keyword`

(comment) @comment
(annot_atom doc: (static_string))

[
  "forall"
  "in"
  "let"
  "default"
  "doc"
  "rec"
  "optional"
  "priority"
  "force"
  "not_exported"
] @keyword

"fun" @keyword.function

"import" @keyword

[ "if" "then" "else" ] @keyword
"match" @keyword

(types) @type
"Array" @type.builtin

; BUILTIN Constants
(bool) @boolean
"null" @constant.builtin
(enum_tag) @constant

(num_literal) @number

(infix_op) @operator

(type_atom) @type

(chunk_literal_single) @string
(chunk_literal_multi) @string

(str_esc_char) @string.escape

[
 "{" "}"
 "(" ")"
 "[|" "|]"
] @punctuation.bracket

[
 ","
 "."
 ":"
 "="
 "|"
 "->"
 "+"
 "-"
 "*"
] @punctuation.delimiter

(multstr_start) @punctuation.bracket
(multstr_end) @punctuation.bracket
(interpolation_start) @punctuation.bracket
(interpolation_end) @punctuation.bracket

(record_field) @field

(builtin) @function.builtin

(fun_expr pats:
  (pattern id:
    (ident) @parameter
  )
)

; application where the head terms is an identifier: function arg1 arg2 arg3
(applicative t1:
  (applicative (record_operand (atom (ident))) @function)
)

; application where the head terms is a record field path: foo.bar.function arg1 arg2 arg3
(applicative t1:
  (applicative (record_operand (record_operation_chain)) @function)
)
