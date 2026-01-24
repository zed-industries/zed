((comment) @injection.content
 (#set! injection.language "comment")
)

(((comment) @_jsdoc_comment
  (#match? @_jsdoc_comment "(?s)^/[*][*][^*].*[*]/$")) @injection.content
  (#set! injection.language "jsdoc"))

(preproc_def
    value: (preproc_arg) @injection.content
    (#set! injection.language "c"))

(preproc_function_def
    value: (preproc_arg) @injection.content
    (#set! injection.language "c"))
