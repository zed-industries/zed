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
  (jsx_element)
  (jsx_self_closing_element)
] @rainbow.scope

[
  "[" "]"
  "{" "}"
  "(" ")"
] @rainbow.bracket

; TypeScript generics (but not JSX tags)
(type_parameters ["<" ">"] @rainbow.bracket)
(type_arguments ["<" ">"] @rainbow.bracket)