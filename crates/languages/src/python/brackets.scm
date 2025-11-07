("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)
((string_start) @open (string_end) @close)

; Rainbow bracket scopes
[
  (future_import_statement)
  (import_from_statement)
  (with_clause)
  (parameters)
  (parenthesized_list_splat)
  (argument_list)
  (tuple_pattern)
  (list_pattern)
  (subscript)
  (list)
  (set)
  (tuple)
  (dictionary)
  (dictionary_comprehension)
  (set_comprehension)
  (list_comprehension)
  (generator_expression)
  (parenthesized_expression)
  (interpolation)
  (format_expression)
] @rainbow.scope

; Rainbow brackets
[
  "(" ")"
  "{" "}"
  "[" "]"
] @rainbow.bracket
