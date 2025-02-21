(comment) @comment

[
  (tag_name)
  (nesting_selector)
  (universal_selector)
] @tag

[
  "~"
  ">"
  "+"
  "-"
  "|"
  "*"
  "/"
  "="
  "^="
  "|="
  "~="
  "$="
  "*="
] @operator

[
  "and"
  "or"
  "not"
  "only"
] @keyword.operator

(attribute_selector (plain_value) @string)

[
  (id_name)
  (class_name)
] @selector

(namespace_name) @namespace
(namespace_selector (tag_name) @namespace "|")

(attribute_name) @attribute
(pseudo_element_selector "::" (tag_name) @attribute)
(pseudo_class_selector ":" (class_name) @attribute)

[
  (feature_name)
  (property_name)
] @property

(function_name) @function

[
  (plain_value)
  (keyframes_name)
  (keyword_query)
] @constant

(
  [
    (property_name)
    (plain_value)
  ] @variable
  (#match? @variable "^--")
)

[
  "@media"
  "@import"
  "@charset"
  "@namespace"
  "@supports"
  "@keyframes"
  (at_keyword)
  (to)
  (from)
  (important)
] @keyword

(string_value) @string
(color_value) @string.special

[
  (integer_value)
  (float_value)
] @number

(unit) @constant.unit

[
  ","
  ":"
  "."
  "::"
  ";"
  (id_selector "#")
] @punctuation.delimiter

[
  "{"
  ")"
  "("
  "}"
  "["
  "]"
] @punctuation.bracket
