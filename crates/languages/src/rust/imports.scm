; (use_declaration) @import_statement

; (scoped_use_list
;     path: (_) @import_prefix
;     list: (_) @prefixed_contents)

; (scoped_identifier
;     path: (_) @import_prefix
;     name: (_) @prefixed_contents)

; (use_list (identifier) @import)

; (use_declaration argument:(identifier) @import)

(use_declaration) @import_statement

(scoped_use_list
    path: (_) @path
    list: (_) @list)

(scoped_identifier
    path: (_) @path
    name: (_) @name)

(use_list (identifier) @name)

(use_declaration (identifier) @name)

(use_declaration (use_list) @list)
