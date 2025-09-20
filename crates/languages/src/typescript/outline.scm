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
        ; Multiple names may be exported - @item is on the declarator to keep
        ; ranges distinct.
        (variable_declarator
            name: (_) @name) @item))

(program
    (lexical_declaration
        ["let" "const"] @context
        ; Multiple names may be defined - @item is on the declarator to keep
        ; ranges distinct.
        (variable_declarator
            name: (_) @name) @item))

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

; Arrow functions in variable declarations (anywhere in the tree, including nested in functions)
(lexical_declaration
    ["let" "const"] @context
    (variable_declarator
        name: (_) @name
        value: (arrow_function)) @item)

; Async arrow functions in variable declarations
(lexical_declaration
    ["let" "const"] @context
    (variable_declarator
        name: (_) @name
        value: (arrow_function
            "async" @context)) @item)

; Named function expressions in variable declarations
(lexical_declaration
    ["let" "const"] @context
    (variable_declarator
        name: (_) @name
        value: (function_expression)) @item)

(comment) @annotation
