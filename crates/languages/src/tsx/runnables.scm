; Add support for (node:test, bun:test and Jest) runnable
; Function expression that has `it`, `test` or `describe` as the function name
(
    (call_expression
        function: (_) @_name
        (#any-of? @_name "it" "test" "describe")
        arguments: (
            arguments . (string
                (string_fragment) @run
            )
        )
    ) @_tsx-test
    (#set! tag tsx-test)
)
