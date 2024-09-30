(preproc_def
    value: (preproc_arg) @content
    (#set! "language" "c++"))

(preproc_function_def
    value: (preproc_arg) @content
    (#set! "language" "c++"))

(raw_string_literal
  delimiter: (raw_string_delimiter) @language
  (raw_string_content) @content)
