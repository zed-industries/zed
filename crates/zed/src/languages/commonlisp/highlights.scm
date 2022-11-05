;; Copied from nvim: https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/master/queries/commonlisp/highlights.scm

(sym_lit) @variable

;; A highlighting for functions/macros in th cl namespace is available in theHamsta/nvim-treesitter-commonlisp
;(list_lit . (sym_lit) @function.builtin (#cl-standard-function? @function.builtin))
;(list_lit . (sym_lit) @function.builtin (#cl-standard-macro? @function.macro))

(dis_expr) @comment

(defun_keyword) @function.macro
(defun_header
  function_name: (_) @function)
(defun_header
  lambda_list: (list_lit (sym_lit) @parameter))
(defun_header
  keyword: (defun_keyword "defmethod")
  lambda_list: (list_lit (list_lit . (sym_lit) . (sym_lit) @symbol)))
(defun_header
  lambda_list: (list_lit (list_lit . (sym_lit) @parameter . (_))))
(defun_header
  specifier: (sym_lit) @symbol)

[":" "::" "."] @punctuation.special

[
  (accumulation_verb)
  (for_clause_word)
  "for"
  "and"
  "finally"
  "thereis"
  "always"
  "when"
  "if"
  "unless"
  "else"
  "do"
  "loop"
  "below"
  "in"
  "from"
  "across"
  "repeat"
  "being"
  "into"
  "with"
  "as"
  "while"
  "until"
  "return"
  "initially"
] @function.macro
"=" @operator

(include_reader_macro) @symbol
["#C" "#c"] @number

[(kwd_lit) (self_referential_reader_macro)] @symbol

(package_lit
  package: (_) @namespace)
"cl" @namespace

(str_lit) @string

(num_lit) @number

((sym_lit)  @boolean (#match? @boolean "^(t|T)$"))

(nil_lit) @constant.builtin

(comment) @comment

;; dynamic variables
((sym_lit) @variable.builtin
 (#match? @variable.builtin "^[*].+[*]$"))

;; quote
"'" @string.escape
(format_specifier) @string.escape
(quoting_lit) @string.escape

;; syntax quote
"`" @string.escape
"," @string.escape
",@" @string.escape
(syn_quoting_lit) @string.escape
(unquoting_lit) @none
(unquote_splicing_lit) @none


["(" ")"] @punctuation.bracket

(block_comment) @comment


(with_clause
  type: (_) @type)
(for_clause
  type: (_) @type)

;; defun-like things
(list_lit
 .
 (sym_lit) @function.macro
 .
 (sym_lit) @function
 (#eq? @function.macro "deftest"))

;;; Macros and Special Operators
(list_lit
 .
 (sym_lit) @function.macro
 ;; For a complete and more efficient version install theHamsta/nvim-treesitter-commonlisp
 (#any-of? @function.macro
          "let"
          "function"
          "the"
          "unwind-protect"
          "labels"
          "flet"
          "tagbody"
          "go"
          "symbol-macrolet"
          "symbol-macrolet"
          "progn"
          "prog1"
          "error"
          "or"
          "and"
          "defvar"
          "defparameter"
          "in-package"
          "defpackage"
          "case"
          "ecase"
          "typecase"
          "etypecase"
          "defstruct"
          "defclass"
          "if"
          "when"
          "unless"
          "cond"
          "switch"
          "declaim"
          "optimize"))

;; constant
((sym_lit) @constant
 (#match? @constant "^[+].+[+]$"))

(var_quoting_lit
  marker: "#'" @symbol
  value: (_) @symbol)

["#" "#p" "#P"] @symbol

(list_lit
 .
 (sym_lit) @function.builtin
 ;; For a complete and more efficient version install theHamsta/nvim-treesitter-commonlisp
 (#any-of? @function.builtin
           "mapcar"
           "reduce"
           "remove-if-not"
           "cons"
           "car"
           "last"
           "nth"
           "equal"
           "cdr"
           "first"
           "rest"
           "format"))

(list_lit
 .
 (sym_lit) @operator
 (#match? @operator "^([+*-+=<>]|<=|>=|/=)$"))


((sym_lit) @symbol
(#match? @symbol "^[&]"))

[(array_dimension) "#0A" "#0a"] @number

(char_lit) @character
