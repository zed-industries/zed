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
