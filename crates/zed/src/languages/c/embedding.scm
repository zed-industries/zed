(declaration
    (type_qualifier)? @context
    type: (_)? @context
    declarator: [
        (function_declarator
            declarator: (_) @name)
        (pointer_declarator
            "*" @context
            declarator: (function_declarator
                declarator: (_) @name))
        (pointer_declarator
            "*" @context
            declarator: (pointer_declarator
                "*" @context
                declarator: (function_declarator
                    declarator: (_) @name)))
    ]
) @item

(function_definition
    (type_qualifier)? @context
    type: (_)? @context
    declarator: [
        (function_declarator
            declarator: (_) @name
                )
        (pointer_declarator
            "*" @context
            declarator: (function_declarator
                declarator: (_) @name
                    ))
        (pointer_declarator
            "*" @context
            declarator: (pointer_declarator
                "*" @context
                declarator: (function_declarator
                    declarator: (_) @name)))
    ]
) @item
