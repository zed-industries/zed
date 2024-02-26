; Properties
;-----------

(bare_key) @property
(quoted_key) @property

; Literals
;---------

(boolean) @constant
(comment) @comment
(string) @string
(integer) @number
(float) @number
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
