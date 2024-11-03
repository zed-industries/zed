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

; ------------------------------------------------------------------------------
; Literals and comments
(integer) @number

(literal
  (number)) @number.float

(char) @character

[
  (string)
  (pat_string)
  (triple_quote_string)
] @string

(comment) @comment

; ------------------------------------------------------------------------------
; Punctuation
[
  "("
  ")"
  "{"
  "@{"
  "}"
  "["
  "[<"
  "]"
] @punctuation.bracket

[
  (comma)
  (colon)
  (pat_op)
  (tuple_operator)
] @punctuation.delimiter

(pat_name
  (loname) @variable.parameter)

; ------------------------------------------------------------------------------
; Types
(signature
  (loname) @type)

(signature
  (caname) @constructor)

(caname) @constructor

; ------------------------------------------------------------------------------
; Keywords, operators, imports
[
  "if"
  "then"
  "else"
  "case"
  "of"
] @keyword.conditional

(module) @module

[
  "import"
  "module"
  "namespace"
  "parameters"
] @keyword.import

[
  (operator)
  (equal)
  (wildcard)
  "."
  "|"
  "=>"
  "⇒"
  "<="
  "⇐"
  "->"
  "→"
  "<-"
  "←"
  "\\"
  "`"
] @operator

(qualified_loname
  (caname) @module)

(qualified_caname
  (caname) @constructor)

(qualified_operator
  (caname) @module)

(import
  (caname) @module)

[
  (where)
  "let"
  "in"
  "rewrite"
  "interface"
  "implementation"
  "using"
  "data"
  "record"
  "as"
  "do"
  (forall)
  (fixity)
  (visibility)
  (totality)
  (quantity)
  (impossible)
  (with)
  (proof)
  "="
  ":="
] @keyword

(hole) @label

[
  (pragma_language)
  (pragma_default)
  (pragma_builtin)
  (pragma_name)
  (pragma_ambiguity_depth)
  (pragma_auto_implicit_depth)
  (pragma_logging)
  (pragma_prefix_record_projections)
  (pragma_transform)
  (pragma_unbound_implicits)
  (pragma_auto_lazy)
  (pragma_search_timeout)
  (pragma_nf_metavar_threshold)
  (pragma_cg)
  (pragma_allow_overloads)
  (pragma_deprecate)
  (pragma_inline)
  (pragma_noinline)
  (pragma_tcinline)
  (pragma_hide)
  (pragma_unhide)
  (pragma_unsafe)
  (pragma_spec)
  (pragma_foreign)
  (pragma_foreign_impl)
  (pragma_export)
  (pragma_nomangle)
  (pragma_hint)
  (pragma_defaulthint)
  (pragma_globalhint)
  (pragma_extern)
  (pragma_macro)
  (pragma_start)
  (pragma_rewrite)
  (pragma_pair)
  (pragma_integerLit)
  (pragma_stringLit)
  (pragma_charLit)
  (pragma_doubleLit)
  (pragma_TTImpLit)
  (pragma_declsLit)
  (pragma_nameLit)
  (pragma_runElab)
  (pragma_search)
  (pragma_World)
  (pragma_MkWorld)
  (pragma_syntactic)
] @label

; ------------------------------------------------------------------------------
; Functions and variables
(exp_name
  (loname) @function)

(exp_name
  (caname) @constructor)

(exp_record_access
  field: (_) @variable.member)

(signature
  name: [
    (loname)
    (caname)
  ] @function)

(function
  (lhs
    (funvar
      subject: [
        (loname)
        (caname)
      ] @function)))

(data
  name: (data_name) @type)

(interface_head
  name: (interface_name) @type)

(implementation_head
  (interface_name) @type)
