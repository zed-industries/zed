; Match backticks within their `code_span` node so the delimiters are always
; paired per span and never across two separate spans.
((code_span
  (code_span_delimiter) @open
  (code_span_delimiter) @close)
  (#set! rainbow.exclude))
