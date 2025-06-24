; Capture metavariables (e.g., $x in macros)
(metavariable) @debug-variable

; Capture function/closure parameters
(parameter (identifier) @debug-variable)

; Capture self
(self) @debug-variable

; Capture static and const variable declarations
(static_item (identifier) @debug-variable)
(const_item (identifier) @debug-variable)

; Capture variables at their declaration sites (left side of assignments)
(let_declaration pattern: (identifier) @debug-variable)

; Capture variables in if let patterns
(let_condition (identifier) @debug-variable)

; Capture match arm patterns
(match_arm (identifier) @debug-variable)

; Capture for loop variables
(for_expression (identifier) @debug-variable)

; Capture closure parameters
(closure_parameters (identifier) @debug-variable)

; Capture variables in assignments (left side)
(assignment_expression (identifier) @debug-variable)

; Capture field access base (the variable being accessed)
(field_expression (identifier) @debug-variable)

; Capture variables in binary expressions
(binary_expression (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]"))

; Capture identifiers in reference expressions
(reference_expression (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]"))

; Capture identifiers in specific expression contexts
(array_expression (identifier) @debug-variable)
(tuple_expression (identifier) @debug-variable)
(return_expression (identifier) @debug-variable)
(await_expression (identifier) @debug-variable)
(try_expression (identifier) @debug-variable)
(index_expression (identifier) @debug-variable)
(range_expression (identifier) @debug-variable)
(unary_expression (identifier) @debug-variable)

; Capture identifiers in if/while conditions
(if_expression (identifier) @debug-variable)
(while_expression (identifier) @debug-variable)

; Capture identifiers in parenthesized expressions
(parenthesized_expression (identifier) @debug-variable)

; Capture arguments in function/method calls
(arguments (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]"))

; Capture identifiers in macro invocations (like vec![x, 2, 3])
(macro_invocation (token_tree (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]")))

; Scopes
(block) @debug-scope
