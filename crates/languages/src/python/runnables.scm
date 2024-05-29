; subclasses of unittest.TestCase or TestCase
(
    (class_definition
        name: (identifier) @run
        superclasses: (argument_list
            [(identifier) @_superclass
            (attribute (identifier) @_superclass)]
        )
        (#eq? @_superclass "TestCase")
    ) @python-unittest-class
    (#set! tag python-unittest-class)
)

; test methods whose names start with `test` in a TestCase
(
    (class_definition
        name: (identifier)
        superclasses: (argument_list
            [(identifier) @_superclass
            (attribute (identifier) @_superclass)]
        )
        (#eq? @_superclass "TestCase")
        body: (block
                (function_definition
                    name: (identifier) @run @_test_func_name
                    (#match? @_test_func_name "^test.*")
                ) @python-unittest-function
                (#set! tag python-unittest-function)
            )
        )
)
