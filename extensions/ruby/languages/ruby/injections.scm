(heredoc_body
  (heredoc_content) @content
  (heredoc_end) @language
  (#downcase! @language))

((regex
  (string_content) @content)
  (#set! "language" "regex"))
