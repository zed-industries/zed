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
] @punctuation.regex.bracket

(group_name) @label.regex

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
] @operator.regex

[
  (boundary_assertion)
  (non_boundary_assertion)
  (backreference_escape)
] @keyword.operator.regex

(count_quantifier
  [
    (decimal_digits) @number
    "," @punctuation.regex.delimiter
  ])

(character_class
  [
    "^" @operator.regex
    (class_range "-" @operator.regex)
  ])

(class_character) @constant.character

(pattern_character) @string
