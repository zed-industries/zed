(
    (line_comment)* @context
    .
    [
        (enum_item
            name: (_) @name) @item
        (struct_item
            name: (_) @name) @item
        (impl_item
            trait: (_)? @name
            "for"? @name
            type: (_) @name) @item
        (trait_item
            name: (_) @name) @item
        (function_item
            name: (_) @name) @item
        (macro_definition
            name: (_) @name) @item
        (function_signature_item
            name: (_) @name) @item
    ]
)
