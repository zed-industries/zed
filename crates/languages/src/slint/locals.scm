[
  (anon_struct_block)
  (block)
  (callback_event)
  (component)
  (enum_block)
  (function_definition)
  (global_definition)
  (imperative_block)
  (struct_block)
] @local.scope

(anon_struct_block
  (_) @local.definition.field)

(argument) @local.definition.var

(callback
  name: (_) @local.definition.member)

(component_definition
  name: (_) @local.definition.type)

(enum_definition
  name: (_) @local.definition.type)

(enum_block
  (_) @local.definition.field)

(function_definition
  name: (_) @local.definition.function)

(global_definition
  name: (_) @local.definition.type)

(import_type
  import_name: (_)
  !local_name) @local.definition.import

(import_type
  import_name: (_)
  local_name: (_) @local.definition.import)

(property
  name: (_) @local.definition.field)

(struct_block
  (_) @local.definition.field)

(struct_definition
  name: (_) @local.definition.type)

(typed_identifier
  name: (_) @local.definition.var)

(argument
  (_) @local.reference)

(binary_expression
  left: (_) @local.reference)

(binary_expression
  right: (_) @local.reference)

(callback_event
  name: (_) @local.reference)

(component
  type: (_) @local.reference
  (#set! reference.kind "type"))

(component_definition
  base_type: (_) @local.reference
  (#set! reference.kind "type"))

(function_call
  name: (_) @local.reference)

(index_op
  index: (_) @local.reference)

(index_op
  left: (_) @local.reference)

(member_access
  base: (_) @local.reference)

(member_access
  member: (_) @local.reference)

(parens_op
  left: (_) @local.reference)

(property
  type: (_) @local.reference
  (#set! reference.kind "type"))

(property_assignment
  property: (_) @local.reference
  (#set! reference.kind "field"))

(property_assignment
  value: (_) @local.reference)

(struct_block
  (_) @local.reference
  (#set! reference.kind "type"))

(tr
  percent: (_) @local.reference)

(typed_identifier
  type: (_) @local.reference
  (#set! reference.kind "type"))

(unary_expression
  left: (_) @local.reference)
