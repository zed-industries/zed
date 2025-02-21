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
] @link.markup

(inline_link ["(" ")"] @markup.link.url)
(image ["(" ")"] @markup.link.url)
[
  (link_destination)
  (uri_autolink)
  (email_autolink)
] @markup.link.url
