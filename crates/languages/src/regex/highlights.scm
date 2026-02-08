[
  "("
  ")"
  "(?"
  "(?:"
  "(?<"
  "(?P="
  "<"
  ">"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(group_name) @label

[
  (identity_escape)
  (control_letter_escape)
  (character_class_escape)
  (control_escape)
] @string.escape

[
  "*"
  "+"
  "?"
  "|"
  "="
  "!"
  (start_assertion)
  (end_assertion)
  (any_character)
  (lazy)
] @operator

[
  (boundary_assertion)
  (non_boundary_assertion)
  (backreference_escape)
  (decimal_escape)
] @keyword.operator

(count_quantifier
  [
    (decimal_digits) @number.quantifier
    "," @punctuation.delimiter
  ])

(character_class
  [
    "^" @operator
    (class_range "-" @operator)
  ])
