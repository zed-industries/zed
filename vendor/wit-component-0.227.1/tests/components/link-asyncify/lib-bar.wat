(module
  (@dylink.0
    (mem-info (memory 1 4))
    (needed "foo")
  )
  (type (func))
  (type (func (param i32)))
  (type (func (result i32)))
  (import "env" "__memory_base" (global $__memory_base  i32))
  (import "env" "__table_base" (global $__table_base  i32))
  (import "env" "__asyncify_state" (global  (mut i32)))
  (import "env" "__asyncify_data" (global  (mut i32)))
  (import "env" "memory" (memory  1))
  (import "env" "__indirect_function_table" (table  0 funcref))
  (func $__wasm_call_ctors  (type 0))
  (func $__wasm_apply_data_relocs  (type 0))
  (func $asyncify_start_unwind  (type 1) (param i32)
    i32.const 1
    global.set 2
    local.get 0
    global.set 3
    global.get 3
    i32.load
    global.get 3
    i32.load offset=4
    i32.gt_u
    if
      unreachable
    end
  )
  (func $asyncify_stop_unwind  (type 0)
    i32.const 0
    global.set 2
    global.get 3
    i32.load
    global.get 3
    i32.load offset=4
    i32.gt_u
    if
      unreachable
    end
  )
  (func $asyncify_start_rewind  (type 1) (param i32)
    i32.const 2
    global.set 2
    local.get 0
    global.set 3
    global.get 3
    i32.load
    global.get 3
    i32.load offset=4
    i32.gt_u
    if
      unreachable
    end
  )
  (func $asyncify_stop_rewind  (type 0)
    i32.const 0
    global.set 2
    global.get 3
    i32.load
    global.get 3
    i32.load offset=4
    i32.gt_u
    if
      unreachable
    end
  )
  (func $asyncify_get_state  (type 2) (result i32)
    global.get 2
  )
  (func $bar (type 0))
  (export "test:test/test#bar" (func $bar))
  (export "__wasm_apply_data_relocs" (func $__wasm_apply_data_relocs))
  (export "asyncify_start_unwind" (func $asyncify_start_unwind))
  (export "asyncify_stop_unwind" (func $asyncify_stop_unwind))
  (export "asyncify_start_rewind" (func $asyncify_start_rewind))
  (export "asyncify_stop_rewind" (func $asyncify_stop_rewind))
  (export "asyncify_get_state" (func $asyncify_get_state))
  (data $.bss  (global.get $__memory_base) "\00\00\00\00")
)
