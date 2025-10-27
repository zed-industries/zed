((line_comment) @injection.content
    (#set! injection.language "comment"))

(macro_invocation
    macro: [(identifier) (scoped_identifier)] @_macro_name
    (#not-any-of? @_macro_name "view" "html")
    (token_tree) @injection.content
    (#set! injection.language "rust"))

; we need a better way for the leptos extension to declare that
; it wants to inject inside of rust, instead of modifying the rust
; injections to support leptos injections
(macro_invocation
    macro: [(identifier) (scoped_identifier)] @_macro_name
    (#any-of? @_macro_name "view" "html")
    (token_tree) @injection.content
    (#set! injection.language "rstml")
    )

(call_expression
    function: (scoped_identifier
       path: (_) @_ty_path
       name: (identifier) @_assoc_fn_name
    )
    arguments: (arguments
        [
            (string_literal (string_content) @injection.content)
            (raw_string_literal (string_content) @injection.content)
        ]
    )
    
    (#match? @_ty_path ".*Regex") 
    (#eq! @_assoc_fn_name "new")
    (#set! injection.language "regex")
)