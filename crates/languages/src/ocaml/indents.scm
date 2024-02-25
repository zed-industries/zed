[
  (let_binding)
  (type_binding)

  (method_definition)
  
  (external)
  (value_specification)
  (method_specification)

  (match_case)

  (function_expression)

  (field_declaration)
  (field_expression)

  (application_expression)
] @indent

(_ "[" "]" @end) @indent
(_ "[|" "|]" @end) @indent
(_ "<" ">" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent

(_ "object" @start "end" @end) @indent

(structure
  "struct" @start
  "end" @end) @indent

(signature
  "sig" @start
  "end" @end) @indent

(parenthesized_expression
  "begin" @start
  "end") @indent

(do_clause
  "do" @start
  "done" @end) @indent

";;" @outdent
