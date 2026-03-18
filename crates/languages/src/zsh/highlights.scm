; Comments
(comment) @comment

; Strings
(string) @string

; Keywords
[
  "if"
  "then"
  "else"
  "elif"
  "fi"
  "for"
  "while"
  "do"
  "done"
  "case"
  "esac"
  "in"
  "function"
  "local"
  "export"
  "unset"
] @keyword.control

; Numbers
(number) @number

; Variables
(variable_name) @variable
(special_variable_name) @variable.special

; Function names
(word) @function
