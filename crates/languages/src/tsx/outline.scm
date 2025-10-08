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

(property_signature
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
        ; Multiple names may be exported - @item is on the declarator to keep
        ; ranges distinct.
        (variable_declarator
            name: (identifier) @name) @item))

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

; Anonymous functions assigned to variables at program level
(program
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (identifier) @name
            value: [(function_expression) (arrow_function)]) @item))

(program
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (identifier) @name
            value: [
                (string)
                (number)
                (true)
                (false)
                (null)
                (undefined)
                (identifier)
                (call_expression)
                (new_expression)
                (await_expression)
                (binary_expression)
                (unary_expression)
                (template_string)
                (array)
                (object)
                (member_expression)
                (jsx_element)
                (jsx_self_closing_element)
            ]) @item))

; Program-level lexical declarations without values (e.g., let b: C)
(program
    (lexical_declaration
        ["let" "const"] @context
        ; Multiple names may be defined - @item is on the declarator to keep
        ; ranges distinct.
        (variable_declarator
            name: (identifier) @name
            !value) @item))

; Lexical declarations directly inside arrow function bodies
(arrow_function
    body: (statement_block
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (identifier) @name) @item)))

(arrow_function
    body: (statement_block
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (array_pattern
                    (identifier) @name @item)))))

; Lexical declarations directly inside function expression bodies
(function_expression
    body: (statement_block
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (identifier) @name) @item)))

(function_expression
    body: (statement_block
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (array_pattern
                    (identifier) @name @item)))))

; Lexical declarations directly inside function declaration bodies
(function_declaration
    body: (statement_block
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (identifier) @name) @item)))

(function_declaration
    body: (statement_block
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (array_pattern
                    (identifier) @name @item)))))

; Lexical declarations directly inside generator function declaration bodies
(generator_function_declaration
    body: (statement_block
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (identifier) @name) @item)))

(generator_function_declaration
    body: (statement_block
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (array_pattern
                    (identifier) @name @item)))))

; Lexical declarations directly inside method definition bodies
(method_definition
    body: (statement_block
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (identifier) @name) @item)))

(method_definition
    body: (statement_block
        (lexical_declaration
            ["let" "const"] @context
            (variable_declarator
                name: (array_pattern
                    (identifier) @name @item)))))

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

; Object pairs with arrow functions
(pair
    key: (_) @name
    value: (arrow_function)) @item

; Variables inside arrow functions assigned to object properties
(pair
    value: (arrow_function
        body: (statement_block
            (lexical_declaration
                ["let" "const"] @context
                (variable_declarator
                    name: (identifier) @name) @item))))

; Object pairs with function expressions
(pair
    key: (_) @name
    value: (function_expression)) @item

; Variables inside function expressions assigned to object properties
(pair
    value: (function_expression
        body: (statement_block
            (lexical_declaration
                ["let" "const"] @context
                (variable_declarator
                    name: (identifier) @name) @item))))

; Object property pairs for non-function values
(pair
    key: (_) @name
    value: [
        (string)
        (number)
        (true)
        (false)
        (null)
        (undefined)
        (identifier)
        (call_expression)
        (new_expression)
        (await_expression)
        (binary_expression)
        (unary_expression)
        (template_string)
        (array)
        (object)
        (member_expression)
        (jsx_element)
        (jsx_self_closing_element)
    ]) @item

(expression_statement
    (assignment_expression
        left: (member_expression
            object: (member_expression
                property: (property_identifier) @_prototype)
            property: (property_identifier) @name)
        (#eq? @_prototype "prototype")
        right: [(function_expression) (arrow_function)]) @item)

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
