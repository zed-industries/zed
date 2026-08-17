(module
  (@dylink.0
    (mem-info (memory 0 4))
  )
  (type (func (param i32) (result i32)))
  (import "test:test/test" "foo" (func $import_foo (type 0)))
  (import "env" "foo" (func $import_foo2 (type 0)))
  (import "GOT.func" "foo" (global $import_foo2_address (mut i32)))
  (export "test:test/test#foo" (func $import_foo2))
)
