(module
  (@dylink.0
    (mem-info (memory 0 4))
    (needed "bar")
  )
  (type (func (param i32) (result i32)))
  (import "test:test/test" "foo" (func $import_foo (type 0)))
  (import "env" "foo" (func $import_foo2 (type 0)))
  (import "env" "bar" (func $import_bar (type 0)))
  (func $foo (type 0) (param i32) (result i32)
    unreachable
  )
  (export "test:test/test#foo" (func $foo))
  (export "foo" (func $foo))
)
