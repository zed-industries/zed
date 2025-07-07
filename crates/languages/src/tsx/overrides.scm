(comment) @comment.inclusive

(string) @string

(template_string (string_fragment) @string)

(jsx_element) @element

[
  (jsx_opening_element)
  (jsx_closing_element)
  (jsx_self_closing_element)
  (jsx_expression)
] @default

(_ value: (call_expression
  function: (identifier) @function_name_before_type_arguments
  type_arguments: (type_arguments)))
