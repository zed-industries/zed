(model_declaration
  (statement_block
      "{"
      (_)* @class.inside
      "}")) @class.around

(datasource_declaration
  (statement_block
      "{"
      (_)* @class.inside
      "}")) @class.around

(generator_declaration
  (statement_block
      "{"
      (_)* @class.inside
      "}")) @class.around

(enum_declaration
  (enum_block
      "{"
      (_)* @class.inside
      "}")) @class.around

(developer_comment)+ @comment.around
