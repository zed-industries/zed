[
  ; {/}
  (declaration_list)
  (field_declaration_list)
  (field_initializer_list)
  (enum_variant_list)
  (block)
  (match_block)
  (use_list)
  (struct_pattern)

  ; (/)
  (ordered_field_declaration_list)
  (arguments)
  (parameters)
  (tuple_type)
  (tuple_expression)
  (tuple_pattern)
  (tuple_struct_pattern)
  (unit_type)
  (unit_expression)
  (visibility_modifier)
  (parenthesized_expression)
  (token_repetition_pattern)

  ; </>
  (type_parameters)
  (type_arguments)
  (bracketed_type)
  (for_lifetimes)

  ; [/]
  (array_type)
  (array_expression)
  (index_expression)
  (slice_pattern)

  ; attributes #[]
  (attribute_item)
  (inner_attribute_item)

  ; macros
  (token_tree_pattern)
  (macro_definition)

  ; closures
  (closure_parameters)
] @rainbow.scope

; attributes like `#[serde(rename_all = "kebab-case")]`
(attribute arguments: (token_tree) @rainbow.scope)

[
  "#"
  "[" "]"
  "(" ")"
  "{" "}"
  "<" ">"
  "|"
] @rainbow.bracket
