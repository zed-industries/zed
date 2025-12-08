; Variables

(identifier) @variable

; Special identifiers

(type_annotation) @type

(type_identifier) @type
(predefined_type) @type.builtin

(type_alias_declaration
  (type_identifier) @type)

(type_alias_declaration
  value: (_
    (type_identifier) @type))

(interface_declaration
  (type_identifier) @type)

(class_declaration
  (type_identifier) @type.class)

(extends_clause
  value: (identifier) @type.class)

(extends_type_clause
  type: (type_identifier) @type)

(implements_clause
  (type_identifier) @type)

;; Enables ts-pretty-errors
;; The Lsp returns "snippets" of typescript, which are not valid typescript in totality,
;; but should still be highlighted
;; Highlights object literals by hijacking the statement_block pattern, but only if
;; the statement block follows an object literal pattern
((statement_block
   (labeled_statement
     ;; highlight the label like a property name
     label: (statement_identifier) @property.name
     body: [
       ;; match a terminating expression statement
       (expression_statement
            ;; single identifier - treat as a type name
           [(identifier) @type.name
            ;; object - treat as a property - type pair
            (object
                (pair
                    key: (_) @property.name
                    value: (_) @type.name))
            ;; subscript_expression - treat as an array declaration
            (subscript_expression
                object: (_) @type.name
                index: (_)
                )
            ;; templated string - treat each identifier contained as a type name
            (template_string
                (template_substitution
                    (identifier) @type.name))
            ])
       ;; match a nested statement block
       (statement_block) @nested
     ])))

(import_specifier
  "type"
  name: (identifier) @type
  alias: (identifier) @type
)

(import_statement
  "type"
  (import_clause
    (named_imports
      (import_specifier
        name: (identifier) @type
        alias: (identifier) @type
      )
    )
  )
)

([
  (identifier)
  (shorthand_property_identifier)
  (shorthand_property_identifier_pattern)
 ] @constant
 (#match? @constant "^_*[A-Z_][A-Z\\d_]*$"))

; Properties

(property_identifier) @property
(shorthand_property_identifier) @property
(shorthand_property_identifier_pattern) @property
(private_property_identifier) @property

; Function and method calls

(call_expression
  function: (identifier) @function)

(call_expression
  function: (member_expression
    property: [(property_identifier) (private_property_identifier)] @function.method))

; Function and method definitions

(function_expression
  name: (identifier) @function)
(function_declaration
  name: (identifier) @function)
(method_definition
  name: [(property_identifier) (private_property_identifier)] @function.method)
(method_definition
    name: (property_identifier) @constructor
    (#eq? @constructor "constructor"))

(pair
  key: [(property_identifier) (private_property_identifier)] @function.method
  value: [(function_expression) (arrow_function)])

(assignment_expression
  left: (member_expression
    property: [(property_identifier) (private_property_identifier)] @function.method)
  right: [(function_expression) (arrow_function)])

(variable_declarator
  name: (identifier) @function
  value: [(function_expression) (arrow_function)])

(assignment_expression
  left: (identifier) @function
  right: [(function_expression) (arrow_function)])

(arrow_function) @function

; Parameters

(required_parameter
  (identifier) @variable.parameter)

(required_parameter
  (_
    ([
      (identifier)
      (shorthand_property_identifier_pattern)
    ]) @variable.parameter))

(optional_parameter
  (identifier) @variable.parameter)

(optional_parameter
  (_
    ([
      (identifier)
      (shorthand_property_identifier_pattern)
    ]) @variable.parameter))

(catch_clause
  parameter: (identifier) @variable.parameter)

(index_signature
  name: (identifier) @variable.parameter)

(arrow_function
  parameter: (identifier) @variable.parameter)

(type_predicate
  name: (identifier) @variable.parameter)

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

(literal_type
  [
    (null)
    (undefined)
    (true)
    (false)
  ] @type.builtin
)

(comment) @comment

(hash_bang_line) @comment

[
  (string)
  (template_string)
  (template_literal_type)
] @string

(escape_sequence) @string.escape

(regex) @string.regex
(regex_flags) @keyword.operator.regex
(number) @number

; Tokens

[
  ";"
  "?."
  "."
  ","
  ":"
  "?"
] @punctuation.delimiter

[
  "..."
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
  "..."
] @operator

(regex "/" @string.regex)

(ternary_expression
  [
    "?"
    ":"
  ] @operator
)

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
]  @punctuation.bracket

(template_substitution
  "${" @punctuation.special
  "}" @punctuation.special) @embedded

(template_type
  "${" @punctuation.special
  "}" @punctuation.special) @embedded

(type_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

(type_parameters
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

(decorator "@" @punctuation.special)

(union_type
  ("|") @punctuation.special)

(intersection_type
  ("&") @punctuation.special)

(type_annotation
  (":") @punctuation.special)

(index_signature
  (":") @punctuation.special)

(type_predicate_annotation
  (":") @punctuation.special)

(public_field_definition
  ("?") @punctuation.special)

(property_signature
  ("?") @punctuation.special)

(method_signature
  ("?") @punctuation.special)

(optional_parameter
  ([
    "?"
    ":"
  ]) @punctuation.special)

; Keywords

[
  "abstract"
  "as"
  "async"
  "await"
  "class"
  "const"
  "debugger"
  "declare"
  "default"
  "delete"
  "enum"
  "export"
  "extends"
  "from"
  "function"
  "get"
  "implements"
  "import"
  "in"
  "infer"
  "instanceof"
  "interface"
  "is"
  "keyof"
  "let"
  "module"
  "namespace"
  "new"
  "of"
  "override"
  "private"
  "protected"
  "public"
  "readonly"
  "satisfies"
  "set"
  "static"
  "target"
  "type"
  "typeof"
  "using"
  "var"
  "void"
  "with"
] @keyword

[
  "break"
  "case"
  "catch"
  "continue"
  "do"
  "else"
  "finally"
  "for"
  "if"
  "return"
  "switch"
  "throw"
  "try"
  "while"
  "yield"
] @keyword.control

(switch_default "default" @keyword.control)
