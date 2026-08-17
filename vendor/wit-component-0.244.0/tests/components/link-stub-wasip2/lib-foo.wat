(module
  (@dylink.0
    (mem-info (memory 4 4))
    (needed "c")
  )
  (type (func))
  (type (func (param i32) (result i32)))
  (import "env" "memory" (memory 1))
  (import "env" "__indirect_function_table" (table 0 funcref))
  (import "env" "__stack_pointer" (global $__stack_pointer (mut i32)))
  (import "env" "__memory_base" (global $__memory_base i32))
  (import "env" "__table_base" (global $__table_base i32))
  (import "env" "malloc" (func $malloc (type 1)))
  (import "env" "abort" (func $abort (type 0)))
  (import "GOT.mem" "um" (global $um (mut i32)))
  (import "test:test/test" "bar" (func $bar (type 1)))
  (func $__wasm_call_ctors (type 0))
  (func $__wasm_apply_data_relocs (type 0))
  (func $foo (type 1) (param i32) (result i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    global.set $__stack_pointer

    i32.const 4
    call $malloc

    i32.const 0
    i32.eq
    if
      call $abort
      unreachable
    end

    local.get 0
    global.get $um
    i32.load offset=16
    i32.add
    i32.const 42
    i32.add

    call $bar

    global.get $__stack_pointer
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (global i32 i32.const 0)
  (export "__wasm_call_ctors" (func $__wasm_call_ctors))
  (export "__wasm_apply_data_relocs" (func $__wasm_apply_data_relocs))
  (export "foo" (func $foo))
  (export "well" (global 4))
  (data $.data (global.get $__memory_base) "\04\00\00\00")
)
