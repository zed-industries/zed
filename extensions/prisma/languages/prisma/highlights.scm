[
 "datasource"
 "enum"
 "generator"
 "model"
 "view"
] @keyword

(comment) @comment
(developer_comment) @comment

(number) @number
(string) @string
(false) @boolean
(true) @boolean
(arguments) @property
(maybe) @punctuation
(call_expression (identifier) @function)
(enumeral) @constant
(identifier) @variable
(column_declaration (identifier) (column_type (identifier) @type))
(attribute (identifier) @label)
(attribute (call_expression (identifier) @label))
(attribute (call_expression (member_expression (identifier) @label)))
(block_attribute_declaration (identifier) @label)
(block_attribute_declaration (call_expression (identifier) @label))
(type_expression (identifier) @property)

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"=" @operator
"@" @operator
"@@" @operator
