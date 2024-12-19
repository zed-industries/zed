; Block Objects
([
  (do_block
    "do"
    .
    (_) @_do
    (_) @_end
    .
    "end")
  (do_block
    "do"
    .
    ((_) @_do) @_end
    .
    "end")
]
  (#make-range! "block.inside" @_do @_end)) @block.around

; Class Objects (Modules, Protocols)
; multiple children
(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "defmodule" "defprotocol" "defimpl"))
  (arguments
    (alias))
  (do_block
    "do"
    .
    (_) @_do
    (_) @_end
    .
    "end")
  (#make-range! "class.inside" @_do @_end)) @class.around

; single child match
(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "defmodule" "defprotocol" "defimpl"))
  (arguments
    (alias))
  (do_block
    "do"
    .
    (_) @class.inside
    .
    "end")) @class.around

; Function, Parameter, and Call Objects
(anonymous_function
  (stab_clause
    right: (body) @function.inside)) @function.around

(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "def" "defmacro" "defmacrop" "defn" "defnp" "defp"))
  (arguments
    [
      (call
        [
          (arguments
            (_) @parameter.inside
            .
            "," @_delimiter)
          (arguments
            ((_) @parameter.inside) @_delimiter .)
        ]
        (#make-range! "parameter.around" @parameter.inside @_delimiter))
      (binary_operator
        left: (call
          [
            (arguments
              (_) @parameter.inside
              .
              "," @_delimiter)
            (arguments
              ((_) @parameter.inside) @_delimiter .)
          ]
          (#make-range! "parameter.around" @parameter.inside @_delimiter)))
    ])
  [
    (do_block
      "do"
      .
      (_) @_do
      (_) @_end
      .
      "end")
    (do_block
      "do"
      .
      ((_) @_do) @_end
      .
      "end")
  ]
  (#make-range! "function.inside" @_do @_end)) @function.around

(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "def" "defmacro" "defmacrop" "defn" "defnp" "defp"))
  (arguments
    [
      (identifier)
      (binary_operator
        (identifier))
    ])
  [
    (do_block
      "do"
      .
      (_) @_do
      (_) @_end
      .
      "end")
    (do_block
      "do"
      .
      ((_) @_do) @_end
      .
      "end")
  ]
  (#make-range! "function.inside" @_do @_end)) @function.around

(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "def" "defmacro" "defmacrop" "defn" "defnp" "defp"))
  (arguments
    [
      (call
        [
          (arguments
            (_) @parameter.inside
            .
            "," @_delimiter)
          (arguments
            ((_) @parameter.inside) @_delimiter .)
        ]
        (#make-range! "parameter.around" @parameter.inside @_delimiter))
      (binary_operator
        left: (call
          [
            (arguments
              (_) @parameter.inside
              .
              "," @_delimiter)
            (arguments
              ((_) @parameter.inside) @_delimiter .)
          ]
          (#make-range! "parameter.around" @parameter.inside @_delimiter)))
    ]
    (keywords
      (pair
        value: (_) @function.inside)))) @function.around

(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "def" "defmacro" "defmacrop" "defn" "defnp" "defp"))
  (arguments
    [
      (identifier)
      (binary_operator
        (identifier))
    ]
    (keywords
      (pair
        value: (_) @function.inside)))) @function.around

; Comment Objects
(comment) @comment.around

; Documentation Objects
(unary_operator
  operator: "@"
  operand: (call
    target: ((identifier) @_identifier
      (#any-of? @_identifier "moduledoc" "typedoc" "shortdoc" "doc"))
    (arguments
      [
        ; attributes style documentation
        ; @doc deprecated: "...."
        (keywords) @comment.inside
        ; heredoc style documentation
        ; @moduledoc """"""
        (string
          (quoted_content) @comment.inside)
      ]))) @comment.around

; Regex Objects
(sigil
  (quoted_content) @regex.inside) @regex.around
