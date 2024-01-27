; Pragma
[
  "pragma"
  "solidity"
] @keyword.directive

(solidity_pragma_token
  "||" @string.special.symbol)

(solidity_pragma_token
  "-" @string.special.symbol)

(solidity_version_comparison_operator) @operator

(solidity_version) @string.special

; Literals
[
  (string)
  (yul_string_literal)
] @string

(hex_string_literal
  "hex" @string.special.symbol
  (_) @string)

(unicode_string_literal
  "unicode" @string.special.symbol
  (_) @string)

[
  (number_literal)
  (yul_decimal_number)
  (yul_hex_number)
] @number

(yul_boolean) @boolean

; Variables
[
  (identifier)
  (yul_identifier)
] @variable

; Types
(type_name
  (identifier) @type)

(type_name
  (user_defined_type
    (identifier) @type))

(type_name
  "mapping" @function.builtin)

[
  (primitive_type)
  (number_unit)
] @type.builtin

(contract_declaration
  name: (identifier) @type)

(struct_declaration
  name: (identifier) @type)

(struct_member
  name: (identifier) @variable.member)

(enum_declaration
  name: (identifier) @type)

(emit_statement
  .
  (identifier) @type)

; Handles ContractA, ContractB in function foo() override(ContractA, contractB) {}
(override_specifier
  (user_defined_type) @type)

; Functions and parameters
(function_definition
  name: (identifier) @function)

(modifier_definition
  name: (identifier) @function)

(yul_evm_builtin) @function.builtin

; Use constructor coloring for special functions
(constructor_definition
  "constructor" @constructor)

(modifier_invocation
  (identifier) @function)

; Handles expressions like structVariable.g();
(call_expression
  .
  (member_expression
    (identifier) @function.method.call))

; Handles expressions like g();
(call_expression
  .
  (identifier) @function.call)

; Function parameters
(event_paramater
  name: (identifier) @variable.parameter)

(parameter
  name: (identifier) @variable.parameter)

; Yul functions
(yul_function_call
  function: (yul_identifier) @function.call)

; Yul function parameters
(yul_function_definition
  .
  (yul_identifier) @function
  (yul_identifier) @variable.parameter)

(meta_type_expression
  "type" @keyword)

(member_expression
  property: (identifier) @variable.member)

(call_struct_argument
  name: (identifier) @variable.member)

(struct_field_assignment
  name: (identifier) @variable.member)

(enum_value) @constant

; Keywords
[
  "contract"
  "interface"
  "library"
  "is"
  "struct"
  "enum"
  "event"
  "assembly"
  "emit"
  "override"
  "modifier"
  "var"
  "let"
  "emit"
  "error"
  "fallback"
  "receive"
  (virtual)
] @keyword

; FIXME: update grammar
; (block_statement "unchecked" @keyword)
(event_paramater
  "indexed" @keyword)

[
  "public"
  "internal"
  "private"
  "external"
  "pure"
  "view"
  "payable"
  (immutable)
] @type.qualifier

[
  "memory"
  "storage"
  "calldata"
  "constant"
] @keyword.storage

[
  "for"
  "while"
  "do"
  "break"
  "continue"
] @keyword.repeat

[
  "if"
  "else"
  "switch"
  "case"
  "default"
] @keyword.conditional

(ternary_expression
  "?" @keyword.conditional.ternary
  ":" @keyword.conditional.ternary)

[
  "try"
  "catch"
  "revert"
] @keyword.exception

[
  "return"
  "returns"
  (yul_leave)
] @keyword.return

"function" @keyword.function

[
  "import"
  "using"
] @keyword.import

(import_directive
  "as" @keyword.import)

(import_directive
  "from" @keyword.import)

((import_directive
  source: (string) @string.special.path)
  (#offset! @string.special.path 0 1 0 -1))

; Punctuation
[
  "{"
  "}"
] @punctuation.bracket

[
  "["
  "]"
] @punctuation.bracket

[
  "("
  ")"
] @punctuation.bracket

[
  "."
  ","
  ":"
  ; FIXME: update grammar
  ; (semicolon)
  "->"
  "=>"
] @punctuation.delimiter

; Operators
[
  "&&"
  "||"
  ">>"
  ">>>"
  "<<"
  "&"
  "^"
  "|"
  "+"
  "-"
  "*"
  "/"
  "%"
  "**"
  "="
  "<"
  "<="
  "=="
  "!="
  "!=="
  ">="
  ">"
  "!"
  "~"
  "-"
  "+"
  "++"
  "--"
  ":="
] @operator

[
  "delete"
  "new"
] @keyword.operator

(import_directive
  "*" @character.special)

; Comments
(comment) @comment @spell

((comment) @comment.documentation
  (#lua-match? @comment.documentation "^///[^/]"))

((comment) @comment.documentation
  (#lua-match? @comment.documentation "^///$"))

((comment) @comment.documentation
  (#lua-match? @comment.documentation "^/[*][*][^*].*[*]/$"))
