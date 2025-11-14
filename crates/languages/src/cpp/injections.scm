((comment) @injection.content
 (#set! injection.language "comment")
)

(preproc_def
    value: (preproc_arg) @injection.content
    (#set! injection.language "c++"))

(preproc_function_def
    value: (preproc_arg) @injection.content
    (#set! injection.language "c++"))

(raw_string_literal
  delimiter: (raw_string_delimiter) @injection.language
  (raw_string_content) @injection.content)
