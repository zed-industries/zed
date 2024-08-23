(macro_invocation
  (token_tree) @content
  (#set! "language" "rust"))

(macro_rule
  (token_tree) @content
  (#set! "language" "rust"))

(block_comment
  (doc_comment) @content
  (#set! "language" "markdown"))

(line_comment
  (doc_comment) @content
  (#set! "language" "markdown"))
