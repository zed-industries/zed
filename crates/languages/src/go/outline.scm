(comment) @annotation

(type_declaration
    "type" @context
    [
        (type_spec
            name: (_) @name) @item
        (
            "("
            (type_spec
                name: (_) @name) @item
            ")"
      )
    ]
)

(function_declaration
    "func" @context
    name: (identifier) @name
    parameters: (parameter_list
      "("
      ")")) @item

(method_declaration
    "func" @context
    receiver: (parameter_list
        "(" @context
        (parameter_declaration
            name: (_) @context
            type: (_) @context)
        ")" @context)
    name: (field_identifier) @name
    parameters: (parameter_list
      "("
      ")")) @item

(const_declaration
    "const" @context
    (const_spec
        name: (identifier) @name) @item)

(source_file
    (var_declaration
        "var" @context
        [
            ; The declaration may define multiple variables, and so @item is on
            ; the identifier so they get distinct ranges.
            (var_spec
                name: (identifier) @name @item)
            (var_spec_list
                (var_spec
                    name: (identifier) @name @item)
            )
        ]
     )
)

(method_elem
    name: (_) @name
    parameters: (parameter_list
      "(" @context
      ")" @context)) @item

; Fields declarations may define multiple fields, and so @item is on the
; declarator so they each get distinct ranges.
(field_declaration
    name: (_) @name @item)
