(identifier) @variable

(type_identifier) @type

(type_spec
  name: (type_identifier) @type.definition)

((type_identifier) @type.builtin
  (#any-of? @type.builtin
    "any" "bool" "byte" "comparable" "complex64" "complex128" "error" "float32" "float64" "int"
    "int8" "int16" "int32" "int64" "rune" "string" "uint" "uint8" "uint16" "uint32" "uint64"
    "uintptr"))

; Type parameters (Go generics, since 1.18). In tree-sitter-go the parameter
; name is an `identifier` node (not `type_identifier`), so this rule sits after
; the `(identifier) @variable` catch-all above to override it. The constraint
; type (e.g. `int` in `[T int]`) is already captured by `(type_identifier) @type`.
(type_parameter_declaration
  name: (identifier) @type.parameter)

(field_identifier) @property

(package_identifier) @namespace

(label_name) @label

(keyed_element
  .
  (literal_element
    (identifier) @property))

(call_expression
  function: (identifier) @function.call)

(call_expression
  function: (selector_expression
    field: (field_identifier) @function.method.call))

((call_expression
  function: (identifier) @function.builtin)
  (#any-of? @function.builtin
    "append" "cap" "clear" "close" "complex" "copy" "delete" "imag" "len" "make" "max" "min" "new"
    "panic" "print" "println" "real" "recover"))

(function_declaration
  name: (identifier) @function)

(method_declaration
  name: (field_identifier) @function.method)

(method_elem
  name: (field_identifier) @function.method)

[
  ";"
  "."
  ","
  ":"
] @punctuation.delimiter

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

[
  "--"
  "-"
  "-="
  ":="
  "!"
  "!="
  "..."
  "*"
  "*="
  "/"
  "/="
  "&"
  "&&"
  "&="
  "%"
  "%="
  "^"
  "^="
  "+"
  "++"
  "+="
  "<-"
  "<"
  "<<"
  "<<="
  "<="
  "="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "|"
  "|="
  "||"
  "~"
] @operator

[
  "break"
  "case"
  "chan"
  "const"
  "continue"
  "default"
  "defer"
  "else"
  "fallthrough"
  "for"
  "func"
  "go"
  "goto"
  "if"
  "import"
  "interface"
  "map"
  "package"
  "range"
  "return"
  "select"
  "struct"
  "switch"
  "type"
  "var"
] @keyword

[
  (interpreted_string_literal)
  (raw_string_literal)
  (rune_literal)
] @string

(escape_sequence) @string.escape

; Highlight struct tags (e.g. `json:"name,omitempty"`) as special strings. They
; sit on `field_declaration` nodes and are almost always interpreted string
; literals, but raw string literals are also valid.
(field_declaration
  tag: [
    (interpreted_string_literal)
    (raw_string_literal)
  ] @string.special)

; Highlight import paths as special strings.
; This covers both plain (`import "fmt"`) and aliased (`import foo "bar/pkg"`)
; import specifications. The `path` field is always a string literal.
(import_spec
  path: [
    (interpreted_string_literal)
    (raw_string_literal)
  ] @string.special)

[
  (int_literal)
  (float_literal)
  (imaginary_literal)
] @number

(const_spec
  name: (identifier) @constant)

; Go convention: SCREAMING_SNAKE_CASE identifiers are treated as constants
; regardless of whether they are declared with `const` or `var`. Placed after
; the `(identifier) @variable` catch-all so it overrides it.
((identifier) @constant
  (#match? @constant "^_*[A-Z][A-Z0-9_]*$"))

[
  (true)
  (false)
] @boolean

[
  (nil)
  (iota)
] @constant.builtin

(comment) @comment

; Go directives
((comment) @preproc
  (#match? @preproc "^//go:"))

((comment) @preproc
  (#match? @preproc "^// \\+build"))
