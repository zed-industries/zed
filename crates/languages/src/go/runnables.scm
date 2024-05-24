(
    (
        (function_declaration name: (_) @run
            (#match? @run "^Test.*"))
    ) @xd
    (#set! tag go-test)
)

(
    (
        (function_declaration name: (_) @run
            (#eq? @run "main"))
    ) @_
    (#set! tag go-main)
)
