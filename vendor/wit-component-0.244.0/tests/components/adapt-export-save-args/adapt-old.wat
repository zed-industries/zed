(module
  (import "__main_module__" "the_entrypoint" (func $entry))

  (global $nargs (mut i32) (i32.const 0))

  (func (export "entrypoint") (param $nargs i32)
    (global.set $nargs (local.get $nargs))
    call $entry
  )

  (func (export "nargs") (result i32)
    global.get $nargs)
)
