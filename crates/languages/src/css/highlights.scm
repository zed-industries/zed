(comment) @comment

[
  "~"
  ">"
  "+"
  "-"
  "*"
  "/"
  "="
  "^="
  "|="
  "~="
  "$="
  "*="
  "and"
  "or"
  "not"
  "only"
] @operator

[
  (tag_name)
  (nesting_selector)
] @tag

(universal_selector ("*") @tag)

(attribute_selector (plain_value) @string)

(attribute_name) @attribute

[
  (class_name)
  (id_name)
  (namespace_name)
  (feature_name)
] @attribute

(pseudo_element_selector (tag_name) @tag)
(pseudo_class_selector (class_name) @function)

(property_name) @property

(function_name) @function

(
  [
    (property_name)
    (plain_value)
  ] @variable.special
  (#match? @variable.special "^--")
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

(unit) @keyword

[
  ","
  ":"
  "."
  "::"
  ";"
  "#"
] @punctuation.delimiter

[
  "{"
  ")"
  "("
  "}"
  "["
  "]"
] @punctuation.bracket
