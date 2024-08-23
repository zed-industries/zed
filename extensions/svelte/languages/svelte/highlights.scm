; Special identifiers
;--------------------

; TODO:
(tag_name) @tag
(attribute_name) @attribute
(erroneous_end_tag_name) @keyword
(comment) @comment

[
  (attribute_value)
  (quoted_attribute_value)
] @string

[
  (text)
  (raw_text_expr)
] @none

[
  (special_block_keyword)
  (then)
  (as)
] @keyword

[
  "{"
  "}"
  "<"
  ">"
  "</"
  "/>"
] @punctuation.bracket

"=" @operator

[
  "#"
  ":"
  "/"
  "@"
] @punctuation.delimiter
