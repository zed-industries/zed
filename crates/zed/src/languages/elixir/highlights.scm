["when" "and" "or" "not" "in" "not in" "fn" "do" "end" "catch" "rescue" "after" "else"] @keyword

(unary_operator
  operator: "&"
  operand: (integer) @operator)

(operator_identifier) @operator

(unary_operator
  operator: _ @operator)

(binary_operator
  operator: _ @operator)

(dot
  operator: _ @operator)

(stab_clause
  operator: _ @operator)

[
  (boolean)
  (nil)
] @constant

[
  (integer)
  (float)
] @number

(alias) @type

(call
  target: (dot
    left: (atom) @type))

(char) @constant

(escape_sequence) @string.escape

[
  (atom)
  (quoted_atom)
  (keyword)
  (quoted_keyword)
] @string.special.symbol

[
  (string)
  (charlist)
] @string

(sigil
  (sigil_name) @__name__
  quoted_start: _ @string
  quoted_end: _ @string
  (#match? @__name__ "^[sS]$")) @string

(sigil
  (sigil_name) @__name__
  quoted_start: _ @string.regex
  quoted_end: _ @string.regex
  (#match? @__name__ "^[rR]$")) @string.regex

(sigil
  (sigil_name) @__name__
  quoted_start: _ @string.special
  quoted_end: _ @string.special) @string.special

(
  (identifier) @comment.unused
  (#match? @comment.unused "^_")
)

(call
  target: [
    (identifier) @function
    (dot
      right: (identifier) @function)
  ])

(call
  target: (identifier) @keyword
  (arguments
    [
      (identifier) @function
      (binary_operator
        left: (identifier) @function
        operator: "when")
      (binary_operator
        operator: "|>"
        right: (identifier))
    ])
  (#match? @keyword "^(def|defdelegate|defguard|defguardp|defmacro|defmacrop|defn|defnp|defp)$"))

(binary_operator
  operator: "|>"
  right: (identifier) @function)

(call
  target: (identifier) @keyword
  (#match? @keyword "^(def|defdelegate|defexception|defguard|defguardp|defimpl|defmacro|defmacrop|defmodule|defn|defnp|defoverridable|defp|defprotocol|defstruct)$"))

(call
  target: (identifier) @keyword
  (#match? @keyword "^(alias|case|cond|else|for|if|import|quote|raise|receive|require|reraise|super|throw|try|unless|unquote|unquote_splicing|use|with)$"))

(
  (identifier) @constant.builtin
  (#match? @constant.builtin "^(__MODULE__|__DIR__|__ENV__|__CALLER__|__STACKTRACE__)$")
)

(unary_operator
  operator: "@" @comment.doc
  operand: (call
    target: (identifier) @__attribute__ @comment.doc
    (arguments
      [
        (string)
        (charlist)
        (sigil)
        (boolean)
      ] @comment.doc))
  (#match? @__attribute__ "^(moduledoc|typedoc|doc)$"))

(comment) @comment

[
 "%"
] @punctuation

[
 ","
 ";"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "<<"
  ">>"
] @punctuation.bracket

(interpolation "#{" @punctuation.special "}" @punctuation.special) @embedded

((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @embedded)
 (#eq? @_sigil_name "H"))
