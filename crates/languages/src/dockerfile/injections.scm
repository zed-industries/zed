; We need impl this
; ((comment) @injection.content
;  (#set! injection.language "comment"))

((shell_command) @content
 (#set! "language" "bash"))
