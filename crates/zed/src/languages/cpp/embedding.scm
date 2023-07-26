(
    (comment)* @context
    .
    (function_definition
        (type_qualifier)? @name
        type: (_)? @name
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
            (reference_declarator
                ["&" "&&"] @name
                (function_declarator
                declarator: (_) @name))
        ]
        (type_qualifier)? @name) @item
    )

(
    (comment)* @context
    .
    (template_declaration
        (class_specifier
            "class" @name
            name: (_) @name)
            ) @item
)

(
    (comment)* @context
    .
    (class_specifier
        "class" @name
        name: (_) @name) @item
    )

(
    (comment)* @context
    .
    (enum_specifier
        "enum" @name
        name: (_) @name) @item
    )

(
    (comment)* @context
    .
    (declaration
        type: (struct_specifier
        "struct" @name)
        declarator: (_) @name) @item
)
