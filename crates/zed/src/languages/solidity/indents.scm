[
    (block_statement)
    (if_statement)
    (emit_statement)
    (revert_statement)
    (try_statement)
    (catch_clause)
    (for_statement)
    (while_statement)
    (do_while_statement)
    (call_expression)
    (error_declaration)
    (enum_declaration)
] @indent.begin

(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent
