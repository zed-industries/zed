(internal_module
    "namespace" @context
    name: (_) @name) @item

(enum_declaration
    "enum" @context
    name: (_) @name) @item

(function_declaration
    "async"? @context
    "function" @context
    name: (_) @name
    parameters: (formal_parameters
      "(" @context
      ")" @context)) @item

(generator_function_declaration
    "async"? @context
    "function" @context
    "*" @context
    name: (_) @name
    parameters: (formal_parameters
      "(" @context
      ")" @context)) @item

(interface_declaration
    "interface" @context
    name: (_) @name) @item

(program
    (export_statement
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (identifier) @name) @item)))

; Exported array destructuring
(program
    (export_statement
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (array_pattern
                    [
                        (identifier) @name @item
                        (assignment_pattern left: (identifier) @name @item)
                        (rest_pattern (identifier) @name @item)
                    ])))))

; Exported object destructuring
(program
    (export_statement
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (object_pattern
                    [(shorthand_property_identifier_pattern) @name @item
                     (pair_pattern
                         value: (identifier) @name @item)
                     (pair_pattern
                         value: (assignment_pattern left: (identifier) @name @item))
                     (rest_pattern (identifier) @name @item)])))))

(program
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (identifier) @name) @item))

; Top-level array destructuring
(program
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (array_pattern
                [
                    (identifier) @name @item
                    (assignment_pattern left: (identifier) @name @item)
                    (rest_pattern (identifier) @name @item)
                ]))))

; Top-level object destructuring
(program
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (object_pattern
                [(shorthand_property_identifier_pattern) @name @item
                 (pair_pattern
                     value: (identifier) @name @item)
                 (pair_pattern
                     value: (assignment_pattern left: (identifier) @name @item))
                 (rest_pattern (identifier) @name @item)]))))

(class_declaration
    "class" @context
    name: (_) @name) @item

; Method definitions in classes (not in object literals)
(class_body
    (method_definition
        [
            "get"
            "set"
            "async"
            "*"
            "readonly"
            "static"
            (override_modifier)
            (accessibility_modifier)
        ]* @context
        name: (_) @name
        parameters: (formal_parameters
          "(" @context
          ")" @context)) @item)

; Object literal methods
(variable_declarator
    value: (object
        (method_definition
            [
                "get"
                "set"
                "async"
                "*"
            ]* @context
            name: (_) @name
            parameters: (formal_parameters
              "(" @context
              ")" @context)) @item))

(public_field_definition
    [
        "declare"
        "readonly"
        "abstract"
        "static"
        (accessibility_modifier)
    ]* @context
    name: (_) @name) @item

; Add support for (node:test, bun:test and Jest) runnable
(
    (call_expression
        function: [
            (identifier) @_name
            (member_expression
                object: [
                    (identifier) @_name
                    (member_expression object: (identifier) @_name)
                ]
            )
        ] @context
        (#any-of? @_name "it" "test" "describe" "context" "suite")
        arguments: (
            arguments . [
                (string (string_fragment) @name)
                (identifier) @name
            ]
        )
    )
) @item

; Add support for parameterized tests
(
    (call_expression
        function: (call_expression
            function: (member_expression
                object: [(identifier) @_name (member_expression object: (identifier) @_name)]
                property: (property_identifier) @_property
            )
            (#any-of? @_name "it" "test" "describe" "context" "suite")
            (#eq? @_property "each")
        )
        arguments: (
            arguments . [
                (string (string_fragment) @name)
                (identifier) @name
            ]
        )
    )
) @item

; Object properties
(pair
    key: [
        (property_identifier) @name
        (string (string_fragment) @name)
        (number) @name
        (computed_property_name) @name
    ]) @item

; Nested variables in function bodies
(statement_block
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (identifier) @name) @item))

; Nested array destructuring in functions
(statement_block
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (array_pattern
                [
                    (identifier) @name @item
                    (assignment_pattern left: (identifier) @name @item)
                    (rest_pattern (identifier) @name @item)
                ]))))

; Nested object destructuring in functions
(statement_block
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (object_pattern
                [(shorthand_property_identifier_pattern) @name @item
                 (pair_pattern value: (identifier) @name @item)
                 (pair_pattern value: (assignment_pattern left: (identifier) @name @item))
                 (rest_pattern (identifier) @name @item)]))))

(comment) @annotation
