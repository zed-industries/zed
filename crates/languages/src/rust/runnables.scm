
((
    (mod_item
        name: (_) @run
        (#eq? @run "tests")
    ) @rust-mod-test
    (#set! tag rust-mod-test)
)

(

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
