; We need impl this
; ((comment) @injection.content
;  (#set! injection.language "comment"))

((shell_command) @injection.content
 (#set! injection.language "bash"))
