;; Literals

(num_lit) @number

[
  (char_lit)
  (str_lit)
] @string

[
 (bool_lit)
 (nil_lit)
] @constant.builtin

(kwd_lit) @constant

;; Comments

(comment) @comment

;; Treat quasiquotation as operators for the purpose of highlighting.

[
 "'"
 "`"
 "~"
 "@"
 "~@"
] @operator


(list_lit
  .
  (sym_lit) @function)

(list_lit
  .
  (sym_lit) @keyword
  (#match? @keyword
    "^(do|if|let|var|fn|fn*|loop*|recur|throw|try|catch|finally|set!|new|quote|->|->>)$"
    ))
