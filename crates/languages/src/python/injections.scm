((comment) @injection.content
 (#set! injection.language "comment")
)

; SQL -----------------------------------------------------------------------------
(
    [
        ; function calls
        (call
            [
                (attribute attribute: (identifier) @function_name)
                (identifier) @function_name
            ]
            arguments: (argument_list
                (comment) @comment
                (string
                    (string_content) @injection.content
                )
        ))

        ; string variables
        ((comment) @comment
            .
            (expression_statement
                (assignment
                    right: (string
                        (string_content) @injection.content
                    )
                )
        ))
    ]
    (#match? @comment "^(#|#\\s+)(?i:sql)\\s*$")
    (#set! injection.language "sql")
)
