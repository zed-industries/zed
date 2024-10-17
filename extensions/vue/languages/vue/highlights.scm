(tag_name) @tag
(attribute) @attribute
(directive_attribute) @attribute
(quoted_attribute_value) @string
(interpolation) @punctuation.special
(raw_text) @embedded

((tag_name) @type
 (#match? @type "^[A-Z]"))

(directive_name) @keyword
(directive_argument) @constant

(start_tag) @punctuation.bracket
(end_tag) @punctuation.bracket
(self_closing_tag) @punctuation.bracket
