(class_declaration
    "class" @context
    name: (type_identifier) @name
    ) @item

(struct_declaration
    "struct" @context
    name: (type_identifier) @name
    ) @item

(enum_declaration
    "enum" @context
    name: (type_identifier) @name
    ) @item

(protocol_declaration
    "protocol" @context
    name: (type_identifier) @name
    ) @item

(function_declaration
    "func" @context
    name: (identifier) @name
    ) @item

(var_declaration
    "var" @context
    name: (identifier) @name
    ) @item

(let_declaration
    "let" @context
    name: (identifier) @name
    ) @item
