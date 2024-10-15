(comment) @comment

(string) @string
(escape_sequence) @string.escape

(pair
  key: (string) @property.json_key)

(number) @number

[
  (true)
  (false)
  (null)
] @constant

[
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket
