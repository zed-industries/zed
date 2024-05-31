; Macros `describe` and `test`.
; This matches the ExUnit test style.
(
    (call
        target: (identifier) @run (#any-of? @run "describe" "test")
    ) @elixir-test
    (#set! tag elixir-test)
)

; Modules containing at least one `describe` or `test`.
; This matches the ExUnit test style.
(
    (call
        target: (identifier) @run (#eq? @run "defmodule")
        (do_block
            (call target: (identifier) @keyword (#any-of? @keyword "describe" "test"))
        )
    ) @elixir-module-test
    (#set! tag elixir-module-test)
)
