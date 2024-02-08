; inherits: html_tags
(frontmatter
  (raw_text) @content
  (#set! "language" "typescript"))

(interpolation
  (raw_text) @content
  (#set! "language" "tsx"))

(script_element
  (raw_text) @content
  (#set! "language" "typescript"))

(style_element
  (start_tag
    (attribute
      (attribute_name) @_lang_attr
      (quoted_attribute_value
        (attribute_value) @_lang_value)))
  (raw_text) @content
  (#eq? @_lang_attr "lang")
  (#eq? @_lang_value "scss")
  (#set! "language" "scss"))
