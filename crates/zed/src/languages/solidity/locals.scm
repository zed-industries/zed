(function_definition) @local.scope
(block_statement) @local.scope

(function_definition (parameter name: (identifier) @local.definition))

; still have to support tuple assignments
(assignment_expression left: (identifier) @local.definition)

(identifier) @local.reference
