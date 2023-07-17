(
    (comment)* @context
    .
    (export_statement
        (function_declaration
            "async"? @name
            "function" @name
            name: (_) @name)) @item
    )

(
    (comment)* @context
    .
    (function_declaration
        "async"? @name
        "function" @name
        name: (_) @name) @item
    )

(
    (comment)* @context
    .
    (export_statement
        (class_declaration
            "class" @name
            name: (_) @name)) @item
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
            "static"
            ]* @name
        name: (_) @name) @item
    )

(
    (comment)* @context
    .
    (export_statement
        (interface_declaration
            "interface" @name
            name: (_) @name)) @item
    )

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
    (export_statement
        (enum_declaration
            "enum" @name
            name: (_) @name)) @item
    )

(
    (comment)* @context
    .
    (enum_declaration
        "enum" @name
        name: (_) @name) @item
    )
