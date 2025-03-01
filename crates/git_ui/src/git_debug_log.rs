// What do we want for the debug view?
// 1. A feed of all run git commands (libgit2 and cli)
// 2. Timings of those commands
//
// What do we want for errors and info?
// 1. we need to parse the output of git commands for `remote:` prefixed things,
// and show them on success
// 2. We need to surface git errors in the UI at all
//
