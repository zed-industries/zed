; Fold markdown links to show only link text
(link
  (link_text) @fold.text
  (link_destination) @fold.url
) @fold.auto
  (#set! fold.display "text")
  (#set! fold.action "open_url:url")
  (#set! fold.proximity_expand true)

; Fold image links showing alt text
(image
  (image_description) @fold.text
  (link_destination) @fold.url
) @fold.auto
  (#set! fold.display "text")
  (#set! fold.action "open_url:url")
  (#set! fold.proximity_expand true)

; Fold reference links
(reference_link
  (link_text) @fold.text
  (link_label)? @fold.label
) @fold.auto
  (#set! fold.display "text")
  (#set! fold.proximity_expand true)

; Fold autolinks (bare URLs)
(uri_autolink) @fold.auto @fold.url
  (#set! fold.display "<link>")
  (#set! fold.action "open_url:url")
  (#set! fold.proximity_expand true)

; Fold email autolinks
(email_autolink) @fold.auto @fold.url
  (#set! fold.display "<email>")
  (#set! fold.action "open_url:url")
  (#set! fold.proximity_expand true)
