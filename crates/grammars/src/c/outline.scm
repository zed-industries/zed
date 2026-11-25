(preproc_def
  "#define" @context
  name: (_) @name) @item

(preproc_function_def
  "#define" @context
  name: (_) @name
  parameters: (preproc_params
    "(" @context
    ")" @context)) @item

(struct_specifier
  "struct" @context
  name: (_) @name) @item @item.struct

(union_specifier
  "union" @context
  name: (_) @name) @item @item.struct

(enum_specifier
  "enum" @context
  name: (_) @name) @item @item.enum

(enumerator
  name: (_) @name) @item @item.enum_member

(field_declaration
  type: (_) @context
  declarator: (field_identifier) @name) @item @item.field

(type_definition
  "typedef" @context
  declarator: (_) @name) @item

(declaration
  (type_qualifier)? @context
  type: (_)? @context
  declarator: [
    (function_declarator
      declarator: (_) @name
      parameters: (parameter_list
        "(" @context
        ")" @context))
    (pointer_declarator
      "*" @context
      declarator: (function_declarator
        declarator: (_) @name
        parameters: (parameter_list
          "(" @context
          ")" @context)))
    (pointer_declarator
      "*" @context
      declarator: (pointer_declarator
        "*" @context
        declarator: (function_declarator
          declarator: (_) @name
          parameters: (parameter_list
            "(" @context
            ")" @context))))
  ]) @item @item.function

(function_definition
  (type_qualifier)? @context
  type: (_)? @context
  declarator: [
    (function_declarator
      declarator: (_) @name
      parameters: (parameter_list
        "(" @context
        ")" @context))
    (pointer_declarator
      "*" @context
      declarator: (function_declarator
        declarator: (_) @name
        parameters: (parameter_list
          "(" @context
          ")" @context)))
    (pointer_declarator
      "*" @context
      declarator: (pointer_declarator
        "*" @context
        declarator: (function_declarator
          declarator: (_) @name
          parameters: (parameter_list
            "(" @context
            ")" @context))))
  ]) @item @item.function

(comment) @annotation
