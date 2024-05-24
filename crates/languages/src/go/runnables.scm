; Functions names start with `Test`
(
    (function_declaration name: (_) @run @_name
        (#match? @_name "^Test.+"))
        (#set! tag go-test)
)

; `t.Run`
(
    (call_expression function: (_) @run @_name
        (#match? @_name "^t.Run.*"))
        (#set! tag go-subtest)
)

; Functions names start with `Benchmark`
(
    (function_declaration name: (_) @run @_name
        (#match? @_name "^Benchmark.+"))
        (#set! tag go-benchmark)
)

; go run
(
    (function_declaration name: (_) @run @_name
        (#eq? @_name "main"))
        (#set! tag go-run)
)
