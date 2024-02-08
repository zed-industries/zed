; inherits: html
"---" @punctuation.delimiter

[
  "{"
  "}"
] @punctuation.special

((start_tag
  (tag_name) @type))

((end_tag
  (tag_name) @type))

((erroneous_end_tag
  (erroneous_end_tag_name) @type))
