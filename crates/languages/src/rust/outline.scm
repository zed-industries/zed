(attribute_item) @annotation
(line_comment) @annotation

(struct_item
    (visibility_modifier)? @context
    "struct" @context
    name: (_) @name) @item

(enum_item
    (visibility_modifier)? @context
    "enum" @context
    name: (_) @name) @item

(enum_variant
    (visibility_modifier)? @context
    name: (_) @name) @item

(impl_item
    "impl" @context
    trait: (_)? @name
    "for"? @context
    type: (_) @name
    body: (_ . "{" @open "}" @close .)) @item

(trait_item
    (visibility_modifier)? @context
    "trait" @context
    name: (_) @name) @item

(function_item
    (visibility_modifier)? @context
    (function_modifiers)? @context
    "fn" @context
    name: (_) @name
    body: (_ . "{" @open "}" @close .)) @item

(function_signature_item
    (visibility_modifier)? @context
    (function_modifiers)? @context
    "fn" @context
    name: (_) @name) @item

(macro_definition
    . "macro_rules!" @context
    name: (_) @name) @item

(mod_item
    (visibility_modifier)? @context
    "mod" @context
    name: (_) @name) @item

(type_item
    (visibility_modifier)? @context
    "type" @context
    name: (_) @name) @item

(associated_type
    "type" @context
    name: (_) @name) @item

(const_item
    (visibility_modifier)? @context
    "const" @context
    name: (_) @name) @item

(static_item
    (visibility_modifier)? @context
    "static" @context
    name: (_) @name) @item

(field_declaration
    (visibility_modifier)? @context
    name: (_) @name) @item
