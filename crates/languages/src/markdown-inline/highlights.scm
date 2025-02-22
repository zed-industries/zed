(emphasis) @emphasis.markup
(strong_emphasis) @emphasis.strong.markup
(code_span) @text.literal.markup ; @raw.markup
(strikethrough) @strikethrough.markup

[
  (inline_link)
  (shortcut_link)
  (collapsed_reference_link)
  (full_reference_link)
  (image)
] @link.markup

(link_text) @link_text.markup ; @link.markup
(link_label) @link_text.markup ; @link.label.markup

(inline_link ["(" ")"] @link_uri.markup) ; @link.uri.markup
(image ["(" ")"] @link_uri.markup) ; @link.uri.markup
[
  (link_destination)
  (uri_autolink)
  (email_autolink)
] @link_uri.markup ; @link.uri.markup
