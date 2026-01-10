(preproc_def
    "#define" @context
    name: (_) @name) @item

(preproc_function_def
    "#define" @context
    name: (_) @name
    parameters: (preproc_params
        "(" @context
        ")" @context)) @item

(struct_specifier
    "struct" @context
    name: (_) @name) @item

(union_specifier
    "union" @context
    name: (_) @name) @item

(enum_specifier
    "enum" @context
    name: (_) @name) @item

(enumerator
    name: (_) @name) @item

(field_declaration
    type: (_) @context
    declarator: (field_identifier) @name) @item

(type_definition
    "typedef" @context
    declarator: (_) @name) @item

(declaration
    (type_qualifier)? @context
    type: (_)? @context
    declarator: [
        (function_declarator
            declarator: (_) @name
            parameters: (parameter_list
                "(" @context
                ")" @context))
        (pointer_declarator
            "*" @context
            declarator: (function_declarator
                declarator: (_) @name
                parameters: (parameter_list
                    "(" @context
                    ")" @context)))
        (pointer_declarator
            "*" @context
            declarator: (pointer_declarator
                "*" @context
                declarator: (function_declarator
                    declarator: (_) @name
                    parameters: (parameter_list
                        "(" @context
                        ")" @context))))
    ]
) @item

(function_definition
    (type_qualifier)? @context
    type: (_)? @context
    declarator: [
        (function_declarator
            declarator: (_) @name
            parameters: (parameter_list
                "(" @context
                ")" @context))
        (pointer_declarator
            "*" @context
            declarator: (function_declarator
                declarator: (_) @name
                parameters: (parameter_list
                    "(" @context
                    ")" @context)))
        (pointer_declarator
            "*" @context
            declarator: (pointer_declarator
                "*" @context
                declarator: (function_declarator
                    declarator: (_) @name
                    parameters: (parameter_list
                        "(" @context
                        ")" @context))))
    ]
) @item

(comment) @annotation
