(comment)+ @comment.around

(function_definition
  body: (_) @function.inside) @function.around

(class_definition
  body: (_) @class.inside) @class.around
