;; Methods
(method_declaration name: (identifier) @function)
(local_function_statement name: (identifier) @function)

;; Types
(interface_declaration name: (identifier) @type)
(class_declaration name: (identifier) @type)
(enum_declaration name: (identifier) @type)
(struct_declaration (identifier) @type)
(record_declaration (identifier) @type)
(record_struct_declaration (identifier) @type)
(namespace_declaration name: (identifier) @type)

(constructor_declaration name: (identifier) @constructor)
(destructor_declaration name: (identifier) @constructor)

[
  (implicit_type)
  (predefined_type)
] @type.builtin

(_ type: (identifier) @type)

;; Enum
(enum_member_declaration (identifier) @property)

;; Literals
[
  (real_literal)
  (integer_literal)
] @number

[
  (character_literal)
  (string_literal)
  (verbatim_string_literal)
  (interpolated_string_text)
  (interpolated_verbatim_string_text)
  "\""
  "$\""
  "@$\""
  "$@\""
 ] @string

[
  (boolean_literal)
  (null_literal)
] @constant

;; Comments
(comment) @comment

;; Tokens
[
  ";"
  "."
  ","
] @punctuation.delimiter

[
  "--"
  "-"
  "-="
  "&"
  "&="
  "&&"
  "+"
  "++"
  "+="
  "<"
  "<="
  "<<"
  "<<="
  "="
  "=="
  "!"
  "!="
  "=>"
  ">"
  ">="
  ">>"
  ">>="
  ">>>"
  ">>>="
  "|"
  "|="
  "||"
  "?"
  "??"
  "??="
  "^"
  "^="
  "~"
  "*"
  "*="
  "/"
  "/="
  "%"
  "%="
  ":"
] @operator

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
]  @punctuation.bracket

;; Keywords
(modifier) @keyword
(this_expression) @keyword
(escape_sequence) @keyword

[
  "add"
  "alias"
  "as"
  "base"
  "break"
  "case"
  "catch"
  "checked"
  "class"
  "continue"
  "default"
  "delegate"
  "do"
  "else"
  "enum"
  "event"
  "explicit"
  "extern"
  "finally"
  "for"
  "foreach"
  "global"
  "goto"
  "if"
  "implicit"
  "interface"
  "is"
  "lock"
  "namespace"
  "notnull"
  "operator"
  "params"
  "return"
  "remove"
  "sizeof"
  "stackalloc"
  "static"
  "struct"
  "switch"
  "throw"
  "try"
  "typeof"
  "unchecked"
  "using"
  "while"
  "new"
  "await"
  "in"
  "yield"
  "get"
  "set"
  "when"
  "out"
  "ref"
  "from"
  "where"
  "select"
  "record"
  "init"
  "with"
  "let"
] @keyword


;; Linq
(from_clause (identifier) @variable)
(group_clause (identifier) @variable)
(order_by_clause (identifier) @variable)
(join_clause (identifier) @variable)
(select_clause (identifier) @variable)
(query_continuation (identifier) @variable) @keyword

;; Record
(with_expression
  (with_initializer_expression
    (simple_assignment_expression
      (identifier) @variable)))

;; Exprs
(binary_expression (identifier) @variable (identifier) @variable)
(binary_expression (identifier)* @variable)
(conditional_expression (identifier) @variable)
(prefix_unary_expression (identifier) @variable)
(postfix_unary_expression (identifier)* @variable)
(assignment_expression (identifier) @variable)
(cast_expression (_) (identifier) @variable)

;; Class
(base_list (identifier) @type) ;; applies to record_base too
(property_declaration (generic_name))
(property_declaration
  name: (identifier) @variable)
(property_declaration
  name: (identifier) @variable)
(property_declaration
  name: (identifier) @variable)

;; Lambda
(lambda_expression) @variable

;; Attribute
(attribute) @attribute

;; Parameter
(parameter
  name: (identifier) @variable)
(parameter (identifier) @variable)
(parameter_modifier) @keyword

;; Variable declarations
(variable_declarator (identifier) @variable)
(for_each_statement left: (identifier) @variable)
(catch_declaration (_) (identifier) @variable)

;; Return
(return_statement (identifier) @variable)
(yield_statement (identifier) @variable)

;; Type
(generic_name (identifier) @type)
(type_parameter (identifier) @property)
(type_argument_list (identifier) @type)
(as_expression right: (identifier) @type)
(is_expression right: (identifier) @type)

;; Type constraints
(type_parameter_constraints_clause (identifier) @property)

;; Switch
(switch_statement (identifier) @variable)
(switch_expression (identifier) @variable)

;; Lock statement
(lock_statement (identifier) @variable)

;; Method calls
(invocation_expression (member_access_expression name: (identifier) @function))
