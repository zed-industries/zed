(module
  (import "qux" "qux1" (func (param i32 i32)))
  (import "qux" "qux2" (func))
  (import "qux" "qux3" (func (param i32)))
  (memory (export "memory") 1)
  (func (export "cabi_realloc") (param i32 i32 i32 i32) (result i32) unreachable)
)
