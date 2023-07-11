(enum_declaration
    "enum" @context
    name: (_) @name) @item

(function_declaration
    "async"? @context
    "function" @context
    name: (_) @name) @item

(interface_declaration
    "interface" @context
    name: (_) @name) @item

(program
    (lexical_declaration
        ["let" "const"] @context
        (variable_declarator
            name: (_) @name) @item))

(class_declaration
    "class" @context
    name: (_) @name) @item

(method_definition
    [
        "get"
        "set"
        "async"
        "*"
        "readonly"
        "static"
        (override_modifier)
        (accessibility_modifier)
        ]* @context
    name: (_) @name) @item
