(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "defmodule" "defprotocol" "defimpl"))
  (do_block
    "do"
    (_)* @class.inside
    "end")) @class.around

(anonymous_function
  (stab_clause
    right: (body) @function.inside)) @function.around

(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "def" "defmacro" "defmacrop" "defn" "defnp" "defp"))
  (do_block
    "do"
    (_)* @function.inside
    "end")) @function.around

(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "def" "defmacro" "defmacrop" "defn" "defnp" "defp"))
  (arguments
    (_)
    (keywords
      (pair
        value: (_) @function.inside)))) @function.around

(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "defdelegate" "defguard" "defguardp"))) @function.around

(comment) @comment.around

(unary_operator
  operator: "@"
  operand: (call
    target: ((identifier) @_identifier
      (#any-of? @_identifier "moduledoc" "typedoc" "shortdoc" "doc"))
    (arguments
      [
        (keywords) @comment.inside
        (string
          (quoted_content) @comment.inside)
      ]))) @comment.around
