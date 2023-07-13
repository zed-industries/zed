(
    (line_comment)* @context
    .
    (enum_item
        name: (_) @name) @item
)

(
    (line_comment)* @context
    .
    (struct_item
        name: (_) @name) @item
)

(
    (line_comment)* @context
    .
    (impl_item
        trait: (_)? @name
        "for"? @name
        type: (_) @name) @item
)

(
    (line_comment)* @context
    .
    (trait_item
        name: (_) @name) @item
)

(
    (line_comment)* @context
    .
    (function_item
        name: (_) @name) @item
)

(
    (line_comment)* @context
    .
    (macro_definition
        name: (_) @name) @item
)

(
    (line_comment)* @context
    .
    (function_signature_item
        name: (_) @name) @item
)
