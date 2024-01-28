(script_element
  (raw_text) @content
  (#set! "language" "javascript"))

(style_element
  (raw_text) @content
  (#set! "language" "css"))

(frontmatter
  (raw_text) @content
  (#set! "language" "typescript"))

(interpolation
  (raw_text) @content
  (#set! "language" "tsx"))
