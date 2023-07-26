(
    (comment)* @context
    .
    [
        (export_statement
            (function_declaration
                "async"? @name
                "function" @name
                name: (_) @name))
        (function_declaration
            "async"? @name
            "function" @name
            name: (_) @name)
        ] @item
    )

(
    (comment)* @context
    .
    [
        (export_statement
            (class_declaration
                "class" @name
                name: (_) @name))
        (class_declaration
            "class" @name
            name: (_) @name)
        ] @item
    )

(
    (comment)* @context
    .
    [
        (export_statement
            (interface_declaration
                "interface" @name
                name: (_) @name))
        (interface_declaration
            "interface" @name
            name: (_) @name)
        ] @item
    )

(
    (comment)* @context
    .
    [
        (export_statement
            (enum_declaration
                "enum" @name
                name: (_) @name))
        (enum_declaration
            "enum" @name
            name: (_) @name)
        ] @item
    )

(
    (comment)* @context
    .
    [
        (export_statement
            (type_alias_declaration
                "type" @name
                name: (_) @name))
        (type_alias_declaration
            "type" @name
            name: (_) @name)
        ] @item
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
