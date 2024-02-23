; injections.scm
; --------------
((script_element
    (start_tag
      (attribute
        (quoted_attribute_value (attribute_value) @_language))?)
     (raw_text) @content)
  (#eq? @_language "")
  (#set! "language" "javascript"))

 ((script_element
     (start_tag
       (attribute
         (quoted_attribute_value (attribute_value) @_language)))
      (raw_text) @content)
    (#eq? @_language "ts")
    (#set! "language" "typescript"))

((script_element
    (start_tag
        (attribute
        (quoted_attribute_value (attribute_value) @_language)))
    (raw_text) @content)
  (#eq? @_language "typescript")
  (#set! "language" "typescript"))

(style_element
  (raw_text) @content
  (#set! "language" "css"))

((raw_text_expr) @content
  (#set! "language" "javascript"))
