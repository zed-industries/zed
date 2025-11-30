((comment) @injection.content
 (#set! injection.language "comment")
)

; SQL -----------------------------------------------------------------------------
; common functions ex: spark.sql("SELECT * FROM tbl")
(call
    [
        (attribute attribute: (identifier) @function_name)
        (identifier) @function_name
    ]
    (#match? @function_name "(?i:sql|read_sql|read_sql_query|execute)")
    arguments: (argument_list
        (string
            (string_content) @injection.content (#set! injection.language "sql")
        )
    )
)

; sqlalchemy ex: from sqlalchemy import text; text("SELECT * FROM tbl")
(call
    function: (identifier) @function_name
    (#match? @function_name "(?i:text)")
    arguments: (argument_list
        (string
            (string_content) @injection.content (#set! injection.language "sql")
        )
    )
)

; string variables
((comment) @comment
    .
    (expression_statement
        (assignment
            right: (string
                (string_content) @injection.content
            )
        )
    )
    (#match? @comment "^(#|#\\s+)(?i:sql)\\s*$")
    (#set! injection.language "sql")
)
