[
  (field_expression)
  (assignment_expression)
  (init_declarator)
  (if_statement)
  (for_statement)
  (while_statement)
  (do_statement)
  (else_clause)
] @indent

(expression_statement
  (_) @indent
  ";" @end)

(_
  "{"
  "}" @end) @indent

(compound_statement
  (case_statement
    ":" @start)
  "}" @end) @indent

(compound_statement
  (case_statement)
  (case_statement) @outdent)

(_
  "("
  ")" @end) @indent

((comment) @indent
  (#match? @indent "^/\\*"))

(if_statement) @start.if

(for_statement) @start.for

(while_statement) @start.while

(do_statement) @start.do

(switch_statement) @start.switch

(else_clause) @start.else
