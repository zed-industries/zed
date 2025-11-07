("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)
("<" @open ">" @close)
("<" @open "/>" @close)
("</" @open ">" @close)
("\"" @open "\"" @close)
("'" @open "'" @close)
("`" @open "`" @close)

((jsx_element (jsx_opening_element) @open (jsx_closing_element) @close) (#set! newline.only))

; Rainbow bracket scopes
[
  (object)
  (array)
  (arguments)
  (formal_parameters)
  (statement_block)
  (parenthesized_expression)
  (call_expression)
  (type_parameters)
  (type_arguments)
  (jsx_element)
  (jsx_self_closing_element)
] @rainbow.scope

; Rainbow brackets
[
  "[" "]"
  "{" "}"
  "(" ")"
] @rainbow.bracket

; TypeScript generics (but not JSX tags)
(type_parameters ["<" ">"] @rainbow.bracket)
(type_arguments ["<" ">"] @rainbow.bracket)
