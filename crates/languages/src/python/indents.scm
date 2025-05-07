(function_definition
  ":" @start
  (block) @indent
)

(if_statement
  ":" @start
  consequence: (block) @indent
  alternative: (_)? @outdent
)

(else_clause
  ":" @start
  (block) @indent
)

(elif_clause
  ":" @start
  (block) @indent
)
