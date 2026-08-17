(module
  (import "foo:foo/adapter-imports" "foo" (func $foo (param i32 i32)))
  (func (export "adapter-bar") (param i32 i32)
    (call $foo (i32.const 0) (i32.const 0))
  )
  (func (export "cabi_export_realloc") (param i32 i32 i32 i32) (result i32) unreachable)
)
