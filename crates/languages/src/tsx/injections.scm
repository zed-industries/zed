((comment) @injection.content
 (#set! injection.language "comment")
)

(((comment) @_jsdoc_comment
  (#match? @_jsdoc_comment "(?s)^/[*][*][^*].*[*]/$")) @injection.content
  (#set! injection.language "jsdoc"))

((regex) @injection.content
  (#set! injection.language "regex"))

(call_expression
  function: (identifier) @_name (#eq? @_name "css")
  arguments: (template_string (string_fragment) @injection.content
                              (#set! injection.language "css"))
)

(call_expression
  function: (member_expression
    object: (identifier) @_obj (#eq? @_obj "styled")
    property: (property_identifier))
  arguments: (template_string (string_fragment) @injection.content
                              (#set! injection.language "css"))
)

(call_expression
  function: (call_expression
    function: (identifier) @_name (#eq? @_name "styled"))
  arguments: (template_string (string_fragment) @injection.content
                              (#set! injection.language "css"))
)

(call_expression
  function: (identifier) @_name (#eq? @_name "html")
  arguments: (template_string (string_fragment) @injection.content
                              (#set! injection.language "html"))
)

(call_expression
  function: (identifier) @_name (#eq? @_name "js")
  arguments: (template_string (string_fragment) @injection.content
                              (#set! injection.language "javascript"))
)

(call_expression
  function: (identifier) @_name (#eq? @_name "json")
  arguments: (template_string (string_fragment) @injection.content
                              (#set! injection.language "json"))
)

(call_expression
  function: (identifier) @_name (#eq? @_name "sql")
  arguments: (template_string (string_fragment) @injection.content
                              (#set! injection.language "sql"))
)

(call_expression
  function: (identifier) @_name (#eq? @_name "ts")
  arguments: (template_string (string_fragment) @injection.content
                              (#set! injection.language "typescript"))
)

(call_expression
  function: (identifier) @_name (#match? @_name "^ya?ml$")
  arguments: (template_string (string_fragment) @injection.content
                              (#set! injection.language "yaml"))
)

(call_expression
  function: (identifier) @_name (#match? @_name "^g(raph)?ql$")
  arguments: (template_string (string_fragment) @injection.content
                              (#set! injection.language "graphql"))
)

(call_expression
  function: (identifier) @_name (#match? @_name "^g(raph)?ql$")
  arguments: (arguments (template_string (string_fragment) @injection.content
                              (#set! injection.language "graphql")))
)

(call_expression
  function: (identifier) @_name(#match? @_name "^iso$")
  arguments: (arguments (template_string (string_fragment) @injection.content
                              (#set! injection.language "isograph")))
)

; Parse the contents of strings and tagged template
; literals with leading ECMAScript comments:
; '/* html */' or '/*html*/'
(
  ((comment) @_ecma_comment [
    (string (string_fragment) @injection.content)
    (template_string (string_fragment) @injection.content)
  ])
  (#match? @_ecma_comment "^\\/\\*\\s*html\\s*\\*\\/")
  (#set! injection.language "html")
)

; '/* sql */' or '/*sql*/'
(
  ((comment) @_ecma_comment [
    (string (string_fragment) @injection.content)
    (template_string (string_fragment) @injection.content)
  ])
  (#match? @_ecma_comment "^\\/\\*\\s*sql\\s*\\*\\/")
  (#set! injection.language "sql")
)

; '/* gql */' or '/*gql*/'
; '/* graphql */' or '/*graphql*/'
(
  ((comment) @_ecma_comment [
    (string (string_fragment) @injection.content)
    (template_string (string_fragment) @injection.content)
  ])
  (#match? @_ecma_comment "^\\/\\*\\s*(gql|graphql)\\s*\\*\\/")
  (#set! injection.language "graphql")
)

; '/* css */' or '/*css*/'
(
  ((comment) @_ecma_comment [
    (string (string_fragment) @injection.content)
    (template_string (string_fragment) @injection.content)
  ])
  (#match? @_ecma_comment "^\\/\\*\\s*(css)\\s*\\*\\/")
  (#set! injection.language "css")
)
