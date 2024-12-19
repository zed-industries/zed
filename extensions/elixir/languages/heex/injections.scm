(
  (directive
    [
      (partial_expression_value)
      (expression_value)
      (ending_expression_value)
    ] @injection.content)
  (#set! injection.language "elixir")
  (#set! injection.combined)
)

((expression (expression_value) @injection.content)
 (#set! injection.language "elixir"))
