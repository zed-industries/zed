; Inject markdown in docstrings
((string_literal) @content
  .
  [
    (module_definition)
    (abstract_definition)
    (struct_definition)
    (function_definition)
    (short_function_definition)
    (assignment)
    (const_statement)
  ]
  (#lua-match? @content "^\"\"\"")
  (#set! "language" "markdown")
  (#offset! @content 0 3 0 -3))

([
  (line_comment)
  (block_comment)
] @injection.content
  (#set! "language" "comment"))

((prefixed_string_literal
  prefix: (identifier) @_prefix) @content
  (#eq? @_prefix "r")
  (#set! "language" "regex")
  (#offset! @content 0 2 0 -1))
