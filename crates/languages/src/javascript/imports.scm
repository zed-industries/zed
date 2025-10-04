(import_statement) @import

(import_statement
    import_clause: (import_clause) @list
    source: (string (string_fragment) @source))

(import_statement
    !import_clause
    source: (string (string_fragment) @source)) @wildcard

(import_specifier
    name: (_) @name
    alias: (_)? @alias)

(import_clause (identifier) @name)
