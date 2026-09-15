[
  (string)
  (raw_string)
  (ansi_c_string)
  (word)
] @string

(variable_name) @variable

(special_variable_name) @variable.special

"export" @keyword

(comment) @comment

(number) @number

[
  (command_substitution)
  (expansion)
] @embedded

[
  "$"
  "="
] @operator

[
  "{"
  "}"
] @punctuation.bracket

(simple_expansion
  "$" @punctuation.special)

(expansion
  "${" @punctuation.special
  "}" @punctuation.special) @embedded

(command_substitution
  "$(" @punctuation.special
  ")" @punctuation.special)
