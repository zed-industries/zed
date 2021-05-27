(field_expression) @inside
(let_declaration) @inside

((_ . "where" @after) _ @until)

(_ "{" @after "}" @until)
(_ "[" @after "]" @until)
(_ "(" @after ")" @until)