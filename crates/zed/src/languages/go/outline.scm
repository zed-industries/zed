(type_declaration
    "type" @context
    (type_spec
        name: (_) @name)) @item

(function_declaration
    "func" @context
    name: (identifier) @name
    parameters: (parameter_list
      "(" @context
      ")" @context)) @item

(method_declaration
    "func" @context
    receiver: (parameter_list
        "(" @context
        (parameter_declaration
            type: (_) @context)
        ")" @context)
    name: (field_identifier) @name
    parameters: (parameter_list
      "(" @context
      ")" @context)) @item

(const_declaration
    "const" @context
    (const_spec
        name: (identifier) @name) @item)

(source_file
    (var_declaration
        "var" @context
        (var_spec
            name: (identifier) @name) @item))

(method_spec
    name: (_) @name
    parameters: (parameter_list
      "(" @context
      ")" @context)) @item

(field_declaration
    name: (_) @name
    type: (_) @context) @item