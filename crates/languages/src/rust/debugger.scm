(metavariable) @debug-variable

(parameter (identifier) @debug-variable)

(self) @debug-variable

(static_item (identifier) @debug-variable)
(const_item (identifier) @debug-variable)

(let_declaration pattern: (identifier) @debug-variable)

(let_condition (identifier) @debug-variable)

(match_arm (identifier) @debug-variable)

(for_expression (identifier) @debug-variable)

(closure_parameters (identifier) @debug-variable)

(assignment_expression (identifier) @debug-variable)

(field_expression (identifier) @debug-variable)

(binary_expression (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]"))

(reference_expression (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]"))

(array_expression (identifier) @debug-variable)
(tuple_expression (identifier) @debug-variable)
(return_expression (identifier) @debug-variable)
(await_expression (identifier) @debug-variable)
(try_expression (identifier) @debug-variable)
(index_expression (identifier) @debug-variable)
(range_expression (identifier) @debug-variable)
(unary_expression (identifier) @debug-variable)

(if_expression (identifier) @debug-variable)
(while_expression (identifier) @debug-variable)

(parenthesized_expression (identifier) @debug-variable)

(arguments (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]"))

(macro_invocation (token_tree (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]")))

(block) @debug-scope
