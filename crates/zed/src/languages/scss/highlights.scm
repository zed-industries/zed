(comment) @comment

; Tag and attribute selectors
[
  (tag_name)
  (attribute_name)
  (property_name)
  (property_identifier)
  (at_keyword)
] @tag

; Operators
(operator) @operator
[
  "~"
  ">"
  "+"
  "-"
  "*"
  "/"
  "="
  "^="
  "!="
  "<="
  ">="
  "and"
  "or"
  "not"
] @operator

; Selectors
[
  (class_name)
  (id_name)
  (pseudo_element_selector (identifier))
  (pseudo_class_selector (identifier))
] @class

; Variables
[
  (variable_name)
  (nesting_selector)
] @variable

; Functions and mixins
[
  (function_name)
  (mixin_name)
] @function

; Special identifiers
[
  (feature_name)
  (if)
  (else)
  (for)
  (each)
  (warn)
  (error)
  (debug)
  (at_root)
  (extend)
  (return)
] @keyword

; Values
[
  (color_value) @color
  (string_value) @string
  (number_value) @number
  (percentage_value) @number
  (boolean_value) @boolean
]

[
  "@media"
  "@import"
  "@charset"
  "@namespace"
  "@supports"
  "@keyframes"
  "@apply"
  "@layer"
  (at_keyword)
  (to)
  (from)
  (important)
]  @keyword

; Punctuation
[
  ","
  ":"
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation

[
  ","
  ":"
] @punctuation.delimiter
