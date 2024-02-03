(function_declaration (function_body) @function.inside) @function.around
(comment) @comment.inside
(comment)+ @comment.around
(class_declaration (aggregate_body) @class.inside) @class.around
(interface_declaration (aggregate_body) @class.inside) @class.around
(struct_declaration (aggregate_body) @class.inside) @class.around
(unittest_declaration (block_statement) @test.inside) @test.around
(parameter) @parameter.inside
(template_parameter) @parameter.inside