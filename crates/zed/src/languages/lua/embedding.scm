(
    (comment)* @context
    .
    (function_declaration
        "function" @name
        name: (_) @name
        (comment)* @collapse
        body: (block) @collapse
    ) @item
)
