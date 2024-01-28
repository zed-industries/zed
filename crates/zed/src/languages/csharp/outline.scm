(class_declaration
    (modifier)? @context
    "class" @context
    name: (identifier) @name
) @item

(constructor_declaration
    (modifier)? @context
    name: (identifier) @name
) @item

(property_declaration
    (modifier)? @context
    type: (identifier)? @context
    type: (predefined_type)? @context
    name: (identifier) @name
) @item

(field_declaration
    (modifier)? @context
    (modifier)? @context
    (variable_declaration) @context
) @item

(method_declaration
    (modifier)? @context
    (modifier)? @context
    type: (generic_name)? @context
    name: (identifier) @name
) @item

(enum_declaration
    (modifier)? @context
    "enum" @context
    name: (identifier) @name
) @item

(namespace_declaration
    "namespace" @context
    name: (qualified_name) @name
) @item

(interface_declaration
    (modifier)? @context
    "interface" @context
    name: (identifier) @name
) @item
