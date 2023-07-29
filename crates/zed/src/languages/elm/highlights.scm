[
    "if"
    "then"
    "else"
    "let"
    "in"
    (case)
    (of)
    (backslash)
    (as)
    (port)
    (exposing)
    (alias)
    (import)
    (module)
    (type)
    (arrow)
 ] @keyword

[
    (eq)
    (operator_identifier)
    (colon)
] @operator

(type_annotation(lower_case_identifier) @function)
(port_annotation(lower_case_identifier) @function)
(function_declaration_left(lower_case_identifier) @function.definition)

(function_call_expr
    target: (value_expr
        name: (value_qid (lower_case_identifier) @function)))

(exposed_value(lower_case_identifier) @function)
(exposed_type(upper_case_identifier) @type)

(field_access_expr(value_expr(value_qid)) @identifier)
(lower_pattern) @variable
(record_base_identifier) @identifier

[
    "("
    ")"
] @punctuation.bracket

[
    "|"
    ","
] @punctuation.delimiter

(number_constant_expr) @constant

(type_declaration(upper_case_identifier) @type)
(type_ref) @type
(type_alias_declaration name: (upper_case_identifier) @type)

(value_expr(upper_case_qid(upper_case_identifier)) @type)

[
    (line_comment)
    (block_comment)
] @comment

(string_escape) @string.escape

[
    (open_quote)
    (close_quote)
    (regular_string_part)
    (open_char)
    (close_char)
] @string
