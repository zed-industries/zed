(function_clause
  body: (_ "->" (_)* @function.inside)) @function.around

(type_alias ty: (_) @class.inside) @class.around

(comment)+ @comment.around
