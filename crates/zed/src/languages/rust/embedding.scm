(struct_item
    (visibility_modifier)? @context
    "struct" @context
    name: (_) @name) @item

(enum_item
    (visibility_modifier)? @context
    "enum" @context
    name: (_) @name) @item

(impl_item
    "impl" @context
    trait: (_)? @name
    "for"? @context
    type: (_) @name) @item

(trait_item
    (visibility_modifier)? @context
    "trait" @context
    name: (_) @name) @item

(function_item
    (visibility_modifier)? @context
    (function_modifiers)? @context
    "fn" @context
    name: (_) @name) @item

(function_signature_item
    (visibility_modifier)? @context
    (function_modifiers)? @context
    "fn" @context
    name: (_) @name) @item

(macro_definition
    . "macro_rules!" @context
    name: (_) @name) @item
