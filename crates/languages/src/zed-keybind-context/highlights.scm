(identifier) @variable

[
  "("
  ")"
] @punctuation.bracket

(_
  .
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

[
  (integer_literal)
  (float_literal)
] @number

(boolean_literal) @boolean

[
  "!="
  "=="
  "=>"
  ">"
  "&&"
  "||"
  "!"
] @operator
