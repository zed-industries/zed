(
    (comment)* @context
    .
    [
        (function_definition
            "function" @name
            name: (_) @name
            body: (_
                "{" @keep
                "}" @keep) @collapse
            )

        (trait_declaration
            "trait" @name
            name: (_) @name)

        (method_declaration
            "function" @name
            name: (_) @name
            body: (_
                "{" @keep
                "}" @keep) @collapse
            )

        (interface_declaration
            "interface" @name
            name: (_) @name
            )

        (enum_declaration
            "enum" @name
            name: (_) @name
            )

        ] @item
    )
