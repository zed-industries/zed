(tag_name) @keyword

(tag (variable_name) @variable)
(variable_name "$" @operator)

(tag
  (tag_name) @keyword
  (#eq? @keyword "@method")
  (name) @function.method)

(primitive_type) @type.builtin
(named_type (name) @type) @type
(named_type (qualified_name) @type) @type
