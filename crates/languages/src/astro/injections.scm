; inherits: html_tags
(frontmatter
  (raw_text) @injection.content
  (#set! injection.language "typescript"))

(interpolation
  (raw_text) @injection.content
  (#set! injection.language "tsx"))

(script_element
  (raw_text) @injection.content
  (#set! injection.language "typescript"))

(style_element
  (raw_text) @injection.content
  (#set! injection.language "css"))
