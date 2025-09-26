(emphasis) @emphasis.markup
(strong_emphasis) @emphasis.strong.markup
(code_span) @raw.markup
(strikethrough) @strikethrough.markup

[
  (inline_link)
  (shortcut_link)
  (collapsed_reference_link)
  (full_reference_link)
  (image)
  (link_text)
] @link.markup

(link_label) @link.label.markup

(inline_link ["(" ")"] @link.uri.markup)
(image ["(" ")"] @link.uri.markup)
[
  (link_destination)
  (uri_autolink)
  (email_autolink)
] @link.uri.markup
