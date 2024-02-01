; Scopes

[
  (infrastructure)
  (call_expression)

  (lambda_expression)
  (subscript_expression)

  (if_statement)
  (for_statement)

  (array)
  (object)
  (interpolation)
] @scope

; References

(property_identifier) @reference

(call_expression
  (identifier) @reference)

(object_property
  (_)
  ":"
  (identifier) @reference)

(resource_expression
  (identifier) @reference)

; Definitions

(type) @definition.associated

(object_property
  (identifier) @definition.field
  (_))

(object_property
  (compatible_identifier) @definition.field
  (_))

(import_name) @definition.import

(module_declaration
  (identifier) @definition.namespace)

(parameter_declaration
  (identifier) @definition.parameter
  (_))

(type_declaration
  (identifier) @definition.type
  (_))

(variable_declaration
  (identifier) @definition.var
  (_))

(metadata_declaration
  (identifier) @definition.var
  (_))

(output_declaration
  (identifier) @definition.var
  (_))

(for_statement
  "for"
  (for_loop_parameters
    (loop_variable) @definition.var
    (loop_enumerator) @definition.var))
