(dotted_identifier_list) @string

; Methods
; --------------------
(super) @function

(function_expression_body (identifier) @type)
; ((identifier)(selector (argument_part)) @function)

(((identifier) @function (#match? @function "^_?[a-z]"))
 . (selector . (argument_part))) @function

; Annotations
; --------------------
(annotation
 name: (identifier) @attribute)

; Operators and Tokens
; --------------------
(template_substitution
 "$" @punctuation.special
 "{" @punctuation.special
 "}" @punctuation.special) @none

(template_substitution
 "$" @punctuation.special
 (identifier_dollar_escaped) @variable) @none

(escape_sequence) @string.escape

[
 "@"
 "=>"
 ".."
 "??"
 "=="
 "?"
 ":"
 "&&"
 "%"
 "<"
 ">"
 "="
 ">="
 "<="
 "||"
 (multiplicative_operator)
 (increment_operator)
 (is_operator)
 (prefix_operator)
 (equality_operator)
 (additive_operator)
 ] @operator

[
 "("
 ")"
 "["
 "]"
 "{"
 "}"
 ] @punctuation.bracket

; Delimiters
; --------------------
[
 ";"
 "."
 ","
 ] @punctuation.delimiter

; Types
; --------------------
(class_definition
 name: (identifier) @type)
(constructor_signature
 name: (identifier) @type)
(scoped_identifier
 scope: (identifier) @type)
(function_signature
 name: (identifier) @function.method)

(getter_signature
 (identifier) @function.method)

(setter_signature
 name: (identifier) @function.method)
(enum_declaration
 name: (identifier) @type)
(enum_constant
 name: (identifier) @type)
(void_type) @type

((scoped_identifier
  scope: (identifier) @type
  name: (identifier) @type)
 (#match? @type "^[a-zA-Z]"))

(type_identifier) @type

(type_alias
 (type_identifier) @type.definition)

; Variables
; --------------------
; var keyword
(inferred_type) @keyword

((identifier) @type
 (#match? @type "^_?[A-Z].*[a-z]"))

("Function" @type)

; properties
(unconditional_assignable_selector
 (identifier) @property)

(conditional_assignable_selector
 (identifier) @property)

; assignments
(assignment_expression
 left: (assignable_expression) @variable)

(this) @variable.builtin

; Parameters
; --------------------
(formal_parameter
 name: (identifier) @variable.parameter)

(named_argument
 (label
    (identifier) @variable.parameter))

; Literals
; --------------------
[
 (hex_integer_literal)
 (decimal_integer_literal)
 (decimal_floating_point_literal)
 ; TODO: inaccessible nodes
 ; (octal_integer_literal)
 ; (hex_floating_point_literal)
 ] @number

(symbol_literal) @string.special.symbol

(string_literal) @string
(true) @boolean
(false) @boolean
(null_literal) @constant.builtin

(comment) @comment

(documentation_comment) @comment.documentation

; Keywords
; --------------------
[
 "import"
 "library"
 "export"
 "as"
 "show"
 "hide"
 ] @keyword.import

; Reserved words (cannot be used as identifiers)
[
  (case_builtin)
 "late"
 "required"
 "extension"
 "on"
 "class"
 "enum"
 "extends"
 "in"
 "is"
 "new"
 "super"
 "with"
 ] @keyword

"return" @keyword.return

; Built in identifiers:
; alone these are marked as keywords
[
  "deferred"
 "factory"
 "get"
 "implements"
 "interface"
 "library"
 "operator"
 "mixin"
 "part"
 "set"
 "typedef"
 ] @keyword

[
  "async"
  "async*"
  "sync*"
  "await"
  "yield"
] @keyword.coroutine

[
 (const_builtin)
 (final_builtin)
 "abstract"
  "covariant"
 "dynamic"
 "external"
 "static"
 "final"
  "base"
  "sealed"
 ] @type.qualifier

; when used as an identifier:
((identifier) @variable.builtin
 (#any-of? @variable.builtin
           "abstract"
           "as"
           "covariant"
           "deferred"
           "dynamic"
           "export"
           "external"
           "factory"
           "Function"
           "get"
           "implements"
           "import"
           "interface"
           "library"
           "operator"
           "mixin"
           "part"
           "set"
           "static"
           "typedef"))

[
  "if"
  "else"
  "switch"
  "default"
] @keyword.conditional

[
  "try"
  "throw"
  "catch"
  "finally"
  (break_statement)
] @keyword.exception

[
  "do"
  "while"
  "continue"
  "for"
] @keyword.repeat
