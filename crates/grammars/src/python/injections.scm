((comment) @injection.content
  (#set! injection.language "comment"))

; SQL -----------------------------------------------------------------------------

; via # sql
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
]
  (#match? @_comment "^(#|#\\s+)(?i:sql)\\s*$")
  (#set! injection.language "sql"))


; via --sql and --end-sql
((string
  (string_content) @injection.content)
  (#match? @injection.content "(?is)^\\s*--\\s*sql\\b")
  (#set! injection.language "sql"))