; Rust mod test
(
    (mod_item
        name: (_) @run
        (#eq? @run "tests")
    )
    (#set! tag rust-mod-test)
)

; Rust test
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
        [(line_comment) (block_comment)] *
        .
        (function_item
            name: (_) @run
            body: _
        ) @_end
    )
    (#set! tag rust-test)
)

; Rust doc test
(
    (
        (line_comment) *
        (line_comment
            doc: (_) @_comment_content
        ) @start
        (#match? @_comment_content "```")
        (line_comment) *
        (line_comment
            doc: (_) @_end_comment_content
        ) @_end_code_block
        (#match? @_end_comment_content "```")
        .
        (attribute_item) *
        .
        [(line_comment) (block_comment)] *
        .
        [(function_item
            name: (_) @run
            body: _
        ) (function_signature_item
            name: (_) @run
        ) (struct_item
            name: (_) @run
        ) (enum_item
            name: (_) @run
            body: _
        ) (
            (attribute_item) ?
            (macro_definition
                name: (_) @run)
        ) (mod_item
            name: (_) @run
        )] @_end
    )
    (#set! tag rust-doc-test)
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
