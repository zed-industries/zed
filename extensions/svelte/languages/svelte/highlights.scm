; ; Special identifiers
; ;--------------------

; comments
(comment) @comment

; property attribute
; TODO: we need to break this down in the source repository to distinguish between
; - directives, eg: foo:bar="baz" and foo="baz"
; - possibly --props
(attribute_name) @attribute

; Style component attributes as @property
(start_tag
    (
        (tag_name) @_tag_name
        (#match? @_tag_name "^[A-Z]")
    )
    (attribute
        (attribute_name) @property
    )
)

(self_closing_tag
    (
        (tag_name) @_tag_name
        (#match? @_tag_name "^[A-Z]")
    )
    (attribute
        (attribute_name) @property
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
