(module
  (memory (export "memory") 1)
  (func (export "cabi_realloc") (param i32 i32 i32 i32) (result i32) unreachable)
  (func (export "a") (result i32) unreachable)
  (func (export "cabi_post_a") (param i32) unreachable)
)
