; Functions names start with `Test`
(
    (function_declaration name: (_) @run
        (#match? @run "^Test.*"))
) @go-test

; Functions names start with `Benchmark`
(
    (function_declaration name: (_) @run
        (#match? @run "^Benchmark.+"))
) @go-benchmark

; `t.Run`
(
    (call_expression function: (_) @run
        (#match? @run "^t.Run.*"))
) @go-subtest

; go run
(
    (function_declaration name: (_) @run
        (#match? @run "^main$"))
) @go-run
