([
  (line_comment)
  (block_comment)
] @injection.content
  (#set! injection.language "comment"))

(macro_invocation
  macro: [
    (identifier) @_macro_name
    (scoped_identifier
      (identifier) @_macro_name .)
  ]
  (#not-any-of? @_macro_name "view" "html" "json")
  (token_tree) @injection.content
  (#set! injection.language "rust"))

; we need a better way for the leptos extension to declare that
; it wants to inject inside of rust, instead of modifying the rust
; injections to support leptos injections
(macro_invocation
  macro: [
    (identifier) @_macro_name
    (scoped_identifier
      (identifier) @_macro_name .)
  ]
  (#any-of? @_macro_name "view" "html")
  (token_tree) @injection.content
  (#set! injection.language "rstml"))

(macro_invocation
  macro: [
    (identifier) @_macro_name
    (scoped_identifier
      (identifier) @_macro_name .)
  ]
  (#any-of? @_macro_name "sql")
  (_) @injection.content
  (#set! injection.language "sql"))

; sqlx
(macro_invocation
  macro: (scoped_identifier) @_macro_name
  [
    ; query, query_scalar
    (token_tree
      . "("
      . [
        (string_literal
          (string_content) @injection.content)
        (raw_string_literal
          (string_content) @injection.content)
      ])
    ; query_as
    (token_tree
      . "("
      . (identifier)
      . ","
      . [
        (string_literal
          (string_content) @injection.content)
        (raw_string_literal
          (string_content) @injection.content)
      ])
  ]
  ; query macro must have a `sqlx::` prefix, to avoid false positives
  (#match? @_macro_name "^sqlx::query(_as|_scalar)?(_unchecked)?$")
  (#set! injection.language "sql"))

(call_expression
  function: (scoped_identifier) @_fn_path
  arguments: [
    ; query, query_scalar, raw_sql
    (arguments
      . [
        (string_literal
          (string_content) @injection.content)
        (raw_string_literal
          (string_content) @injection.content)
      ])
    ; query_as
    (arguments
      . (identifier)
      . [
        (string_literal
          (string_content) @injection.content)
        (raw_string_literal
          (string_content) @injection.content)
      ])
  ]
  ; query function must have a `sqlx::` prefix, to avoid false positives
  (#match? @_fn_path "^sqlx::((query(_as|_scalar)?(_with)?)|raw_sql)$")
  (#set! injection.language "sql"))

; lazy_regex
(macro_invocation
  macro: [
    (identifier) @_macro_name
    (scoped_identifier
      (identifier) @_macro_name .)
  ]
  (token_tree
    [
      (string_literal
        (string_content) @injection.content)
      (raw_string_literal
        (string_content) @injection.content)
    ])
  (#set! injection.language "regex")
  (#any-of? @_macro_name "regex" "bytes_regex"))

(call_expression
  function: (scoped_identifier) @_fn_path
  arguments: (arguments
    [
      (string_literal
        (string_content) @injection.content)
      (raw_string_literal
        (string_content) @injection.content)
    ])
  (#match? @_fn_path ".*Regex(Builder)?::new")
  (#set! injection.language "regex"))
