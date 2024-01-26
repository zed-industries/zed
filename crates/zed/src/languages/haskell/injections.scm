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

; CSS: Text.Cassius, Text.Lucius
(quasiquote
  (quoter) @_name
  (#any-of? @_name "cassius" "lucius")
  (quasiquote_body) @injection.content
  (#set! "language" "css"))

; HTML: Text.Hamlet
(quasiquote
  (quoter) @_name
  (#any-of? @_name "shamlet" "xshamlet" "hamlet" "xhamlet" "ihamlet")
  (quasiquote_body) @injection.content
  (#set! "language" "html"))

; JS: Text.Julius
(quasiquote
  (quoter) @_name
  (#any-of? @_name "js" "julius")
  (quasiquote_body) @injection.content
  (#set! "language" "javascript"))

; TS: Text.TypeScript
(quasiquote
  (quoter) @_name
  (#any-of? @_name "tsc" "tscJSX")
  (quasiquote_body) @injection.content
  (#set! "language" "typescript"))

; -----------------------------------------------------------------------------
; HSX
(quasiquote
  (quoter) @_name
  (#eq? @_name "hsx")
  (quasiquote_body) @injection.content
  (#set! "language" "html"))

; -----------------------------------------------------------------------------
; Inline JSON from aeson
(quasiquote
  (quoter) @_name
  (#eq? @_name "aesonQQ")
  (quasiquote_body) @injection.content
  (#set! "language" "json"))
