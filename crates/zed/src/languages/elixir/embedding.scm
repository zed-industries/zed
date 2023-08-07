(
    (unary_operator
        operator: "@"
        operand: (call
            target: (identifier) @unary
            (#match? @unary "^(doc)$"))
        ) @context
    .
    (call
        target: (identifier) @name
        (arguments
            [
            (identifier) @name
            (call
                target: (identifier) @name)
                (binary_operator
                    left: (call
                    target: (identifier) @name)
                    operator: "when")
            ])
        (#match? @name "^(def|defp|defdelegate|defguard|defguardp|defmacro|defmacrop|defn|defnp)$")) @item
        )

    (call
        target: (identifier) @name
        (arguments (alias) @name)
        (#match? @name "^(defmodule|defprotocol)$")) @item
