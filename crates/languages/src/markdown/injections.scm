(fenced_code_block
  (info_string
    (language) @language)
  (code_fence_content) @content)

((inline) @content
 (#set! "language" "markdown-inline"))

((html_block) @content
  (#set! "language" "html"))

((minus_metadata) @content (#set! "language" "yaml"))

((plus_metadata) @content (#set! "language" "toml"))
