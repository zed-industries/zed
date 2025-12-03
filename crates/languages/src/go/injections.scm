; Refer to https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/go/injections.scm#L4C1-L16C41
((comment) @injection.content
 (#set! injection.language "comment")
)

(call_expression
  (selector_expression) @_function
  (#any-of? @_function
    "regexp.Match" "regexp.MatchReader" "regexp.MatchString" "regexp.Compile" "regexp.CompilePOSIX"
    "regexp.MustCompile" "regexp.MustCompilePOSIX")
  (argument_list
    .
    [
      (raw_string_literal)
      (interpreted_string_literal)
    ] @injection.content
    (#set! injection.language "regex")
    ))

; INJECT SQL
(
    [
        (const_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (var_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (assignment_statement
        left: (expression_list)
        "="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (short_var_declaration
        left: (expression_list)
        ":="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (composite_literal
            body: (literal_value
            (keyed_element
            (comment) @_comment
            value: (literal_element
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))))

        (expression_statement
            (call_expression
            (argument_list
            (comment) @_comment
            [
               	(interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        )))
    ]
  (#match? @_comment "^\\/\\*\\s*sql\\s*\\*\\/$")
  (#set! injection.language "sql")
)

; INJECT JSON
(
    [
        (const_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (var_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (assignment_statement
        left: (expression_list)
        "="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (short_var_declaration
        left: (expression_list)
        ":="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (composite_literal
            body: (literal_value
            (keyed_element
            (comment) @_comment
            value: (literal_element
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))))

        (expression_statement
            (call_expression
            (argument_list
            (comment) @_comment
            [
               	(interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        )))
    ]
    (#match? @_comment "^\\/\\*\\s*json\\s*\\*\\/") ; /* json */ or /*json*/
    (#set! injection.language "json")
)

; INJECT YAML
(
    [
        (const_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (var_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (assignment_statement
        left: (expression_list)
        "="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (short_var_declaration
        left: (expression_list)
        ":="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (composite_literal
            body: (literal_value
            (keyed_element
            (comment) @_comment
            value: (literal_element
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))))

        (expression_statement
            (call_expression
            (argument_list
            (comment) @_comment
            [
               	(interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        )))
    ]
    (#match? @_comment "^\\/\\*\\s*yaml\\s*\\*\\/") ; /* yaml */ or /*yaml*/
    (#set! injection.language "yaml")
)

; INJECT XML
(
    [
        (const_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (var_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (assignment_statement
        left: (expression_list)
        "="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (short_var_declaration
        left: (expression_list)
        ":="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (composite_literal
            body: (literal_value
            (keyed_element
            (comment) @_comment
            value: (literal_element
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))))

        (expression_statement
            (call_expression
            (argument_list
            (comment) @_comment
            [
               	(interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        )))
    ]
    (#match? @_comment "^\\/\\*\\s*xml\\s*\\*\\/") ; /* xml */ or /*xml*/
    (#set! injection.language "xml")
)

; INJECT HTML
(
    [
        (const_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (var_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (assignment_statement
        left: (expression_list)
        "="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (short_var_declaration
        left: (expression_list)
        ":="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (composite_literal
            body: (literal_value
            (keyed_element
            (comment) @_comment
            value: (literal_element
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))))

        (expression_statement
            (call_expression
            (argument_list
            (comment) @_comment
            [
               	(interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        )))
    ]
    (#match? @_comment "^\\/\\*\\s*html\\s*\\*\\/") ; /* html */ or /*html*/
    (#set! injection.language "html")
)

; INJECT JS
(
    [
        (const_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (var_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (assignment_statement
        left: (expression_list)
        "="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (short_var_declaration
        left: (expression_list)
        ":="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (composite_literal
            body: (literal_value
            (keyed_element
            (comment) @_comment
            value: (literal_element
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))))

        (expression_statement
            (call_expression
            (argument_list
            (comment) @_comment
            [
               	(interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        )))
    ]
    (#match? @_comment "^\\/\\*\\s*js\\s*\\*\\/") ; /* js */ or /*js*/
    (#set! injection.language "javascript")
)


; INJECT CSS
(
    [
        (const_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (var_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (assignment_statement
        left: (expression_list)
        "="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (short_var_declaration
        left: (expression_list)
        ":="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (composite_literal
            body: (literal_value
            (keyed_element
            (comment) @_comment
            value: (literal_element
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))))

        (expression_statement
            (call_expression
            (argument_list
            (comment) @_comment
            [
               	(interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        )))
    ]
    (#match? @_comment "^\\/\\*\\s*css\\s*\\*\\/") ; /* css */ or /*css*/
    (#set! injection.language "css")
)


; INJECT LUA
(
    [
        (const_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (var_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (assignment_statement
        left: (expression_list)
        "="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (short_var_declaration
        left: (expression_list)
        ":="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (composite_literal
            body: (literal_value
            (keyed_element
            (comment) @_comment
            value: (literal_element
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))))

        (expression_statement
            (call_expression
            (argument_list
            (comment) @_comment
            [
               	(interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        )))
    ]
    (#match? @_comment "^\\/\\*\\s*lua\\s*\\*\\/") ; /* lua */ or /*lua*/
    (#set! injection.language "lua")
)

; INJECT BASH
(
    [
        (const_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (var_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (assignment_statement
        left: (expression_list)
        "="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (short_var_declaration
        left: (expression_list)
        ":="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (composite_literal
            body: (literal_value
            (keyed_element
            (comment) @_comment
            value: (literal_element
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))))

        (expression_statement
            (call_expression
            (argument_list
            (comment) @_comment
            [
               	(interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        )))
    ]
    (#match? @_comment "^\\/\\*\\s*bash\\s*\\*\\/") ; /* bash */ or /*bash*/
    (#set! injection.language "bash")
)

; INJECT CSV
(
    [
        (const_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (var_spec
            name: (identifier)
            "="
            (comment) @_comment
            value: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (assignment_statement
        left: (expression_list)
        "="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (short_var_declaration
        left: (expression_list)
        ":="
        (comment) @_comment
        right: (expression_list
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        ((comment) @_comment
            value: (literal_element
            [
                (interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        ))

        (argument_list
            (comment) @_comment
            [
               	(interpreted_string_literal (interpreted_string_literal_content) @injection.content)
                (raw_string_literal (raw_string_literal_content) @injection.content)
            ]
        )
    ]
    (#match? @_comment "^\\/\\*\\s*csv\\s*\\*\\/") ; /* csv */ or /*csv */
    (#set! injection.language "csv")
)
