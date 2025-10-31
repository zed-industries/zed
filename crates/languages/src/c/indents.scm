[
  (field_expression)
  (assignment_expression)
  (for_statement)
  (while_statement)
  (do_statement)
] @indent

(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent

(if_statement
  condition: (_) @start
  alternative: (_) @end) @indent

(if_statement
  condition: (_) @start
  consequence: (_)
  !alternative) @indent

(if_statement
  alternative: (_) @indent
  (#not-kind-eq? @indent "if_statement"))

((if_statement
  consequence: (_) @indent.begin)
  (#not-kind-eq? @indent.begin "compound_statement"))

((if_statement
  alternative: (_) @indent.begin)
  (#not-kind-eq? @indent.begin "compound_statement"))

(for_statement) @start.for
(while_statement) @start.while
(do_statement) @start.do
(switch_statement) @start.switch
