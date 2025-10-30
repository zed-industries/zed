(import_statement
    import_clause: (import_clause
        [
            (identifier) @name
            (named_imports
                (import_specifier
                    name: (_) @name
                    alias: (_)? @alias))
            (namespace_import) @wildcard
        ])
    source: (string (string_fragment) @source)) @import

(import_statement
    !source
    import_clause: (import_require_clause
        source: (string (string_fragment) @source))) @wildcard @import

(import_statement
    !import_clause
    source: (string (string_fragment) @source)) @wildcard @import
