(internal_module
    "namespace" @context
    name: (_) @name) @item

(enum_declaration
    "enum" @context
    name: (_) @name) @item

(type_alias_declaration
    "type" @context
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

(export_statement
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (object_pattern
                (shorthand_property_identifier_pattern) @name @item))))

(export_statement
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (object_pattern
                (pair_pattern
                    key: (_) @name) @item))))

(export_statement
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (array_pattern
                (identifier) @name @item))))

(export_statement
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (identifier) @name
            value: (call_expression)) @item))

(export_statement
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (identifier) @name
            value: (object)) @item))

(export_statement
    (lexical_declaration
        ["let" "const"] @context
        ; Multiple names may be exported - @item is on the declarator to keep
        ; ranges distinct.
        (variable_declarator
            name: (identifier) @name
            !value) @item))

(program
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (object_pattern
                (shorthand_property_identifier_pattern) @name @item))))

(program
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (object_pattern
                (pair_pattern
                    key: (_) @name) @item))))

(program
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (array_pattern
                (identifier) @name @item))))

(program
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (identifier) @name
            value: (call_expression)) @item))

(program
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (identifier) @name
            value: (object)) @item))

(program
    (lexical_declaration
        ["let" "const"] @context
        ; Multiple names may be defined - @item is on the declarator to keep
        ; ranges distinct.
        (variable_declarator
            name: (identifier) @name
            !value) @item))

(statement_block
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (identifier) @name
            value: (arrow_function)) @item))

(statement_block
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (object_pattern
                (shorthand_property_identifier_pattern) @name @item))))

(statement_block
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (object_pattern
                (pair_pattern
                    key: (_) @name) @item))))

(statement_block
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (array_pattern
                (identifier) @name @item))))

(statement_block
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (identifier) @name
            value: (call_expression)) @item))

(statement_block
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (identifier) @name
            value: (object)) @item))

(statement_block
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (identifier) @name
            !value) @item))

(class_declaration
    "class" @context
    name: (_) @name) @item

(abstract_class_declaration
    "abstract" @context
    "class" @context
    name: (_) @name) @item

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
      ")" @context)) @item

(public_field_definition
    [
        "declare"
        "readonly"
        "abstract"
        "static"
        (accessibility_modifier)
    ]* @context
    name: (_) @name) @item

(pair
    key: (_) @name
    value: (arrow_function)) @item

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
            (#any-of? @_property "each")
        )
        arguments: (
            arguments . [
                (string (string_fragment) @name)
                (identifier) @name
            ]
        )
    )
) @item

(comment) @annotation
