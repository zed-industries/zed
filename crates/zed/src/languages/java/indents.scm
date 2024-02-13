[
    (assignment_expression)
    (binary_expression)
    (instanceof_expression)
    (lambda_expression)
    (ternary_expression)
    (update_expression)
    (primary_expression)
    (unary_expression)
    (cast_expression)
    (switch_expression)
] @indent

(_ "[" "]" @end) @indent
(_ "<" ">" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent
