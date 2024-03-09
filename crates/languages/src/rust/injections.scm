(macro_invocation
  (token_tree) @content
  (#set! "language" "rust"))

(macro_rule
  (token_tree) @content
  (#set! "language" "rust"))

([(line_comment) (block_comment)] @content
 (#set! "language" "comment"))
