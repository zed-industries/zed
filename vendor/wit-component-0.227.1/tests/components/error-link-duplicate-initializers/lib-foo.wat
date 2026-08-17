(module
  (@dylink.0
    (mem-info (memory 0 4))
    (needed "bar")
  )
  (type (func (param i32) (result i32)))
  (type (func))
  (import "test:test/test" "foo" (func $import_foo (type 0)))
  (import "env" "foo" (func $import_foo2 (type 0)))
  (import "env" "bar" (func $import_bar (type 0)))
  (func $foo (type 0) (param i32) (result i32)
    unreachable
  )
  (func $_initialize (type 1)
    unreachable
  )
  (export "test:test/test#foo" (func $foo))
  (export "foo" (func $foo))
  (export "_initialize" (func $_initialize))
  (export "__wasm_call_ctors" (func $_initialize))  
)
