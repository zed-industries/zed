; Parameter declarations
(parameter_declaration
  declarator: (identifier) @debug-variable)
(parameter_declaration
  declarator: (pointer_declarator
    declarator: (identifier) @debug-variable))

; Variable declarations
(declaration
  declarator: (init_declarator
    declarator: (identifier) @debug-variable))
(declaration
  declarator: (init_declarator
    declarator: (pointer_declarator
      declarator: (identifier) @debug-variable)))

; For loop initializers
(for_statement
  initializer: (declaration
    declarator: (init_declarator
      declarator: (identifier) @debug-variable)))

; Binary expressions
(binary_expression
  left: (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$"))
(binary_expression
  right: (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$"))

; Unary expressions
(unary_expression
  argument: (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$"))
(update_expression
  argument: (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$"))

; Assignment expressions
(assignment_expression
  left: (identifier) @debug-variable)

; Return statements
(return_statement
  (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$"))

; Parenthesized expressions
(parenthesized_expression
  (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$"))

; Field expressions (struct member access)
(field_expression
  argument: (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$"))

; Subscript expressions (array access)
(subscript_expression
  argument: (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$"))
(subscript_expression
  (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$"))

; Function call arguments
(call_expression
  arguments: (argument_list
    (identifier) @debug-variable
    (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$")))

; Conditional expressions
(conditional_expression
  condition: (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$"))
(conditional_expression
  consequence: (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$"))
(conditional_expression
  alternative: (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z_][A-Z_0-9]*$"))

; Scopes
(compound_statement) @debug-scope
(function_definition) @debug-scope
