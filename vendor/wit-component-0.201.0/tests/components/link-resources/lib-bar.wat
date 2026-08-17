(module
  (@dylink.0
    (mem-info (memory 0 4))
    (needed "foo")
  )
  (type (func (param i32) (result i32)))
  (func $x_ctor (type 0) (param i32) (result i32)
    unreachable
  )
  (func $x_get (type 0) (param i32) (result i32)
    unreachable
  )
  (export "test:test/bar#[constructor]x" (func $x_ctor))
  (export "test:test/bar#[method]x.get" (func $x_get))
)
