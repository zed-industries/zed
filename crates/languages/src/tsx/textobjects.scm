(comment)+ @comment.around

(function_declaration
    body: (_
        "{"
        (_)* @function.inside
        "}")) @function.around

(method_definition
    body: (_
        "{"
        (_)* @function.inside
        "}")) @function.around

(function_expression
    body: (_
        "{"
        (_)* @function.inside
        "}")) @function.around

(arrow_function
    body: (statement_block
        "{"
        (_)* @function.inside
        "}")) @function.around

(arrow_function) @function.around
(function_signature) @function.around

(generator_function
    body: (_
        "{"
        (_)* @function.inside
        "}")) @function.around

(generator_function_declaration
    body: (_
        "{"
        (_)* @function.inside
        "}")) @function.around

(class_declaration
    body: (_
        "{"
        [(_) ";"?]* @class.inside
        "}" )) @class.around

(class
    body: (_
        "{"
        (_)* @class.inside
        "}" )) @class.around

(interface_declaration
    body: (_
        "{"
        [(_) ";"?]* @class.inside
        "}" )) @class.around

(enum_declaration
    body: (_
        "{"
        [(_) ","?]* @class.inside
        "}" )) @class.around

(ambient_declaration
    (module
    body: (_
        "{"
        [(_) ";"?]* @class.inside
        "}" ))) @class.around

(internal_module
    body: (_
        "{"
        [(_) ";"?]* @class.inside
        "}" )) @class.around

(type_alias_declaration) @class.around
