(line_comment) @comment.inside
(line_comment)+ @comment.around
(block_comment) @comment.inside
(block_comment)+ @comment.around

((type_annotation)?
  (value_declaration
    (function_declaration_left (lower_case_identifier))
    (eq)
    (_) @function.inside
  )
) @function.around

(parenthesized_expr
  (anonymous_function_expr
    (
      (arrow)
      (_) @function.inside
    )
  )
) @function.around

(value_declaration
  (function_declaration_left
    (lower_pattern
      (lower_case_identifier) @parameter.inside @parameter.around
    )
  )
)

(value_declaration
  (function_declaration_left
    (pattern) @parameter.inside @parameter.around
  )
)

(value_declaration
  (function_declaration_left
    (tuple_pattern
      (pattern) @parameter.inside
    ) @parameter.around
  )
)

(value_declaration
  (function_declaration_left
    (record_pattern
      (lower_pattern
        (lower_case_identifier) @parameter.inside
      )
    ) @parameter.around
  )
)

(parenthesized_expr
  (anonymous_function_expr
    (
      (backslash)
      (pattern) @parameter.inside
      (arrow)
    )
  )
)
