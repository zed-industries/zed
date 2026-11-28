((comment) @injection.content
  (#set! injection.language "comment"))

(((comment) @_jsdoc_comment
  (#match? @_jsdoc_comment "(?s)^/[*][*][^*].*[*]/$")) @injection.content
  (#set! injection.language "jsdoc"))

((regex) @injection.content
  (#set! injection.language "regex"))

(call_expression
  function: (identifier) @_name
  (#eq? @_name "css")
  arguments: (template_string
    (string_fragment) @injection.content
    (#set! injection.language "css")))

(call_expression
  function: (identifier) @_name
  (#eq? @_name "css")
  arguments: (arguments
    (object
      (pair
        value: (template_string
          (string_fragment) @injection.content
          (#set! injection.language "css"))))))

(call_expression
  function: (member_expression
    object: (identifier) @_obj
    (#eq? @_obj "styled")
    property: (property_identifier))
  arguments: (template_string
    (string_fragment) @injection.content
    (#set! injection.language "css")))

(call_expression
  function: (call_expression
    function: (identifier) @_name
    (#eq? @_name "styled"))
  arguments: (template_string
    (string_fragment) @injection.content
    (#set! injection.language "css")))

(call_expression
  function: (identifier) @_name
  (#match? @_name "^(html|svg|mathml|raw)$")
  arguments: (template_string) @injection.content
  (#set! injection.language "html"))

(call_expression
  function: (identifier) @_name
  (#eq? @_name "js")
  arguments: (template_string
    (string_fragment) @injection.content
    (#set! injection.language "javascript")))

(call_expression
  function: (identifier) @_name
  (#eq? @_name "json")
  arguments: (template_string
    (string_fragment) @injection.content
    (#set! injection.language "json")))

(call_expression
  function: (identifier) @_name
  (#eq? @_name "sql")
  arguments: (template_string
    (string_fragment) @injection.content
    (#set! injection.language "sql")))

(call_expression
  function: (identifier) @_name
  (#eq? @_name "ts")
  arguments: (template_string
    (string_fragment) @injection.content
    (#set! injection.language "typescript")))

(call_expression
  function: (identifier) @_name
  (#match? @_name "^ya?ml$")
  arguments: (template_string
    (string_fragment) @injection.content
    (#set! injection.language "yaml")))

(call_expression
  function: (identifier) @_name
  (#match? @_name "^g(raph)?ql$")
  arguments: (template_string
    (string_fragment) @injection.content
    (#set! injection.language "graphql")))

(call_expression
  function: (identifier) @_name
  (#match? @_name "^g(raph)?ql$")
  arguments: (arguments
    (template_string
      (string_fragment) @injection.content
      (#set! injection.language "graphql"))))

(call_expression
  function: (identifier) @_name
  (#match? @_name "^iso$")
  arguments: (arguments
    (template_string
      (string_fragment) @injection.content
      (#set! injection.language "isograph"))))

; Parse the contents of strings and tagged template
; literals with leading ECMAScript comments:
; '/* html */' or '/*html*/'
(((comment) @_ecma_comment
  [
    (string
      (string_fragment) @injection.content)
    (template_string
      (string_fragment) @injection.content)
  ])
  (#match? @_ecma_comment "^\\/\\*\\s*html\\s*\\*\\/")
  (#set! injection.language "html"))

; '/* sql */' or '/*sql*/'
(((comment) @_ecma_comment
  [
    (string
      (string_fragment) @injection.content)
    (template_string
      (string_fragment) @injection.content)
  ])
  (#match? @_ecma_comment "^\\/\\*\\s*sql\\s*\\*\\/")
  (#set! injection.language "sql"))

; '/* gql */' or '/*gql*/'
; '/* graphql */' or '/*graphql*/'
(((comment) @_ecma_comment
  [
    (string
      (string_fragment) @injection.content)
    (template_string
      (string_fragment) @injection.content)
  ])
  (#match? @_ecma_comment "^\\/\\*\\s*(gql|graphql)\\s*\\*\\/")
  (#set! injection.language "graphql"))

; '/* css */' or '/*css*/'
(((comment) @_ecma_comment
  [
    (string
      (string_fragment) @injection.content)
    (template_string
      (string_fragment) @injection.content)
  ])
  (#match? @_ecma_comment "^\\/\\*\\s*(css)\\s*\\*\\/")
  (#set! injection.language "css"))

; '/* glsl */' or '/*glsl*/'
(((comment) @_ecma_comment
  [
    (string
      (string_fragment) @injection.content)
    (template_string
      (string_fragment) @injection.content)
  ])
  (#match? @_ecma_comment "^\\/\\*\\s*glsl\\s*\\*\\/")
  (#set! injection.language "glsl"))

; '/* wgsl */' or '/*wgsl*/'
(((comment) @_ecma_comment
  [
    (string
      (string_fragment) @injection.content)
    (template_string
      (string_fragment) @injection.content)
  ])
  (#match? @_ecma_comment "^\\/\\*\\s*wgsl\\s*\\*\\/")
  (#set! injection.language "WGSL/WESL"))

; ArkType definitions (https://arktype.io): the arguments of type(...),
; scope(...) and the other entrypoints and chained methods, and regex(...)'s
; pattern. The language comes from the ArkType extension.
((call_expression
  function: (identifier) @_name
  .
  arguments: (arguments) @injection.content)
  (#match? @_name "^([A-Za-z0-9_$]*[$])?(type|generic|scope|define|match|fn|module|[aA]rk[a-zA-Z]*)$")
  (#set! injection.language "arktype")
  (#set! injection.include-children))

; $.type(...), ark.scope(...), and chained methods: T.or(...), T.merge(...)
((call_expression
  function: (member_expression
    property: (property_identifier) @_name)
  .
  arguments: (arguments) @injection.content)
  (#match? @_name "^(([A-Za-z0-9_$]*[$])?(type|generic|scope|define|match|fn|module|[aA]rk[a-zA-Z]*)|and|or|case|in|extends|ifExtends|intersect|merge|exclude|extract|overlaps|subsumes|to|satisfies)$")
  (#set! injection.language "arktype")
  (#set! injection.include-children))

; regex(...): its first argument, when a string, is the pattern
((call_expression
  function: [
    (identifier) @_name
    (member_expression
      property: (property_identifier) @_name)
  ]
  .
  arguments: (arguments
    .
    [
      (string)
      (template_string)
    ] @injection.content))
  (#eq? @_name "regex")
  (#set! injection.language "arktype")
  (#set! injection.include-children))
