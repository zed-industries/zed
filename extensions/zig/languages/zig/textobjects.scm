(function_declaration
    body: (_
        "{"
        (_)* @function.inside
        "}")) @function.around

(test_declaration
    (block
        "{"
        (_)* @function.inside
        "}")) @function.around

(variable_declaration
  (struct_declaration
    "struct"
    "{"
    [(_) ","]* @class.inside
    "}")) @class.around

(variable_declaration
  (enum_declaration
    "enum"
    "{"
    (_)* @class.inside
    "}")) @class.around

(comment)+ @comment.around
