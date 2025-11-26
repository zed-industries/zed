((comment) @injection.content
  (#match? @injection.content "^(///|//!|/\\*\\*|/\\*!)(.*)")
  (#set! injection.language "doxygen")
  (#set! injection.include-children))

(preproc_def
    value: (preproc_arg) @injection.content
    (#set! injection.language "c"))

(preproc_function_def
    value: (preproc_arg) @injection.content
    (#set! injection.language "c"))
