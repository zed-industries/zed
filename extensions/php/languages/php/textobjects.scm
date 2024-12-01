(class_declaration
  body: (_) @class.inside) @class.around

(interface_declaration
  body: (_) @class.inside) @class.around

(trait_declaration
  body: (_) @class.inside) @class.around

(enum_declaration
  body: (_) @class.inside) @class.around

(function_definition
  body: (_) @function.inside) @function.around

(method_declaration
  body: (_) @function.inside) @function.around

(arrow_function
  body: (_) @function.inside) @function.around

(anonymous_function_creation_expression
  body: (_) @function.inside) @function.around

(anonymous_function_use_clause
  ((_) @parameter.inside . ","? @parameter.around) @parameter.around)

(formal_parameters
  ([
    (simple_parameter)
    (variadic_parameter)
    (property_promotion_parameter)
  ] @parameter.inside . ","? @parameter.around) @parameter.around)

(arguments
  ((_) @parameter.inside . ","? @parameter.around) @parameter.around)

(comment) @comment.inside

(comment)+ @comment.around

(array_creation_expression
  (array_element_initializer
    (_) @entry.inside
  ) @entry.around @entry.movement)

(list_literal
  (_) @entry.inside @entry.around @entry.movement)

[
  (enum_case)
] @entry.around @entry.movement
