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
] @punctuation.bracket

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
] @escape

[
  "*"
  "+"
  "?"
  "|"
  "="
  "!"
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

(class_character) @constant.character

(pattern_character) @string
