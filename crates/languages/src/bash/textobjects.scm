(function_definition
  body: (_) @function.inside) @function.around

(command
  argument: (_) @parameter.inside)

(comment) @comment.inside

(comment)+ @comment.around

(array
  (_) @entry.around)
