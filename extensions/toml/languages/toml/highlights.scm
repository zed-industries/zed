; Properties
;-----------

(bare_key) @property
(quoted_key) @property

; Literals
;---------

(boolean) @constant
(comment) @comment
(integer) @number
(float) @number
(string) @string
(escape_sequence) @string.escape
(offset_date_time) @string.special
(local_date_time) @string.special
(local_date) @string.special
(local_time) @string.special

; Punctuation
;------------

[
  "."
  ","
] @punctuation.delimiter

"=" @operator

[
  "["
  "]"
  "[["
  "]]"
  "{"
  "}"
]  @punctuation.bracket
