; Special identifiers
;--------------------

; Treat capitalized tag names as constructors and types
((tag_name) @type
 (#match? @type "^[A-Z]"))

; Regular (lowercase) tag names
((tag_name) @tag
 (#match? @tag "^[a-z]"))

; TODO:
(attribute_name) @property
(erroneous_end_tag_name) @keyword
(comment) @comment

[
  (attribute_value)
  (quoted_attribute_value)
] @string

[
  (text)
  (raw_text_expr)
  (raw_text_each)
] @none

[
  (special_block_keyword)
  (then)
  (as)
] @keyword

[
  "{"
  "}"
] @punctuation.bracket

"=" @operator

[
  "<"
  ">"
  "</"
  "/>"
  "#"
  ":"
  "/"
  "@"
] @tag.delimiter
