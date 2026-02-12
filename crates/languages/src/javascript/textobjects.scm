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

((arrow_function
    body: (statement_block
        "{"
        (_)* @function.inside
        "}")) @function.around
 (#not-has-parent? @function.around variable_declarator))

; Arrow function in variable declaration - capture the full declaration
([
    (lexical_declaration
        (variable_declarator
            value: (arrow_function
                body: (statement_block
                    "{"
                    (_)* @function.inside
                    "}"))))
    (variable_declaration
        (variable_declarator
            value: (arrow_function
                body: (statement_block
                    "{"
                    (_)* @function.inside
                    "}"))))
]) @function.around

; Arrow function in variable declaration (captures body for expression-bodied arrows)
([
    (lexical_declaration
        (variable_declarator
            value: (arrow_function
                body: (_) @function.inside)))
    (variable_declaration
        (variable_declarator
            value: (arrow_function
                body: (_) @function.inside)))
]) @function.around

; Catch-all for arrow functions in other contexts (callbacks, etc.)
((arrow_function
    body: (_) @function.inside) @function.around
 (#not-has-parent? @function.around variable_declarator))

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
        [(_) ";"?]* @class.inside
        "}" )) @class.around
