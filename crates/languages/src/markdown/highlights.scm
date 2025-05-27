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

(setext_heading
  (paragraph) @markup.heading.1
  (setext_h1_underline) @markup.heading.1)

(setext_heading
  (paragraph) @markup.heading.2
  (setext_h2_underline) @markup.heading.2)

(atx_heading
  (atx_h1_marker)) @markup.heading.1

(atx_heading
  (atx_h2_marker)) @markup.heading.2

(atx_heading
  (atx_h3_marker)) @markup.heading.3

(atx_heading
  (atx_h4_marker)) @markup.heading.4

(atx_heading
  (atx_h5_marker)) @markup.heading.5

(atx_heading
  (atx_h6_marker)) @markup.heading.6

(fenced_code_block
  (info_string
    (language) @text.literal))
