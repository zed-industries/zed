(type_declaration
    "type" @context
    (type_spec
        name: (_) @name)) @item

(function_declaration
    "func" @context
    name: (identifier) @name) @item

(method_declaration
    "func" @context
    receiver: (parameter_list
        (parameter_declaration
            type: (_) @context))
    name: (field_identifier) @name) @item

(const_declaration
    "const" @context
    (const_spec
        name: (identifier) @name)) @item

(source_file
    (var_declaration
        "var" @context
        (var_spec
            name: (identifier) @name)) @item)
