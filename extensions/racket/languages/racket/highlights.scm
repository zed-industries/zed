["(" ")" "[" "]" "{" "}"] @punctuation.bracket

[(string)
 (here_string)
 (byte_string)] @string
(regex) @string.regex
(escape_sequence) @string.escape

[(comment)
 (block_comment)
 (sexp_comment)] @comment

(symbol) @variable

(number) @number
(character) @constant.builtin
(boolean) @constant.builtin
(keyword) @constant
(quote . (symbol)) @constant

(extension) @keyword
(lang_name) @variable.special

((symbol) @operator
 (#match? @operator "^(\\+|-|\\*|/|=|>|<|>=|<=)$"))

(list
  .
  (symbol) @function)

(list
  .
  (symbol) @keyword
  (#match? @keyword
   "^(unit-from-context|for/last|syntax-case|match-let\\*-values|define-for-syntax|define/subexpression-pos-prop|set-field!|class-field-accessor|invoke-unit|#%stratified-body|for\\*/and|for\\*/weak-set|flat-rec-contract|for\\*/stream|planet|for/mutable-seteqv|log-error|delay|#%declare|prop:dict/contract|->d|lib|override\\*|define-local-member-name|send-generic|for\\*/hasheq|define-syntax|submod|except|include-at/relative-to/reader|public\\*|define-member-name|define/public|let\\*|for/and|for\\*/first|for|delay/strict|define-values-for-export|==|match-define-values|for/weak-seteq|for\\*/async|for/stream|for/weak-seteqv|set!-values|lambda|for\\*/product|augment-final\\*|pubment\\*|command-line|contract|case|struct-field-index|contract-struct|unless|for/hasheq|for/seteqv|with-method|define-values-for-syntax|for-template|pubment|for\\*/list|syntax-case\\*|init-field|define-serializable-class|=>|for/foldr/derived|letrec-syntaxes|overment\\*|unquote-splicing|_|inherit-field|for\\*|stream-lazy|match-lambda\\*|contract-pos/neg-doubling|unit/c|match-define|for\\*/set|unit/s|nor|#%expression|class/c|this%|place/context|super-make-object|when|set!|parametric->/c|syntax-id-rules|include/reader|compound-unit|override-final|get-field|gen:dict|for\\*/seteqv|for\\*/hash|#%provide|combine-out|link|with-contract-continuation-mark|define-struct/derived|stream\\*|λ|rename-out|define-serializable-class\\*|augment|define/augment|let|define-signature-form|letrec-syntax|abstract|define-namespace-anchor|#%module-begin|#%top-interaction|for\\*/weak-seteqv|do|define/subexpression-pos-prop/name|absent|send/apply|with-handlers\\*|all-from-out|provide-signature-elements|gen:stream|define/override-final|for\\*/mutable-seteqv|rename|quasisyntax/loc|instantiate|for/list|extends|include-at/relative-to|mixin|define/pubment|#%plain-lambda|except-out|#%plain-module-begin|init|for\\*/last|relative-in|define-unit/new-import-export|->dm|member-name-key|nand|interface\\*|struct|define/override|else|define/augment-final|failure-cont|open|log-info|define/final-prop|all-defined-out|for/sum|for\\*/sum|recursive-contract|define|define-logger|match\\*|log-debug|rename-inner|->|struct/derived|unit|class\\*|prefix-out|any|define/overment|define-signature|match-letrec-values|let-syntaxes|for/mutable-set|define/match|cond|super-instantiate|define-contract-struct|import|hash/dc|define-custom-set-types|public-final|for/vector|for-label|prefix-in|for\\*/foldr/derived|define-unit-binding|object-contract|syntax-rules|augride|for\\*/mutable-seteq|quasisyntax|inner|for-syntax|overment|send/keyword-apply|generic|let\\*-values|->m|define-values|struct-copy|init-depend|struct/ctc|match-lambda|#%printing-module-begin|match\\*/derived|case->m|this|file|stream-cons|inspect|field|for/weak-set|struct\\*|gen:custom-write|thunk\\*|combine-in|unquote|for/lists|define/private|for\\*/foldr|define-unit/s|with-continuation-mark|begin|prefix|quote-syntax/prune|object/c|interface|match/derived|for/hasheqv|current-contract-region|define-compound-unit|override|define/public-final|recontract-out|let/cc|augride\\*|inherit|send|define-values/invoke-unit|for/mutable-seteq|#%datum|for/first|match-let\\*|invoke-unit/infer|define/contract|syntax/loc|for\\*/hasheqv|define-sequence-syntax|let/ec|for/product|for\\*/fold/derived|define-syntax-rule|lazy|unconstrained-domain->|augment-final|private|class|define-splicing-for-clause-syntax|for\\*/fold|prompt-tag/c|contract-out|match/values|public-final\\*|case-lambda|for/fold|unsyntax|for/set|begin0|#%require|time|public|define-struct|include|define-values/invoke-unit/infer|only-space-in|struct/c|only-meta-in|unit/new-import-export|place|begin-for-syntax|shared|inherit/super|quote|for/or|struct/contract|export|inherit/inner|struct-out|let-syntax|augment\\*|for\\*/vector|rename-in|match-let|define-unit|:do-in|~@|for\\*/weak-seteq|private\\*|and|except-in|log-fatal|gen:equal\\+hash|provide|require|thunk|invariant-assertion|define-match-expander|init-rest|->\\*|class/derived|super-new|for/fold/derived|for\\*/mutable-set|match-lambda\\*\\*|only|with-contract|~\\?|opt/c|let-values|delay/thread|->i|for/foldr|for-meta|only-in|send\\+|\\.\\.\\.|struct-guard/c|->\\*m|gen:set|struct/dc|define-syntaxes|if|parameterize|module\\*|module|send\\*|#%variable-reference|compound-unit/infer|#%plain-app|for/hash|contracted|case->|match|for\\*/lists|#%app|letrec-values|log-warning|super|define/augride|local-require|provide/contract|define-struct/contract|match-let-values|quote-syntax|for\\*/seteq|define-compound-unit/infer|parameterize\\*|values/drop|for/seteq|tag|stream|delay/idle|module\\+|define-custom-hash-types|cons/dc|define-module-boundary-contract|or|protect-out|define-opt/c|implies|letrec-syntaxes\\+values|for\\*/or|unsyntax-splicing|override-final\\*|for/async|parameterize-break|syntax|place\\*|for-space|quasiquote|with-handlers|delay/sync|define-unit-from-context|match-letrec|#%top|define-unit/contract|delay/name|new|field-bound\\?|letrec|class-field-mutator|with-syntax|flat-murec-contract|rename-super|local)$"
   ))

((symbol) @comment
 (#match? @comment "^#[cC][iIsS]$"))
