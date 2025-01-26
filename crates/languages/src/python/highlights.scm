; Identifier naming conventions; these "soft conventions" should stay at the top of the file as they're often overridden

; CamelCase for classes
((identifier) @type.class
  (#match? @type.class "^_*[A-Z][A-Za-z0-9_]*$"))

; ALL_CAPS for constants:
((identifier) @constant
  (#match? @constant "^_*[A-Z][A-Z0-9_]*$"))

(attribute attribute: (identifier) @property)
(type (identifier) @type)
(generic_type (identifier) @type)
(comment) @comment
(string) @string
(escape_sequence) @string.escape

; Type alias
(type_alias_statement "type" @keyword)

; TypeVar with constraints in type parameters
(type
  (tuple (identifier) @type)
)

; Forward references
(type
  (string) @type
)


; Function calls

(call
  function: (attribute attribute: (identifier) @function.method.call))
(call
  function: (identifier) @function.call)

(decorator "@" @punctuation.special)
(decorator
  "@" @punctuation.special
  [
    (identifier) @function.decorator
    (attribute attribute: (identifier) @function.decorator)
    (call function: (identifier) @function.decorator.call)
    (call (attribute attribute: (identifier) @function.decorator.call))
  ])

; Function and class definitions

(function_definition
  name: (identifier) @function.definition)

; Function arguments
(function_definition
  parameters: (parameters
  [
      (identifier) @function.arguments ; Simple parameters
      (typed_parameter
        (identifier) @function.arguments) ; Typed parameters
      (default_parameter
        name: (identifier) @function.arguments) ; Default parameters
  ]))

; Keyword arguments
(call
  arguments: (argument_list
    (keyword_argument
      name: (identifier) @function.kwargs)))

; Class definitions and calling: needs to come after the regex matching above

(class_definition
  name: (identifier) @type.class.definition)

(class_definition
  superclasses: (argument_list
  (identifier) @type.class.inheritance))

(call
  function: (identifier) @type.class.call
  (#match? @type.class.call "^_*[A-Z][A-Za-z0-9_]*$"))

; Builtins

((call
  function: (identifier) @function.builtin)
 (#match?
   @function.builtin
   "^(abs|all|any|ascii|bin|bool|breakpoint|bytearray|bytes|callable|chr|classmethod|compile|complex|delattr|dict|dir|divmod|enumerate|eval|exec|filter|float|format|frozenset|getattr|globals|hasattr|hash|help|hex|id|input|int|isinstance|issubclass|iter|len|list|locals|map|max|memoryview|min|next|object|oct|open|ord|pow|print|property|range|repr|reversed|round|set|setattr|slice|sorted|staticmethod|str|sum|super|tuple|type|vars|zip|__import__)$"))

((identifier) @type.builtin
    (#any-of? @type.builtin "int" "float" "complex" "bool" "list" "tuple" "range" "str" "bytes" "bytearray" "memoryview" "set" "frozenset" "dict"))

; Literals

[
  (none)
  (true)
  (false)
  (ellipsis)
] @constant.builtin

[
  (integer)
  (float)
] @number

; Self references

[
  (parameters (identifier) @variable.special)
  (attribute (identifier) @variable.special)
  (#match? @variable.special "^self|cls$")
]

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(interpolation
  "{" @punctuation.special
  "}" @punctuation.special) @embedded

; Docstrings.
(function_definition
  "async"?
  "def"
  name: (_)
  (parameters)?
  body: (block . (expression_statement (string) @string.doc)))

(class_definition
  body: (block
    . (comment) @comment*
    . (expression_statement (string) @string.doc)))

(module
  . (comment) @comment*
  . (expression_statement (string) @string.doc))

(module
  [
    (expression_statement (assignment))
    (type_alias_statement)
  ]
  . (expression_statement (string) @string.doc))

(class_definition
  body: (block
    (expression_statement (assignment))
    . (expression_statement (string) @string.doc)))

(class_definition
  body: (block
    (function_definition
      name: (identifier) @function.method.constructor
      (#eq? @function.method.constructor "__init__")
      body: (block
        (expression_statement (assignment))
        . (expression_statement (string) @string.doc)))))


[
  "-"
  "-="
  "!="
  "*"
  "**"
  "**="
  "*="
  "/"
  "//"
  "//="
  "/="
  "&"
  "%"
  "%="
  "^"
  "+"
  "->"
  "+="
  "<"
  "<<"
  "<="
  "<>"
  "="
  ":="
  "=="
  ">"
  ">="
  ">>"
  "|"
  "~"
] @operator

[
  "and"
  "in"
  "is"
  "not"
  "or"
  "is not"
  "not in"
] @keyword.operator

[
  "as"
  "assert"
  "async"
  "await"
  "break"
  "class"
  "continue"
  "def"
  "del"
  "elif"
  "else"
  "except"
  "except*"
  "exec"
  "finally"
  "for"
  "from"
  "global"
  "if"
  "import"
  "lambda"
  "nonlocal"
  "pass"
  "print"
  "raise"
  "return"
  "try"
  "while"
  "with"
  "yield"
  "match"
  "case"
] @keyword

; Definition keywords def, class, async def, lambda
[
  "async"
  "def"
  "class"
  "lambda"
] @keyword.definition

((identifier) @attribute.builtin
  (#any-of? @attribute.builtin "classmethod" "staticmethod" "property"))

; Builtin types as identifiers
[
  (call
    function: (identifier) @type.builtin)
  (call
    arguments: (argument_list
    (identifier) @type.builtin))
  (call
    arguments: (argument_list
      (keyword_argument
        value: (identifier) @type.builtin)))
  (type (identifier) @type.builtin)
  ; also check if type binary operator left identifier for union types
  (type
    (binary_operator
      left: (identifier) @type.builtin))
  (#any-of? @type.builtin "bool" "bytearray" "bytes" "complex" "dict" "float" "frozenset" "int" "list" "memoryview" "object" "range" "set" "slice" "str" "tuple")
] @type.builtin
