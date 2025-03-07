[
  "("
  ")"
  "(?"
  "(?:"
  "(?<"
  ">"
  "["
  "]"
  "{"
  "}"
] @string

(group_name) @property

[
  (identity_escape)
  (control_letter_escape)
  (character_class_escape)
  (control_escape)
  (start_assertion)
  (end_assertion)
  (boundary_assertion)
  (non_boundary_assertion)
] @string.escape

[
  "*"
  "+"
  "?"
  "|"
  "="
  "!"
  (any_character)
] @operator

(count_quantifier
  [
    (decimal_digits) @number
    "," @punctuation.delimiter
  ])

(character_class
  [
    "^" @operator
    (class_range "-" @operator)
  ])
