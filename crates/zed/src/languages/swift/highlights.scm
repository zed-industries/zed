[
  "."
  ";"
  ":"
  ","
] @punctuation.delimiter

; TODO: "\\(" ")" in interpolations should be @punctuation.special
[
  "\\("
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

; Identifiers
(attribute) @variable

(type_identifier) @type

(self_expression) @variable.builtin

; Declarations
"func" @keyword.function

[
  (visibility_modifier)
  (member_modifier)
  (function_modifier)
  (property_modifier)
  (parameter_modifier)
  (inheritance_modifier)
  (mutation_modifier)
] @type.qualifier

(function_declaration
  (simple_identifier) @function.method)

(function_declaration
  "init" @constructor)

(throws) @keyword

(where_keyword) @keyword

(parameter
  external_name: (simple_identifier) @variable.parameter)

(parameter
  name: (simple_identifier) @variable.parameter)

(type_parameter
  (type_identifier) @variable.parameter)

(inheritance_constraint
  (identifier
    (simple_identifier) @variable.parameter))

(equality_constraint
  (identifier
    (simple_identifier) @variable.parameter))

(pattern
  bound_identifier: (simple_identifier)) @variable

[
  "actor"
  "associatedtype"
  "class"
  "convenience"
  "enum"
  "extension"
  "indirect"
  "nonisolated"
  "override"
  "protocol"
  "required"
  "some"
  "struct"
  "typealias"
] @keyword

[
  "async"
  "await"
] @keyword.coroutine

[
  (getter_specifier)
  (setter_specifier)
  (modify_specifier)
] @keyword

(class_body
  (property_declaration
    (pattern
      (simple_identifier) @variable.member)))

(protocol_property_declaration
  (pattern
    (simple_identifier) @variable.member))

(navigation_expression
  (navigation_suffix
    (simple_identifier) @variable.member))

(value_argument
  name: (value_argument_label) @variable.member)

(import_declaration
  "import" @keyword.import)

(enum_entry
  "case" @keyword)

; Function calls
(call_expression
  (simple_identifier) @function.call) ; foo()

(call_expression
  ; foo.bar.baz(): highlight the baz()
  (navigation_expression
    (navigation_suffix
      (simple_identifier) @function.call)))

(call_expression
  (prefix_expression
    (simple_identifier) @function.call)) ; .foo()

((navigation_expression
  (simple_identifier) @type) ; SomeType.method(): highlight SomeType as a type
  (#lua-match? @type "^[A-Z]"))

(directive) @function.macro

(diagnostic) @function.macro

; Statements
(for_statement
  "for" @keyword.repeat)

(for_statement
  "in" @keyword.repeat)

(for_statement
  (pattern) @variable)

(else) @keyword

(as_operator) @keyword

[
  "while"
  "repeat"
  "continue"
  "break"
] @keyword.repeat

[
  "let"
  "var"
] @keyword

(guard_statement
  "guard" @keyword.conditional)

(if_statement
  "if" @keyword.conditional)

(switch_statement
  "switch" @keyword.conditional)

(switch_entry
  "case" @keyword)

(switch_entry
  "fallthrough" @keyword)

(switch_entry
  (default_keyword) @keyword)

"return" @keyword.return

(ternary_expression
  [
    "?"
    ":"
  ] @keyword.conditional)

[
  "do"
  (throw_keyword)
  (catch_keyword)
] @keyword

(statement_label) @label

; Comments
[
  (comment)
  (multiline_comment)
] @comment @spell

((comment) @comment.documentation
  (#lua-match? @comment.documentation "^///[^/]"))

((comment) @comment.documentation
  (#lua-match? @comment.documentation "^///$"))

((multiline_comment) @comment.documentation
  (#lua-match? @comment.documentation "^/[*][*][^*].*[*]/$"))

; String literals
(line_str_text) @string

(str_escaped_char) @string

(multi_line_str_text) @string

(raw_str_part) @string

(raw_str_end_part) @string

(raw_str_interpolation_start) @punctuation.special

[
  "\""
  "\"\"\""
] @string

; Lambda literals
(lambda_literal
  "in" @keyword.operator)

; Basic literals
[
  (integer_literal)
  (hex_literal)
  (oct_literal)
  (bin_literal)
] @number

(real_literal) @number.float

(boolean_literal) @boolean

"nil" @constant.builtin

; Regex literals
(regex_literal) @string.regexp

; Operators
(custom_operator) @operator

[
  "try"
  "try?"
  "try!"
  "+"
  "-"
  "*"
  "/"
  "%"
  "="
  "+="
  "-="
  "*="
  "/="
  "<"
  ">"
  "<="
  ">="
  "++"
  "--"
  "&"
  "~"
  "%="
  "!="
  "!=="
  "=="
  "==="
  "??"
  "->"
  "..<"
  "..."
  (bang)
] @operator
