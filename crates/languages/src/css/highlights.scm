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

(attribute_selector (plain_value) @string)

(attribute_name) @attribute
(pseudo_element_selector (tag_name) @attribute)
(pseudo_class_selector (class_name) @attribute)

[
  (class_name)
  (id_name)
  (namespace_name)
  (feature_name)
] @property

(property_name) @constant

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
]  @keyword

(string_value) @string
(color_value) @string.special

[
  (integer_value)
  (float_value)
] @number

(unit) @type

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
