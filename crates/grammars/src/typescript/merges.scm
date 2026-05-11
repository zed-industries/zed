; Auto-Resolve structural merge rules for TypeScript / TSX.
;
; @merge.set marks a node whose direct children form an unordered set: when
; both sides of a merge conflict add disjoint children to the same node, the
; additions can be combined automatically.
;
; @merge.key is an optional sub-capture inside an item: its text is used as
; the item's identity. Two items with the same key but different text are
; treated as "the same item, modified on both sides" — handled if exactly one
; side modified it, deferred to manual resolution otherwise.

(program) @merge.set

(class_body) @merge.set

(interface_body) @merge.set

(object_type) @merge.set

(function_declaration name: (identifier) @merge.key)

(function_signature name: (identifier) @merge.key)

(class_declaration name: (type_identifier) @merge.key)

(interface_declaration name: (type_identifier) @merge.key)

(enum_declaration name: (identifier) @merge.key)

(type_alias_declaration name: (type_identifier) @merge.key)

(method_definition name: (property_identifier) @merge.key)

(method_signature name: (property_identifier) @merge.key)

(public_field_definition name: (property_identifier) @merge.key)

(property_signature name: (property_identifier) @merge.key)
