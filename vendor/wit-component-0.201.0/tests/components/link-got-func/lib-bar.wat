(module
  (@dylink.0
    (mem-info (memory 0 4))
  )
  (type (func (param i32) (result i32)))
  (func $foo (type 0) (param i32) (result i32)
    unreachable
  )
  (export "foo" (func $foo))
)
