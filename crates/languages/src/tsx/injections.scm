(((comment) @_jsdoc_comment
  (#match? @_jsdoc_comment "(?s)^/[*][*][^*].*[*]/$")) @content
  (#set! "language" "jsdoc"))

((comment) @content
  (#set! "language" "comment"))

((regex) @content
  (#set! "language" "regex"))
