((comment) @content
  (#set! "language" "comment")
  (#match? @content "^//"))

; https://github.com/helix-editor/helix/pull/2763
; Parse JSDoc annotations in multiline comments (#7826)
; ((comment) @content
;  (#set! "language" "jsdoc")
;  (#match? @content "^/\\*+"))
