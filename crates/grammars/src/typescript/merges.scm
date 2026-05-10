; Auto-Resolve structural merge rules for TypeScript / TSX.
;
; @merge.set marks a node whose direct children form an unordered set: when
; both sides of a merge conflict add disjoint children to the same node, the
; additions can be combined automatically.

(program) @merge.set

(class_body) @merge.set

(interface_body) @merge.set

(object_type) @merge.set
