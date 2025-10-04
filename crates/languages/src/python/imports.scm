(import_statement) @import

(import_from_statement) @import

(dotted_name
    (identifier)* @namespace
    (identifier) @name .)

(import_from_statement
    module_name: (_) @namespace
    name: (_)+ @name
    (wildcard_import)? @wildcard)

(aliased_import
    name: (_) @name
    alias: (_) @alias)
