(impl_item
    "impl" @context
    type: (_) @name) @item

(function_item
    (visibility_modifier)? @context
    "fn" @context
    name: (identifier) @name) @item

(struct_item
    (visibility_modifier)? @context
    "struct" @context
    name: (type_identifier) @name) @item

(field_declaration
    (visibility_modifier)? @context
    name: (field_identifier) @name) @item
