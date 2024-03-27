; CREDITS @stumash (stuart.mashaal@gmail.com)

(class_definition
  name: (identifier) @type)

(enum_definition
  name: (identifier) @enum)

(object_definition
  name: (identifier) @type)

(trait_definition
  name: (identifier) @type)

(full_enum_case
  name: (identifier) @type)

(simple_enum_case
  name: (identifier) @type)

;; variables

(class_parameter
  name: (identifier) @property)

(self_type (identifier) @property)

(interpolation (identifier) @none)
(interpolation (block) @none)

;; types

(type_definition
  name: (type_identifier) @type.definition)

(type_identifier) @type

;; val/var definitions/declarations

(val_definition
  pattern: (identifier) @variable)

(var_definition
  pattern: (identifier) @variable)

(val_declaration
  name: (identifier) @variable)

(var_declaration
  name: (identifier) @variable)

; method definition

(function_declaration
      name: (identifier) @function.method)

(function_definition
      name: (identifier) @function.method)

; imports/exports
((import_declaration
  path: (identifier) @type) (#match? @type "^[A-Z]"))

(import_declaration
  path: (identifier) @namespace)

((stable_identifier (identifier) @type) (#match? @type "^[A-Z]"))

((stable_identifier (identifier) @namespace))

(export_declaration
  path: (identifier) @namespace)
((stable_identifier (identifier) @namespace))

((export_declaration
  path: (identifier) @type) (#match? @type "^[A-Z]"))
((stable_identifier (identifier) @type) (#match? @type "^[A-Z]"))

((namespace_selectors (identifier) @type) (#match? @type "^[A-Z]"))

; method invocation

(call_expression
  function: (identifier) @function)

(call_expression
  function: (operator_identifier) @function)

(call_expression
  function: (field_expression
    field: (identifier) @function.method))

((call_expression
   function: (identifier) @constructor)
 (#match? @constructor "^[A-Z]"))

(generic_function
  function: (identifier) @function)

(interpolated_string_expression
  interpolator: (identifier) @function)

; function definitions

(function_definition
  name: (identifier) @function)

(parameter
  name: (identifier) @parameter)

(binding
  name: (identifier) @parameter)

; expressions

(field_expression field: (identifier) @property)
(field_expression value: (identifier) @type
 (#match? @type "^[A-Z]"))

(infix_expression operator: (identifier) @operator)
(infix_expression operator: (operator_identifier) @operator)
(infix_type operator: (operator_identifier) @operator)
(infix_type operator: (operator_identifier) @operator)

; literals

(boolean_literal) @boolean
(integer_literal) @number
(floating_point_literal) @float

[
  (symbol_literal)
  (string)
  (character_literal)
  (interpolated_string_expression)
] @string

(interpolation "$" @punctuation.special)

;; keywords

(opaque_modifier) @type.qualifier
(infix_modifier) @keyword
(transparent_modifier) @type.qualifier
(open_modifier) @type.qualifier

[
  "case"
  "class"
  "enum"
  "extends"
  "derives"
  "finally"
;; `forSome` existential types not implemented yet
;; `macro` not implemented yet
  "object"
  "override"
  "package"
  "trait"
  "type"
  "val"
  "var"
  "with"
  "given"
  "using"
  "end"
  "implicit"
  "extension"
  "with"
] @keyword

[
  "abstract"
  "final"
  "lazy"
  "sealed"
  "private"
  "protected"
] @type.qualifier

(inline_modifier) @label

(null_literal) @constant

(wildcard) @parameter

(annotation) @attribute

;; special keywords

"new" @operator

[
  "else"
  "if"
  "match"
  "then"
] @keyword

[
 "("
 ")"
 "["
 "]"
 "{"
 "}"
]  @punctuation.bracket

[
 "."
 ","
] @punctuation.delimiter

[
  "do"
  "for"
  "while"
  "yield"
] @keyword

"def" @keyword

[
 "=>"
 "<-"
 "@"
] @operator

["import" "export"] @keyword ; @include

[
  "try"
  "catch"
  "throw"
] @keyword

"return" @keyword

[
  (comment)
  (block_comment)
  "_end_ident"
] @comment

;; `case` is a conditional keyword in case_block

(case_block
  (case_clause ("case") @keyword))
(indented_cases
  (case_clause ("case") @keyword))

(operator_identifier) @operator

((identifier) @type (#match? @type "^[A-Z]"))
((identifier) @variable.special
 (#match? @variable.special "^this$"))

(
  (identifier) @function
  (#match? @function "^super$")
)

;; Scala CLI using directives
(using_directive_key) @parameter
(using_directive_value) @string