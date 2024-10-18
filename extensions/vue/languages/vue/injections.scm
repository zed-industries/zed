; <script>
((script_element
    (start_tag) @_no_lang
    (raw_text) @content)
  (#not-match? @_no_lang "lang=")
  (#set! "language" "javascript"))

; <script lang="js">
((script_element
  (start_tag
    (attribute
      (attribute_name) @_lang
      (quoted_attribute_value
        (attribute_value) @_js)))
  (raw_text) @content)
  (#eq? @_lang "lang")
  (#eq? @_js "js")
  (#set! "language" "javascript"))

; <script lang="ts">
((script_element
  (start_tag
    (attribute
      (attribute_name) @_lang
      (quoted_attribute_value
        (attribute_value) @_ts)))
  (raw_text) @content)
  (#eq? @_lang "lang")
  (#eq? @_ts "ts")
  (#set! "language" "typescript"))

; <script lang="tsx">
; <script lang="jsx">
; Zed built-in tsx, we mark it as tsx ^:)
(script_element
  (start_tag
    (attribute
      (attribute_name) @_attr
      (quoted_attribute_value
        (attribute_value) @language)))
  (#eq? @_attr "lang")
  (#any-of? @language "tsx" "jsx")
  (raw_text) @content)


; {{ }}
((interpolation
  (raw_text) @content)
  (#set! "language" "typescript"))

; v-
(directive_attribute
  (quoted_attribute_value
    (attribute_value) @content
    (#set! "language" "typescript")))

; Vue <style lang="css"> injections
(style_element
    (start_tag
        (attribute
            (attribute_name) @_attr_name
            (#eq? @_attr_name "lang")
            (quoted_attribute_value
                (attribute_value) @language
            )
        )
    )
    (raw_text) @content
)

; Vue <style> css injections (no lang attribute)
(style_element
    (start_tag
        (attribute
            (attribute_name) @_attr_name
        )*
    )
    (raw_text) @content
    (#not-any-of? @_attr_name "lang")
    (#set! language "css")
)
