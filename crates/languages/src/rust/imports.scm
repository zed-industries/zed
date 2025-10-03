(use_declaration) @import

(scoped_use_list
    path: (_) @namespace
    list: (_) @list)

(scoped_identifier
    path: (_) @namespace
    name: (identifier) @name)

(use_list (identifier) @name)

(use_declaration (identifier) @name)

(use_as_clause
    path: (scoped_identifier
       path: (_) @namespace
       name: (_) @name)
    alias: (_) @alias)

(use_as_clause
    path: (identifier) @name
    alias: (_) @alias)

(use_wildcard
    (_)? @namespace
    "*" @wildcard)
