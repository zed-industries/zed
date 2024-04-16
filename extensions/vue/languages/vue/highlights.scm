(attribute) @property
(directive_attribute) @property
(quoted_attribute_value) @string
(interpolation) @punctuation.special
(raw_text) @embedded

((tag_name) @type
 (#match? @type "^[A-Z]"))

((directive_name) @keyword
 (#match? @keyword "^v-"))

(start_tag) @tag
(end_tag) @tag
(self_closing_tag) @tag
