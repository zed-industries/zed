; JSONL (JSON Lines) syntax highlighting
; This file is derived from crates/languages/src/json/highlights.scm
; and should be kept in sync with the JSON highlighting rules

(comment) @comment

(string) @string
(escape_sequence) @string.escape

(pair
  key: (string) @property.json_key)

(number) @number

[
  (true)
  (false)
] @boolean

(null) @constant.builtin

[
  ","
  ":"
] @punctuation.delimiter

[
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket
