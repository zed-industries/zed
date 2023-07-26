(type_declaration
    (type) @context
    (upper_case_identifier) @name) @item

(type_alias_declaration
    (type) @context
    (alias) @context
    name: (upper_case_identifier) @name) @item

(type_alias_declaration
    typeExpression:
        (type_expression
            part: (record_type
                (field_type
                    name: (lower_case_identifier) @name) @item)))

(union_variant
    name: (upper_case_identifier) @name) @item

(value_declaration
    functionDeclarationLeft:
        (function_declaration_left(lower_case_identifier) @name)) @item
