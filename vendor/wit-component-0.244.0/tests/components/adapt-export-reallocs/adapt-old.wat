(module
  (import "__main_module__" "main" (func $main (param i32 i32)))
  (import "new" "read" (func $read (param i32 i32)))


  (global $sp (mut i32) (i32.const 0))

  (func $start
    (global.set $sp
      (i32.mul
        (memory.grow (i32.const 1))
        (i32.const 65536)))
  )

  (func (export "entrypoint") (param i32 i32)
    unreachable
  )

  (func (export "cabi_export_realloc") (param i32 i32 i32 i32) (result i32)
    unreachable
  )

  (func (export "read") (param $ptr i32) (param $len i32)
    (local $fp i32)
    (global.set $sp
      (local.tee $fp
        (i32.sub
          (global.get $sp)
          (i32.const 8)
        )
      )
    )

    (call $read (local.get $len) (local.get $fp))

    (global.set $sp
      (i32.add
        (local.get $fp)
        (i32.const 8)
      )
    )
  )

  (func (export "cabi_import_realloc") (param i32 i32 i32 i32) (result i32)
    unreachable
  )
)
