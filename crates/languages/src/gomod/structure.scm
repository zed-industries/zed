(require_directive
  "require" @structure.anchor
  ("(") @structure.open
  (")") @structure.close
)

(exclude_directive
  "exclude" @structure.anchor
  ("(") @structure.open
  (")") @structure.close
)

(module_directive
  "module" @structure.anchor
  ("(") @structure.open
  (")") @structure.close
)

(replace_directive
  "replace" @structure.anchor
  ("(") @structure.open
  (")") @structure.close
)

(retract_directive
  "retract" @structure.anchor
  ("(") @structure.open
  (")") @structure.close
)

(ignore_directive
  "ignore" @structure.anchor
  ("(") @structure.open
  (")") @structure.close
)
