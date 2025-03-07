[
    (call_expression)
    (assignment_expression)
    (member_expression)
    (lexical_declaration)
    (variable_declaration)
    (assignment_expression)
    ; below handled by  `(_ "{" "}" @end) @indent`
    ; (if_statement)
    ; (for_statement)
    ; (while_statement)
] @indent

(_ "[" "]" @end) @indent
(_ "<" ">" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent
