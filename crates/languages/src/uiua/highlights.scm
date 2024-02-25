[
  (openParen)
  (closeParen)
  (openCurly)
  (closeCurly)
  (openBracket)
  (closeBracket)
] @punctuation.bracket

[
  (branchSeparator)
  (underscore)
] @constructor
; ] @punctuation.delimiter

[ (character) ] @constant.character
[ (comment) ] @comment
[ (constant) ] @constant.numeric
[ (identifier) ] @variable
[ (leftArrow) ] @keyword
[ (function) ] @function
[ (modifier1) ] @operator
[ (modifier2) ] @operator
[ (number) ] @constant.numeric
[ (placeHolder) ] @special
[ (otherConstant) ] @string.special
[ (signature) ] @type
[ (system) ] @function.builtin
[ (tripleMinus) ] @module

; planet
[
  "id"
  "identity"
  "∘"
  "dip"
  "⊙"
  "gap"
  "⋅"
] @tag

[
  (string)
  (multiLineString)
] @string

; [
;   (deprecated)
;   (identifierDeprecated)
; ] @warning
