; All raw text between Jinja constructs is parsed as one combined SQL document,
; so statements spanning a `{{ ref(...) }}` hole still highlight correctly.
((text) @injection.content
  (#set! injection.language "SQL (dbt)")
  (#set! injection.combined))

; Jinja expressions are Python-like; `ref('model')` gets call/string highlighting.
((expression) @injection.content
  (#set! injection.language "Python"))
