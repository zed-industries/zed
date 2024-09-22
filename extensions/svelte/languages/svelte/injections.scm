; ; injections.scm
; ; --------------

; Match script tags with a lang attribute
(script_element
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

; Match script tags without a lang attribute
(script_element
    (start_tag
        (attribute
            (attribute_name) @_attr_name
        )*
    )
    (raw_text) @content
    (#not-any-of? @_attr_name "lang")
    (#set! language "javascript")
)

; Match the contents of the script's generics="T extends string" as typescript code
;
; Disabled for the time-being because tree-sitter is treating the generics
; attribute as a top-level typescript statement, where `T extends string` is
; not a valid top-level typescript statement.
;
; (script_element
;     (start_tag
;         (attribute
;             (attribute_name) @_attr_name
;             (#eq? @_attr_name "generics")
;             (quoted_attribute_value
;                 (attribute_value) @content
;             )
;         )
;     )
; 	(#set! language "typescript")
; )


; Mark everything as typescript because it's
; a more generic superset of javascript
; Not sure if it's possible to somehow refer to the
; script's language attribute here.
((svelte_raw_text) @content
    (#set! "language" "ts")
)

; Match style tags with a lang attribute
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

; Match style tags without a lang attribute
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


; Downstream TODO: Style highlighting for `style:background="red"` and `style="background: red"` strings
; Downstream TODO: Style component comments as markdown
