; Bracket matching pairs
("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)
("<" @open ">" @close)
("<" @open "/>" @close)
("</" @open ">" @close)
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
] @rainbow.scope

; Rainbow brackets
[
  "[" "]"
  "{" "}"
  "(" ")"
] @rainbow.bracket