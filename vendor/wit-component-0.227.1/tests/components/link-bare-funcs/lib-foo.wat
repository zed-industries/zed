(module
  (@dylink.0
    (mem-info (memory 4 4))
    (needed "c")
  )
  (import "env" "memory" (memory 1))
  (import "$root" "foo1" (func))
  (import "$root" "bar" (func (param i32)))

  (func (export "baz"))

  (func (export "foo2") (param i32 i32) (result i32) unreachable)
  (func (export "cabi_post_foo2") (param i32) unreachable)

  (func (export "cabi_realloc") (param i32 i32 i32 i32) (result i32) unreachable)
  (func (export "cabi_import_realloc") (param i32 i32 i32 i32) (result i32) unreachable)
  (func (export "cabi_export_realloc") (param i32 i32 i32 i32) (result i32) unreachable)  
)
