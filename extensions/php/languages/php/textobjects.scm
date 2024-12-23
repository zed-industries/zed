(function_definition
    body: (_
        "{"
        (_)* @function.inside
        "}" )) @function.around

(method_declaration
    body: (_
        "{"
        (_)* @function.inside
        "}" )) @function.around

(method_declaration) @function.around

(class_declaration
    body: (_
        "{"
        (_)* @class.inside
        "}")) @class.around

(interface_declaration
    body: (_
        "{"
        (_)* @class.inside
        "}")) @class.around

(trait_declaration
    body: (_
        "{"
        (_)* @class.inside
        "}")) @class.around

(enum_declaration
    body: (_
        "{"
        (_)* @class.inside
        "}")) @class.around

(namespace_definition
    body: (_
        "{"
        (_)* @class.inside
        "}")) @class.around

(comment)+ @comment.around
