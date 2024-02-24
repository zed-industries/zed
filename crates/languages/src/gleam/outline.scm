(external_type
    (visibility_modifier)? @context
    "type" @context
    (type_name) @name) @item

(type_definition
    (visibility_modifier)? @context
    (opacity_modifier)? @context
    "type" @context
    (type_name) @name) @item

(data_constructor
    (constructor_name) @name) @item

(data_constructor_argument
    (label) @name) @item

(type_alias
    (visibility_modifier)? @context
    "type" @context
    (type_name) @name) @item

(function
    (visibility_modifier)? @context
    "fn" @context
    name: (_) @name) @item

(constant
    (visibility_modifier)? @context
    "const" @context
    name: (_) @name) @item
