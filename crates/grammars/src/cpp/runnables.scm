; Tag the main function
((function_definition
  declarator: (function_declarator
    declarator: (identifier) @run)) @_cpp-main
  (#eq? @run "main")
  (#set! tag cpp-main))

; Catch2 TEST_CASE
((call_expression
  function: (identifier) @_macro
  (#eq? @_macro "TEST_CASE")
  arguments: (argument_list
    .
    (string_literal) @run @_test_name)) @_
  (#set! tag catch2-test))

; Catch2 SECTION inside a TEST_CASE.
(_
  (expression_statement
    (call_expression
      function: (identifier) @_outer_macro
      (#eq? @_outer_macro "TEST_CASE")
      arguments: (argument_list
        .
        (string_literal) @_test_name)))
  .
  (compound_statement
    (expression_statement
      (call_expression
        function: (identifier) @_inner_macro
        (#eq? @_inner_macro "SECTION")
        arguments: (argument_list
          .
          (string_literal) @run @_section_name))))
  (#set! tag catch2-section))
