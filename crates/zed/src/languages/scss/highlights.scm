(comment) @comment
(single_line_comment) @comment

(tag_name) @tag
(nesting_selector) @tag
(universal_selector) @tag

"~" @operator
">" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"=" @operator
"^=" @operator
"|=" @operator
"~=" @operator
"$=" @operator
"*=" @operator

"and" @operator
"or" @operator
"not" @operator
"only" @operator

(attribute_selector (plain_value) @string)
(pseudo_element_selector (tag_name) @attribute)
(pseudo_class_selector (class_name) @attribute)

(class_name) @property
(id_name) @property
(namespace_name) @property
(property_name) @property
(feature_name) @property

(attribute_name) @attribute

(function_name) @function

((property_name) @variable
 (match? @variable "^--"))
((plain_value) @variable
 (match? @variable "^--"))

; At-Rules

; SCSS-Specific

"@use" @keyword
"@forward" @keyword
"@import" @keyword
"@mixin" @keyword
"@include" @keyword
"@function" @keyword
"@extend" @keyword
"@error" @keyword
"@warn" @keyword
"@debug" @keyword
"@at-root" @keyword

; Flow Control

"@if" @keyword
"@each" @keyword
"@for" @keyword
"@while" @keyword

; CSS

"@media" @keyword
"@import" @keyword
"@charset" @keyword
"@namespace" @keyword
"@supports" @keyword
"@keyframes" @keyword

; Other Common

"@mixin" @keyword
"@include" @keyword

(at_keyword) @keyword
(to) @keyword
(from) @keyword
(important) @keyword

(string_value) @string
(color_value) @string.special

(integer_value) @number
(float_value) @number
(unit) @type

; Mixins

(mixin_statement (name) @function)

; Includes

(include_statement
    ("@include" @keyword
    (identifier) @function
    (arguments
        "(" @punctuation
            (argument
                (argument_value) @variable
            )
        ")" @punctuation
        )
    )
)

"#" @punctuation.delimiter
"," @punctuation.delimiter
