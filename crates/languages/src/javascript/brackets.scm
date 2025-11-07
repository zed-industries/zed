("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)
("<" @open ">" @close)
("<" @open "/>" @close)
("</" @open ">" @close)
("\"" @open "\"" @close)
("'" @open "'" @close)
("`" @open "`" @close)

; Rainbow scopes describe syntactic constructs whose bracket depth we want to track.
[
  (program)
  (statement_block)
  (switch_case)
  (switch_default)
  (class_body)
  (object)
  (object_pattern)
  (array)
  (array_pattern)
  (arguments)
  (formal_parameters)
  (parenthesized_expression)
  (parenthesized_type)
  (type_arguments)
  (type_parameters)
  (tuple_type)
  (jsx_element)
] @rainbow.scope

; Individual bracket tokens to colorize.
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @rainbow.bracket
