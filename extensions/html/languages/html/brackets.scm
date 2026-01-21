(("<" @open
  "/>" @close)
  (#set! rainbow.exclude))

(("</" @open
  ">" @close)
  (#set! rainbow.exclude))

(("<" @open
  ">" @close)
  (#set! rainbow.exclude))

(("\"" @open
  "\"" @close)
  (#set! rainbow.exclude))

((element
  (start_tag) @open
  (end_tag) @close)
  (#set! newline.only)
  (#set! rainbow.exclude))
