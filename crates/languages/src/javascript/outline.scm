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

(property_signature
    name: (_) @name) @item

(class_declaration
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

; All variable declarators with identifiers
(lexical_declaration
    ["let" "const"] @context
    (variable_declarator
        name: (identifier) @name) @item)

; Object destructuring - shorthand
(lexical_declaration
    ["let" "const"] @context
    (variable_declarator
        name: (object_pattern
            (shorthand_property_identifier_pattern) @name @item)))

; Object destructuring - pair pattern
(lexical_declaration
    ["let" "const"] @context
    (variable_declarator
        name: (object_pattern
            (pair_pattern
                key: (_) @name) @item)))

; Array destructuring
(lexical_declaration
    ["let" "const"] @context
    (variable_declarator
        name: (array_pattern
            (identifier) @name @item)))

; Object pairs with functions
(pair
    key: (_) @name
    value: [(arrow_function) (function_expression)]) @item

; Object pairs with non-function values
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
    ]) @item

(expression_statement
    (assignment_expression
        left: (member_expression
            object: (member_expression
                property: (property_identifier) @_prototype)
            property: (property_identifier) @name)
        (#eq? @_prototype "prototype")
        right: [(function_expression) (arrow_function)]) @item)

; Method calls in call chains - capture only the arguments part to avoid nesting
(call_expression
    function: (member_expression
        property: (property_identifier) @context)
    arguments: (arguments
        . (string (string_fragment) @name)) @item)

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

; Function calls inside arrow functions and function expressions
(arrow_function
    body: (statement_block
        (expression_statement
            (call_expression
                function: (identifier) @name) @item)))

(function_expression
    body: (statement_block
        (expression_statement
            (call_expression
                function: (identifier) @name) @item)))

(comment) @annotation
