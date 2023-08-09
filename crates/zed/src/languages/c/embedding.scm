(
    (comment)* @context
    .
    (declaration
        declarator: [
            (function_declarator
                declarator: (_) @name)
            (pointer_declarator
                "*" @name
                declarator: (function_declarator
                    declarator: (_) @name))
            (pointer_declarator
                "*" @name
                declarator: (pointer_declarator
                    "*" @name
                    declarator: (function_declarator
                        declarator: (_) @name)))
            ]
        ) @item
    )

(
    (comment)* @context
    .
    (function_definition
        declarator: [
            (function_declarator
                declarator: (_) @name
                )
            (pointer_declarator
                "*" @name
                declarator: (function_declarator
                    declarator: (_) @name
                    ))
            (pointer_declarator
                "*" @name
                declarator: (pointer_declarator
                    "*" @name
                    declarator: (function_declarator
                        declarator: (_) @name)))
            ]
        ) @item
    )
