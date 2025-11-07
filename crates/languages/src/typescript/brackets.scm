("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)
("<" @open ">" @close)
("\"" @open "\"" @close)
("'" @open "'" @close)
("`" @open "`" @close)

; Rainbow bracket scopes
[
  (object)
  (array)
  (arguments)
  (formal_parameters)
  (statement_block)
  (parenthesized_expression)
  (call_expression)
  (type_parameters)
  (type_arguments)
] @rainbow.scope

; Rainbow brackets
[
  "[" "]"
  "{" "}"
  "(" ")"
  "<" ">"
] @rainbow.bracket
