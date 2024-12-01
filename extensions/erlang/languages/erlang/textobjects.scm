(function_clause
  args: (expr_args (_)? @parameter.inside)
  body: (_) @function.inside) @function.around

(anonymous_fun
  (fun_clause body: (_) @function.inside)) @function.around

(comment) @comment.around

; EUnit test names.
; (CommonTest cases are not recognizable by syntax alone.)
((function_clause
   name: (atom) @_name
   args: (expr_args (_)? @parameter.inside)
   body: (_) @test.inside) @test.around
 (#match? @_name "_test$"))
