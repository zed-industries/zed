(module
  (@dylink.0
    (mem-info (memory 20 4))
    (needed "foo")
  )
  (type (func (param i32) (result i32)))
  (type (func))
  (import "env" "memory" (memory 1))
  (import "env" "__indirect_function_table" (table 0 funcref))
  (import "env" "__memory_base" (global $__memory_base i32))
  (import "env" "__table_base" (global $__table_base i32))
  (import "env" "foo" (func $foo (type 0)))
  (import "GOT.mem" "well" (global $well (mut i32)))
  (func $__wasm_call_ctors (type 1))
  (func $__wasm_apply_data_relocs (type 1))
  (func $bar (type 0) (param i32) (result i32)
    local.get 0
    call $foo
    global.get $well
    i32.load
    i32.add
  )
  (global i32 i32.const 0)
  (export "__wasm_call_ctors" (func $__wasm_call_ctors))
  (export "__wasm_apply_data_relocs" (func $__wasm_apply_data_relocs))
  (export "test:test/test#bar" (func $bar))
  (export "um" (global 3))
  (data $.data (global.get $__memory_base) "\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00\05\00\00\00")
)
