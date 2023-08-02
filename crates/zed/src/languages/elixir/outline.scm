(call
  target: (identifier) @context
  (arguments (alias) @name)
  (#match? @context "^(defmodule|defprotocol)$")) @item

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
