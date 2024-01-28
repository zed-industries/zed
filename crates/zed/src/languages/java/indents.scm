; format-ignore
[
  ; ... refers to the portion that this indent query will have effects on
  (class_body)                        ; { ... } of `class X`
  (enum_body)                         ; { ... } of `enum X`
  (interface_body)                    ; { ... } of `interface X`
  (constructor_body)                  ; { `modifier` X() {...} } inside `class X`
  (annotation_type_body)              ; { ... } of `@interface X`
  (block)                             ; { ... } that's not mentioned in this scope
  (switch_block)                      ; { ... } in `switch X`
  (array_initializer)                 ; [1, 2]
  (argument_list)                     ; foo(...)
  (formal_parameters)                 ; method foo(...)
  (annotation_argument_list)          ; @Annotation(...)
  (element_value_array_initializer)   ; { a, b } inside @Annotation()
] @indent.begin

(expression_statement
  (method_invocation) @indent.begin)

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @indent.branch

(annotation_argument_list
  ")" @indent.end) ; This should be a special cased as `()` here doesn't have ending `;`

(_ "{" "}" @end) @indent

(line_comment) @indent.ignore

[
  (ERROR)
  (block_comment)
] @indent.auto
