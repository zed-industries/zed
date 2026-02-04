[
  (string)
  (raw_string)
  (heredoc_body)
  (heredoc_start)
  (heredoc_end)
  (ansi_c_string)
  (word)
] @string

(variable_name) @variable

[
  "export"
  "function"
  "unset"
  "local"
  "declare"
] @keyword

[
  "case"
  "do"
  "done"
  "elif"
  "else"
  "esac"
  "fi"
  "for"
  "if"
  "in"
  "select"
  "then"
  "until"
  "while"
] @keyword.control

(comment) @comment

(function_definition name: (word) @function)
(command_name (word) @function)

[
  (file_descriptor)
  (number)
] @number

(regex) @string.regex

[
  (command_substitution)
  (process_substitution)
  (expansion)
] @embedded


[
  "$"
  "&&"
  ">"
  "<<"
  ">>"
  ">&"
  ">&-"
  "<"
  "|"
  ":"
  "//"
  "/"
  "%"
  "%%"
  "#"
  "##"
  "="
  "=="
] @operator

(test_operator) @keyword.operator

[
  ";"
] @punctuation.delimiter

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

(simple_expansion
  "$" @punctuation.special)
(expansion
  "${" @punctuation.special
  "}" @punctuation.special) @embedded

(command_substitution
  "$(" @punctuation.special
  ")" @punctuation.special)

(
  (command (_) @constant)
  (#match? @constant "^-")
)
