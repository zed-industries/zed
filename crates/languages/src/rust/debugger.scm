(metavariable) @debug-variable
(parameter (identifier) @debug-variable)
(self) @debug-variable
(reference_expression
  (identifier) @debug-variable)

(binary_expression
  left: (identifier) @debug-variable)

(binary_expression
  right: (identifier) @debug-variable)

; Capture identifiers in assignment expressions
(assignment_expression
  left: (identifier) @debug-variable)

(identifier) @debug-variable


; For assignment expressions where the right side is any expression,
; we rely on other patterns to capture identifiers within that expression
