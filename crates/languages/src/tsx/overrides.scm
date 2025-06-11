(comment) @comment.inclusive

[
  (string)
  (template_string)
] @string

(jsx_element) @element

[
  (jsx_opening_element)
  (jsx_closing_element)
  (jsx_self_closing_element)
  (jsx_expression)
] @default

(_ value: (call_expression
  function: (identifier)
  type_arguments: (type_arguments)) @call_expression_with_type_arguments)
