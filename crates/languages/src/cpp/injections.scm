((comment) @injection.content
 (#set! injection.language "comment")
)

((comment) @injection.content
  (#match? @injection.content "^(///|//!|/\\*\\*|/\\*!)(.*)")
  (#set! injection.language "doxygen")
  (#set! injection.include-children))

(preproc_def
    value: (preproc_arg) @injection.content
    (#set! injection.language "c++"))

(preproc_function_def
    value: (preproc_arg) @injection.content
    (#set! injection.language "c++"))

(raw_string_literal
  delimiter: (raw_string_delimiter) @injection.language
  (raw_string_content) @injection.content)
