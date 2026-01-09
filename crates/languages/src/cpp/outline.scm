(preproc_def
    "#define" @context
    name: (_) @name) @item

(preproc_function_def
    "#define" @context
    name: (_) @name
    parameters: (preproc_params
        "(" @context
        ")" @context)) @item

(namespace_definition
    "inline"? @context
    "namespace" @context
    name: (_) @name) @item

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
    [
        "class"
        "struct"
    ]? @context
    name: (_) @name) @item

(union_specifier
    "union" @context
    name: (_) @name) @item

(enumerator
    name: (_) @name) @item

(concept_definition
    "concept" @context
    name: (_) @name) @item

(declaration
    [
        (storage_class_specifier)
        (type_qualifier)
    ]* @context
    type: (_) @context
    declarator: (init_declarator
      ; The declaration may define multiple variables, using @item on the
      ; declarator so that they get distinct ranges.
      declarator: (_) @item @name))

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
        (reference_declarator
            ["&" "&&"] @context
            (function_declarator
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
        (pointer_declarator
            "*" @context
            declarator: (pointer_declarator
                "*" @context
                declarator: (function_declarator
                    declarator: (_) @name
                    parameters: (parameter_list
                        "(" @context
                        ")" @context))))
        (reference_declarator
            ["&" "&&"] @context
            (function_declarator
                declarator: (_) @name
                parameters: (parameter_list
                    "(" @context
                    ")" @context)))
    ]
    (type_qualifier)? @context) @item

(field_declaration
    [
        (storage_class_specifier)
        (type_qualifier)
    ]* @context
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
        (pointer_declarator
            "*" @context
            declarator: (pointer_declarator
                "*" @context
                declarator: (function_declarator
                    declarator: (_) @name
                    parameters: (parameter_list
                        "(" @context
                        ")" @context))))
        (reference_declarator
            ["&" "&&"] @context
            (function_declarator
                declarator: (_) @name
                parameters: (parameter_list
                    "(" @context
                    ")" @context)))
    ; Fields declarations may define multiple fields, and so @item is on the
    ; declarator so they each get distinct ranges.
    ] @item
    (type_qualifier)? @context)

(comment) @annotation
