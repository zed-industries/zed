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
; ----------------------------------------------------------------------------
; Parameters and variables
; NOTE: These are at the top, so that they have low priority,
; and don't override destructured parameters
(variable) @variable

(pat_wildcard) @variable

(function
  patterns:
    (patterns
      (_) @variable.parameter))

(exp_lambda
  (_)+ @variable.parameter
  "->")

(function
  infix:
    (infix
      lhs: (_) @variable.parameter))

(function
  infix:
    (infix
      rhs: (_) @variable.parameter))

; ----------------------------------------------------------------------------
; Literals and comments
(integer) @number

(exp_negation) @number

(exp_literal
  (float)) @number.float

(char) @character

(string) @string

(con_unit) @string.special.symbol ; unit, as in ()

(comment) @comment

; FIXME: The below documentation comment queries are inefficient
; and need to be anchored, using something like
; ((comment) @_first . (comment)+ @comment.documentation)
; once https://github.com/neovim/neovim/pull/24738 has been merged.
;
; ((comment) @comment.documentation
;   (#lua-match? @comment.documentation "^-- |"))
;
; ((comment) @_first @comment.documentation
;  (comment) @comment.documentation
;   (#lua-match? @_first "^-- |"))
;
; ((comment) @comment.documentation
;   (#lua-match? @comment.documentation "^-- %^"))
;
; ((comment) @_first @comment.documentation
;  (comment) @comment.documentation
;   (#lua-match? @_first "^-- %^"))
;
; ((comment) @comment.documentation
;   (#lua-match? @comment.documentation "^{-"))
;
; ((comment) @_first @comment.documentation
;  (comment) @comment.documentation
;   (#lua-match? @_first "^{-"))
; ----------------------------------------------------------------------------
; Punctuation
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

; ----------------------------------------------------------------------------
; Keywords, operators, includes
[
  "forall"
  "∀"
] @keyword.repeat

(pragma) @keyword.directive

[
  "if"
  "then"
  "else"
  "case"
  "of"
] @keyword.conditional

[
  "import"
  "qualified"
  "module"
] @keyword.import

[
  (operator)
  (constructor_operator)
  (type_operator)
  (tycon_arrow)
  (qualified_module) ; grabs the `.` (dot), ex: import System.IO
  (qualified_type)
  (qualified_variable)
  (all_names)
  (wildcard)
  "."
  ".."
  "="
  "|"
  "::"
  "=>"
  "->"
  "<-"
  "\\"
  "`"
  "@"
] @operator

(module) @module

((qualified_module
  (module) @constructor)
  .
  (module))

(qualified_type
  (module) @module)

(qualified_variable
  (module) @module)

(import
  (module) @module)

(import
  (module) @constructor
  .
  (module))

[
  (where)
  "let"
  "in"
  "class"
  "instance"
  "pattern"
  "data"
  "newtype"
  "family"
  "type"
  "as"
  "hiding"
  "deriving"
  "via"
  "stock"
  "anyclass"
  "do"
  "mdo"
  "rec"
  "infix"
  "infixl"
  "infixr"
] @keyword

; ----------------------------------------------------------------------------
; Functions and variables
(signature
  name: (variable) @function)

(function
  name: (variable) @function)

(function
  name: (variable) @variable
  rhs:
    [
      (exp_literal)
      (exp_apply
        (exp_name
          [
            (constructor)
            (variable)
            (qualified_variable)
          ]))
      (quasiquote)
      ((exp_name)
        .
        (operator))
    ])

(function
  name: (variable) @variable
  rhs:
    (exp_infix
      [
        (exp_literal)
        (exp_apply
          (exp_name
            [
              (constructor)
              (variable)
              (qualified_variable)
            ]))
        (quasiquote)
        ((exp_name)
          .
          (operator))
      ]))

; Consider signatures (and accompanying functions)
; with only one value on the rhs as variables
(signature
  .
  (variable) @variable
  .
  (_) .)

((signature
  .
  (variable) @_name
  .
  (_) .)
  .
  (function
    name: (variable) @variable)
  (#eq? @_name @variable))

; but consider a type that involves 'IO' a function
(signature
  name: (variable) @function
  .
  (type_apply
    (type_name) @_type)
  (#eq? @_type "IO"))

((signature
  name: (variable) @_name
  .
  (type_apply
    (type_name) @_type)
  (#eq? @_type "IO"))
  .
  (function
    name: (variable) @function)
  (#eq? @_name @function))

; functions with parameters
; + accompanying signatures
(function
  name: (variable) @function
  patterns: (patterns))

((signature) @function
  .
  (function
    name: (variable) @function
    patterns: (patterns)))

(function
  name: (variable) @function
  rhs: (exp_lambda))

; view patterns
(pat_view
  (exp_name
    [
      (variable) @function.call
      (qualified_variable
        (variable) @function.call)
    ]))

; consider infix functions as operators
(exp_infix
  [
    (variable) @operator
    (qualified_variable
      (variable) @operator)
  ])

; partially applied infix functions (sections) also get highlighted as operators
(exp_section_right
  [
    (variable) @operator
    (qualified_variable
      (variable) @operator)
  ])

(exp_section_left
  [
    (variable) @operator
    (qualified_variable
      (variable) @operator)
  ])

; function calls with an infix operator
; e.g. func <$> a <*> b
(exp_infix
  (exp_name
    [
      (variable) @function.call
      (qualified_variable
        ((module) @module
          (variable) @function.call))
    ])
  .
  (operator))

; infix operators applied to variables
((exp_name
  (variable) @variable)
  .
  (operator))

((operator)
  .
  (exp_name
    [
      (variable) @variable
      (qualified_variable
        (variable) @variable)
    ]))

; function calls with infix operators
((exp_name
  [
    (variable) @function.call
    (qualified_variable
      (variable) @function.call)
  ])
  .
  (operator) @_op
  (#any-of? @_op "$" "<$>" ">>=" "=<<"))

; right hand side of infix operator
((exp_infix
  [
    (operator)
    (variable)
  ] ; infix or `func`
  .
  (exp_name
    [
      (variable) @function.call
      (qualified_variable
        (variable) @function.call)
    ]))
  .
  (operator) @_op
  (#any-of? @_op "$" "<$>" "=<<"))

; function composition, arrows, monadic composition (lhs)
((exp_name
  [
    (variable) @function
    (qualified_variable
      (variable) @function)
  ])
  .
  (operator) @_op
  (#any-of? @_op "." ">>>" "***" ">=>" "<=<"))

; right hand side of infix operator
((exp_infix
  [
    (operator)
    (variable)
  ] ; infix or `func`
  .
  (exp_name
    [
      (variable) @function
      (qualified_variable
        (variable) @function)
    ]))
  .
  (operator) @_op
  (#any-of? @_op "." ">>>" "***" ">=>" "<=<"))

; function composition, arrows, monadic composition (rhs)
((operator) @_op
  .
  (exp_name
    [
      (variable) @function
      (qualified_variable
        (variable) @function)
    ])
  (#any-of? @_op "." ">>>" "***" ">=>" "<=<"))

; function defined in terms of a function composition
(function
  name: (variable) @function
  rhs:
    (exp_infix
      (_)
      .
      (operator) @_op
      .
      (_)
      (#any-of? @_op "." ">>>" "***" ">=>" "<=<")))

(exp_apply
  (exp_name
    [
      (variable) @function.call
      (qualified_variable
        (variable) @function.call)
    ]))

; function compositions, in parentheses, applied
; lhs
(exp_apply
  .
  (exp_parens
    (exp_infix
      (exp_name
        [
          (variable) @function.call
          (qualified_variable
            (variable) @function.call)
        ])
      .
      (operator))))

; rhs
(exp_apply
  .
  (exp_parens
    (exp_infix
      (operator)
      .
      (exp_name
        [
          (variable) @function.call
          (qualified_variable
            (variable) @function.call)
        ]))))

; variables being passed to a function call
(exp_apply
  (_)+
  .
  (exp_name
    [
      (variable) @variable
      (qualified_variable
        (variable) @variable)
    ]))

; Consider functions with only one value on the rhs
; as variables, e.g. x = Rec {} or x = foo
(function
  .
  (variable) @variable
  .
  [
    (exp_record)
    (exp_name
      [
        (variable)
        (qualified_variable)
      ])
    (exp_list)
    (exp_tuple)
    (exp_cond)
  ] .)

; main is always a function
; (this prevents `main = undefined` from being highlighted as a variable)
(function
  name: (variable) @function
  (#eq? @function "main"))

; scoped function types (func :: a -> b)
(pat_typed
  pattern:
    (pat_name
      (variable) @function)
  type: (fun))

; signatures that have a function type
; + functions that follow them
(signature
  (variable) @function
  (fun))

((signature
  (variable) @_type
  (fun))
  .
  (function
    (variable) @function)
  (#eq? @function @_type))

(signature
  (variable) @function
  (context
    (fun)))

((signature
  (variable) @_type
  (context
    (fun)))
  .
  (function
    (variable) @function)
  (#eq? @function @_type))

((signature
  (variable) @function
  (forall
    (context
      (fun))))
  .
  (function
    (variable)))

((signature
  (variable) @_type
  (forall
    (context
      (fun))))
  .
  (function
    (variable) @function)
  (#eq? @function @_type))

; ----------------------------------------------------------------------------
; Types
(type) @type

(type_star) @type

(type_variable) @type

(constructor) @constructor

; True or False
((constructor) @boolean
  (#any-of? @boolean "True" "False"))

; otherwise (= True)
((variable) @boolean
  (#eq? @boolean "otherwise"))

; ----------------------------------------------------------------------------
; Quasi-quotes
(quoter) @function.call

(quasiquote
  [
    (quoter) @_name
    (_
      (variable) @_name)
  ]
  (#eq? @_name "qq")
  (quasiquote_body) @string)

(quasiquote
  (_
    (variable) @_name)
  (#eq? @_name "qq")
  (quasiquote_body) @string)

; namespaced quasi-quoter
(quasiquote
  (_
    (module) @module
    .
    (variable) @function.call))

; Highlighting of quasiquote_body for other languages is handled by injections.scm
; ----------------------------------------------------------------------------
; Exceptions/error handling
((variable) @keyword.exception
  (#any-of? @keyword.exception "error" "undefined" "try" "tryJust" "tryAny" "catch" "catches" "catchJust" "handle" "handleJust" "throw" "throwIO" "throwTo" "throwError" "ioError" "mask" "mask_" "uninterruptibleMask" "uninterruptibleMask_" "bracket" "bracket_" "bracketOnErrorSource" "finally" "fail" "onException" "expectationFailure"))

; ----------------------------------------------------------------------------
; Debugging
((variable) @keyword.debug
  (#any-of? @keyword.debug "trace" "traceId" "traceShow" "traceShowId" "traceWith" "traceShowWith" "traceStack" "traceIO" "traceM" "traceShowM" "traceEvent" "traceEventWith" "traceEventIO" "flushEventLog" "traceMarker" "traceMarkerIO"))

; ----------------------------------------------------------------------------
; Fields
(field
  (variable) @variable.member)

(pat_field
  (variable) @variable.member)

(exp_projection
  field: (variable) @variable.member)

(import_item
  (type)
  .
  (import_con_names
    (variable) @variable.member))

(exp_field
  field:
    [
      (variable) @variable.member
      (qualified_variable
        (variable) @variable.member)
    ])
