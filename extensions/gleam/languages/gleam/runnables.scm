; Functions with names ending in `_test`.
; This matches the standalone test style used by Startest and Gleeunit.
(
    (
        (function name: (_) @run
            (#match? @run ".*_test$"))
    ) @gleam-test
    (#set! tag gleam-test)
)


; `describe` API for Startest.
(
    (function_call
        function: (_) @name
        (#any-of? @name "describe" "it")
        arguments: (arguments
            .
            (argument
                value: (string (quoted_content) @run)
            )
        )
    )
    (#set! tag gleam-test)
) @gleam-test
