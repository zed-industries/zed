(module
  (import "foo:foo/foo" "a" (func (param i32)))
  (memory (export "memory") 1)
  (func (export "cabi_realloc") (param i32 i32 i32 i32) (result i32) unreachable)
  (func (export "a") (param i32) unreachable)
)
