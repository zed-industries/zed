; Phoenix HTML template

((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @content)
 (#eq? @_sigil_name "H")
 (#set! language "heex"))
