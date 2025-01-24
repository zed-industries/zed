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
(arguments) @property
(call_expression (identifier) @function)
(enumeral) @constant
(identifier) @variable
(string) @string
(column_declaration (identifier) (column_type (identifier) @type))
(attribute (identifier) @tag)
(attribute (call_expression (identifier) @tag))
(attribute (call_expression (member_expression (identifier) @tag)))
(type_expression (identifier) @type)

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"=" @operator
"@" @operator
