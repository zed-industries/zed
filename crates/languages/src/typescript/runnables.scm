; Add support for (node:test, bun:test, Jest and Deno.test) runnable
; Function expression that has `it`, `test` or `describe` as the function name
(
    (call_expression
        function: [
            (identifier) @_name
            (member_expression
                object: [
                    (identifier) @_name
                    (member_expression object: (identifier) @_name)
                ]
            )
        ]
        (#any-of? @_name "it" "test" "describe" "context" "suite")
        arguments: (
            arguments . [
                (string (string_fragment) @run)
                (identifier) @run
            ]
        )
    ) @_js-test

    (#set! tag js-test)
)

; Add support for parameterized tests
(
    (call_expression
        function: (call_expression
            function: (member_expression
                object: [(identifier) @_name (member_expression object: (identifier) @_name)]
                property: (property_identifier) @_property
            )
            (#any-of? @_name "it" "test" "describe" "context" "suite")
            (#any-of? @_property "each")
        )
        arguments: (
            arguments . [
                (string (string_fragment) @run)
                (identifier) @run
            ]
        )
    ) @_js-test

    (#set! tag js-test)
)

; Add support for Deno.test with string names
(
    (call_expression
        function: (member_expression
            object: (identifier) @_namespace
            property: (property_identifier) @_method
        )
        (#eq? @_namespace "Deno")
        (#eq? @_method "test")
        arguments: (
            arguments . [
                (string (string_fragment) @run @DENO_TEST_NAME)
                (identifier) @run @DENO_TEST_NAME
            ]
        )
    ) @_js-test

    (#set! tag js-test)
)

; Add support for Deno.test with named function expressions
(
    (call_expression
        function: (member_expression
            object: (identifier) @_namespace
            property: (property_identifier) @_method
        )
        (#eq? @_namespace "Deno")
        (#eq? @_method "test")
        arguments: (
            arguments . (function_expression
                name: (identifier) @run @DENO_TEST_NAME
            )
        )
    ) @_js-test

    (#set! tag js-test)
)
