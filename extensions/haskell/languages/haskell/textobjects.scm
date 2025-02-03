(comment)+ @comment.around

[
  (adt)
  (type_alias)
  (newtype)
] @class.around

(record_fields "{" (_)* @class.inside "}")

((signature)? (function)+) @function.around
(function rhs:(_) @function.inside)
