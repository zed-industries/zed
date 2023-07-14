(
    (comment)* @context
    .
    (enum_declaration
        "enum" @context
        name: (_) @name) @item
    )

(
    (comment)* @context
    .
    [
        (export_statement
            (function_declaration
                "async"? @name
                "function" @name
                name: (_) @name)
            ) @item
        (function_declaration
            "async"? @name
            "function" @name
            name: (_) @name) @item
        ])

(
    (comment)* @context
    .
    (interface_declaration
        "interface" @name
        name: (_) @name) @item
    )

(
    (comment)* @context
    .
    (class_declaration
        "class" @name
        name: (_) @name) @item
    )

(
    (comment)* @context
    .
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
            ]* @name
        name: (_) @name) @item
    )
