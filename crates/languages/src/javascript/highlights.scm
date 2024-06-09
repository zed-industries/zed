; Variables

(identifier) @variable

; Properties

(property_identifier) @property

; Function and method calls

(call_expression
  function: (identifier) @function)

(call_expression
  function: (member_expression
    property: (property_identifier) @function.method))

; Function and method definitions

(function_expression
  name: (identifier) @function)
(function_declaration
  name: (identifier) @function)
(method_definition
  name: (property_identifier) @function.method)

(pair
  key: (property_identifier) @function.method
  value: [(function_expression) (arrow_function)])

(assignment_expression
  left: (member_expression
    property: (property_identifier) @function.method)
  right: [(function_expression) (arrow_function)])

(variable_declarator
  name: (identifier) @function
  value: [(function_expression) (arrow_function)])

(assignment_expression
  left: (identifier) @function
  right: [(function_expression) (arrow_function)])

; Special identifiers

((identifier) @type
 (#match? @type "^[A-Z]"))
(type_identifier) @type
(predefined_type) @type.builtin

([
  (identifier)
  (shorthand_property_identifier)
  (shorthand_property_identifier_pattern)
 ] @constant
 (#match? @constant "^_*[A-Z_][A-Z\\d_]*$"))

; Literals

(this) @variable.special
(super) @variable.special

[
  (null)
  (undefined)
] @constant.builtin

[
  (true)
  (false)
] @boolean

(comment) @comment

[
  (string)
  (template_string)
] @string

(escape_sequence) @string.escape

(regex) @string.regex
(number) @number

; Tokens

[
  ";"
  "?."
  "."
  ","
  ":"
] @punctuation.delimiter

[
  "-"
  "--"
  "-="
  "+"
  "++"
  "+="
  "*"
  "*="
  "**"
  "**="
  "/"
  "/="
  "%"
  "%="
  "<"
  "<="
  "<<"
  "<<="
  "="
  "=="
  "==="
  "!"
  "!="
  "!=="
  "=>"
  ">"
  ">="
  ">>"
  ">>="
  ">>>"
  ">>>="
  "~"
  "^"
  "&"
  "|"
  "^="
  "&="
  "|="
  "&&"
  "||"
  "??"
  "&&="
  "||="
  "??="
] @operator

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
]  @punctuation.bracket

[
  "as"
  "async"
  "await"
  "break"
  "case"
  "catch"
  "class"
  "const"
  "continue"
  "debugger"
  "default"
  "delete"
  "do"
  "else"
  "export"
  "extends"
  "finally"
  "for"
  "from"
  "function"
  "get"
  "if"
  "import"
  "in"
  "instanceof"
  "let"
  "new"
  "of"
  "return"
  "set"
  "static"
  "switch"
  "target"
  "throw"
  "try"
  "typeof"
  "var"
  "void"
  "while"
  "with"
  "yield"
] @keyword

(template_substitution
  "${" @punctuation.special
  "}" @punctuation.special) @embedded

(type_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

; Keywords

[ "abstract"
  "declare"
  "enum"
  "export"
  "implements"
  "interface"
  "keyof"
  "namespace"
  "private"
  "protected"
  "public"
  "type"
  "readonly"
  "override"
] @keyword

; JSX elements
(jsx_opening_element (identifier) @tag (#match? @tag "^[a-z][^.]*$"))
(jsx_closing_element (identifier) @tag (#match? @tag "^[a-z][^.]*$"))
(jsx_self_closing_element (identifier) @tag (#match? @tag "^[a-z][^.]*$"))

(jsx_attribute (property_identifier) @attribute)
(jsx_opening_element (["<" ">"]) @punctuation.bracket)
(jsx_closing_element (["</" ">"]) @punctuation.bracket)
(jsx_self_closing_element (["<" "/>"]) @punctuation.bracket)
