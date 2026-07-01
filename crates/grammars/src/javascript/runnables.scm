; Add support for (node:test, bun:test and Jest) runnable
; Function expression that has `it`, `test` or `describe` as the function name
; Also matches direct modifiers: .skip, .todo, .only, .failing (Jest, Bun, Vitest)
((call_expression
  function: [
    (identifier) @_name
    (member_expression
      object: [
        (identifier) @_name
        (member_expression
          object: (identifier) @_name)
      ])
  ]
  (#any-of? @_name "it" "test" "describe" "context" "suite")
  arguments: (arguments
    .
    [
      (string
        (string_fragment) @run)
      (identifier) @run
    ])) @_js-test
  (#set! tag js-test))

; Bun, Vitest (each also in Jest): parameterized and conditional tests
((call_expression
  function: (call_expression
    function: (member_expression
      object: [
        (identifier) @_name
        (member_expression
          object: (identifier) @_name)
      ]
      property: (property_identifier) @_property)
    (#any-of? @_name "it" "test" "describe" "context" "suite")
    (#any-of? @_property "each" "skipIf" "runIf"))
  arguments: (arguments
    .
    [
      (string
        (string_fragment) @run)
      (identifier) @run
    ])) @_js-test
  (#set! tag js-test))

; Bun only: if, todoIf (https://bun.sh/docs/test/writing-tests#test-modifiers)
((call_expression
  function: (call_expression
    function: (member_expression
      object: [
        (identifier) @_name
        (member_expression
          object: (identifier) @_name)
      ]
      property: (property_identifier) @_property)
    (#any-of? @_name "it" "test" "describe" "context" "suite")
    (#any-of? @_property "if" "todoIf"))
  arguments: (arguments
    .
    [
      (string
        (string_fragment) @run)
      (identifier) @run
    ])) @_js-test
  (#set! tag js-test))
