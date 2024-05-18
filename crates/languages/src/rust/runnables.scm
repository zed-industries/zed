(
    (attribute_item (attribute
            (identifier) @_attribute
            (#eq? @_attribute "cfg")

            arguments: (token_tree (
                (identifier) @_argument
                (#eq? @_argument "test")
            ))
        )
    )
    .
    (attribute_item) *
    .
    (mod_item
        name: (_) @run
        (#eq? @run "tests")
    ) @rust-file-test
)

(
    (attribute_item (attribute
        [((identifier) @_attribute)
        (scoped_identifier (identifier) @_attribute)
            ])
        (#eq? @_attribute "test"))
    .
    (attribute_item) *
    .
    (function_item
        name: (_) @run)
) @rust-test
