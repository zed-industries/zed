(macro_invocation
  (token_tree) @content
  (#set! "language" "rust"))

(macro_rule
  (token_tree) @content
  (#set! "language" "rust"))


;; Inject the tucan grammar into Rust strings that are bound to `let tucan =`
((let_declaration
   pattern: (identifier) @_new
   (#eq? @_new "tucan")
   value: [(raw_string_literal
             (string_content) @content)
           (string_literal
             (string_content) @content)])
 (#set! "language" "tucan"))
