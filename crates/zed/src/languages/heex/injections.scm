((directive (partial_expression_value) @content)
 (#set! language "elixir")
 (#set! include-children)
 (#set! combined))

; Regular expression_values do not need to be combined
((directive (expression_value) @content)
 (#set! language "elixir"))

((expression (expression_value) @content)
 (#set! language "elixir"))
