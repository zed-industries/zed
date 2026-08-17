(module
  (import "bar" "bar1" (func (param i32 i32)))
  (import "bar" "bar2" (func (param i32)))
  (memory (export "memory") 1)
  (func (export "cabi_realloc") (param i32 i32 i32 i32) (result i32) unreachable)
)
