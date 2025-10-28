((line_comment) @injection.content
    (#set! injection.language "comment"))

(macro_invocation
    macro: [
        ((identifier) @_macro_name)
        (scoped_identifier (identifier) @_macro_name .)
    ]
    (token_tree) @injection.content
    (#set! injection.language "rust"))

; we need a better way for the leptos extension to declare that
; it wants to inject inside of rust, instead of modifying the rust
; injections to support leptos injections
(macro_invocation
    macro: [
        ((identifier) @_macro_name)
        (scoped_identifier (identifier) @_macro_name .)
    ]
    (#any-of? @_macro_name "view" "html")
    (token_tree) @injection.content
    (#set! injection.language "rstml")
    )

(macro_invocation
    macro: [
        ((identifier) @_macro_name)
        (scoped_identifier (identifier) @_macro_name .)
    ]
    (#any-of? @_macro_name "sql")
    (_) @injection.content
    (#set! injection.language "sql")
    )

; lazy_regex
(macro_invocation
    macro: [
        ((identifier) @_macro_name)
        (scoped_identifier (identifier) @_macro_name .)
    ]
    (token_tree [
        (string_literal (string_content) @injection.content)
        (raw_string_literal (string_content) @injection.content)
    ])
    (#set! injection.language "regex")
    (#any-of? @_macro_name "regex" "bytes_regex")
)

(call_expression
    function: (scoped_identifier) @_fn_path
    arguments: (arguments
        [
            (string_literal (string_content) @injection.content)
            (raw_string_literal (string_content) @injection.content)
        ]
    )

    (#match? @_fn_path ".*Regex(Builder)?::new")
    (#set! injection.language "regex")
)
