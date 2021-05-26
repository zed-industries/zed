(type_identifier) @type
(primitive_type) @type.builtin

(field_identifier) @property

(call_expression
  function: [
    (identifier) @function
    (scoped_identifier
      name: (identifier) @function)
    (field_expression
      field: (field_identifier) @function.method)
  ])

(function_item name: (identifier) @function.definition)
(function_signature_item name: (identifier) @function.definition)

[
  "async"
  "break"
  "const"
  "continue"
  "default"
  "dyn"
  "else"
  "enum"
  "extern"
  "for"
  "fn"
  "if"
  "in"
  "impl"
  "let"
  "loop"
  "macro_rules!"
  "match"
  "mod"
  "move"
  "pub"
  "return"
  "static"
  "struct"
  "trait"
  "type"
  "use"
  "where"
  "while"
  "union"
  "unsafe"
  (mutable_specifier)
  (super)
] @keyword

[
  (string_literal)
  (raw_string_literal)
  (char_literal)
] @string

[
  (line_comment)
  (block_comment)
] @comment
