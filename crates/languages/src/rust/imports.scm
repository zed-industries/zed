(use_declaration) @import_statement

(scoped_use_list
    path: (_) @path
    list: (_) @list)

(scoped_identifier
    path: (_) @path
    name: (_) @name)

(use_list (identifier) @name)

(use_declaration (identifier) @name)

(use_wildcard
    (_)? @path
    "*" @wildcard)
