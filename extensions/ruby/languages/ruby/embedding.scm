(
    (comment)* @context
    .
    [
        (module
            "module" @name
            name: (_) @name)
        (method
            "def" @name
            name: (_) @name
            body: (body_statement) @collapse)
        (class
            "class" @name
            name: (_) @name)
        (singleton_method
            "def" @name
            object: (_) @name
            "." @name
            name: (_) @name
            body: (body_statement) @collapse)
        ] @item
    )
