(fenced_code_block
  (info_string
    (language) @language)
  (code_fence_content) @content)

((inline) @content
 (#set! "language" "markdown-inline"))

((html_block) @content
  (#set! "language" "html"))
