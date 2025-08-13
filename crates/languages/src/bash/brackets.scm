; Bracket matching pairs
("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)
("\"" @open "\"" @close)
("`" @open "`" @close)
(("do" @open "done" @close) (#set! newline.only))
((case_statement ("in" @open "esac" @close)) (#set! newline.only))
((if_statement (elif_clause ("then" @open)) (else_clause ("else" @close))) (#set! newline.only))
((if_statement (else_clause ("else" @open)) "fi" @close) (#set! newline.only))
((if_statement ("then" @open) (elif_clause ("elif" @close))) (#set! newline.only))
((if_statement ("then" @open) (else_clause ("else" @close))) (#set! newline.only))
((if_statement ("then" @open "fi" @close)) (#set! newline.only))

; Rainbow bracket scopes
[
  (function_definition)
  (compound_statement)
  (subshell)
  (test_command)
  (subscript)
  (parenthesized_expression)
  (array)
  (expansion)
  (command_substitution)
] @rainbow.scope

; Rainbow brackets
[
  "(" ")"
  "((" "))"
  "${" "$("
  "{" "}"
  "[" "]"
  "[[" "]]"
] @rainbow.bracket