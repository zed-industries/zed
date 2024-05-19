(frontmatter
  (frontmatter_js_block) @content
  (#set! "language" "typescript"))

(attribute_interpolation
  (attribute_js_expr) @content
  (#set! "language" "typescript"))

(html_interpolation
  (permissible_text) @content
  (#set! "language" "typescript"))

(script_element
  (raw_text) @content
  (#set! "language" "typescript"))

; TODO: add scss/less or more injections
; https://github.com/virchau13/tree-sitter-astro/blob/4be180759ec13651f72bacee65fa477c64222a1a/queries/injections.scm#L18-L27
(style_element
  (raw_text) @content
  (#set! "language" "css"))
