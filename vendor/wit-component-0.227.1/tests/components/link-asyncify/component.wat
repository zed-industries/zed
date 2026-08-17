(component
  (core module (;0;)
    (table (;0;) 1 funcref)
    (memory (;0;) 17)
    (global (;0;) (mut i32) i32.const 1048576)
    (global (;1;) (mut i32) i32.const 0)
    (global (;2;) (mut i32) i32.const 0)
    (global (;3;) i32 i32.const 1048592)
    (global (;4;) i32 i32.const 1)
    (global (;5;) i32 i32.const 1048608)
    (global (;6;) i32 i32.const 1)
    (global (;7;) (mut i32) i32.const 1048624)
    (global (;8;) (mut i32) i32.const 1114112)
    (export "__stack_pointer" (global 0))
    (export "__asyncify_state" (global 1))
    (export "__asyncify_data" (global 2))
    (export "bar:memory_base" (global 3))
    (export "bar:table_base" (global 4))
    (export "foo:memory_base" (global 5))
    (export "foo:table_base" (global 6))
    (export "__heap_base" (global 7))
    (export "__heap_end" (global 8))
    (export "__indirect_function_table" (table 0))
    (export "memory" (memory 0))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core module (;1;)
    (@dylink.0
      (mem-info (memory 1 4))
      (needed "foo")
    )
    (type (;0;) (func))
    (type (;1;) (func (param i32)))
    (type (;2;) (func (result i32)))
    (import "env" "__memory_base" (global $__memory_base (;0;) i32))
    (import "env" "__table_base" (global $__table_base (;1;) i32))
    (import "env" "__asyncify_state" (global (;2;) (mut i32)))
    (import "env" "__asyncify_data" (global (;3;) (mut i32)))
    (import "env" "memory" (memory (;0;) 1))
    (import "env" "__indirect_function_table" (table (;0;) 0 funcref))
    (export "test:test/test#bar" (func $bar))
    (export "__wasm_apply_data_relocs" (func $__wasm_apply_data_relocs))
    (export "asyncify_start_unwind" (func $asyncify_start_unwind))
    (export "asyncify_stop_unwind" (func $asyncify_stop_unwind))
    (export "asyncify_start_rewind" (func $asyncify_start_rewind))
    (export "asyncify_stop_rewind" (func $asyncify_stop_rewind))
    (export "asyncify_get_state" (func $asyncify_get_state))
    (func $__wasm_call_ctors (;0;) (type 0))
    (func $__wasm_apply_data_relocs (;1;) (type 0))
    (func $asyncify_start_unwind (;2;) (type 1) (param i32)
      i32.const 1
      global.set 2
      local.get 0
      global.set 3
      global.get 3
      i32.load
      global.get 3
      i32.load offset=4
      i32.gt_u
      if ;; label = @1
        unreachable
      end
    )
    (func $asyncify_stop_unwind (;3;) (type 0)
      i32.const 0
      global.set 2
      global.get 3
      i32.load
      global.get 3
      i32.load offset=4
      i32.gt_u
      if ;; label = @1
        unreachable
      end
    )
    (func $asyncify_start_rewind (;4;) (type 1) (param i32)
      i32.const 2
      global.set 2
      local.get 0
      global.set 3
      global.get 3
      i32.load
      global.get 3
      i32.load offset=4
      i32.gt_u
      if ;; label = @1
        unreachable
      end
    )
    (func $asyncify_stop_rewind (;5;) (type 0)
      i32.const 0
      global.set 2
      global.get 3
      i32.load
      global.get 3
      i32.load offset=4
      i32.gt_u
      if ;; label = @1
        unreachable
      end
    )
    (func $asyncify_get_state (;6;) (type 2) (result i32)
      global.get 2
    )
    (func $bar (;7;) (type 0))
    (data $.bss (;0;) (global.get $__memory_base) "\00\00\00\00")
  )
  (core module (;2;)
    (@dylink.0
      (mem-info (memory 1 4))
      (needed "bar")
    )
    (type (;0;) (func))
    (type (;1;) (func (param i32)))
    (type (;2;) (func (result i32)))
    (import "env" "__memory_base" (global $__memory_base (;0;) i32))
    (import "env" "__table_base" (global $__table_base (;1;) i32))
    (import "env" "__asyncify_state" (global (;2;) (mut i32)))
    (import "env" "__asyncify_data" (global (;3;) (mut i32)))
    (import "env" "memory" (memory (;0;) 1))
    (import "env" "__indirect_function_table" (table (;0;) 0 funcref))
    (export "__wasm_apply_data_relocs" (func $__wasm_apply_data_relocs))
    (export "asyncify_start_unwind" (func $asyncify_start_unwind))
    (export "asyncify_stop_unwind" (func $asyncify_stop_unwind))
    (export "asyncify_start_rewind" (func $asyncify_start_rewind))
    (export "asyncify_stop_rewind" (func $asyncify_stop_rewind))
    (export "asyncify_get_state" (func $asyncify_get_state))
    (func $__wasm_call_ctors (;0;) (type 0))
    (func $__wasm_apply_data_relocs (;1;) (type 0))
    (func $asyncify_start_unwind (;2;) (type 1) (param i32)
      i32.const 1
      global.set 2
      local.get 0
      global.set 3
      global.get 3
      i32.load
      global.get 3
      i32.load offset=4
      i32.gt_u
      if ;; label = @1
        unreachable
      end
    )
    (func $asyncify_stop_unwind (;3;) (type 0)
      i32.const 0
      global.set 2
      global.get 3
      i32.load
      global.get 3
      i32.load offset=4
      i32.gt_u
      if ;; label = @1
        unreachable
      end
    )
    (func $asyncify_start_rewind (;4;) (type 1) (param i32)
      i32.const 2
      global.set 2
      local.get 0
      global.set 3
      global.get 3
      i32.load
      global.get 3
      i32.load offset=4
      i32.gt_u
      if ;; label = @1
        unreachable
      end
    )
    (func $asyncify_stop_rewind (;5;) (type 0)
      i32.const 0
      global.set 2
      global.get 3
      i32.load
      global.get 3
      i32.load offset=4
      i32.gt_u
      if ;; label = @1
        unreachable
      end
    )
    (func $asyncify_get_state (;6;) (type 2) (result i32)
      global.get 2
    )
  )
  (core module (;3;)
    (type (;0;) (func))
    (type (;1;) (func (param i32)))
    (import "env" "memory" (memory (;0;) 0))
    (import "env" "__indirect_function_table" (table (;0;) 0 funcref))
    (import "bar" "__wasm_apply_data_relocs" (func (;0;) (type 0)))
    (import "foo" "__wasm_apply_data_relocs" (func (;1;) (type 0)))
    (start 2)
    (elem (;0;) (i32.const 1) func)
    (elem (;1;) (i32.const 1) func)
    (func (;2;) (type 0)
      call 0
      call 1
    )
    (data (;0;) (i32.const 1048576) "\00\00\00\00\00\00\10\00")
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core instance (;0;) (instantiate 0))
  (alias core export 0 "memory" (core memory (;0;)))
  (alias core export 0 "bar:memory_base" (core global (;0;)))
  (alias core export 0 "bar:table_base" (core global (;1;)))
  (alias core export 0 "__asyncify_state" (core global (;2;)))
  (alias core export 0 "__asyncify_data" (core global (;3;)))
  (alias core export 0 "__indirect_function_table" (core table (;0;)))
  (core instance (;1;)
    (export "__memory_base" (global 0))
    (export "__table_base" (global 1))
    (export "__asyncify_state" (global 2))
    (export "__asyncify_data" (global 3))
    (export "memory" (memory 0))
    (export "__indirect_function_table" (table 0))
  )
  (core instance (;2;) (instantiate 1
      (with "env" (instance 1))
    )
  )
  (alias core export 0 "foo:memory_base" (core global (;4;)))
  (alias core export 0 "foo:table_base" (core global (;5;)))
  (core instance (;3;)
    (export "__memory_base" (global 4))
    (export "__table_base" (global 5))
    (export "__asyncify_state" (global 2))
    (export "__asyncify_data" (global 3))
    (export "memory" (memory 0))
    (export "__indirect_function_table" (table 0))
  )
  (core instance (;4;) (instantiate 2
      (with "env" (instance 3))
    )
  )
  (core instance (;5;) (instantiate 3
      (with "env" (instance 0))
      (with "bar" (instance 2))
      (with "foo" (instance 4))
    )
  )
  (type (;0;) (func))
  (alias core export 2 "test:test/test#bar" (core func (;0;)))
  (func (;0;) (type 0) (canon lift (core func 0)))
  (component (;0;)
    (type (;0;) (func))
    (import "import-func-bar" (func (;0;) (type 0)))
    (type (;1;) (func))
    (export (;1;) "bar" (func 0) (func (type 1)))
  )
  (instance (;0;) (instantiate 0
      (with "import-func-bar" (func 0))
    )
  )
  (export (;1;) "test:test/test" (instance 0))
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
