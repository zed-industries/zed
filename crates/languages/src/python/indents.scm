(function_definition
  ":" @start
  body: (block) @indent
)

(if_statement
  ":" @start
  consequence: (block) @indent
  alternative: (_)? @outdent
)

(else_clause
  ":" @start
  body: (block) @indent
)

(elif_clause
  ":" @start
  consequence: (block) @indent
)

(for_statement
  ":" @start
  body: (block) @indent
)

(with_statement
  ":" @start
  body: (block) @indent
)

(while_statement
  ":" @start
  body: (block) @indent
)

(match_statement
  ":" @start
  body: (block) @indent
)

(class_definition
  ":" @start
  body: (block) @indent
)

(case_clause
  ":" @start
  consequence: (block) @indent
)

(try_statement
  ":" @start
  body: (block) @indent
  (except_clause)? @outdent
  (else_clause)? @outdent
  (finally_clause)? @outdent
)

(except_clause
  ":" @start
  (block) @indent
)

(finally_clause
  ":" @start
  (block) @indent
)
