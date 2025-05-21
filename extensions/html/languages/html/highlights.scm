(tag_name) @tag
(doctype) @tag.doctype
(attribute_name) @attribute
[
  "\""
  "'"
  (attribute_value)
] @string
(comment) @comment

"=" @punctuation.delimiter.html

[
  "<"
  ">"
  "<!"
  "</"
  "/>"
] @punctuation.bracket.html
