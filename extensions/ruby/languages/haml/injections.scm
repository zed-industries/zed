(
  (ruby_block_output
    (ruby_code) @content)
  (#set! "language" "ruby")
)

(
  (ruby_block_run
    (ruby_code) @content)
  (#set! "language" "ruby")
)

(filter
  (filter_name) @language
  (filter_body) @content)
