(identifier) @variable
(metavariable) @variable
(type_identifier) @type
(fragment_specifier) @type
(primitive_type) @type.builtin
(self) @variable.special
(field_identifier) @property

(trait_item name: (type_identifier) @type.interface)
(impl_item trait: (type_identifier) @type.interface)
(abstract_type trait: (type_identifier) @type.interface)
(dynamic_type trait: (type_identifier) @type.interface)
(trait_bounds (type_identifier) @type.interface)

(call_expression
  function: [
    (identifier) @function
    (scoped_identifier
      name: (identifier) @function)
    (field_expression
      field: (field_identifier) @function.method)
  ])

(generic_function
  function: [
    (identifier) @function
    (scoped_identifier
      name: (identifier) @function)
    (field_expression
      field: (field_identifier) @function.method)
  ])

(function_item name: (identifier) @function.definition)
(function_signature_item name: (identifier) @function.definition)

(macro_invocation
  macro: [
    (identifier) @function.special
    (scoped_identifier
      name: (identifier) @function.special)
  ])

(macro_definition
  name: (identifier) @function.special.definition)

; Identifier conventions

; Assume uppercase names are types/enum-constructors
((identifier) @type
 (#match? @type "^[A-Z]"))

; Assume all-caps names are constants
((identifier) @constant
 (#match? @constant "^_*[A-Z][A-Z\\d_]*$"))

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

(_
  .
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

[
  "."
  ";"
  ","
  "::"
] @punctuation.delimiter

[
  "#"
] @punctuation.special

[
  "as"
  "async"
  "await"
  "break"
  "const"
  "continue"
  "default"
  "dyn"
  "else"
  "enum"
  "extern"
  "fn"
  "for"
  "if"
  "impl"
  "in"
  "let"
  "loop"
  "macro_rules!"
  "match"
  "mod"
  "move"
  "pub"
  "raw"
  "ref"
  "return"
  "static"
  "struct"
  "trait"
  "type"
  "union"
  "unsafe"
  "use"
  "where"
  "while"
  "yield"
  (crate)
  (mutable_specifier)
  (super)
] @keyword

[
  (string_literal)
  (raw_string_literal)
  (char_literal)
] @string

(escape_sequence) @string.escape

[
  (integer_literal)
  (float_literal)
] @number

(boolean_literal) @boolean

[
  (line_comment)
  (block_comment)
] @comment

[
  (line_comment (doc_comment))
  (block_comment (doc_comment))
] @comment.doc

[
  "!="
  "%"
  "%="
  "&"
  "&="
  "&&"
  "*"
  "*="
  "+"
  "+="
  "-"
  "-="
  "->"
  ".."
  "..="
  "..."
  "/="
  ":"
  "<<"
  "<<="
  "<"
  "<="
  "="
  "=="
  "=>"
  ">"
  ">="
  ">>"
  ">>="
  "@"
  "^"
  "^="
  "|"
  "|="
  "||"
  "?"
] @operator

; Avoid highlighting these as operators when used in doc comments.
(unary_expression "!" @operator)
operator: "/" @operator

(lifetime) @lifetime

(parameter (identifier) @variable.parameter)

(attribute_item (attribute [
  (identifier) @attribute
  (scoped_identifier name: (identifier) @attribute)
  (token_tree (identifier) @attribute (#match? @attribute "^[a-z\\d_]*$"))
  (token_tree (identifier) @variable "::" (identifier) @type (#match? @type "^[A-Z]"))
]))

(inner_attribute_item (attribute [
  (identifier) @attribute
  (scoped_identifier name: (identifier) @attribute)
  (token_tree (identifier) @attribute (#match? @attribute "^[a-z\\d_]*$"))
  (token_tree (identifier) @variable "::" (identifier) @type (#match? @type "^[A-Z]"))
]))
