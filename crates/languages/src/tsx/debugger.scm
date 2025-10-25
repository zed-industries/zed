; Variable declarations
(lexical_declaration (variable_declarator name: (identifier) @debug-variable))

; For loop variables
(for_in_statement left: (identifier) @debug-variable)
(for_statement initializer: (lexical_declaration (variable_declarator name: (identifier) @debug-variable)))

; Binary expressions
(binary_expression left: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))
(binary_expression right: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

; Unary expressions
(unary_expression argument: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))
(update_expression argument: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

; Return statements
(return_statement (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

; Parenthesized expressions
(parenthesized_expression (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

; JSX expressions
(jsx_expression (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

; Array elements
(array (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

; Object properties
(pair value: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

; Member expressions
(member_expression object: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

; Scopes
(statement_block) @debug-scope
(program) @debug-scope
