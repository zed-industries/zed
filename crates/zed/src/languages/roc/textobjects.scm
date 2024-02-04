(anon_fun_expr
  (expr_body) @function.inside
) @function.around

(argument_patterns
  ((_) @parameter.inside . ","? @parameter.around) @parameter.around
)

(function_type
  ((_) @parameter.inside . ","? @parameter.around) @parameter.around(#not-eq? @parameter.inside "->")
)

(function_call_expr
  .
  (_)
  (parenthesized_expr (expr_body) @parameter.inside) @parameter.around
)

(function_call_expr
  .
  (_) ((_) @parameter.inside) @parameter.around
)

[
  (annotation_type_def ) @class.inside
  (alias_type_def ) @class.inside
  (opaque_type_def ) @class.inside
] @class.around

(apply_type_arg) @parameter.inside

(expect
  (expr_body) @test.inside
) @test.around

(line_comment) @comment.around
(doc_comment) @comment.around


