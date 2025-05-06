; Capture decorators for standalone annotation highlighting
(decorator) @annotation

; class definitions
[
    (module
        (class_definition
            "class" @context
            name: (identifier) @name
            ) @item)

    (block
        (class_definition
            "class" @context
            name: (identifier) @name
            ) @item)

    (decorated_definition
        (decorator)+ @context.extra
        definition:
        (class_definition
            "class" @context
            name: (identifier) @name
            ) )@item
    ]

; function definitions
[
    (module
        (function_definition
            "async"? @context
            "def" @context
            name: (_) @name
            ) @item
        )

    (block
        (function_definition
            "async"? @context
            "def" @context
            name: (_) @name
            ) @item
        )

    (decorated_definition
        (decorator)+ @context.extra
        definition: (function_definition
            "async"? @context
            "def" @context
            name: (_) @name )
        ) @item
    ]
