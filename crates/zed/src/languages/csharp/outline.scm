(class_declaration
    "class" @context
    name: (identifier) @name
) @item

(constructor_declaration
    name: (identifier) @name
) @item

(property_declaration
    type: (identifier)? @context
    type: (predefined_type)? @context
    name: (identifier) @name
) @item

(field_declaration
    (variable_declaration) @context
) @item

(method_declaration
    name: (identifier) @name
    parameters: (parameter_list) @context
) @item

(enum_declaration
    "enum" @context
    name: (identifier) @name
) @item

(namespace_declaration
    "namespace" @context
    name: (qualified_name) @name
) @item

(interface_declaration
    "interface" @context
    name: (identifier) @name
) @item
