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
            ) @_python-pytest-method
        )
    (#set! tag python-pytest-method)
    )

; decorated pytest functions
(
    (module
        (decorated_definition
            (decorator)+ @_decorator
            definition: (function_definition
                name: (identifier) @run @_pytest_method_name
                (#match? @_pytest_method_name "^test_")
                )
            ) @_python-pytest-method
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


; decorated pytest classes
(
    (module
        (decorated_definition
            (decorator)+ @_decorator
            definition: (class_definition
                name: (identifier) @run @_pytest_class_name
                (#match? @_pytest_class_name "^Test")
                )
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
                [(decorated_definition
                    (decorator)+ @_decorator
                    definition: (function_definition
                        name: (identifier) @run @_pytest_method_name
                        (#match? @_pytest_method_name "^test_")
                        )
                    )
                (function_definition
                    name: (identifier) @run @_pytest_method_name
                    (#match? @_pytest_method_name "^test")
                    )
                ] @_python-pytest-method)
            (#set! tag python-pytest-method)
            )
        )
    )

; decorated pytest class methods
(
    (module
        (decorated_definition
            (decorator)+ @_decorator
            definition: (class_definition
                name: (identifier) @_pytest_class_name
                (#match? @_pytest_class_name "^Test")
                body: (block
                    [(decorated_definition
                        (decorator)+ @_decorator
                        definition: (function_definition
                            name: (identifier) @run @_pytest_method_name
                            (#match? @_pytest_method_name "^test_")
                            )
                        )
                    (function_definition
                        name: (identifier) @run @_pytest_method_name
                        (#match? @_pytest_method_name "^test")
                        )
                    ] @_python-pytest-method)
                (#set! tag python-pytest-method)
                )
            )
        )
    )

; module main method
(
    (module
        (if_statement
            condition: (comparison_operator
                (identifier) @run @_lhs
                operators: "=="
                (string) @_rhs
                )
            (#eq? @_lhs "__name__")
            (#match? @_rhs "^[\"']__main__[\"']$")
            (#set! tag python-module-main-method)
            )
        )
    )
