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

(declaration
    type: (_) @context
    declarator: (function_declarator
        declarator: (_) @name
        parameters: (parameter_list
            "(" @context
            ")" @context))) @item

(function_definition
    type: (_) @context
    declarator: (function_declarator
        declarator: (_) @name
        parameters: (parameter_list
            "(" @context
            ")" @context))) @item
