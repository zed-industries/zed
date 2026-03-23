(parameter_declaration (identifier) @debug-variable)

(short_var_declaration (expression_list (identifier) @debug-variable))

(var_declaration (var_spec (identifier) @debug-variable))

(const_declaration (const_spec (identifier) @debug-variable))

(assignment_statement (expression_list (identifier) @debug-variable))

(binary_expression (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]"))

(call_expression (argument_list (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]")))

(return_statement (expression_list (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]")))

(range_clause (expression_list (identifier) @debug-variable))

(parenthesized_expression (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]"))

(block) @debug-scope
(function_declaration) @debug-scope
