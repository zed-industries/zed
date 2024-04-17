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

(code_span) @text.literal

(fenced_code_block
  (info_string
    (language) @text.literal))

; hack to deal with incorrect grammar parsing
(atx_heading
  (block_quote_marker) @punctuation.block_quote_marker
)

; hack to deal with incorrect grammar parsing
(paragraph
  (block_quote_marker) @punctuation.block_quote_marker
)

; hack to deal with incorrect grammar parsing
(list_item
  (block_quote_marker) @punctuation.block_quote_marker
)

(block_quote
  (block_quote_marker) @punctuation.block_quote_marker
)

(block_quote
  (paragraph) @text.block_quote
)

(link_destination) @link_uri
(link_text) @link_text
