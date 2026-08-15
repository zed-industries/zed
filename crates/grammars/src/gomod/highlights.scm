[
  "require"
  "replace"
  "go"
  "toolchain"
  "tool"
  "exclude"
  "retract"
  "module"
  "ignore"
] @keyword

"=>" @operator

(comment) @comment

[
  (version)
  (go_version)
] @string

((comment) @comment.todo
  (#match? @comment.todo "(?i)\\b(todo|fixme)\\b"))
