
; comments
(comment) @comment

; property attribute
(attribute_directive) @attribute.function
(attribute_identifier) @attribute
(attribute_modifier) @attribute.special

; Style component attributes as @property
(start_tag
    (
        (tag_name) @_tag_name
        (#match? @_tag_name "^[A-Z]")
    )
    (attribute
        (attribute_name
            (attribute_identifier) @tag.property
        )
    )
)

(self_closing_tag
    (
        (tag_name) @_tag_name
        (#match? @_tag_name "^[A-Z]")
    )
    (attribute
        (attribute_name
            (attribute_identifier) @tag.property
        )
    )
)


; style elements starting with lowercase letters as tags
(
    (tag_name) @tag
    (#match? @tag "^[a-z]")
)

; style elements starting with uppercase letters as components (types)
; Also valid might be to treat them as constructors
(
    (tag_name) @tag @tag.component.type.constructor
    (#match? @tag "^[A-Z]")
)

[
  "<"
  ">"
  "</"
  "/>"
] @tag.punctuation.bracket


[
  "{"
  "}"
] @punctuation.bracket

[
    "|"
] @punctuation.delimiter


[
  "@"
  "#"
  ":"
  "/"
] @tag.punctuation.special

"=" @operator


; Treating (if, each, ...) as a keyword inside of blocks
; like {#if ...} or {#each ...}
(block_start_tag
    tag: _ @tag.keyword
)

(block_tag
    tag: _ @tag.keyword
)

(block_end_tag
    tag: _ @tag.keyword
)

(expression_tag
    tag: _ @tag.keyword
)

; Style quoted string attribute values
(quoted_attribute_value) @string


; Highlight the `as` keyword in each blocks
(each_start
    ("as") @tag.keyword
)


; Highlight the snippet name as a function
; (e.g. {#snippet foo(bar)}
(snippet_name) @function
