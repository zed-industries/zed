(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent

[
  (if_statement)
  (for_statement)
  (while_statement)
  (with_statement)
  (function_definition)
  (class_definition)
  (match_statement)
  (try_statement)
] @indent

[
  (else_clause)
  (elif_clause)
  (except_clause)
  (finally_clause)
] @outdent

[
  (block)
  (case_clause)
] @indent
