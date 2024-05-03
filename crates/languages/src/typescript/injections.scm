(((comment) @_jsdoc_comment
  (#match? @_jsdoc_comment "(?s)^/[*][*][^*].*[*]/$")) @content
  (#set! "language" "jsdoc"))

(((comment) @reference
  (#match? @reference "^///\\s+<reference\\s+types=\"\\S+\"\\s*/>\\s*$")) @content
  (#set! "language" "html"))

((regex) @content
  (#set! "language" "regex"))
