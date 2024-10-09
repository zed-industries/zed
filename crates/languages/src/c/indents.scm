[
  (field_expression)
  (assignment_expression)
  (for_statement)
] @indent

((if_statement)
  .
  (ERROR
    "else" @indent.begin))

(if_statement
  condition: (_) @indent.begin)

(else_clause
  (_
    .
    "{" @indent.branch))

(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent
