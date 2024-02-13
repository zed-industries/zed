(module_declaration
    "open"? @context
    "module" @context
    name: (_) @name) @item

(package_declaration
    "package" @context
    (_) @name) @item

(class_declaration
    (modifiers)? @context
    "class" @context
    name: (_) @name
    superclass: (_)? @name
    interfaces: (_)? @name
    permits: (_)? @name) @item

(record_declaration
    (modifiers)? @context
    "record" @context
    name: (_) @name
    interfaces: (_)? @name) @item

(interface_declaration
    (modifiers)? @context
    "interface" @context
    name: (_) @name
    (extends_interfaces)? @context
    permits: (_)? @name) @item

(annotation_type_declaration
    (modifiers)? @context
    "@interface" @context
    name: (_) @name) @item

(enum_declaration
    (modifiers)? @context
    "enum" @context
    name: (_) @name
    interfaces: (_)? @name) @item
