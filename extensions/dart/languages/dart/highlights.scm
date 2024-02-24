(dotted_identifier_list) @string

; Methods
; --------------------
(super) @function

; TODO: add method/call_expression to grammar and
; distinguish method call from variable access
(function_expression_body
  (identifier) @function)

; ((identifier)(selector (argument_part)) @function)
; NOTE: This query is a bit of a work around for the fact that the dart grammar doesn't
; specifically identify a node as a function call
(((identifier) @function
  (#lua-match? @function "^_?[%l]"))
  .
  (selector
    .
    (argument_part))) @function

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
  (#lua-match? @type "^[%u%l]"))

(type_identifier) @type

(type_alias
  (type_identifier) @type.definition)

; Variables
; --------------------
; var keyword
(inferred_type) @keyword

((identifier) @type
  (#lua-match? @type "^_?[%u].*[%l]")) ; catch Classes or IClasses not CLASSES

"Function" @type

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

(comment) @comment @spell

(documentation_comment) @comment.documentation @spell

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
  ; TODO:
  ; "rethrow" cannot be targeted at all and seems to be an invisible node
  ; TODO:
  ; the assert keyword cannot be specifically targeted
  ; because the grammar selects the whole node or the content
  ; of the assertion not just the keyword
  ; assert
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
    "abstract" "as" "covariant" "deferred" "dynamic" "export" "external" "factory" "Function" "get"
    "implements" "import" "interface" "library" "operator" "mixin" "part" "set" "static" "typedef"))

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
