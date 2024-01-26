((text) @content
  (#set! "language" "html")
  (#set! "combined"))

((php_statement) @injection.content
  (#set! injection.language php))

((php_only) @injection.content
  (#has-ancestor? @injection.content "php_statement")
  (#set! injection.language php))
