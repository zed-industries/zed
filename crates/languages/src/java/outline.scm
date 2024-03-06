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

(field_declaration
    (modifiers)? @context
    type: (_) @context
    declarator: (_) @context) @item

(constructor_declaration
    (modifiers)? @context
    name: (_) @name
    parameters: (formal_parameters) @context) @item

; TODO: definitely needs more testing in projects with methods that actually use
;       all of these rules
(method_declaration
    (modifiers _+ @context)?
    (_
      type_parameters: (_) @context
      [
        (marker_annotation) @context
        (annotation) @context
      ]*
    )?
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
      ")" @context)
    dimensions: (dimensions
      (
        (_)* @context
        "[" @context
        "]" @context
      )+)?
    (throws)? @context) @item

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
