[
  (line_comment)
  (block_comment)
] @comment

; Keywords
;---------
[
  (module)
  (import)
  (exposing)
  (port)
  (type)
  (alias)
  (infix)
  (as)
  (case)
  (of)
  "if"
  "then"
  "else"
  "let"
  "in"
] @keyword

; Punctuation
;------------
(double_dot) @punctuation.special

[
  ","
  "|"
  (dot)
] @punctuation.delimiter

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

; Variables
;----------
(value_qid
  (lower_case_identifier) @variable)

(value_declaration
  (function_declaration_left
    (lower_case_identifier) @variable))

(type_annotation
  (lower_case_identifier) @variable)

(port_annotation
  (lower_case_identifier) @variable)

(record_base_identifier
  (lower_case_identifier) @variable)

(lower_pattern
  (lower_case_identifier) @variable)

(exposed_value
  (lower_case_identifier) @variable)

(value_qid
  ((dot)
    (lower_case_identifier) @variable))

(field_access_expr
  ((dot)
    (lower_case_identifier) @variable))

(function_declaration_left
  (lower_pattern
    (lower_case_identifier) @variable))

; Functions
;----------
(value_declaration
  functionDeclarationLeft: (function_declaration_left
    (lower_case_identifier) @function
    (pattern)))

(value_declaration
  functionDeclarationLeft: (function_declaration_left
    (lower_case_identifier) @function
    pattern: (_)))

(value_declaration
  functionDeclarationLeft: (function_declaration_left
    (lower_case_identifier) @function)
  body: (anonymous_function_expr))

(type_annotation
  name: (lower_case_identifier) @function
  typeExpression: (type_expression
    (arrow)))

(port_annotation
  name: (lower_case_identifier) @function
  typeExpression: (type_expression
    (arrow)))

(function_call_expr
  target: (value_expr
    (value_qid
      (lower_case_identifier) @function)))

; Operators
;----------
[
  (operator_identifier)
  (eq)
  (colon)
  (arrow)
  (backslash)
  "::"
] @operator

; Modules
;--------
(module_declaration
  (upper_case_qid
    (upper_case_identifier) @title))

(import_clause
  (upper_case_qid
    (upper_case_identifier) @title))

(as_clause
  (upper_case_identifier) @title)

(value_expr
  (value_qid
    (upper_case_identifier) @title))

; Types
;------
(type_declaration
  (upper_case_identifier) @type)

(type_ref
  (upper_case_qid
    (upper_case_identifier) @type))

(type_variable
  (lower_case_identifier) @type)

(lower_type_name
  (lower_case_identifier) @type)

(exposed_type
  (upper_case_identifier) @type)

(type_alias_declaration
  (upper_case_identifier) @type)

(field_type
  name: (lower_case_identifier) @property)

(field
  name: (lower_case_identifier) @property)

(type_declaration
  (union_variant
    (upper_case_identifier) @constructor))

(nullary_constructor_argument_pattern
  (upper_case_qid
    (upper_case_identifier) @constructor))

(union_pattern
  (upper_case_qid
    (upper_case_identifier) @constructor))

(value_expr
  (upper_case_qid
    (upper_case_identifier)) @constructor)

; Literals
;---------
(number_constant_expr
  (number_literal) @number)

(upper_case_qid
  ((upper_case_identifier) @boolean
    (#any-of? @boolean "True" "False")))

[
  (open_quote)
  (close_quote)
  (open_char)
  (close_char)
] @string

(string_constant_expr
  (string_escape) @string)

(string_constant_expr
  (regular_string_part) @string)

(char_constant_expr
  (string_escape) @string)

(char_constant_expr
  (regular_string_part) @string)
