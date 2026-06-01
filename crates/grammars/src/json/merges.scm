; Auto-Resolve structural merge rules for JSON.
;
; Object keys identify each pair; values may differ but a key is one item.

(object) @merge.set

(array) @merge.ordered_list

(pair key: (string (string_content) @merge.key))
