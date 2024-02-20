[
  (struct_definition
    "end" @end)
  (macro_definition
    "end" @end)
  (function_definition
    "end" @end)
  (compound_statement)
  (if_statement
    "end" @end)
  (try_statement
    "end" @end)
  (for_statement
    "end" @end)
  (while_statement
    "end" @end)
  (let_statement
    "end" @end)
  (quote_statement
    "end" @end)
  (do_clause
    "end" @end)
  (assignment)
  (for_binding)
  (call_expression)
  (parenthesized_expression)
  (tuple_expression)
  (comprehension_expression)
  (matrix_expression)
  (vector_expression)
] @indent

(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent

[
  (else_clause)
  (elseif_clause)
  (catch_clause)
  (finally_clause)
] @outdent
