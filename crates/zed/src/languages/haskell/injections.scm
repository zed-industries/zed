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
; -----------------------------------------------------------------------------
; General language injection
(quasiquote
  (quoter) @injection.language
  (quasiquote_body) @injection.content)

((comment) @injection.content
  (#set! injection.language "comment"))

; -----------------------------------------------------------------------------
; shakespeare library
; NOTE: doesn't support templating
; TODO: add once CoffeeScript parser is added
; ; CoffeeScript: Text.Coffee
; (quasiquote
;  (quoter) @_name
;  (#eq? @_name "coffee")
;  ((quasiquote_body) @injection.content
;   (#set! injection.language "coffeescript")))
; CSS: Text.Cassius, Text.Lucius
(quasiquote
  (quoter) @_name
  (#any-of? @_name "cassius" "lucius")
  (quasiquote_body) @injection.content
  (#set! injection.language "css"))

; HTML: Text.Hamlet
(quasiquote
  (quoter) @_name
  (#any-of? @_name "shamlet" "xshamlet" "hamlet" "xhamlet" "ihamlet")
  (quasiquote_body) @injection.content
  (#set! injection.language "html"))

; JS: Text.Julius
(quasiquote
  (quoter) @_name
  (#any-of? @_name "js" "julius")
  (quasiquote_body) @injection.content
  (#set! injection.language "javascript"))

; TS: Text.TypeScript
(quasiquote
  (quoter) @_name
  (#any-of? @_name "tsc" "tscJSX")
  (quasiquote_body) @injection.content
  (#set! injection.language "typescript"))

; -----------------------------------------------------------------------------
; HSX
(quasiquote
  (quoter) @_name
  (#eq? @_name "hsx")
  (quasiquote_body) @injection.content
  (#set! injection.language "html"))

; -----------------------------------------------------------------------------
; Inline JSON from aeson
(quasiquote
  (quoter) @_name
  (#eq? @_name "aesonQQ")
  (quasiquote_body) @injection.content
  (#set! injection.language "json"))

; -----------------------------------------------------------------------------
; SQL
; postgresql-simple
(quasiquote
  (quoter) @injection.language
  (#eq? @injection.language "sql")
  (quasiquote_body) @injection.content)

(quasiquote
  (quoter) @_name
  (#any-of? @_name "persistUpperCase" "persistLowerCase" "persistWith")
  (quasiquote_body) @injection.content
  (#set! injection.language "haskell_persistent"))
