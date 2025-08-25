(identifier) @debug-variable
(#eq? @debug-variable "self")

(assignment left: (identifier) @debug-variable)
(assignment left: (pattern_list (identifier) @debug-variable))
(assignment left: (tuple_pattern (identifier) @debug-variable))

(augmented_assignment left: (identifier) @debug-variable)

(for_statement left: (identifier) @debug-variable)
(for_statement left: (pattern_list (identifier) @debug-variable))
(for_statement left: (tuple_pattern (identifier) @debug-variable))

(for_in_clause left: (identifier) @debug-variable)
(for_in_clause left: (pattern_list (identifier) @debug-variable))
(for_in_clause left: (tuple_pattern (identifier) @debug-variable))

(as_pattern (identifier) @debug-variable)

(binary_operator left: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))
(binary_operator right: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))
(comparison_operator (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(list (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))
(tuple (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))
(set (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(subscript value: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(attribute object: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(return_statement (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(parenthesized_expression (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(argument_list (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(if_statement condition: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(while_statement condition: (identifier) @debug-variable (#not-match? @debug-variable "^[A-Z]"))

(block) @debug-scope
(module) @debug-scope
