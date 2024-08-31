; HCL Outline Scheme
; Comments
(comment) @annotation

; Block with and without string_lit
; Example:
;   terraform { ... }
;   module "vpc" { ... }
;   resource "resource" "name" { ... }
(config_file
    (body
        (block
            (identifier) @context
            (string_lit)? @name
            (string_lit)? @name
        ) @item
    )
)

; Inside block with identifier
(config_file
    (body
        (block
            (identifier)
            (body
                (attribute
                    (identifier) @context
                ) @item
            )
        )
    )
)

; Inside block with identifier and string_lit
(config_file
  (body
    (block
      (identifier)
      (body
        (block
            (identifier) @context
            (string_lit)? @name
        ) @item
      )
    )
  )
)

; Root Attribute block
; Example:
; inputs = { ... }
(config_file
  (body
    (attribute
      (identifier) @context
    ) @item
  )
)

; Inside Root Attribute block
(config_file
  (body
    (attribute
      (identifier)
        (expression
          (collection_value
            (object
              (object_elem
                key: (expression (variable_expr (identifier) @context))
              ) @item
            )
          )
        )
    )
  )
)
