; Auto-Resolve structural merge rules for Rust.
;
; @merge.set marks a node whose direct children form an unordered set: when
; both sides of a merge conflict add disjoint children to the same node, the
; additions can be combined automatically.
;
; @merge.key is an optional sub-capture inside an item: its text is used as
; the item's identity. Two items with the same key but different text are
; treated as "the same item, modified on both sides" — handled if exactly one
; side modified it, deferred to manual resolution otherwise.

(source_file) @merge.set

(declaration_list) @merge.set

(field_declaration_list) @merge.set

(enum_variant_list) @merge.set

; Match arms have meaningful positions (pattern matching has fall-through
; semantics in some cases, so order matters).
(match_block) @merge.ordered_list

(function_item name: (identifier) @merge.key)

(function_signature_item name: (identifier) @merge.key)

(struct_item name: (type_identifier) @merge.key)

(enum_item name: (type_identifier) @merge.key)

(trait_item name: (type_identifier) @merge.key)

(const_item name: (identifier) @merge.key)

(static_item name: (identifier) @merge.key)

(mod_item name: (identifier) @merge.key)

(type_item name: (type_identifier) @merge.key)

(union_item name: (type_identifier) @merge.key)

(field_declaration name: (field_identifier) @merge.key)

(enum_variant name: (identifier) @merge.key)
