; injections.scm
; --------------
((script_element
    (start_tag
      (attribute
        (quoted_attribute_value (attribute_value) @_language))?)
     (raw_text) @injection.content)
  (#eq? @_language "")
  (#set! injection.language "javascript"))

 ((script_element
     (start_tag
       (attribute
         (quoted_attribute_value (attribute_value) @_language)))
      (raw_text) @injection.content)
    (#eq? @_language "ts")
    (#set! injection.language "typescript"))

((script_element
    (start_tag
        (attribute
        (quoted_attribute_value (attribute_value) @_language)))
    (raw_text) @injection.content)
  (#eq? @_language "typescript")
  (#set! injection.language "typescript"))

(style_element
  (raw_text) @injection.content
  (#set! injection.language "css"))

((raw_text_expr) @injection.content
  (#set! injection.language "javascript"))
