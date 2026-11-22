("(" @open
  ")" @close)

("[" @open
  "]" @close)

("{" @open
  "}" @close)

(("<" @open
  ">" @close)
  (#set! rainbow.exclude))

(("<" @open
  "/>" @close)
  (#set! rainbow.exclude))

(("</" @open
  ">" @close)
  (#set! rainbow.exclude))

(("\"" @open
  "\"" @close)
  (#set! rainbow.exclude))

(("'" @open
  "'" @close)
  (#set! rainbow.exclude))

(("`" @open
  "`" @close)
  (#set! rainbow.exclude))

((jsx_element
  (jsx_opening_element
  	"<" @open
    ">" @open)
  (jsx_closing_element
    "</" @close
    ">" @close))
  (#set! newline.only))

((jsx_element
  (jsx_opening_element
  	"<" @open
  	(identifier) @open
    ">" @open)
  (jsx_closing_element) @close)
  (#set! newline.only))

((jsx_element
  (jsx_self_closing_element
    "<" @open
  	(identifier) @open
    "/>" @close))
  (#set! newline.only))
