(preproc_def
    "#define" @context
    name: (_) @name) @item

(preproc_function_def
    "#define" @context
    name: (_) @name
    parameters: (preproc_params
        "(" @context
        ")" @context)) @item

(type_definition
    "typedef" @context
    declarator: (_) @name) @item

(struct_specifier
    "struct" @context
    name: (_) @name) @item

(class_specifier
    "class" @context
    name: (_) @name) @item

(enum_specifier
    "enum" @context
    name: (_) @name) @item

(enumerator
    name: (_) @name) @item

(declaration
    (storage_class_specifier) @context
    (type_qualifier)? @context
    type: (_) @context
    declarator: (init_declarator
      declarator: (_) @name)) @item

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
    ]
    (type_qualifier)? @context) @item

(declaration
    (type_qualifier)? @context
    type: (_)? @context
    declarator: [
        (field_identifier) @name
        (pointer_declarator
            "*" @context
            declarator: (field_identifier) @name)
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
    ]
    (type_qualifier)? @context) @item

(field_declaration
    (type_qualifier)? @context
    type: (_) @context
    declarator: [
        (field_identifier) @name
        (pointer_declarator
            "*" @context
            declarator: (field_identifier) @name)
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
    ]
    (type_qualifier)? @context) @item
