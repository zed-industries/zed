(import_statement) @import

(import_statement
    (import_clause) @list
    source: (string (string_fragment) @source))

; todo! the "import_clause" node does not have a field name, so it can't be
; negated to make this disjoint with the above match.
;
; (import_statement
;     source: (_) @namespace) @wildcard @import

(import_specifier
    name: (_) @name
    alias: (_)? @alias)

(import_clause (identifier) @name)
