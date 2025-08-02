[
  ; c
  (preproc_params)
  (preproc_defined)
  (argument_list)
  (attribute_specifier)
  (ms_declspec_modifier)
  (declaration_list)
  (parenthesized_declarator)
  (parenthesized_expression)
  (abstract_parenthesized_declarator)
  (array_declarator)
  (compound_statement)
  (initializer_list)
  (compound_literal_expression)
  (enumerator_list)
  (field_declaration_list)
  (parameter_list)
  (for_statement)
  ; (macro_type_specifier) - not part of cpp
  (subscript_expression)
  (subscript_designator)
  (cast_expression)

  ; cpp
  (decltype)
  (explicit_function_specifier)
  (template_parameter_list)
  (template_argument_list)
  (parameter_list)
  (argument_list)
  (structured_binding_declarator)
  (noexcept)
  (throw_specifier)
  (static_assert_declaration)
  (condition_clause)
  (for_range_loop)
  (new_declarator)
  (delete_expression "[" "]")
  (lambda_capture_specifier)
  (sizeof_expression)
] @rainbow.scope

[
  "(" ")"
  "{" "}"
  "[" "]"
  "<" ">"
] @rainbow.bracket
