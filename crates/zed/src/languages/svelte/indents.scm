[
  (element)
  (if_statement)
  (each_statement)
  (await_statement)
  (script_element)
  (style_element)
] @indent

[
  (end_tag)
  (else_statement)
  (if_end_expr)
  (each_end_expr)
  (await_end_expr)
  ">"
  "/>"
] @branch

(comment) @ignore
