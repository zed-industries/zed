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

(_
  "{"
  "}" @end) @indent

(field_declaration_list
  (access_specifier) @start
  "}" @end) @indent

(field_declaration_list
  (access_specifier)
  (access_specifier) @outdent)

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
