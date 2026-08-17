(module
  (@dylink.0
    (mem-info (memory 0 4))
    (needed "foo")
  )
  (type (func (param i32) (result i32)))
  (import "env" "foo" (func $import_foo (type 0)))
  (func $bar (type 0) (param i32) (result i32)
    unreachable
  )
  (export "bar" (func $bar))
  (export "foo" (func $bar))
)
