(preproc_include) @import

(preproc_include
    path: (system_lib_string) @source @wildcard)

(preproc_include
    path: (string_literal (string_content) @source) @wildcard)
