; Functions with names ending in `_test`.
; This matches the standalone test style used by Startest and Gleeunit.
(
    (function name: (_) @run
        (#match? @run ".*_test$"))
) @gleam-test
