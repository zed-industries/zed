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
  (raw_text) @content
  (#set! "language" "css"))
