[
  "replace"
  "go"
  "use"
] @keyword

"=>" @operator

(comment) @comment

[
  (version)
  (go_version)
] @string

((comment) @comment.todo
  (#match? @comment.todo "(?i)\\b(todo|fixme)\\b"))
