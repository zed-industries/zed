(((comment) @_jsdoc_comment
  (#match? @_jsdoc_comment "(?s)^/[*][*][^*].*[*]/$")) @content
  (#set! "language" "jsdoc"))

((regex) @content
  (#set! "language" "regex"))
