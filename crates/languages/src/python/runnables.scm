; subclasses of unittest.TestCase or TestCase
(
    (class_definition
        name: (identifier) @run @_unittest_class_name
        superclasses: (argument_list
            [(identifier) @_superclass
            (attribute (identifier) @_superclass)]
        )
        (#eq? @_superclass "TestCase")
    ) @_python-unittest-class
    (#set! tag python-unittest-class)
)

; test methods whose names start with `test` in a TestCase
(
    (class_definition
        name: (identifier) @_unittest_class_name
        superclasses: (argument_list
            [(identifier) @_superclass
            (attribute (identifier) @_superclass)]
        )
        (#eq? @_superclass "TestCase")
        body: (block
                (function_definition
                    name: (identifier) @run @_unittest_method_name
                    (#match? @_unittest_method_name "^test.*")
                ) @_python-unittest-method
                (#set! tag python-unittest-method)
            )
        )
)

; pytest functions
(
    (module
        (function_definition
            name: (identifier) @run @_pytest_method_name
            (#match? @_pytest_method_name "^test_")
        ) @python-pytest-method
    )
    (#set! tag python-pytest-method)
)

; pytest classes
(
    (module
        (class_definition
            name: (identifier) @run @_pytest_class_name
            (#match? @_pytest_class_name "^Test")
        )
        (#set! tag python-pytest-class)
    )
)

; pytest class methods
(
    (module
        (class_definition
            name: (identifier) @_pytest_class_name
            (#match? @_pytest_class_name "^Test")
            body: (block
                    (function_definition
                        name: (identifier) @run @_pytest_method_name
                        (#match? @_pytest_method_name "^test")
                    ) @python-pytest-method
                    (#set! tag python-pytest-method)
            )
        )
    )
)
