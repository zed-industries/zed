(identifier) @variable

((identifier) @keyword
  (#eq? @keyword "value")
  (#has-ancestor? @keyword accessor_declaration))

((identifier) @variable.builtin
  (#eq? @variable.builtin "_"))

(method_declaration
  name: (identifier) @function.method)

(local_function_statement
  name: (identifier) @function.method)

(method_declaration
  type: (identifier) @type)

(declaration_pattern
  type: (identifier) @type)

(local_function_statement
  type: (identifier) @type)

(interpolation) @none

(member_access_expression
  name: (identifier) @variable.member)

(invocation_expression
  (member_access_expression
    name: (identifier) @function.method.call))

(invocation_expression
  function:
    (conditional_access_expression
      (member_binding_expression
        name: (identifier) @function.method.call)))

(namespace_declaration
  name:
    [
      (qualified_name)
      (identifier)
    ] @module)

(qualified_name
  (identifier) @type)

(namespace_declaration
  name:
    [
      (qualified_name
        (identifier) @module)
      (identifier) @module
    ])

(invocation_expression
  (identifier) @function.method.call)

(field_declaration
  (variable_declaration
    (variable_declarator
      (identifier) @variable.member)))

(initializer_expression
  (assignment_expression
    left: (identifier) @variable.member))

(parameter_list
  (parameter
    name: (identifier) @variable.parameter))

(implicit_parameter_list
  (parameter
    name: (identifier) @variable.parameter))

(parameter_list
  (parameter
    type: (identifier) @type))

(integer_literal) @number

(real_literal) @number.float

(null_literal) @constant.builtin

(character_literal) @character

[
  (string_literal)
  (verbatim_string_literal)
  (interpolated_string_expression)
] @string

(boolean_literal) @boolean

(predefined_type) @type.builtin

(implicit_type) @keyword

(comment) @comment @spell

((comment) @comment.documentation
  (#lua-match? @comment.documentation "^/[*][*][^*].*[*]/$"))

((comment) @comment.documentation
  (#lua-match? @comment.documentation "^///[^/]"))

((comment) @comment.documentation
  (#lua-match? @comment.documentation "^///$"))

(using_directive
  (identifier) @type)

(using_directive
  (name_equals
    (identifier) @type.definition))

(property_declaration
  name: (identifier) @property)

(property_declaration
  type: (identifier) @type)

(nullable_type
  (identifier) @type)

(catch_declaration
  type: (identifier) @type)

(interface_declaration
  name: (identifier) @type)

(class_declaration
  name: (identifier) @type)

(record_declaration
  name: (identifier) @type)

(enum_declaration
  name: (identifier) @type)

(constructor_declaration
  name: (identifier) @constructor)

(constructor_initializer
  "base" @constructor)

(variable_declaration
  (identifier) @type)

(object_creation_expression
  (identifier) @type)

; Generic Types.
(type_of_expression
  (generic_name
    (identifier) @type))

(type_argument_list
  (generic_name
    (identifier) @type))

(base_list
  (generic_name
    (identifier) @type))

(type_constraint
  (generic_name
    (identifier) @type))

(object_creation_expression
  (generic_name
    (identifier) @type))

(property_declaration
  (generic_name
    (identifier) @type))

(_
  type:
    (generic_name
      (identifier) @type))

; Generic Method invocation with generic type
(invocation_expression
  function:
    (generic_name
      .
      (identifier) @function.method.call))

(invocation_expression
  (member_access_expression
    (generic_name
      (identifier) @function.method)))

(base_list
  (identifier) @type)

(type_argument_list
  (identifier) @type)

(type_parameter_list
  (type_parameter) @type)

(type_parameter_constraints_clause
  target: (identifier) @type)

(attribute
  name: (identifier) @attribute)

(for_each_statement
  type: (identifier) @type)

(tuple_element
  type: (identifier) @type)

(tuple_expression
  (argument
    (declaration_expression
      type: (identifier) @type)))

(as_expression
  right: (identifier) @type)

(type_of_expression
  (identifier) @type)

(name_colon
  (identifier) @variable.parameter)

(warning_directive) @comment.warning

(error_directive) @keyword.exception

(define_directive
  (identifier) @constant) @constant.macro

(undef_directive
  (identifier) @constant) @constant.macro

(line_directive) @constant.macro

(line_directive
  (preproc_integer_literal) @constant
  (preproc_string_literal)? @string)

(pragma_directive
  (identifier) @constant) @constant.macro

(pragma_directive
  (preproc_string_literal) @string) @constant.macro

[
  (nullable_directive)
  (region_directive)
  (endregion_directive)
] @constant.macro

[
  "if"
  "else"
  "switch"
  "break"
  "case"
  "when"
  (if_directive)
  (elif_directive)
  (else_directive)
  (endif_directive)
] @keyword.conditional

(if_directive
  (identifier) @constant)

(elif_directive
  (identifier) @constant)

[
  "while"
  "for"
  "do"
  "continue"
  "goto"
  "foreach"
] @keyword.repeat

[
  "try"
  "catch"
  "throw"
  "finally"
] @keyword.exception

[
  "+"
  "?"
  ":"
  "++"
  "-"
  "--"
  "&"
  "&&"
  "|"
  "||"
  "!"
  "!="
  "=="
  "*"
  "/"
  "%"
  "<"
  "<="
  ">"
  ">="
  "="
  "-="
  "+="
  "*="
  "/="
  "%="
  "^"
  "^="
  "&="
  "|="
  "~"
  ">>"
  ">>>"
  "<<"
  "<<="
  ">>="
  ">>>="
  "=>"
  "??"
  "??="
] @operator

[
  ";"
  "."
  ","
  ":"
] @punctuation.delimiter

(conditional_expression
  [
    "?"
    ":"
  ] @keyword.conditional.ternary)

[
  "["
  "]"
  "{"
  "}"
  "("
  ")"
] @punctuation.bracket

(type_argument_list
  [
    "<"
    ">"
  ] @punctuation.bracket)

[
  (this_expression)
  (base_expression)
  "this"
] @variable.builtin

[
  "using"
  "as"
] @keyword.import

(alias_qualified_name
  (identifier
    "global") @keyword.import)

[
  "with"
  "new"
  "typeof"
  "sizeof"
  "is"
  "and"
  "or"
  "not"
  "stackalloc"
  "in"
  "out"
  "ref"
] @keyword.operator

[
  "lock"
  "params"
  "operator"
  "default"
  "implicit"
  "explicit"
  "override"
  "class"
  "delegate"
  "enum"
  "interface"
  "namespace"
  "struct"
  "get"
  "set"
  "init"
  "where"
  "record"
  "event"
  "add"
  "remove"
  "checked"
  "unchecked"
  "fixed"
  "alias"
] @keyword

[
  "async"
  "await"
] @keyword.coroutine

[
  "const"
  "extern"
  "readonly"
  "static"
  "volatile"
  "required"
] @keyword.storage

[
  "abstract"
  "private"
  "protected"
  "internal"
  "public"
  "partial"
  "sealed"
  "virtual"
] @type.qualifier

(parameter_modifier) @operator

(query_expression
  (_
    [
      "from"
      "orderby"
      "select"
      "group"
      "by"
      "ascending"
      "descending"
      "equals"
      "let"
    ] @keyword))

[
  "return"
  "yield"
] @keyword.return
