[
  "break"
  "case"
  "const"
  "continue"
  "default"
  "do"
  "else"
  "enum"
  "extern"
  "for"
  "if"
  "inline"
  "return"
  "sizeof"
  "static"
  "struct"
  "switch"
  "typedef"
  "union"
  "volatile"
  "while"
] @keyword

[
  "#define"
  "#elif"
  "#else"
  "#endif"
  "#if"
  "#ifdef"
  "#ifndef"
  "#include"
  (preproc_directive)
] @keyword

[
  "--"
  "-"
  "-="
  "->"
  "="
  "!="
  "*"
  "&"
  "&&"
  "+"
  "++"
  "+="
  "<"
  "=="
  ">"
  "||"
] @operator

[
  "."
  ";"
] @punctuation.delimiter

[
  "{"
  "}"
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

[
  (string_literal)
  (system_lib_string)
  (char_literal)
] @string

(comment) @comment

(number_literal) @number

[
  (true)
  (false)
  (null)
] @constant

(identifier) @variable

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]*$"))

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function))
(function_declarator
  declarator: (identifier) @function)
(preproc_function_def
  name: (identifier) @function.special)

(field_identifier) @property
(statement_identifier) @label

[
  (type_identifier)
  (primitive_type)
  (sized_type_specifier)
] @type

