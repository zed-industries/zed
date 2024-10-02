; Only produce one embedding for the entire file.
(document) @item

; Collapse arrays, except for the first object.
(array
  "[" @keep
  .
  (object)? @keep
  "]" @keep) @collapse

; Collapse string values (but not keys).
(pair value: (string
  "\"" @keep
  "\"" @keep) @collapse)
