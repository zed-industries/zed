(module
  (@dylink.0
    (mem-info (memory 0 4))
  )
  (type (func (param i32) (result i32)))
  (type (func (param i32 i32) (result i32)))
  (import "test:test/test" "foo" (func $import_foo (type 0)))
  (import "env" "dlopen" (func $dlopen (type 1)))
  (func $foo (type 0) (param i32) (result i32)
    unreachable
  )
  (global $what i32 i32.const 42)
  (global $um i32 i32.const 0)
  (export "test:test/test#foo" (func $foo))
  (export "bar" (func $foo))
  (export "baz" (func $foo))
  (export "what" (global $what))
  (export "um" (global $um))
)
