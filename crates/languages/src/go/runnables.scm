(
    (function_declaration name: (_) @run
        (#match? @run "^Test.*"))
) @go-test

(
    (function_declaration name: (_) @run
        (#eq? @run "main"))
) @go-main
