(
    (attribute_item (attribute) @_attribute
        (#match? @_attribute ".*test"))
    .
    (function_item
        name: (_) @run)
) @rust-test
