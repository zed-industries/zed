((line_comment) @injection.content
    (#set! injection.language "comment"))

(macro_invocation
    macro: (identifier) @_macro_name
    (#not-any-of? @_macro_name "view" "html")
    (token_tree) @injection.content
    (#set! injection.language "rust"))

; we need a better way for the leptos extension to declare that
; it wants to inject inside of rust, instead of modifying the rust
; injections to support leptos injections
(macro_invocation
    macro: (identifier) @_macro_name
    (#any-of? @_macro_name "view" "html")
    (token_tree) @injection.content
    (#set! injection.language "rstml")
    )
