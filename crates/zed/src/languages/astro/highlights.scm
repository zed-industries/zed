(tag_name) @tag
(erroneous_end_tag_name) @keyword
(doctype) @constant
(attribute_name) @property
(attribute_value) @string
(comment) @comment

[
  (attribute_value)
  (quoted_attribute_value)
] @string

"=" @operator

[
  "{"
  "}"
] @punctuation.bracket

[
  "<"
  ">"
  "</"
  "/>"
] @tag.delimiter
