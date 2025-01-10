["(" ")" "[" "]" "{" "}"] @punctuation.bracket

(number) @number
(character) @constant.builtin
(boolean) @constant.builtin

(symbol) @variable
(string) @string

(escape_sequence) @string.escape

[(comment)
 (block_comment)
 (directive)] @comment

((symbol) @operator
 (#match? @operator "^(\\+|-|\\*|/|=|>|<|>=|<=)$"))

(list
  .
  (symbol) @function)

(list
  .
  (symbol) @keyword
  (#match? @keyword
   "^(define-syntax|let\\*|lambda|λ|case|=>|quote-splicing|unquote-splicing|set!|let|letrec|letrec-syntax|let-values|let\\*-values|do|else|define|cond|syntax-rules|unquote|begin|quote|let-syntax|and|if|quasiquote|letrec|delay|or|when|unless|identifier-syntax|assert|library|export|import|rename|only|except|prefix)$"
   ))
