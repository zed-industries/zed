((comment) @injection.content
  (#set! injection.language "comment"))

; SQL -----------------------------------------------------------------------------
([
  ; function calls
  (call
    [
      (attribute
        attribute: (identifier))
      (identifier)
    ]
    arguments: (argument_list
      (comment) @_comment
      (string
        (string_content) @injection.content)))
  ; string variables
  ((comment) @_comment
    .
    (expression_statement
      (assignment
        right: (string
          (string_content) @injection.content))))
  ; string variables after a leading comment in a block; such comments are
  ; attached to the enclosing compound statement rather than the block, so the
  ; comment is a sibling of the block itself
  ((comment) @_comment
    .
    (block
      .
      (expression_statement
        (assignment
          right: (string
            (string_content) @injection.content)))))
]
  (#match? @_comment "^(#|#\\s+)(?i:sql)\\s*$")
  (#set! injection.language "sql"))
