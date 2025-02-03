(function_definition
  body: (_) @function.inside) @function.around

(function_declaration
  body: (_) @function.inside) @function.around

(comment)+ @comment.around
