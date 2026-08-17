(module
  (func (import "$root" "[async-lower]foo") (param i32 i32 i32) (result i32))
  (func (import "foo:foo/bar" "[async-lower]foo") (param i32 i32 i32) (result i32))
  (func (import "$root" "foo") (param i32 i32 i32))
  (func (import "foo:foo/bar" "foo") (param i32 i32 i32))
  (memory (export "memory") 1)
  (func (export "cabi_realloc") (param i32 i32 i32 i32) (result i32) unreachable)
)
