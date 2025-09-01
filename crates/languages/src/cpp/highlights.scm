(identifier) @variable
(field_identifier) @property
(namespace_identifier) @namespace

(concept_definition
    name: (identifier) @concept)

(requires_clause
    constraint: (template_type
        name: (type_identifier) @concept))

(module_name
  (identifier) @module)

(module_declaration
  name: (module_name
    (identifier) @module))

(import_declaration
  name: (module_name
    (identifier) @module))

(import_declaration
  partition: (module_partition
    (module_name
      (identifier) @module)))

(call_expression
  function: (qualified_identifier
    name: (identifier) @function))

(call_expression
  (qualified_identifier
    (identifier) @function.call))

(call_expression
  (qualified_identifier
    (qualified_identifier
      (identifier) @function.call)))

(call_expression
  (qualified_identifier
    (qualified_identifier
      (qualified_identifier
        (identifier) @function.call))))

((qualified_identifier
  (qualified_identifier
    (qualified_identifier
      (qualified_identifier
        (identifier) @function.call)))) @_parent
  (#has-ancestor? @_parent call_expression))

(call_expression
  function: (identifier) @function)

(call_expression
  function: (field_expression
    field: (field_identifier) @function))

(preproc_function_def
  name: (identifier) @function.special)

(template_function
  name: (identifier) @function)

(template_method
  name: (field_identifier) @function)

(function_declarator
  declarator: (identifier) @function)

(function_declarator
  declarator: (qualified_identifier
    name: (identifier) @function))

(function_declarator
  declarator: (field_identifier) @function)

(operator_name
  (identifier)? @operator) @function

(operator_name
  "<=>" @operator.spaceship)

(destructor_name (identifier) @function)

((namespace_identifier) @type
 (#match? @type "^[A-Z]"))

(auto) @type
(type_identifier) @type
type: (primitive_type) @type.builtin
(sized_type_specifier) @type.builtin

(attribute
    name: (identifier) @attribute)

((identifier) @constant.builtin
 (#match? @constant.builtin "^_*[A-Z][A-Z\\d_]*$"))

(statement_identifier) @label
(this) @variable.builtin
("static_assert") @function.builtin

[
  "alignas"
  "alignof"
  "break"
  "case"
  "catch"
  "class"
  "co_await"
  "co_return"
  "co_yield"
  "concept"
  "consteval"
  "constexpr"
  "constinit"
  "continue"
  "decltype"
  "default"
  "delete"
  "do"
  "else"
  "enum"
  "explicit"
  "export"
  "extern"
  "final"
  "for"
  "friend"
  "goto"
  "if"
  "import"
  "inline"
  "module"
  "namespace"
  "new"
  "noexcept"
  "operator"
  "override"
  "private"
  "protected"
  "public"
  "requires"
  "return"
  "sizeof"
  "struct"
  "switch"
  "template"
  "thread_local"
  "throw"
  "try"
  "typedef"
  "typename"
  "union"
  "using"
  "virtual"
  "while"
  (storage_class_specifier)
  (type_qualifier)
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
] @keyword.directive

(comment) @comment

[
  (true)
  (false)
] @boolean

[
  (null)
  ("nullptr")
] @constant.builtin

(number_literal) @number

[
  (string_literal)
  (system_lib_string)
  (char_literal)
  (raw_string_literal)
] @string

[
  ","
  ":"
  "::"
  ";"
  (raw_string_delimiter)
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
  "."
  ".*"
  "->*"
  "~"
  "-"
  "--"
  "-="
  "->"
  "="
  "!"
  "!="
  "|"
  "|="
  "||"
  "^"
  "^="
  "&"
  "&="
  "&&"
  "+"
  "++"
  "+="
  "*"
  "*="
  "/"
  "/="
  "%"
  "%="
  "<<"
  "<<="
  ">>"
  ">>="
  "<"
  "=="
  ">"
  "<="
  ">="
  "?"
  "and"
  "and_eq"
  "bitand"
  "bitor"
  "compl"
  "not"
  "not_eq"
  "or"
  "or_eq"
  "xor"
  "xor_eq"
] @operator

"<=>" @operator.spaceship

(binary_expression
  operator: "<=>" @operator.spaceship)

(conditional_expression ":" @operator)
(user_defined_literal (literal_suffix) @operator)
