(component
  (type $ty-new (;0;)
    (instance
      (type (;0;) (tuple u32 u32))
      (type (;1;) (func (result 0)))
      (export (;0;) "get-two" (func (type 1)))
    )
  )
  (import "new" (instance $new (;0;) (type $ty-new)))
  (core module $main (;0;)
    (type (;0;) (func (result i32)))
    (type (;1;) (func (param i32 i32 i32 i32) (result i32)))
    (import "old" "get_sum" (func (;0;) (type 0)))
    (memory (;0;) 1)
    (export "memory" (memory 0))
    (export "cabi_realloc" (func $cabi_realloc))
    (export "cabi_realloc_adapter" (func $cabi_realloc_adapter))
    (func $cabi_realloc (;1;) (type 1) (param i32 i32 i32 i32) (result i32)
      i32.const 123456789
    )
    (func $cabi_realloc_adapter (;2;) (type 1) (param i32 i32 i32 i32) (result i32)
      i32.const 987654321
    )
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (core module $wit-component:adapter:old (;1;)
    (type (;0;) (func (param i32)))
    (type (;1;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;2;) (func (result i32)))
    (type (;3;) (func))
    (import "env" "memory" (memory (;0;) 0))
    (import "new" "get-two" (func $get_two (;0;) (type 0)))
    (import "__main_module__" "cabi_realloc_adapter" (func $cabi_realloc (;1;) (type 1)))
    (global $__stack_pointer (;0;) (mut i32) i32.const 0)
    (global $some_other_mutable_global (;1;) (mut i32) i32.const 0)
    (global $allocation_state (;2;) (mut i32) i32.const 0)
    (export "get_sum" (func 2))
    (func (;2;) (type 2) (result i32)
      (local i32 i32)
      call $allocate_stack
      global.get $allocation_state
      i32.const 2
      i32.ne
      if ;; label = @1
        unreachable
      end
      i32.const 0
      i32.const 0
      i32.const 8
      i32.const 65536
      call $cabi_realloc
      local.set 0
      local.get 0
      i32.const 42
      i32.store
      local.get 0
      i32.const 42
      i32.store offset=65532
      global.get $__stack_pointer
      local.tee 0
      i32.const 8
      i32.sub
      local.tee 1
      global.set $__stack_pointer
      local.get 1
      call $get_two
      local.get 1
      i32.load
      local.get 1
      i32.load offset=4
      i32.add
      global.get $some_other_mutable_global
      global.set $some_other_mutable_global
      local.get 0
      global.set $__stack_pointer
    )
    (func $allocate_stack (;3;) (type 3)
      global.get $allocation_state
      i32.const 0
      i32.eq
      if ;; label = @1
        i32.const 1
        global.set $allocation_state
        i32.const 0
        i32.const 0
        i32.const 8
        i32.const 65536
        call $cabi_realloc
        i32.const 65536
        i32.add
        global.set $__stack_pointer
        i32.const 2
        global.set $allocation_state
      end
    )
  )
  (core module $wit-component-shim-module (;2;)
    (type (;0;) (func (result i32)))
    (type (;1;) (func (param i32)))
    (table (;0;) 2 2 funcref)
    (export "0" (func $adapt-old-get_sum))
    (export "1" (func $indirect-new-get-two))
    (export "$imports" (table 0))
    (func $adapt-old-get_sum (;0;) (type 0) (result i32)
      i32.const 0
      call_indirect (type 0)
    )
    (func $indirect-new-get-two (;1;) (type 1) (param i32)
      local.get 0
      i32.const 1
      call_indirect (type 1)
    )
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core module $wit-component-fixup (;3;)
    (type (;0;) (func (result i32)))
    (type (;1;) (func (param i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "1" (func (;1;) (type 1)))
    (import "" "$imports" (table (;0;) 2 2 funcref))
    (elem (;0;) (i32.const 0) func 0 1)
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core instance $wit-component-shim-instance (;0;) (instantiate $wit-component-shim-module))
  (alias core export $wit-component-shim-instance "0" (core func $adapt-old-get_sum (;0;)))
  (core instance $old (;1;)
    (export "get_sum" (func $adapt-old-get_sum))
  )
  (core instance $main (;2;) (instantiate $main
      (with "old" (instance $old))
    )
  )
  (alias core export $main "memory" (core memory $memory (;0;)))
  (core instance $env (;3;)
    (export "memory" (memory $memory))
  )
  (alias core export $wit-component-shim-instance "1" (core func $indirect-new-get-two (;1;)))
  (core instance $new (;4;)
    (export "get-two" (func $indirect-new-get-two))
  )
  (alias core export $main "cabi_realloc_adapter" (core func $cabi_realloc_adapter (;2;)))
  (core instance $__main_module__ (;5;)
    (export "cabi_realloc_adapter" (func $cabi_realloc_adapter))
  )
  (core instance $"#core-instance6 old" (@name "old") (;6;) (instantiate $wit-component:adapter:old
      (with "env" (instance $env))
      (with "new" (instance $new))
      (with "__main_module__" (instance $__main_module__))
    )
  )
  (alias core export $wit-component-shim-instance "$imports" (core table $"shim table" (;0;)))
  (alias core export $"#core-instance6 old" "get_sum" (core func $get_sum (;3;)))
  (alias export $new "get-two" (func $get-two (;0;)))
  (core func $"#core-func4 indirect-new-get-two" (@name "indirect-new-get-two") (;4;) (canon lower (func $get-two) (memory $memory)))
  (core instance $fixup-args (;7;)
    (export "$imports" (table $"shim table"))
    (export "0" (func $get_sum))
    (export "1" (func $"#core-func4 indirect-new-get-two"))
  )
  (core instance $fixup (;8;) (instantiate $wit-component-fixup
      (with "" (instance $fixup-args))
    )
  )
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
