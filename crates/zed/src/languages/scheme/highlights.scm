;; Copied from nvim: https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/scheme/highlights.scm

;; A highlight query can override the highlights queries before it.
;; So the order is important.
;; We should highlight general rules, then highlight special forms.

(number) @number
(character) @character
(boolean) @boolean
(string) @string
[(comment)
 (block_comment)] @comment

;; highlight for datum comment
;; copied from ../clojure/highlights.scm
([(comment) (directive)] @comment
 (#set! "priority" 105))

(escape_sequence) @string.escape

["(" ")" "[" "]" "{" "}"] @punctuation.bracket

;; variables

(symbol) @variable
((symbol) @variable.builtin
 (#any-of? @variable.builtin "..." "."))

;; procedure

(list
 .
 (symbol) @function)

;; special forms

(list
 "["
 (symbol)+ @variable
 "]")

(list
 .
 (symbol) @_f
 .
 (list
   (symbol) @variable)
 (#any-of? @_f "lambda" "λ"))

(list
 .
 (symbol) @_f
 .
 (list
   (list
     (symbol) @variable))
 (#any-of? @_f
  "let" "let*" "let-syntax" "let-values" "let*-values" "letrec" "letrec*" "letrec-syntax"))

;; operators

((symbol) @operator
 (#any-of? @operator
  "+" "-" "*" "/" "=" "<=" ">=" "<" ">"))

;; keyword

((symbol) @keyword
 (#any-of? @keyword
  "define" "lambda" "λ" "begin" "do" "define-syntax"
  "and" "or"
  "if" "cond" "case" "when" "unless" "else" "=>"
  "let" "let*" "let-syntax" "let-values" "let*-values" "letrec" "letrec*" "letrec-syntax"
  "set!"
  "syntax-rules" "identifier-syntax"
  "quote" "unquote" "quote-splicing" "quasiquote" "unquote-splicing"
  "delay"
  "assert"
  "library" "export" "import" "rename" "only" "except" "prefix"))

((symbol) @conditional
 (#any-of? @conditional "if" "cond" "case" "when" "unless"))

;; quote

(abbreviation
 "'"
 (symbol)) @symbol

(list
 .
 (symbol) @_f
 (#eq? @_f "quote")) @symbol

;; library

(list
 .
 (symbol) @_lib
 .
 (symbol) @namespace

 (#eq? @_lib "library"))

;; builtin procedures
;; procedures in R5RS and R6RS but not in R6RS-lib

((symbol) @function.builtin
 (#any-of? @function.builtin
  ;; eq
  "eqv?" "eq?" "equal?"
  ;; number
  "number?" "complex?" "real?" "rational?" "integer?"
  "exact?" "inexact?"
  "zero?" "positive?" "negative?" "odd?" "even?" "finite?" "infinite?" "nan?"
  "max" "min"
  "abs" "quotient" "remainder" "modulo"
  "div" "div0" "mod" "mod0" "div-and-mod" "div0-and-mod0"
  "gcd" "lcm" "numerator" "denominator"
  "floor" "ceiling" "truncate" "round"
  "rationalize"
  "exp" "log" "sin" "cos" "tan" "asin" "acos" "atan"
  "sqrt" "expt"
  "exact-integer-sqrt"
  "make-rectangular" "make-polar" "real-part" "imag-part" "magnitude" "angle"
  "real-valued" "rational-valued?" "integer-valued?"
  "exact" "inexact" "exact->inexact" "inexact->exact"
  "number->string" "string->number"
  ;; boolean
  "boolean?" "not" "boolean=?"
  ;; pair
  "pair?" "cons" 
  "car" "cdr" 
  "caar" "cadr" "cdar" "cddr" 
  "caaar" "caadr" "cadar" "caddr" "cdaar" "cdadr" "cddar" "cdddr"
  "caaaar" "caaadr" "caadar" "caaddr" "cadaar" "cadadr" "caddar" "cadddr"
  "cdaaar" "cdaadr" "cdadar" "cdaddr" "cddaar" "cddadr" "cdddar" "cddddr"
  "set-car!" "set-cdr!"
  ;; list
  "null?" "list?"
  "list" "length" "append" "reverse" "list-tail" "list-ref"
  "map" "for-each"
  "memq" "memv" "member" "assq" "assv" "assoc"
  ;; symbol
  "symbol?" "symbol->string" "string->symbol" "symbol=?"
  ;; char
  "char?" "char=?" "char<?" "char>?" "char<=?" "char>=?"
  "char-ci=?" "char-ci<?" "char-ci>?" "char-ci<=?" "char-ci>=?"
  "char-alphabetic?" "char-numeric?" "char-whitespace?" "char-upper-case?" "char-lower-case?"
  "char->integer" "integer->char"
  "char-upcase" "char-downcase"
  ;; string
  "string?" "make-string" "string" "string-length" "string-ref" "string-set!"
  "string=?" "string-ci=?" "string<?" "string>?" "string<=?" "string>=?"
  "string-ci<?" "string-ci>?" "string-ci<=?" "string-ci>=?"
  "substring" "string-append" "string->list" "list->string"
  "string-for-each"
  "string-copy" "string-fill!"
  "string-upcase" "string-downcase"
  ;; vector
  "vector?" "make-vector" "vector" "vector-length" "vector-ref" "vector-set!"
  "vector->list" "list->vector" "vector-fill!" "vector-map" "vector-for-each"
  ;; bytevector
  "bytevector?" "native-endianness"
  "make-bytevector" "bytevector-length" "bytevector=?" "bytevector-fill!"
  "bytevector-copy!" "bytevector-copy"
  ;; error
  "error" "assertion-violation"
  ;; control
  "procedure?" "apply" "force"
  "call-with-current-continuation" "call/cc"
  "values" "call-with-values" "dynamic-wind"
  "eval" "scheme-report-environment" "null-environment" "interaction-environment"
  ;; IO
  "call-with-input-file" "call-with-output-file" "input-port?" "output-port?"
  "current-input-port" "current-output-port" "with-input-from-file" "with-output-to-file"
  "open-input-file" "open-output-file" "close-input-port" "close-output-port"
  ;; input
  "read" "read-char" "peek-char" "eof-object?" "char-ready?"
  ;; output
  "write" "display" "newline" "write-char"
  ;; system
  "load" "transcript-on" "transcript-off"))