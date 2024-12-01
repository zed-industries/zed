(comment) @comment.inside

[
  (adt)
  (type_alias)
  (newtype)
] @class.around

((signature)? (function rhs:(_) @function.inside)) @function.around
(exp_lambda) @function.around

(adt (type_variable) @parameter.inside)
(patterns (_) @parameter.inside)
