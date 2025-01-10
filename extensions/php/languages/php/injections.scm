((text) @content
 (#set! "language" "html")
 (#set! "combined"))

((comment) @content
  (#match? @content "^/\\*\\*[^*]")
  (#set! "language" "phpdoc"))

((heredoc_body) (heredoc_end) @language) @content
