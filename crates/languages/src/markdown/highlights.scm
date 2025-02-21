[
  (paragraph)
  (indented_code_block)
  (pipe_table)
] @text

[
  (atx_heading)
  (setext_heading)
  (thematic_break)
] @heading.markup
(setext_heading (paragraph) @heading.markup)

[
  (list_marker_plus)
  (list_marker_minus)
  (list_marker_star)
  (list_marker_dot)
  (list_marker_parenthesis)
  (block_quote_marker)
] @punctuation.markup

(pipe_table_header "|" @punctuation.markup)
(pipe_table_row "|" @punctuation.markup)
(pipe_table_delimiter_row "|" @punctuation.markup)
(pipe_table_delimiter_cell "-" @punctuation.markup)

(fenced_code_block
  (info_string
    (language) @punctuation.embedded.markup))
(fenced_code_block_delimiter) @punctuation.embedded.markup

(link_reference_definition) @link.markup
(link_destination) @link.uri.markup
