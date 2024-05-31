(heredoc_body
  (heredoc_end) @language) @content

((regex
  (string_content) @content)
  (#set! "language" "regex"))
