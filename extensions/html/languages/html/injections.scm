(script_element
  (raw_text) @injection.content
  (#set! injection.language "javascript"))

(style_element
  (raw_text) @injection.content
  (#set! injection.language "css"))

(attribute
    (attribute_name) @_attribute_name (#match? @_attribute_name "^style$")
    (quoted_attribute_value (attribute_value) @injection.content)
    (#set! injection.language "css"))

(attribute
    (attribute_name) @_attribute_name (#match? @_attribute_name "^on[a-z]+$")
    (quoted_attribute_value (attribute_value) @injection.content)
    (#set! injection.language "javascript"))
