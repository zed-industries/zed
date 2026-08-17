(component
  (type (;0;)
    (instance
      (type (;0;) (func (param "v" s32) (result s32)))
      (export (;0;) "bar" (func (type 0)))
    )
  )
  (import "test:test/test" (instance (;0;) (type 0)))
  (core module (;0;)
    (table (;0;) 1 funcref)
    (memory (;0;) 17)
    (global (;0;) (mut i32) i32.const 1048576)
    (global (;1;) i32 i32.const 1048592)
    (global (;2;) i32 i32.const 1)
    (global (;3;) (mut i32) i32.const 0)
    (global (;4;) i32 i32.const 1048624)
    (global (;5;) i32 i32.const 1)
    (global (;6;) i32 i32.const 1048624)
    (global (;7;) i32 i32.const 1)
    (global (;8;) (mut i32) i32.const 0)
    (global (;9;) (mut i32) i32.const 1048640)
    (global (;10;) (mut i32) i32.const 1114112)
    (export "__stack_pointer" (global 0))
    (export "bar:memory_base" (global 1))
    (export "bar:table_base" (global 2))
    (export "bar:well" (global 3))
    (export "c:memory_base" (global 4))
    (export "c:table_base" (global 5))
    (export "foo:memory_base" (global 6))
    (export "foo:table_base" (global 7))
    (export "foo:um" (global 8))
    (export "__heap_base" (global 9))
    (export "__heap_end" (global 10))
    (export "__indirect_function_table" (table 0))
    (export "memory" (memory 0))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core module (;1;)
    (@dylink.0
      (mem-info (memory 0 4))
    )
    (type (;0;) (func))
    (type (;1;) (func (param i32) (result i32)))
    (import "GOT.mem" "__heap_base" (global $__heap_base (;0;) (mut i32)))
    (import "GOT.mem" "__heap_end" (global $__heap_end (;1;) (mut i32)))
    (global $heap (;2;) (mut i32) i32.const 0)
    (export "malloc" (func $malloc))
    (export "abort" (func $abort))
    (start $start)
    (func $start (;0;) (type 0)
      global.get $__heap_base
      global.set $heap
    )
    (func $malloc (;1;) (type 1) (param i32) (result i32)
      global.get $heap
      global.get $heap
      local.get 0
      i32.add
      global.set $heap
    )
    (func $abort (;2;) (type 0)
      unreachable
    )
  )
  (core module (;2;)
    (@dylink.0
      (mem-info (memory 4 4))
      (needed "c")
    )
    (type (;0;) (func))
    (type (;1;) (func (param i32) (result i32)))
    (import "env" "memory" (memory (;0;) 1))
    (import "env" "__indirect_function_table" (table (;0;) 0 funcref))
    (import "env" "__stack_pointer" (global $__stack_pointer (;0;) (mut i32)))
    (import "env" "__memory_base" (global $__memory_base (;1;) i32))
    (import "env" "__table_base" (global $__table_base (;2;) i32))
    (import "env" "malloc" (func $malloc (;0;) (type 1)))
    (import "env" "abort" (func $abort (;1;) (type 0)))
    (import "GOT.mem" "um" (global $um (;3;) (mut i32)))
    (import "test:test/test" "bar" (func $bar (;2;) (type 1)))
    (global (;4;) i32 i32.const 0)
    (export "_initialize" (func $_initialize))
    (export "__wasm_apply_data_relocs" (func $__wasm_apply_data_relocs))
    (export "foo" (func $foo))
    (export "well" (global 4))
    (func $_initialize (;3;) (type 0))
    (func $__wasm_apply_data_relocs (;4;) (type 0))
    (func $foo (;5;) (type 1) (param i32) (result i32)
      global.get $__stack_pointer
      i32.const 16
      i32.sub
      global.set $__stack_pointer
      i32.const 4
      call $malloc
      i32.const 0
      i32.eq
      if ;; label = @1
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
    (data $.data (;0;) (global.get $__memory_base) "\04\00\00\00")
  )
  (core module (;3;)
    (@dylink.0
      (mem-info (memory 20 4))
      (needed "foo")
    )
    (type (;0;) (func (param i32) (result i32)))
    (type (;1;) (func))
    (import "env" "memory" (memory (;0;) 1))
    (import "env" "__indirect_function_table" (table (;0;) 0 funcref))
    (import "env" "__memory_base" (global $__memory_base (;0;) i32))
    (import "env" "__table_base" (global $__table_base (;1;) i32))
    (import "env" "foo" (func $foo (;0;) (type 0)))
    (import "GOT.mem" "well" (global $well (;2;) (mut i32)))
    (global (;3;) i32 i32.const 0)
    (export "_initialize" (func $_initialize))
    (export "__wasm_apply_data_relocs" (func $__wasm_apply_data_relocs))
    (export "test:test/test#bar" (func $bar))
    (export "um" (global 3))
    (func $_initialize (;1;) (type 1))
    (func $__wasm_apply_data_relocs (;2;) (type 1))
    (func $bar (;3;) (type 0) (param i32) (result i32)
      local.get 0
      call $foo
      global.get $well
      i32.load
      i32.add
    )
    (data $.data (;0;) (global.get $__memory_base) "\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00\05\00\00\00")
  )
  (core module (;4;)
    (type (;0;) (func))
    (type (;1;) (func (param i32)))
    (import "env" "memory" (memory (;0;) 0))
    (import "env" "__indirect_function_table" (table (;0;) 0 funcref))
    (import "bar" "__wasm_apply_data_relocs" (func (;0;) (type 0)))
    (import "bar" "_initialize" (func (;1;) (type 0)))
    (import "env" "foo:memory_base" (global (;0;) i32))
    (import "foo" "well" (global (;1;) i32))
    (import "env" "bar:well" (global (;2;) (mut i32)))
    (import "foo" "__wasm_apply_data_relocs" (func (;2;) (type 0)))
    (import "foo" "_initialize" (func (;3;) (type 0)))
    (import "env" "bar:memory_base" (global (;3;) i32))
    (import "bar" "um" (global (;4;) i32))
    (import "env" "foo:um" (global (;5;) (mut i32)))
    (start 4)
    (elem (;0;) (i32.const 1) func)
    (elem (;1;) (i32.const 1) func)
    (func (;4;) (type 0)
      global.get 0
      global.get 1
      i32.add
      global.set 2
      global.get 3
      global.get 4
      i32.add
      global.set 5
      call 0
      call 2
      call 1
      call 3
    )
    (data (;0;) (i32.const 1048576) "\00\00\00\00\00\00\10\00")
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core instance (;0;) (instantiate 0))
  (alias core export 0 "memory" (core memory (;0;)))
  (alias core export 0 "__heap_base" (core global (;0;)))
  (alias core export 0 "__heap_end" (core global (;1;)))
  (core instance (;1;)
    (export "__heap_base" (global 0))
    (export "__heap_end" (global 1))
  )
  (core instance (;2;) (instantiate 1
      (with "GOT.mem" (instance 1))
    )
  )
  (alias core export 0 "__indirect_function_table" (core table (;0;)))
  (alias core export 0 "__stack_pointer" (core global (;2;)))
  (alias core export 0 "foo:memory_base" (core global (;3;)))
  (alias core export 0 "foo:table_base" (core global (;4;)))
  (alias core export 2 "malloc" (core func (;0;)))
  (alias core export 2 "abort" (core func (;1;)))
  (core instance (;3;)
    (export "memory" (memory 0))
    (export "__indirect_function_table" (table 0))
    (export "__stack_pointer" (global 2))
    (export "__memory_base" (global 3))
    (export "__table_base" (global 4))
    (export "malloc" (func 0))
    (export "abort" (func 1))
  )
  (alias core export 0 "foo:um" (core global (;5;)))
  (core instance (;4;)
    (export "um" (global 5))
  )
  (alias export 0 "bar" (func (;0;)))
  (core func (;2;) (canon lower (func 0)))
  (core instance (;5;)
    (export "bar" (func 2))
  )
  (core instance (;6;) (instantiate 2
      (with "env" (instance 3))
      (with "GOT.mem" (instance 4))
      (with "test:test/test" (instance 5))
    )
  )
  (alias core export 0 "bar:memory_base" (core global (;6;)))
  (alias core export 0 "bar:table_base" (core global (;7;)))
  (alias core export 6 "foo" (core func (;3;)))
  (core instance (;7;)
    (export "memory" (memory 0))
    (export "__indirect_function_table" (table 0))
    (export "__memory_base" (global 6))
    (export "__table_base" (global 7))
    (export "foo" (func 3))
  )
  (alias core export 0 "bar:well" (core global (;8;)))
  (core instance (;8;)
    (export "well" (global 8))
  )
  (core instance (;9;) (instantiate 3
      (with "env" (instance 7))
      (with "GOT.mem" (instance 8))
    )
  )
  (core instance (;10;) (instantiate 4
      (with "env" (instance 0))
      (with "bar" (instance 9))
      (with "foo" (instance 6))
    )
  )
  (type (;1;) (func (param "v" s32) (result s32)))
  (alias core export 9 "test:test/test#bar" (core func (;4;)))
  (func (;1;) (type 1) (canon lift (core func 4)))
  (component (;0;)
    (type (;0;) (func (param "v" s32) (result s32)))
    (import "import-func-bar" (func (;0;) (type 0)))
    (type (;1;) (func (param "v" s32) (result s32)))
    (export (;1;) "bar" (func 0) (func (type 1)))
  )
  (instance (;1;) (instantiate 0
      (with "import-func-bar" (func 1))
    )
  )
  (export (;2;) "test:test/test" (instance 1))
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
