[
    (module_definition) ; Top-level module definition
    (struct_definition) ; Struct definitions
    (function_definition) ; Function definitions
    (spec_block) ; Specification blocks
    (if_expression) ; If expressions
    (loop_expression) ; While or infinite loops
    (block) ; Any nested block
    (match_expression) ; Match statements (similar to switch-case)
] @indent.begin

; For ending delimiters that may indicate an outdent
(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent
