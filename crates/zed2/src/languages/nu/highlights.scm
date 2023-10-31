;;; ---
;;; keywords
[
    "def"
    "def-env"
    "alias"
    "export-env"
    "export"
    "extern"
    "module"

    "let"
    "let-env"
    "mut"
    "const"

    "hide-env"

    "source"
    "source-env"

    "overlay"
    "register"

    "loop"
    "while"
    "error"

    "do"
    "if"
    "else"
    "try"
    "catch"
    "match"

    "break"
    "continue"
    "return"

] @keyword

(hide_mod "hide" @keyword)
(decl_use "use" @keyword)

(ctrl_for
    "for" @keyword
    "in" @keyword
)
(overlay_list "list" @keyword)
(overlay_hide "hide" @keyword)
(overlay_new "new" @keyword)
(overlay_use
    "use" @keyword
    "as" @keyword
)
(ctrl_error "make" @keyword)

;;; ---
;;; literals
(val_number) @constant
(val_duration
    unit: [
        "ns" "µs" "us" "ms" "sec" "min" "hr" "day" "wk"
    ] @variable
)
(val_filesize
    unit: [
        "b" "B"

        "kb" "kB" "Kb" "KB"
        "mb" "mB" "Mb" "MB"
        "gb" "gB" "Gb" "GB"
        "tb" "tB" "Tb" "TB"
        "pb" "pB" "Pb" "PB"
        "eb" "eB" "Eb" "EB"
        "zb" "zB" "Zb" "ZB"

        "kib" "kiB" "kIB" "kIb" "Kib" "KIb" "KIB"
        "mib" "miB" "mIB" "mIb" "Mib" "MIb" "MIB"
        "gib" "giB" "gIB" "gIb" "Gib" "GIb" "GIB"
        "tib" "tiB" "tIB" "tIb" "Tib" "TIb" "TIB"
        "pib" "piB" "pIB" "pIb" "Pib" "PIb" "PIB"
        "eib" "eiB" "eIB" "eIb" "Eib" "EIb" "EIB"
        "zib" "ziB" "zIB" "zIb" "Zib" "ZIb" "ZIB"
    ] @variable
)
(val_binary
    [
       "0b"
       "0o"
       "0x"
    ] @constant
    "[" @punctuation.bracket
    digit: [
        "," @punctuation.delimiter
        (hex_digit) @constant
    ]
    "]" @punctuation.bracket
) @constant
(val_bool) @constant.builtin
(val_nothing) @constant.builtin
(val_string) @string
(val_date) @constant
(inter_escape_sequence) @constant
(escape_sequence) @constant
(val_interpolated [
    "$\""
    "$\'"
    "\""
    "\'"
] @string)
(unescaped_interpolated_content) @string
(escaped_interpolated_content) @string
(expr_interpolated ["(" ")"] @variable)

;;; ---
;;; operators
(expr_binary [
    "+"
    "-"
    "*"
    "/"
    "mod"
    "//"
    "++"
    "**"
    "=="
    "!="
    "<"
    "<="
    ">"
    ">="
    "=~"
    "!~"
    "and"
    "or"
    "xor"
    "bit-or"
    "bit-xor"
    "bit-and"
    "bit-shl"
    "bit-shr"
    "in"
    "not-in"
    "starts-with"
    "ends-with"
] @operator)

(expr_binary opr: ([
    "and"
    "or"
    "xor"
    "bit-or"
    "bit-xor"
    "bit-and"
    "bit-shl"
    "bit-shr"
    "in"
    "not-in"
    "starts-with"
    "ends-with"
]) @keyword)

(where_command [
    "+"
    "-"
    "*"
    "/"
    "mod"
    "//"
    "++"
    "**"
    "=="
    "!="
    "<"
    "<="
    ">"
    ">="
    "=~"
    "!~"
    "and"
    "or"
    "xor"
    "bit-or"
    "bit-xor"
    "bit-and"
    "bit-shl"
    "bit-shr"
    "in"
    "not-in"
    "starts-with"
    "ends-with"
] @operator)

(assignment [
    "="
    "+="
    "-="
    "*="
    "/="
    "++="
] @operator)

(expr_unary ["not" "-"] @operator)

(val_range [
    ".."
    "..="
    "..<"
] @operator)

["=>" "=" "|"] @operator

[
    "o>"   "out>"
    "e>"   "err>"
    "e+o>" "err+out>"
    "o+e>" "out+err>"
] @special

;;; ---
;;; punctuation
[
    ","
    ";"
] @punctuation.delimiter

(param_short_flag "-" @punctuation.delimiter)
(param_long_flag ["--"] @punctuation.delimiter)
(long_flag ["--"] @punctuation.delimiter)
(param_rest "..." @punctuation.delimiter)
(param_type [":"] @punctuation.special)
(param_value ["="] @punctuation.special)
(param_cmd ["@"] @punctuation.special)
(param_opt ["?"] @punctuation.special)

[
    "(" ")"
    "{" "}"
    "[" "]"
] @punctuation.bracket

(val_record
  (record_entry ":" @punctuation.delimiter))
;;; ---
;;; identifiers
(param_rest
    name: (_) @variable)
(param_opt
    name: (_) @variable)
(parameter
    param_name: (_) @variable)
(param_cmd
    (cmd_identifier) @string)
(param_long_flag) @variable
(param_short_flag) @variable

(short_flag) @variable
(long_flag) @variable

(scope_pattern [(wild_card) @function])

(cmd_identifier) @function

(command
    "^" @punctuation.delimiter
    head: (_) @function
)

"where" @function

(path
  ["." "?"] @punctuation.delimiter
) @variable

(val_variable
  "$" @operator
  [
   (identifier) @variable
   "in" @type.builtin
   "nu" @type.builtin
   "env" @type.builtin
   "nothing" @type.builtin
   ]  ; If we have a special styling, use it here
)
;;; ---
;;; types
(flat_type) @type.builtin
(list_type
    "list" @type
    ["<" ">"] @punctuation.bracket
)
(collection_type
    ["record" "table"] @type
    "<" @punctuation.bracket
    key: (_) @variable
    ["," ":"] @punctuation.delimiter
    ">" @punctuation.bracket
)

(shebang) @comment
(comment) @comment
