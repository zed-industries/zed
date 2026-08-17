(component
  (core module $main (;0;)
    (type (;0;) (func))
    (table (;0;) 1 1 funcref)
    (memory (;0;) 65536 (pagesize 0x1))
    (global (;0;) (mut i32) i32.const 1024)
    (export "memory" (memory 0))
    (func (;0;) (type 0))
    (func (;1;) (type 0)
      call 0
      call 0
    )
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (core instance $main (;0;) (instantiate $main))
  (alias core export $main "memory" (core memory $memory (;0;)))
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
