([
  (block_comment)
  (line_comment)
] @injection.content
  (#set! injection.language "comment"))

((block_comment) @injection.content
  (#lua-match? @injection.content "/[*][!<*][^a-zA-Z]")
  (#set! injection.language "doxygen"))

((method_invocation
  name: (identifier) @_method
  arguments:
    (argument_list
      .
      (string_literal
        .
        (_) @injection.content)))
  (#any-of? @_method "format" "printf")
  (#set! injection.language "printf"))