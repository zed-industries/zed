
(
    (mod_item
        name: (_) @run
        (#eq? @run "tests")
    )
    (#set! tag rust-mod-test)
)

(
    (
        (attribute_item (attribute
            [((identifier) @_attribute)
                (scoped_identifier (identifier) @_attribute)
                ])
            (#match? @_attribute "test")
        ) @_start
        .
        (attribute_item) *
        .
        (function_item
            name: (_) @run
            body: _
        ) @_end
    )
    (#set! tag rust-test)
)

; Rust main function
(
    (
        (function_item
            name: (_) @run
            body: _
        ) @_rust_main_function_end
        (#eq? @run "main")
    )
    (#set! tag rust-main)
)
