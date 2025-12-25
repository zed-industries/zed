(declaration
    declarator: (function_declarator)) @function.around

(function_definition
    body: (_
        "{"
        (_)* @function.inside
        "}" )) @function.around

(preproc_function_def
    value: (_) @function.inside) @function.around

(comment) @comment.around

(struct_specifier
    body: (_
        "{"
        (_)* @class.inside
        "}")) @class.around

(enum_specifier
    body: (_
        "{"
        [(_) ","?]* @class.inside
        "}")) @class.around

(union_specifier
    body: (_
        "{"
        (_)* @class.inside
        "}")) @class.around

(class_specifier
  body: (_
      "{"
      [(_) ":"? ";"?]* @class.inside
      "}"?)) @class.around
