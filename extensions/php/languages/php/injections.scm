((text) @injection.content
 (#set! injection.language "html")
 (#set! injection.combined))

((comment) @injection.content
  (#match? @injection.content "^/\\*\\*[^*]")
  (#set! injection.language "phpdoc"))

((heredoc_body) (heredoc_end) @injection.language) @injection.content
