(call
  target: (identifier) @context
  (arguments (alias) @name)
  (#match? @context "^(defmodule|defprotocol)$")) @item

(unary_operator
  operator: "@" @name
  operand: (call
  target: (identifier) @context
    (arguments
      [
        (binary_operator
          left: (identifier) @name)
        (binary_operator
          left: (call
          target: (identifier) @name
          (arguments
            "(" @context.extra
            _* @context.extra
            ")" @context.extra)))
      ]
    )
)
(#match? @context "^(callback|type|typep)$")) @item

(call
  target: (identifier) @context
  (arguments
    [
      (identifier) @name
      (call
          target: (identifier) @name
          (arguments
              "(" @context.extra
              _* @context.extra
              ")" @context.extra))
      (binary_operator
        left: (call
            target: (identifier) @name
            (arguments
                "(" @context.extra
                _* @context.extra
                ")" @context.extra))
        operator: "when")
    ])
  (#match? @context "^(def|defp|defdelegate|defguard|defguardp|defmacro|defmacrop|defn|defnp)$")) @item
