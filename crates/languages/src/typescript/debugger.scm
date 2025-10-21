; Include all JavaScript patterns

; Variable declarations (const, let, var)
(variable_declarator
  name: (identifier) @debug-variable)

; Function parameters
(formal_parameters
  (identifier) @debug-variable)

; TypeScript required parameters
(required_parameter
  pattern: (identifier) @debug-variable)

; TypeScript optional parameters
(optional_parameter
  pattern: (identifier) @debug-variable)

; Arrow function parameters
(arrow_function
  parameter: (identifier) @debug-variable)

; Rest parameters
(formal_parameters
  (rest_pattern
    (identifier) @debug-variable))

(required_parameter
  pattern: (rest_pattern
    (identifier) @debug-variable))

; Default parameters
(formal_parameters
  (assignment_pattern
    left: (identifier) @debug-variable))

; Destructuring - object shorthand
(variable_declarator
  name: (object_pattern
    (shorthand_property_identifier) @debug-variable))

; Destructuring - object with renaming
(variable_declarator
  name: (object_pattern
    (pair_pattern
      value: (identifier) @debug-variable)))

; Destructuring - array
(variable_declarator
  name: (array_pattern
    (identifier) @debug-variable))

; Destructuring - rest in array
(variable_declarator
  name: (array_pattern
    (rest_pattern
      (identifier) @debug-variable)))

; For loop variables
(for_statement
  initializer: (variable_declaration
    (variable_declarator
      name: (identifier) @debug-variable)))

; For-in loops
(for_in_statement
  left: (identifier) @debug-variable)

(for_in_statement
  left: (variable_declaration
    (variable_declarator
      name: (identifier) @debug-variable)))

; Catch clause
(catch_clause
  parameter: (identifier) @debug-variable)

; Identifiers in expressions (excluding capitalized constants)
(binary_expression
  (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]"))

(call_expression
  arguments: (arguments
    (identifier) @debug-variable
    (#not-match? @debug-variable "^[A-Z]")))

(return_statement
  (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]"))

(parenthesized_expression
  (identifier) @debug-variable
  (#not-match? @debug-variable "^[A-Z]"))

; Scopes
(statement_block) @debug-scope

(function_declaration
  body: (statement_block) @debug-scope)

(arrow_function
  body: (statement_block) @debug-scope)

(method_definition
  body: (statement_block) @debug-scope)

(function_expression
  body: (statement_block) @debug-scope)

(class_declaration
  body: (class_body) @debug-scope)

(for_statement
  body: (statement_block) @debug-scope)

(while_statement
  body: (statement_block) @debug-scope)

(do_statement
  body: (statement_block) @debug-scope)

(if_statement
  consequence: (statement_block) @debug-scope
  alternative: (statement_block)? @debug-scope)

(try_statement
  body: (statement_block) @debug-scope
  handler: (catch_clause
    body: (statement_block) @debug-scope)
  finalizer: (statement_block)? @debug-scope)

; TypeScript-specific scopes
(module
  body: (statement_block) @debug-scope)

(namespace_declaration
  body: (statement_block) @debug-scope)
