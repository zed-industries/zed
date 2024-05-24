((
    (attribute_item (attribute
        [((identifier) @_attribute)
        (scoped_identifier (identifier) @_attribute)
            ])
        (#eq? @_attribute "test")) @_start
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
