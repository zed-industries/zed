[
    (field_expression)
    (if_statement)
    (for_statement)
    (while_statement)
    (do_statement)
    (else_clause)
] @indent

; Handle multi-line declarations - indent continuation but reset after semicolon
(declaration
    declarator: (init_declarator) @indent
    ";" @end) @indent

; Handle multi-line assignment expressions within expression statements
(expression_statement
    (assignment_expression) @indent
    ";" @end) @indent

(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent

(if_statement) @start.if
(for_statement) @start.for
(while_statement) @start.while
(do_statement) @start.do
(switch_statement) @start.switch
(else_clause) @start.else
