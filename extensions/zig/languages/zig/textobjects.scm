(function_declaration
  (_) @function.inside) @function.around

(test_declaration
  (_) @test.inside) @test.around

(struct_declaration
  (_) @class.inside) @class.around

(union_declaration
  (_) @class.inside) @class.around

(enum_declaration
  (_) @class.inside) @class.around

(parameters
  ((_) @parameter.inside . ","? @parameter.around) @parameter.around)

(comment)+ @comment.around
