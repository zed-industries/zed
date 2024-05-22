; Functions names start with `Test`
(
    (function_declaration name: (_) @run @_name
        (#match? @_name "^Test.+"))
) @go-test

; `t.Run`
(
    (call_expression function: (_) @run @_name
        (#match? @_name "^t.Run.*"))
) @go-subtest

; Functions names start with `Benchmark`
(
    (function_declaration name: (_) @run @_name
        (#match? @_name "^Benchmark.+"))
) @go-benchmark

; go run
(
    (function_declaration name: (_) @run @_name
        (#match? @_name "^main$"))
) @go-run
