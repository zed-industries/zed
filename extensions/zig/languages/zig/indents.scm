[
  (block)
  (switch_expression)
  (initializer_list)
] @indent.begin

(block
  "}" @indent.end)

(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent

[
  (comment)
  (multiline_string)
] @indent.ignore
