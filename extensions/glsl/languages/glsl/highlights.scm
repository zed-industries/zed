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
  "."
  ";"
] @operator

[
  (string_literal)
  (system_lib_string)
] @string

(null) @constant.builtin

[
  (number_literal)
  (char_literal)
] @number

(identifier) @variable

(field_identifier) @property

(statement_identifier) @label

[
  (type_identifier)
  (primitive_type)
  (sized_type_specifier)
] @type

(call_expression
  function: (identifier) @function)

(call_expression
  function: (field_expression
    field: (field_identifier) @function))

(function_declarator
  declarator: (identifier) @function)

(preproc_function_def
  name: (identifier) @function.special)

((identifier) @constant
  (#match? @constant "^[A-Z][A-Z\\d_]*$"))

(comment) @comment

[
  "in"
  "out"
  "inout"
  "uniform"
  "shared"
  "layout"
  "attribute"
  "varying"
  "buffer"
  "coherent"
  "readonly"
  "writeonly"
  "precision"
  "highp"
  "mediump"
  "lowp"
  "centroid"
  "sample"
  "patch"
  "smooth"
  "flat"
  "noperspective"
  "invariant"
  "precise"
] @type.qualifier

"subroutine" @keyword.function

(extension_storage_class) @storageclass

((identifier) @variable.builtin
  (#match? @variable.builtin "^gl_"))
