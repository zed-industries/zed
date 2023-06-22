(
  (directive
    [
      (partial_expression_value)
      (expression_value)
      (ending_expression_value)
    ] @content)
  (#set! language "elixir")
  (#set! combined)
)

; expressions live within HTML tags, and do not need to be combined
;     <link href={ Routes.static_path(..) } />
((expression (expression_value) @content)
 (#set! language "elixir"))
