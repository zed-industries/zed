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
] @punctuation.bracket.regex

(group_name) @label.regex

[
  (identity_escape)
  (control_letter_escape)
  (character_class_escape)
  (control_escape)
] @string.escape.regex

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
] @operator.regex

[
  (boundary_assertion)
  (non_boundary_assertion)
  (backreference_escape)
  (decimal_escape)
] @keyword.operator.regex

(count_quantifier
  [
    (decimal_digits) @number.quantifier.regex
    "," @punctuation.delimiter.regex
  ])

(character_class
  [
    "^" @operator.regex
    (class_range "-" @operator.regex)
  ])
