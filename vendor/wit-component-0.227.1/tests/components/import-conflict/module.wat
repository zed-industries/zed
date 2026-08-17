(module
  (import "foo:foo/foo" "a" (func))
  (import "foo:foo/bar" "a" (func (param i64 i32 i32)))
  (import "foo:foo/baz" "baz" (func (param i32 i32 i32)))
  (memory (export "memory") 1)
  (func (export "cabi_realloc") (param i32 i32 i32 i32) (result i32) unreachable)
)
