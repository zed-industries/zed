(integer) @number
(float) @number
(complex) @number

(string) @string
(string (string_content (escape_sequence) @string.escape))

(comment) @comment

[
  "?" ":=" "=" "<-" "<<-" "->" "->>"
  "~" "|>" "||" "|" "&&" "&"
  "<" "<=" ">" ">=" "==" "!="
  "+" "-" "*" "/" "::" ":::"
  "**" "^" "$" "@" ":" "!"
  "special"
] @operator

[
  "("  ")"
  "{"  "}"
  "["  "]"
  "[[" "]]"
] @punctuation.bracket

(comma) @punctuation.delimiter

(identifier) @variable

(binary_operator
    lhs: (identifier) @function
    operator: "<-"
    rhs: (function_definition)
)

(binary_operator
    lhs: (identifier) @function
    operator: "="
    rhs: (function_definition)
)

(call function: (identifier) @function)

(
    (call function: (identifier) @keyword)
    (#eq? @keyword "return")
)

(parameters (parameter name: (identifier) @variable.parameter))
(arguments (argument name: (identifier) @variable.parameter))

(namespace_operator lhs: (identifier) @namespace)

(call
    function: (namespace_operator rhs: (identifier) @function)
)

(function_definition name: "function" @keyword.function)
(function_definition name: "\\" @operator)

[
  "in"
  (next)
  (break)
] @keyword

[
  "if"
  "else"
] @conditional

[
  "while"
  "repeat"
  "for"
] @repeat

[
  (true)
  (false)
] @boolean

[
  (null)
  (inf)
  (nan)
  (na)
  (dots)
  (dot_dot_i)
] @constant.builtin

(ERROR) @error
