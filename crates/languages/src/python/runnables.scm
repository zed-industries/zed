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

