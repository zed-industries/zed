(comment) @comment @spell

; Different types:
(string_value) @string @spell

(escape_sequence) @string.escape

(color_value) @constant

[
  (children_identifier)
  (easing_kind_identifier)
] @constant.builtin

(bool_value) @boolean

[
  (int_value)
  (physical_length_value)
] @number

[
  (angle_value)
  (duration_value)
  (float_value)
  (length_value)
  (percent_value)
  (relative_font_size_value)
] @number.float

(purity) @type.qualifier

(function_visibility) @type.qualifier

(property_visibility) @type.qualifier

(builtin_type_identifier) @type.builtin

(reference_identifier) @variable.builtin

(type
  [
    (type_list)
    (user_type_identifier)
    (anon_struct_block)
  ]) @type

(user_type_identifier) @type

; Functions and callbacks
(argument) @variable.parameter

(function_call
  name: (_) @function.call)

; definitions
(callback
  name: (_) @function)

(callback_alias
  name: (_) @function)

(callback_event
  name: (simple_identifier) @function.call)

(component
  id: (_) @variable)

(enum_definition
  name: (_) @type)

(function_definition
  name: (_) @function)

(struct_definition
  name: (_) @type)

(typed_identifier
  type: (_) @type)

; Operators
(binary_expression
  op: (_) @operator)

(unary_expression
  op: (_) @operator)

[
  (comparison_operator)
  (mult_prec_operator)
  (add_prec_operator)
  (unary_prec_operator)
  (assignment_prec_operator)
] @operator

[
  ":="
  "=>"
  "->"
  "<=>"
] @operator

; Punctuation
[
  ";"
  "."
  ","
  ":"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(property
  [
    "<"
    ">"
  ] @punctuation.bracket)

; Properties, Variables and Constants:
(component
  id: (simple_identifier) @constant)

(property
  name: (simple_identifier) @property)

(binding_alias
  name: (simple_identifier) @property)

(binding
  name: (simple_identifier) @property)

(struct_block
  (simple_identifier) @variable.member)

(anon_struct_block
  (simple_identifier) @variable.member)

(property_assignment
  property: (simple_identifier) @property)

(states_definition
  name: (simple_identifier) @variable)

(callback
  name: (simple_identifier) @variable)

(typed_identifier
  name: (_) @variable)

(simple_indexed_identifier
  name: (simple_identifier) @variable
  index_var: (simple_identifier) @variable)

(expression
  (simple_identifier) @variable)

(member_access
  member:
    (expression
      (simple_identifier) @property))

(states_definition
  name: (simple_identifier) @constant)

; Attributes:
[
  (linear_gradient_identifier)
  (radial_gradient_identifier)
  (radial_gradient_kind)
] @attribute

(image_call
  "@image-url" @attribute)

(tr
  "@tr" @attribute)

; Keywords:
(animate_option_identifier) @keyword

(export) @keyword

(if_statement
  "if" @keyword.conditional)

(if_expr
  [
    "if"
    "else"
  ] @keyword.conditional)

(ternary_expression
  [
    "?"
    ":"
  ] @keyword.conditional.ternary)

(animate_statement
  "animate" @keyword)

(callback
  "callback" @keyword)

(component_definition
  [
    "component"
    "inherits"
  ] @keyword)

(enum_definition
  "enum" @keyword)

(for_loop
  [
    "for"
    "in"
  ] @keyword.repeat)

(function_definition
  "function" @keyword.function)

(global_definition
  "global" @keyword)

(imperative_block
  "return" @keyword.return)

(import_statement
  [
    "import"
    "from"
  ] @keyword.import)

(import_type
  "as" @keyword.import)

(property
  "property" @keyword)

(states_definition
  [
    "states"
    "when"
  ] @keyword)

(struct_definition
  "struct" @keyword)

(transitions_definition
  [
    "transitions"
    "in"
    "out"
  ] @keyword)
