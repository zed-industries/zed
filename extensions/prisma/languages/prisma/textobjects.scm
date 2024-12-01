(model_declaration
  ((statement_block) @class.inside)) @class.around

(call_expression
  (arguments (_) @parameter.inside . ","? @parameter.around) @parameter.around)

(column_declaration) @entry.around

(array (_) @entry.around)

(assignment_expression
  (_) @entry.inside) @entry.around

(developer_comment) @comment.inside

(developer_comment)+ @comment.around
