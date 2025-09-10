; JSONL (JSON Lines) embedding configuration  
; This file is derived from crates/languages/src/json/embedding.scm
; and should be kept in sync with the JSON embedding rules

; Produce one embedding for the entire document, with selective collapsing
; to reduce noise while preserving structure for semantic search
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
