(module
  (import "new" "read" (func $read (param i32)))
  (import "env" "memory" (memory 0))

  (func (export "read") (param i32 i32)
    i32.const 8
    call $read
    unreachable
  )

  (func (export "cabi_import_realloc") (param i32 i32 i32 i32) (result i32)
    unreachable
  )
)
