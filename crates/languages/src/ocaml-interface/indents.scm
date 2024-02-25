[
  (type_binding)

  (value_specification)
  (method_specification)

  (external)
  (field_declaration)
] @indent

(_ "<" ">" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent

(_ "object" @start "end" @end) @indent

(signature
  "sig" @start
  "end" @end) @indent

";;" @outdent
