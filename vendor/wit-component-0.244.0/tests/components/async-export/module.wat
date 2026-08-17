(module
  (func (export "[async-lift-stackful]foo") (param i32 i32) unreachable)
  (func (export "[async-lift-stackful]foo:foo/bar#foo") (param i32 i32) unreachable)
  (func (export "[async-lift-stackful]foo:foo/bar#get-string") unreachable)
  (func (export "[async-lift-stackful]foo:foo/bar#get-big") unreachable)
  (memory (export "memory") 1)
  (func (export "cabi_realloc") (param i32 i32 i32 i32) (result i32) unreachable)
)
