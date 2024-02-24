;; Copyright 2022 nvim-treesitter
;;
;; Licensed under the Apache License, Version 2.0 (the "License");
;; you may not use this file except in compliance with the License.
;; You may obtain a copy of the License at
;;
;;     http://www.apache.org/licenses/LICENSE-2.0
;;
;; Unless required by applicable law or agreed to in writing, software
;; distributed under the License is distributed on an "AS IS" BASIS,
;; WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
;; See the License for the specific language governing permissions and
;; limitations under the License.

;; ----------------------------------------------------------------------------
;; Literals and comments

(integer) @number
(exp_negation) @number
(exp_literal (number)) @float
(char) @character
[
  (string)
  (triple_quote_string)
] @string

(comment) @comment


;; ----------------------------------------------------------------------------
;; Punctuation

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

[
  (comma)
  ";"
] @punctuation.delimiter


;; ----------------------------------------------------------------------------
;; Keywords, operators, includes

[
  "forall"
  "∀"
] @keyword

;; (pragma) @constant

[
  "if"
  "then"
  "else"
  "case"
  "of"
] @keyword

[
  "import"
  "module"
] @keyword

[
  (operator)
  (constructor_operator)
  (type_operator)
  (qualified_module)  ; grabs the `.` (dot), ex: import System.IO
  (all_names)
  (wildcard)
  "="
  "|"
  "::"
  "=>"
  "->"
  "<-"
  "\\"
  "`"
  "@"
  "∷"
  "⇒"
  "<="
  "⇐"
  "→"
  "←"
] @operator

(module) @title

[
  (where)
  "let"
  "in"
  "class"
  "instance"
  "derive"
  "foreign"
  "data"
  "newtype"
  "type"
  "as"
  "hiding"
  "do"
  "ado"
  "infix"
  "infixl"
  "infixr"
] @keyword


;; ----------------------------------------------------------------------------
;; Functions and variables

(variable) @variable
(pat_wildcard) @variable

(signature name: (variable) @type)
(function
  name: (variable) @function
  patterns: (patterns))


(exp_infix (exp_name) @function (#set! "priority" 101))
(exp_apply . (exp_name (variable) @function))
(exp_apply . (exp_name (qualified_variable (variable) @function)))


;; ----------------------------------------------------------------------------
;; Types

(type) @type
(type_variable) @type

(constructor) @constructor

; True or False
((constructor) @_bool (#match? @_bool "(True|False)")) @boolean
