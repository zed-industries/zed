(emphasis) @markup.emphasis
(strong_emphasis) @markup.emphasis.strong
(code_span) @markup.raw
(strikethrough) @markup.strikethrough

[
  (inline_link)
  (shortcut_link)
  (collapsed_reference_link)
  (full_reference_link)
  (image)
] @markup.link

(inline_link ["(" ")"] @markup.link.url)
(image ["(" ")"] @markup.link.url)
[
  (link_destination)
  (uri_autolink)
  (email_autolink)
] @markup.link.url
