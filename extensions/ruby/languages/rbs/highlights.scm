; Taken from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/rbs/highlights.scm
; Use directive
(use_clause
  [
    (type_name)
    (simple_type_name)
  ] @type)

; Builtin constants and Keywords
[
  "true"
  "false"
] @boolean

"nil" @constant.builtin

[
  "use"
  "as"
  "module"
  "def"
  "attr_reader"
  "attr_writer"
  "attr_accessor"
  "end"
  "alias"
] @keyword

[
  "interface"
  "type"
  "class"
] @keyword.type

(class_decl
  "end" @keyword.type)

(interface_decl
  "end" @keyword.type)

"def" @keyword.function

; Members of declaration
[
  "include"
  "extend"
  "prepend"
] @function.method

(visibility) @keyword.modifier

(comment) @comment

(method_member
  (method_name
    [
      (identifier)
      (constant)
      (operator)
      (setter)
    ] @function.method))

[
  (ivar_name)
  (cvar_name)
] @variable.member

(alias_member
  (method_name) @function)

(class_name
  (constant) @type)

(module_name
  (constant) @type)

(interface_name
  (interface) @type)

(alias_name
  (identifier) @type)

(type_variable) @constant

(namespace
  (constant) @module)

(builtin_type) @type.builtin

(const_name
  (constant) @constant)

(global_name) @variable

; Standard Arguments
(parameter
  (var_name) @variable.parameter)

; Keyword Arguments
(keyword) @variable.parameter

; Self
(self) @variable.builtin

; Literal
(type
  (symbol_literal) @string.special.symbol)

(type
  (string_literal
    (escape_sequence) @string.escape))

(type
  (string_literal) @string)

(type
  (integer_literal) @number)

; Operators
[
  "="
  "->"
  "<"
  "**"
  "*"
  "&"
  "|"
  "^"
] @operator

; Punctuation
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  "."
] @punctuation.delimiter
