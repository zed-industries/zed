(expr_body) @local.scope


(value_declaration(decl_left 
  (identifier_pattern 
   (identifier)@local.definition)))

;;This would make parameters that are functions not get highlighted as functions but as parameters.. not sure if that's something we ever want 
; (argument_patterns(identifier_pattern
;             (identifier)@local.definition))

; (argument_patterns(long_identifier)@local.definition)

(exposes_list(ident)@local.reference)

(opaque_type_def(apply_type(concrete_type)@local.definition))
(alias_type_def(apply_type(concrete_type)@local.definition))

(exposes(ident)@local.reference)
(identifier)@local.reference
(tag_expr(tag))@local.reference
