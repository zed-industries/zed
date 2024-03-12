(emphasis) @emphasis
(strong_emphasis) @emphasis.strong

[
  (atx_heading)
  (setext_heading)
] @title

[
  (list_marker_plus)
  (list_marker_minus)
  (list_marker_star)
  (list_marker_dot)
  (list_marker_parenthesis)
] @punctuation.list_marker

[
  (emphasis_delimiter)
  (code_span_delimiter)
  (fenced_code_block_delimiter)
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

(code_span) @text.literal

(fenced_code_block
  (info_string
    (language) @text.literal))

(image_description) @link_text

(link_destination) @link_uri
(link_text) @link_text
