(import_statement
    name: [
        (dotted_name
            ((identifier) @namespace ".")*
            (identifier) @namespace .)
        (aliased_import
            name: (dotted_name
                ((identifier) @namespace ".")*
                (identifier) @namespace .))
    ]) @wildcard @import

(import_from_statement
    module_name: [
        (dotted_name
            ((identifier) @namespace ".")*
            (identifier) @namespace .)
        (relative_import
            (dotted_name
                ((identifier) @namespace ".")*
                (identifier) @namespace .)?)
    ]
    (wildcard_import)? @wildcard
    name: [
        (dotted_name
            ((identifier) @namespace ".")*
            (identifier) @name .)
        (aliased_import
            name: (dotted_name
                ((identifier) @namespace ".")*
                (identifier) @name .)
            alias: (identifier) @alias)
    ]?) @import
