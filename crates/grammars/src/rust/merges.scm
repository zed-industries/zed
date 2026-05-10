; Auto-Resolve structural merge rules for Rust.
;
; @merge.set marks a node whose direct children form an unordered set: when
; both sides of a merge conflict add disjoint children to the same node, the
; additions can be combined automatically.

(source_file) @merge.set

(declaration_list) @merge.set

(field_declaration_list) @merge.set

(enum_variant_list) @merge.set
