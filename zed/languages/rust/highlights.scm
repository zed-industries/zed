(type_identifier) @type

(field_identifier) @property

(call_expression
  function: [
    (identifier) @function
    (scoped_identifier
      name: (identifier) @function)
    (field_expression
      field: (field_identifier) @function.method)
  ])

(function_item
  name: (identifier) @function.definition)

[
  "async"
  "break"
  "const"
  "continue"
  "dyn"
  "else"
  "enum"
  "for"
  "fn"
  "if"
  "impl"
  "let"
  "loop"
  "match"
  "mod"
  "move"
  "pub"
  "return"
  "struct"
  "trait"
  "type"
  "use"
  "where"
  "while"
] @keyword

(string_literal) @string

[
  (line_comment)
  (block_comment)
] @comment
