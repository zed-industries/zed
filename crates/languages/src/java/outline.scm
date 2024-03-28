(module_declaration
    "open"? @context
    "module" @context
    name: (_) @name) @item

(package_declaration
    "package" @context
    (_) @name) @item

(class_declaration
    (modifiers
        [
            "private"
            "protected"
            "public"
        ]+ @context)?
    "class" @context
    name: (_) @name) @item

(field_declaration
    (modifiers
        [
            "private"
            "protected"
            "public"
        ]+ @context)?
    type: (_) @context
    declarator: (variable_declarator
        name: (_) @name)) @item

(constructor_declaration
    (modifiers
        [
            "private"
            "protected"
            "public"
        ]+ @context)?
    name: (_) @name
    ; TODO: fix multiline parameters causing space between parenthesis
    parameters: (formal_parameters
        "(" @context
        [
            (receiver_parameter) @context
            (
                (
                    (receiver_parameter) @context
                    "," @context
                )?
                (
                    [
                        (formal_parameter) @context
                        (spread_parameter) @context
                    ]
                    (
                        "," @context
                        [
                            (formal_parameter) @context
                            (spread_parameter) @context
                        ]
                    )*
                )?
            )
        ]
        ")" @context)) @item

(method_declaration
    (modifiers
        [
            "private"
            "protected"
            "public"
        ]+ @context)?
    type: (_) @context
    name: (_) @name
    ; TODO: fix multiline parameters causing space between parenthesis
    parameters: (formal_parameters
        "(" @context
        [
            (receiver_parameter) @context
            (
                (
                    (receiver_parameter) @context
                    "," @context
                )?
                (
                    [
                        (formal_parameter) @context
                        (spread_parameter) @context
                    ]
                    (
                        "," @context
                        [
                            (formal_parameter) @context
                            (spread_parameter) @context
                        ]
                    )*
                )?
            )
        ]
        ")" @context)) @item

(record_declaration
    (modifiers
        [
            "private"
            "protected"
            "public"
        ]+ @context)?
    "record" @context
    name: (_) @name) @item

(interface_declaration
    (modifiers
        [
            "private"
            "protected"
            "public"
        ]+ @context)?
    "interface" @context
    name: (_) @name) @item

(annotation_type_declaration
    (modifiers
        [
            "private"
            "protected"
            "public"
        ]+ @context)?
    "@interface" @context
    name: (_) @name) @item

(enum_declaration
    (modifiers
        [
            "private"
            "protected"
            "public"
        ]+ @context)?
    "enum" @context
    name: (_) @name) @item
