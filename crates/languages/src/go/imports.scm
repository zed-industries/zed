(import_spec
    name: [
        (dot)
        (package_identifier)
    ]
    path: (interpreted_string_literal
        (interpreted_string_literal_content) @namespace)
) @wildcard @import

(import_spec
    !name
    path: (interpreted_string_literal
        (interpreted_string_literal_content) @namespace)
) @wildcard @import
