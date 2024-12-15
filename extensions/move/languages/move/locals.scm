; Define the scope of a function based on function definition
((function_definition
  name: (identifier) @local.definition))

; Parameters in a function are defined as local variables
(function_parameters
  (function_parameter
    (identifier) @local.definition))

; When a variable is assigned a value, it's a definition
; Assuming assignments are handled within expressions or specific assignment structures in Move
(let_statement
  name: (identifier) @local.definition)

; Reference to identifiers within the scope of expressions
; This general case assumes usage of variables within expressions or conditions
((identifier) @local.reference)
