(component
  (type (;0;)
    (instance
      (type (;0;) (list u8))
      (type (;1;) (func (param "amt" u32) (result 0)))
      (export (;0;) "read" (func (type 1)))
    )
  )
  (import "new" (instance (;0;) (type 0)))
  (core module (;0;)
    (type (;0;) (func (param i32 i32)))
    (import "old" "read" (func (;0;) (type 0)))
    (memory (;0;) 1)
    (export "main" (func 1))
    (export "memory" (memory 0))
    (func (;1;) (type 0) (param $args i32) (param $argv i32))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (core module (;1;)
    (type (;0;) (func (param i32 i32)))
    (type (;1;) (func (param i32 i32 i32 i32) (result i32)))
    (import "new" "read" (func $read (;0;) (type 0)))
    (global $sp (;0;) (mut i32) i32.const 0)
    (export "entrypoint" (func 1))
    (export "cabi_export_realloc" (func 2))
    (export "read" (func 3))
    (export "cabi_import_realloc" (func 4))
    (func (;1;) (type 0) (param i32 i32)
      unreachable
    )
    (func (;2;) (type 1) (param i32 i32 i32 i32) (result i32)
      unreachable
    )
    (func (;3;) (type 0) (param i32 i32)
      (local i32)
      global.get $sp
      i32.const 8
      i32.sub
      local.tee 2
      global.set $sp
      local.get 1
      local.get 2
      call $read
      local.get 2
      i32.const 8
      i32.add
      global.set $sp
    )
    (func (;4;) (type 1) (param i32 i32 i32 i32) (result i32)
      unreachable
    )
  )
  (core module (;2;)
    (type (;0;) (func (param i32 i32)))
    (type (;1;) (func (param i32 i32)))
    (table (;0;) 2 2 funcref)
    (export "0" (func $adapt-old-read))
    (export "1" (func $indirect-new-read))
    (export "$imports" (table 0))
    (func $adapt-old-read (;0;) (type 0) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 0
      call_indirect (type 0)
    )
    (func $indirect-new-read (;1;) (type 1) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 1
      call_indirect (type 1)
    )
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core module (;3;)
    (type (;0;) (func (param i32 i32)))
    (type (;1;) (func (param i32 i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "1" (func (;1;) (type 1)))
    (import "" "$imports" (table (;0;) 2 2 funcref))
    (elem (;0;) (i32.const 0) func 0 1)
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core instance (;0;) (instantiate 2))
  (alias core export 0 "0" (core func (;0;)))
  (core instance (;1;)
    (export "read" (func 0))
  )
  (core instance (;2;) (instantiate 0
      (with "old" (instance 1))
    )
  )
  (alias core export 2 "memory" (core memory (;0;)))
  (alias core export 0 "1" (core func (;1;)))
  (core instance (;3;)
    (export "read" (func 1))
  )
  (core instance (;4;) (instantiate 1
      (with "new" (instance 3))
    )
  )
  (alias core export 0 "$imports" (core table (;0;)))
  (alias core export 4 "read" (core func (;2;)))
  (alias export 0 "read" (func (;0;)))
  (alias core export 4 "cabi_import_realloc" (core func (;3;)))
  (core func (;4;) (canon lower (func 0) (memory 0) (realloc 3)))
  (core instance (;5;)
    (export "$imports" (table 0))
    (export "0" (func 2))
    (export "1" (func 4))
  )
  (core instance (;6;) (instantiate 3
      (with "" (instance 5))
    )
  )
  (type (;1;) (list string))
  (type (;2;) (func (param "args" 1)))
  (alias core export 4 "entrypoint" (core func (;5;)))
  (alias core export 4 "cabi_export_realloc" (core func (;6;)))
  (func (;1;) (type 2) (canon lift (core func 5) (memory 0) (realloc 6) string-encoding=utf8))
  (export (;2;) "entrypoint" (func 1))
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
