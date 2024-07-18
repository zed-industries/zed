; injections.scm
; --------------

; match script tags without a lang tag
((script_element
  (start_tag
    (attribute
      (attribute_name) @_name)*)
    (raw_text) @content)
  (#not-eq? @_name "lang")
  (#set! "language" "javascript"))

; match javascript
((script_element
  (start_tag
    (attribute
      (attribute_name) @_name
      (quoted_attribute_value (attribute_value) @_value)))
    (raw_text) @content)
  (#eq? @_name "lang")
  (#eq? @_value "js")
  (#set! "language" "javascript"))

; match typescript
((script_element
  (start_tag
    (attribute
      (attribute_name) @_name
      (quoted_attribute_value (attribute_value) @_value)))
    (raw_text) @content)
  (#eq? @_name "lang")
  (#eq? @_value "ts")
  (#set! "language" "typescript"))

(style_element
  (raw_text) @content
  (#set! "language" "css"))

; match style tags without a lang tag
((style_element
  (start_tag
    (attribute
      (attribute_name) @_name)*)
    (raw_text) @content)
  (#not-eq? @_name "lang")
  (#set! "language" "css"))

; match css
((style_element
  (start_tag
    (attribute
      (attribute_name) @_name
      (quoted_attribute_value (attribute_value) @_value)))
    (raw_text) @content)
  (#eq? @_name "lang")
  (#eq? @_value "css")
  (#set! "language" "css"))

; match scss
((style_element
  (start_tag
    (attribute
      (attribute_name) @_name
      (quoted_attribute_value (attribute_value) @_value)))
    (raw_text) @content)
  (#eq? @_name "lang")
  (#eq? @_value "scss")
  (#set! "language" "scss"))

((raw_text_expr) @content
  (#set! "language" "javascript"))
