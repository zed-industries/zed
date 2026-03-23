(lexical_declaration (variable_declarator name: (identifier) @debug-variable))

(for_in_statement left: (identifier) @debug-variable)
(for_statement initializer: (lexical_declaration (variable_declarator name: (identifier) @debug-variable)))

(binary_expression left: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))
(binary_expression right: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(unary_expression argument: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))
(update_expression argument: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(return_statement (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(parenthesized_expression (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(array (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(pair value: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(member_expression object: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(statement_block) @debug-scope
(program) @debug-scope
