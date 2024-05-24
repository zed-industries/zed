; Functions names start with `Test`
(
    (function_declaration name: (_) @run @_name
        (#match? @_name "^Test.+"))
        (#set! tag go-test)
) @go-test

; `t.Run`
(
    (call_expression 
      function: (
      	selector_expression
      	field: _ @run @_name
        (#eq? @_name "Run")
      )
      arguments: (
        argument_list
        . 
        (interpreted_string_literal) @_subtest_name
        .
        (func_literal
          parameters: (
            parameter_list
            (parameter_declaration
              name: (identifier) @_param_name
              type: (pointer_type
                (qualified_type
                  package: (package_identifier) @_pkg
                  name: (type_identifier) @_type
                  (#eq? @_pkg "testing")
                  (#eq? @_type "T")
                )
              )
            )
          )
        ) @second_argument
      )
    )
    (#set! tag go-subtest)
) @go-subtest

; Functions names start with `Benchmark`
(
    (function_declaration name: (_) @run @_name
        (#match? @_name "^Benchmark.+"))
        (#set! tag go-benchmark)
) @go-benchmark

; go run
(
    (function_declaration name: (_) @run @_name
        (#eq? @_name "main"))
        (#set! tag go-run)
) @go-run
