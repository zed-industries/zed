; Bracket matching pairs
("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)

; Rainbow bracket scopes
[
  (character_class)
  (anonymous_capturing_group)
  (named_capturing_group)
  (non_capturing_group)
  (count_quantifier)
  (character_class_escape)
] @rainbow.scope

; Rainbow brackets
[
  "(?" "(?:"
  "(?<" ">"
  "(" ")"
  "[" "]"
  "{" "}"
] @rainbow.bracket