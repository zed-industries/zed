(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent

(function_definition) @indent @indent.def
(class_definition) @indent @indent.class
(if_statement) @indent @indent.if
(for_statement) @indent @indent.for
(while_statement) @indent @indent.while
(with_statement) @indent @indent.with
(match_statement) @indent @indent.match
(try_statement) @indent @indent.try

(elif_clause) @indent @indent.elif
(else_clause) @indent @indent.else
(except_clause) @indent @indent.except
(finally_clause) @indent @indent.finally
(case_pattern) @indent @indent.case
