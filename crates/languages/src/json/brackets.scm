("[" @open "]" @close)
("{" @open "}" @close)
("\"" @open "\"" @close)

; Rainbow bracket scopes
[
  (object)
  (array)
] @rainbow.scope

; Rainbow brackets
[
  "[" "]"
  "{" "}"
] @rainbow.bracket
