(("<" @open
  "/>" @close)
  (#set! rainbow.exclude))

(("<" @open
  ">" @close)
  (#set! rainbow.exclude))

(("</" @open
  ">" @close)
  (#set! rainbow.exclude))

(("\"" @open
  "\"" @close)
  (#set! rainbow.exclude))

((doctype
	"<!" @open
    "doctype" @open
    ">" @close)
  (#set! newline.only))

((element
  (start_tag
  	"<" @open
  	(tag_name) @open
    ">" @open)
  (end_tag) @close)
  (#set! newline.only))

((element
  (start_tag
    "<" @open
  	(tag_name) @open
    ">" @close)
  .)
  (#set! newline.only))

((element
  (self_closing_tag
    "<" @open
  	(tag_name) @open
    "/>" @close))
  (#set! newline.only))

((element
  (erroneous_end_tag
    "</" @open
  	(erroneous_end_tag_name) @open
    ">" @close))
  (#set! newline.only))

((element
  (start_tag
  	"<" @open
  	(tag_name) @open
    ">" @close)
  (erroneous_end_tag))
  (#set! newline.only))
