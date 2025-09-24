(((comment) @_jsdoc_comment
  (#match? @_jsdoc_comment "(?s)^/[*][*][^*].*[*]/$")) @injection.content
  (#set! injection.language "jsdoc"))

(((comment) @_reference
  (#match? @_reference "^///\\s+<reference\\s+types=\"\\S+\"\\s*/>\\s*$")) @injection.content
  (#set! injection.language "html"))

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
  arguments: (template_string) @injection.content
                              (#set! injection.language "html")
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

;; Angular Component template injection
(call_expression
  function: [
    (identifier) @_decorator (#eq? @_decorator "Component")
    (member_expression property: (property_identifier) @_decorator (#eq? @_decorator "Component"))
  ]
  arguments: (arguments (object
    (pair
      key: (property_identifier) @_prop (#eq? @_prop "template")
      value: [
        (string) @injection.content
        (template_string) @injection.content
        (template_string (string_fragment) @injection.content)
      ]
    )))
  (#set! injection.language "angular"))

;; Angular Component styles injection
(call_expression
  function: [
    (identifier) @_decorator (#eq? @_decorator "Component")
    (member_expression property: (property_identifier) @_decorator (#eq? @_decorator "Component"))
  ]
  arguments: (arguments (object
    (pair
      key: (property_identifier) @_prop (#eq? @_prop "styles")
      value: [
        (string) @injection.content
        (template_string) @injection.content
        (template_string (string_fragment) @injection.content)
        (array (string) @injection.content)
        (array (template_string) @injection.content)
        (array (template_string (string_fragment)) @injection.content)
      ]
    )))
  (#set! injection.language "css"))
