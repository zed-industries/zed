(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent

(function_definition (block) @indent)
(if_statement (block) @indent)
(else_clause (block) @indent)
(elif_clause (block) @indent)

; (try_statement
;     body: (_) @start
;     [(except_clause) (finally_clause)] @end
;     ) @indent

; (if_statement
;     consequence: (_) @start
;     alternative: (_) @end
;     ) @indent

; (_
;     alternative: (elif_clause) @start
;     alternative: (_) @end
;     ) @indent
