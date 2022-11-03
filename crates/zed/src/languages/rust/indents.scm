[
    ((where_clause) _ @end)
    (field_expression)
    (call_expression)
    (assignment_expression)
    (let_declaration)
    (let_chain)
] @indent

(_ "[" "]" @end) @indent
(_ "<" ">" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent
