((text) @content
 (#set! "language" "html")
 (#set! "combined"))

((comment) @content
  (#match? @content "^/\\*\\*[^*]")
  (#set! "language" "phpdoc"))
