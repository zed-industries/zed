; Functions names start with `Test`
(
  (
    (function_declaration name: (_) @run
      (#match? @run "^Test.*"))
  ) @_
  (#set! tag go-test)
)

; Suite test methods (testify/suite)
(
    (method_declaration
      receiver: (parameter_list
        (parameter_declaration
            type: [
                (pointer_type (type_identifier) @_suite_name)
                (type_identifier) @_suite_name
            ]
        )
      )
      name: (field_identifier) @run @_subtest_name
      (#match? @_subtest_name "^Test.*")
      (#match? @_suite_name ".*Suite")
    ) @_
    (#set! tag go-testify-suite)
)

; `go:generate` comments
(
    ((comment) @_comment @run
    (#match? @_comment "^//go:generate"))
    (#set! tag go-generate)
)

; `t.Run`
(
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
        [
          (interpreted_string_literal)
          (raw_string_literal)
        ] @_subtest_name
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
        ) @_second_argument
      )
    )
  ) @_
  (#set! tag go-subtest)
)

; Functions names start with `Benchmark`
(
  (
    (function_declaration name: (_) @run @_name
      (#match? @_name "^Benchmark.*"))
  ) @_
  (#set! tag go-benchmark)
)

; Functions names start with `Fuzz`
(
  (
    (function_declaration name: (_) @run @_name
      (#match? @_name "^Fuzz"))
  ) @_
  (#set! tag go-fuzz)
)

; go run
(
  (
    (function_declaration name: (_) @run
      (#eq? @run "main"))
  ) @_
  (#set! tag go-main)
)

; Table test cases - slice and map
(
  (short_var_declaration
    left: (expression_list (identifier) @_collection_var)
    right: (expression_list
      (composite_literal
        type: [
          (slice_type)
          (map_type
            key: (type_identifier) @_key_type
            (#eq? @_key_type "string")
          )
        ]
        body: (literal_value
          [
            (literal_element
              (literal_value
                (keyed_element
                  (literal_element
                    (identifier) @_field_name
                  )
                  (literal_element
                    [
                      (interpreted_string_literal) @run @_table_test_case_name
                      (raw_string_literal) @run @_table_test_case_name
                    ]
                  )
                )
              )
            )
            (keyed_element
              (literal_element
                [
                  (interpreted_string_literal) @run @_table_test_case_name
                  (raw_string_literal) @run @_table_test_case_name
                ]
              )
            )
          ]
        )
      )
    )
  )
  (for_statement
    (range_clause
      left: (expression_list
        [
          (
            (identifier)
            (identifier) @_loop_var
          )
          (identifier) @_loop_var
        ]
      )
      right: (identifier) @_range_var
      (#eq? @_range_var @_collection_var)
    )
    body: (block
      (expression_statement
        (call_expression
          function: (selector_expression
            operand: (identifier) @_t_var
            field: (field_identifier) @_run_method
            (#eq? @_run_method "Run")
          )
          arguments: (argument_list
            .
            [
              (selector_expression
                operand: (identifier) @_tc_var
                (#eq? @_tc_var @_loop_var)
                field: (field_identifier) @_field_check
                (#eq? @_field_check @_field_name)
              )
              (identifier) @_arg_var
              (#eq? @_arg_var @_loop_var)
            ]
            .
            (func_literal
              parameters: (parameter_list
                (parameter_declaration
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
            )
          )
        )
      )
    )
  ) @_
  (#set! tag go-table-test-case)
)
