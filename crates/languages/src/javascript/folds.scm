(arguments) @fold
(for_in_statement body: (_) @fold)
(for_statement body: (_) @fold)
(while_statement body: (_) @fold)
(arrow_function body: (_) @fold)
(function_expression body: (statement_block) @fold)
(function_declaration body: (statement_block) @fold)
(class_declaration body: (class_body) @fold)
(method_definition body: (statement_block) @fold)
(do_statement body: (_) @fold)
(with_statement body: (_) @fold)
(switch_statement body: (switch_body) @fold)
(switch_case) @fold
(switch_default) @fold
((import_statement)+ @fold)
(if_statement consequence: (_) @fold)
(try_statement body: (statement_block) @fold)
(catch_clause body: (statement_block) @fold)
(array) @fold
(object) @fold
(generator_function body: (statement_block) @fold)
(generator_function_declaration body: (statement_block) @fold)
(comment)+ @fold
(jsx_element) @fold
