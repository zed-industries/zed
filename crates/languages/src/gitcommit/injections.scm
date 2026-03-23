((comment) @content
 (#set! injection.language "comment")
)

((scissors) @content
 (#set! "language" "diff"))

((rebase_command) @content
 (#set! "language" "git_rebase"))
