; Preproc
(control_key) @keyword.directive

; Namespace
(namespace) @module

; Includes
"use" @keyword.import

; Builtins
(primitive) @type.builtin

[
  "enum"
  "intEnum"
  "list"
  "map"
  "set"
] @type.builtin

; Fields (Members)
; (field) @variable.member
(key_identifier) @variable.member

(shape_member
  (field) @variable.member)

(operation_field) @variable.member

(operation_error_field) @variable.member

; Constants
(enum_member
  (enum_field) @constant)

; Types
(identifier) @type

(structure_resource
  (shape_id) @type)

; Attributes
(mixins
  (shape_id) @attribute)

(trait_statement
  (shape_id
    (#set! "priority" 105)) @attribute)

; Operators
[
  "@"
  "-"
  "="
  ":="
] @operator

; Keywords
[
  "namespace"
  "service"
  "structure"
  "operation"
  "union"
  "resource"
  "metadata"
  "apply"
  "for"
  "with"
] @keyword

; Literals
(string) @string

(escape_sequence) @string.escape

(number) @number

(float) @number.float

(boolean) @boolean

(null) @constant.builtin

; Misc
[
  "$"
  "#"
] @punctuation.special

[
  "{"
  "}"
] @punctuation.bracket

[
  "("
  ")"
] @punctuation.bracket

[
  "["
  "]"
] @punctuation.bracket

[
  ":"
  "."
] @punctuation.delimiter

; Comments
(comment) @comment @spell

(documentation_comment) @comment.documentation @spell
