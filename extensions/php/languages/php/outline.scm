(class_declaration
    "class" @context
    name: (name) @name
    ) @item

(function_definition
    "function" @context
    name: (_) @name
    ) @item

(method_declaration
    "function" @context
    name: (_) @name
    ) @item

(interface_declaration
    "interface" @context
    name: (_) @name
    ) @item

(enum_declaration
    "enum" @context
    name: (_) @name
    ) @item

(trait_declaration
    "trait" @context
    name: (_) @name
    ) @item

; Add support for Pest runnable
(function_call_expression
    function: (_) @context
    (#any-of? @context "it" "test" "describe")
    arguments: (arguments
        .
        (argument
            [
              (encapsed_string (string_value) @name)
              (string (string_value) @name)
            ]
        )
    )
) @item
