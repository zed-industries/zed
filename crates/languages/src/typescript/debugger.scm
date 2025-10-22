; TypeScript Debug Variables Query
; Only using verified node types from tree-sitter-typescript

; Variable declarations
(variable_declarator
  name: (identifier) @debug-variable)

; Function/method parameters
(required_parameter
  pattern: (identifier) @debug-variable)

(optional_parameter
  pattern: (identifier) @debug-variable)

; Arrow function single parameter
(arrow_function
  parameter: (identifier) @debug-variable)

; For loop variables
(for_in_statement
  left: (identifier) @debug-variable)

; Scopes
(statement_block) @debug-scope

(function_declaration
  body: (statement_block) @debug-scope)

(method_definition
  body: (statement_block) @debug-scope)

(arrow_function
  body: (statement_block) @debug-scope)

(class_declaration
  body: (class_body) @debug-scope)

(for_statement
  body: (statement_block) @debug-scope)

(while_statement
  body: (statement_block) @debug-scope)

(if_statement
  consequence: (statement_block) @debug-scope
  alternative: (statement_block)? @debug-scope)

(try_statement
  body: (statement_block) @debug-scope)

(catch_clause
  body: (statement_block) @debug-scope)
