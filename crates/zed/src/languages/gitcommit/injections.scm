((diff) @injection.content
 (#set! injection.combined)
 (#set! injection.language "diff"))

((rebase_command) @injection.content
 (#set! injection.combined)
 (#set! injection.language "git_rebase"))

