; Copied from https://github.com/nickel-lang/tree-sitter-nickel/
; Replaced `@injection.content` -> `@content`
; Replaced `injection.language` -> `"language"`

(annot_atom doc:
            (static_string) @content
            (#set! "language" "markdown"))
