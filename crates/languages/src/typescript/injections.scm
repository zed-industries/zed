(call_expression
  (identifier) @language
  (template_string) @content)

((comment) @content
  (#set! "language" "jsdoc"))

((regex) @content
  (#set! "language" "regex"))