(
    (attribute_item (attribute) @attribute
        (#match? @attribute ".*test"))
    .
    (function_item
        name: (_) @run)
) @rust-test
