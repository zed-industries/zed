(
    [(line_comment) (attribute_item)]* @context
    .
    [
        (attribute_item) @collapse
        (use_declaration) @collapse

        (struct_item
            name: (_) @name)

        (enum_item
            name: (_) @name)

        (impl_item
            trait: (_)? @name
            "for"? @name
            type: (_) @name)

        (trait_item
            name: (_) @name)

        (function_item
            name: (_) @name
            body: (block
                "{" @keep
                "}" @keep) @collapse)

        (macro_definition
            name: (_) @name)
        ] @item
    )
