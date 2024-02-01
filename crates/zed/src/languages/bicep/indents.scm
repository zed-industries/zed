[
  (array)
  (object)
] @indent

"}" @indent_end

[ "{" "}" ] @branch

[ "[" "]" ] @branch

[ "(" ")" ] @branch

[
  (ERROR)
  (comment)
  (diagnostic_comment)
] @auto
