(php_tag) @tag
"?>" @tag

; Types

(primitive_type) @type.builtin
(cast_type) @type.builtin
(named_type (name) @type) @type
(named_type (qualified_name) @type) @type

; Functions

(array_creation_expression "array" @function.builtin)
(list_literal "list" @function.builtin)

(method_declaration
  name: (name) @function.method)

(function_call_expression
  function: [(qualified_name (name)) (name)] @function)

(scoped_call_expression
  name: (name) @function)

(member_call_expression
  name: (name) @function.method)

(function_definition
  name: (name) @function)

; Member

(property_element
  (variable_name) @property)

(member_access_expression
  name: (variable_name (name)) @property)
(member_access_expression
  name: (name) @property)

; Variables

(relative_scope) @variable.builtin

((name) @constant
 (#match? @constant "^_?[A-Z][A-Z\\d_]+$"))
((name) @constant.builtin
 (#match? @constant.builtin "^__[A-Z][A-Z\d_]+__$"))

((name) @constructor
 (#match? @constructor "^[A-Z]"))

((name) @variable.builtin
 (#eq? @variable.builtin "this"))

(variable_name) @variable

; Basic tokens
[
  (string)
  (string_value)
  (encapsed_string)
  (heredoc)
  (heredoc_body)
  (nowdoc_body)
] @string
(boolean) @constant.builtin
(null) @constant.builtin
(integer) @number
(float) @number
(comment) @comment

"$" @operator

; Keywords

"abstract" @keyword
"and" @keyword
"as" @keyword
"break" @keyword
"callable" @keyword
"case" @keyword
"catch" @keyword
"class" @keyword
"clone" @keyword
"const" @keyword
"continue" @keyword
"declare" @keyword
"default" @keyword
"do" @keyword
"echo" @keyword
"else" @keyword
"elseif" @keyword
"enum" @keyword
"enddeclare" @keyword
"endfor" @keyword
"endforeach" @keyword
"endif" @keyword
"endswitch" @keyword
"endwhile" @keyword
"extends" @keyword
"final" @keyword
"readonly" @keyword
"finally" @keyword
"for" @keyword
"foreach" @keyword
"fn" @keyword
"function" @keyword
"global" @keyword
"goto" @keyword
"if" @keyword
"implements" @keyword
"include_once" @keyword
"include" @keyword
"instanceof" @keyword
"insteadof" @keyword
"interface" @keyword
"match" @keyword
"namespace" @keyword
"new" @keyword
"or" @keyword
"print" @keyword
"private" @keyword
"protected" @keyword
"public" @keyword
"readonly" @keyword
"require_once" @keyword
"require" @keyword
"return" @keyword
"static" @keyword
"switch" @keyword
"throw" @keyword
"trait" @keyword
"try" @keyword
"use" @keyword
"var" @keyword
"while" @keyword
"xor" @keyword
"yield" @keyword
"yield from" @keyword
