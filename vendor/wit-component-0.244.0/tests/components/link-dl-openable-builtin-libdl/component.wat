(component
  (type $ty-test:test/test (;0;)
    (instance
      (type (;0;) (func (param "v" s32) (result s32)))
      (export (;0;) "foo" (func (type 0)))
    )
  )
  (import "test:test/test" (instance $test:test/test (;0;) (type $ty-test:test/test)))
  (core module $main (;0;)
    (table (;0;) 4 funcref)
    (memory (;0;) 17)
    (global (;0;) (mut i32) i32.const 1048576)
    (global (;1;) (mut i32) i32.const 1048576)
    (global (;2;) (mut i32) i32.const 0)
    (global (;3;) i32 i32.const 1048688)
    (global (;4;) i32 i32.const 4)
    (global (;5;) i32 i32.const 1048688)
    (global (;6;) i32 i32.const 4)
    (global (;7;) i32 i32.const 1048688)
    (global (;8;) i32 i32.const 4)
    (global (;9;) i32 i32.const 1049096)
    (global (;10;) i32 i32.const 4)
    (global (;11;) (mut i32) i32.const 1049104)
    (global (;12;) (mut i32) i32.const 1114112)
    (export "__stack_pointer" (global 0))
    (export "__stack_high" (global 1))
    (export "__stack_low" (global 2))
    (export "foo:memory_base" (global 3))
    (export "foo:table_base" (global 4))
    (export "libc.so:memory_base" (global 5))
    (export "libc.so:table_base" (global 6))
    (export "libdl.so:memory_base" (global 7))
    (export "libdl.so:table_base" (global 8))
    (export "wit-component:stubs:memory_base" (global 9))
    (export "wit-component:stubs:table_base" (global 10))
    (export "__heap_base" (global 11))
    (export "__heap_end" (global 12))
    (export "__indirect_function_table" (table 0))
    (export "memory" (memory 0))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core module $libc.so (;1;)
    (@dylink.0
      (mem-info (memory 0 4))
    )
    (type (;0;) (func))
    (type (;1;) (func (param i32) (result i32)))
    (import "GOT.mem" "__heap_base" (global $__heap_base (;0;) (mut i32)))
    (import "GOT.mem" "__heap_end" (global $__heap_end (;1;) (mut i32)))
    (global $errno (;2;) i32 i32.const 0)
    (global $heap (;3;) (mut i32) i32.const 0)
    (export "malloc" (func $malloc))
    (export "abort" (func $abort))
    (export "errno" (global $errno))
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
  (core module $wit-component:stubs (;2;)
    (type (;0;) (func (param i32 i32 i32) (result i32)))
    (type (;1;) (func (param i32) (result i32)))
    (export "memcmp" (func 0))
    (export "strlen" (func 1))
    (func (;0;) (type 0) (param i32 i32 i32) (result i32)
      unreachable
    )
    (func (;1;) (type 1) (param i32) (result i32)
      unreachable
    )
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core module $libdl.so (;3;)
    (@dylink.0
      (mem-info (memory 408 2))
      (needed "libc.so")
    )
    (type (;0;) (func (param i32) (result i32)))
    (type (;1;) (func (param i32 i32 i32) (result i32)))
    (type (;2;) (func))
    (type (;3;) (func (result i32)))
    (type (;4;) (func (param i32 i32) (result i32)))
    (type (;5;) (func (param i32)))
    (import "env" "memory" (memory (;0;) 1))
    (import "env" "__indirect_function_table" (table (;0;) 0 funcref))
    (import "env" "__memory_base" (global (;0;) i32))
    (import "env" "__table_base" (global (;1;) i32))
    (import "env" "strlen" (func (;0;) (type 0)))
    (import "env" "memcmp" (func (;1;) (type 1)))
    (global (;2;) i32 i32.const 140)
    (export "_initialize" (func 4))
    (export "dlclose" (func 5))
    (export "dlerror" (func 7))
    (export "dlopen" (func 8))
    (export "dlsym" (func 9))
    (export "__wasm_set_libraries" (func 10))
    (export "_ZN17compiler_builtins4math9libm_math7generic4sqrt9RSQRT_TAB17hb104bb8b888712f2E" (global 2))
    (start 3)
    (func (;2;) (type 2))
    (func (;3;) (type 2)
      i32.const 396
      global.get 0
      i32.add
      i32.const 0
      i32.const 12
      memory.fill
    )
    (func (;4;) (type 2)
      block ;; label = @1
        global.get 0
        i32.const 396
        i32.add
        i32.load
        i32.eqz
        br_if 0 (;@1;)
        unreachable
      end
      global.get 0
      i32.const 396
      i32.add
      i32.const 1
      i32.store
      call 2
    )
    (func (;5;) (type 0) (param i32) (result i32)
      (local i32 i32 i32)
      block ;; label = @1
        global.get 0
        i32.const 404
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@1;)
        local.get 1
        i32.load
        i32.const 1
        i32.add
        local.set 2
        i32.const -16
        local.set 3
        block ;; label = @2
          loop ;; label = @3
            local.get 2
            i32.const -1
            i32.add
            local.tee 2
            i32.eqz
            br_if 1 (;@2;)
            local.get 1
            i32.load offset=4
            local.get 3
            i32.const 16
            i32.add
            local.tee 3
            i32.add
            local.get 0
            i32.ne
            br_if 0 (;@3;)
          end
          i32.const 0
          return
        end
        global.get 0
        local.tee 2
        i32.const 400
        i32.add
        local.get 2
        i32.const 0
        i32.add
        i32.store
        i32.const -1
        return
      end
      call 6
      unreachable
    )
    (func (;6;) (type 2)
      unreachable
    )
    (func (;7;) (type 3) (result i32)
      (local i32 i32)
      global.get 0
      i32.const 400
      i32.add
      local.tee 0
      i32.load
      local.set 1
      local.get 0
      i32.const 0
      i32.store
      local.get 1
    )
    (func (;8;) (type 4) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        block ;; label = @2
          block ;; label = @3
            block ;; label = @4
              global.get 0
              i32.const 404
              i32.add
              i32.load
              local.tee 2
              i32.eqz
              br_if 0 (;@4;)
              global.get 0
              local.set 3
              block ;; label = @5
                local.get 1
                i32.const -260
                i32.and
                i32.eqz
                br_if 0 (;@5;)
                local.get 3
                i32.const 58
                i32.add
                local.set 4
                br 3 (;@2;)
              end
              local.get 0
              call 0
              local.set 1
              local.get 2
              i32.load
              local.set 3
              global.get 0
              i32.const 23
              i32.add
              local.set 4
              local.get 2
              i32.load offset=4
              local.set 5
              i32.const 0
              local.set 2
              block ;; label = @5
                local.get 3
                br_table 3 (;@2;) 2 (;@3;) 0 (;@5;)
              end
              i32.const 0
              local.set 2
              loop ;; label = @5
                local.get 3
                i32.const 1
                i32.shr_u
                local.tee 6
                local.get 2
                i32.add
                local.set 4
                local.get 2
                local.get 4
                local.get 5
                local.get 4
                i32.const 4
                i32.shl
                i32.add
                local.tee 7
                i32.const 4
                i32.add
                i32.load
                local.get 0
                local.get 7
                i32.load
                local.tee 7
                local.get 1
                local.get 7
                local.get 1
                i32.lt_u
                select
                call 1
                local.tee 8
                local.get 7
                local.get 1
                i32.sub
                local.get 8
                select
                i32.const 0
                i32.gt_s
                select
                local.set 2
                local.get 3
                local.get 6
                i32.sub
                local.tee 3
                i32.const 1
                i32.gt_u
                br_if 0 (;@5;)
                br 2 (;@3;)
              end
            end
            call 6
            unreachable
          end
          local.get 5
          local.get 2
          i32.const 4
          i32.shl
          i32.add
          local.tee 3
          i32.const 4
          i32.add
          i32.load
          local.get 0
          local.get 3
          i32.load
          local.tee 2
          local.get 1
          local.get 2
          local.get 1
          i32.lt_u
          select
          call 1
          local.set 4
          global.get 0
          local.set 7
          local.get 4
          local.get 2
          local.get 1
          i32.sub
          local.get 4
          select
          i32.eqz
          br_if 1 (;@1;)
          local.get 7
          i32.const 23
          i32.add
          local.set 4
        end
        global.get 0
        i32.const 400
        i32.add
        local.get 4
        i32.store
        i32.const 0
        local.set 3
      end
      local.get 3
    )
    (func (;9;) (type 4) (param i32 i32) (result i32)
      (local i32 i32 i32 i32 i32 i32 i32)
      block ;; label = @1
        local.get 0
        i32.const 1
        i32.add
        i32.const 1
        i32.gt_u
        br_if 0 (;@1;)
        global.get 0
        local.tee 2
        i32.const 400
        i32.add
        local.get 2
        i32.const 89
        i32.add
        i32.store
        i32.const 0
        return
      end
      block ;; label = @1
        block ;; label = @2
          global.get 0
          i32.const 404
          i32.add
          i32.load
          local.tee 3
          i32.eqz
          br_if 0 (;@2;)
          local.get 3
          i32.load
          i32.const 1
          i32.add
          local.set 2
          i32.const -16
          local.set 4
          loop ;; label = @3
            local.get 2
            i32.const -1
            i32.add
            local.tee 2
            i32.eqz
            br_if 2 (;@1;)
            local.get 3
            i32.load offset=4
            local.get 4
            i32.const 16
            i32.add
            local.tee 4
            i32.add
            local.get 0
            i32.ne
            br_if 0 (;@3;)
          end
          local.get 1
          call 0
          local.set 2
          local.get 0
          i32.load offset=12
          local.set 5
          i32.const 0
          local.set 4
          block ;; label = @3
            block ;; label = @4
              block ;; label = @5
                block ;; label = @6
                  local.get 0
                  i32.load offset=8
                  local.tee 0
                  br_table 2 (;@4;) 1 (;@5;) 0 (;@6;)
                end
                i32.const 0
                local.set 4
                loop ;; label = @6
                  local.get 0
                  i32.const 1
                  i32.shr_u
                  local.tee 6
                  local.get 4
                  i32.add
                  local.set 3
                  local.get 4
                  local.get 3
                  local.get 5
                  local.get 3
                  i32.const 12
                  i32.mul
                  i32.add
                  local.tee 7
                  i32.const 4
                  i32.add
                  i32.load
                  local.get 1
                  local.get 7
                  i32.load
                  local.tee 7
                  local.get 2
                  local.get 7
                  local.get 2
                  i32.lt_u
                  select
                  call 1
                  local.tee 8
                  local.get 7
                  local.get 2
                  i32.sub
                  local.get 8
                  select
                  i32.const 0
                  i32.gt_s
                  select
                  local.set 4
                  local.get 0
                  local.get 6
                  i32.sub
                  local.tee 0
                  i32.const 1
                  i32.gt_u
                  br_if 0 (;@6;)
                end
              end
              local.get 5
              local.get 4
              i32.const 12
              i32.mul
              i32.add
              local.tee 0
              i32.const 4
              i32.add
              i32.load
              local.get 1
              local.get 0
              i32.load
              local.tee 4
              local.get 2
              local.get 4
              local.get 2
              i32.lt_u
              select
              call 1
              local.tee 3
              local.get 4
              local.get 2
              i32.sub
              local.get 3
              select
              i32.eqz
              br_if 1 (;@3;)
            end
            global.get 0
            local.tee 2
            i32.const 400
            i32.add
            local.get 2
            i32.const 41
            i32.add
            i32.store
            i32.const 0
            return
          end
          local.get 0
          i32.load offset=8
          return
        end
        call 6
        unreachable
      end
      global.get 0
      local.tee 2
      i32.const 400
      i32.add
      local.get 2
      i32.const 0
      i32.add
      i32.store
      i32.const 0
    )
    (func (;10;) (type 5) (param i32)
      global.get 0
      i32.const 404
      i32.add
      local.get 0
      i32.store
    )
    (data (;0;) (global.get 0) "invalid library handle\00library not found\00symbol not found\00dlopen flags not yet supported\00dlsym RTLD_NEXT and RTLD_DEFAULT not yet supported\00Q\b4\f0\b2\96\b1D\b0\f9\ae\b6\ady\acC\ab\14\aa\eb\a8\c8\a7\aa\a6\92\a5\80\a4s\a3k\a2h\a1j\a0p\9f{\9e\8a\9d\9d\9c\b5\9b\d1\9a\f0\99\13\99:\98e\97\93\96\c4\95\f8\940\94k\93\a9\92\ea\91.\91u\90\be\8f\0a\8fY\8e\aa\8d\fe\8cT\8c\ac\8b\07\8bd\8a\c4\89%\89\89\88\ee\87V\87\c0\86+\86\99\85\08\85y\84\ec\83a\83\d8\82P\82\c9\81E\81\c2\80@\80\02\ff\0e\fd%\fbG\f9s\f7\aa\f5\ea\f34\f2\87\f0\e3\eeG\ed\b3\eb'\ea\a3\e8'\e7\b2\e5C\e4\dc\e2z\e1 \e0\cb\de}\dd4\dc\f1\da\b3\d9{\d8H\d7\1a\d6\f1\d4\cd\d3\ad\d2\92\d1{\d0i\cf[\ceQ\cdJ\ccH\cbJ\caO\c9X\c8d\c7t\c6\87\c5\9d\c4\b7\c3\d4\c2\f4\c1\16\c1<\c0e\bf\90\be\be\bd\ef\bc#\bcY\bb\91\ba\cc\b9\0a\b9J\b8\8c\b7\d0\b6\17\b6`\b5")
  )
  (core module $foo (;4;)
    (@dylink.0
      (mem-info (memory 0 4))
    )
    (type (;0;) (func (param i32) (result i32)))
    (type (;1;) (func (param i32 i32) (result i32)))
    (import "test:test/test" "foo" (func $import_foo (;0;) (type 0)))
    (import "env" "dlopen" (func $dlopen (;1;) (type 1)))
    (global $what (;0;) i32 i32.const 42)
    (export "test:test/test#foo" (func $foo))
    (export "bar" (func $foo))
    (export "baz" (func $foo))
    (export "what" (global $what))
    (func $foo (;2;) (type 0) (param i32) (result i32)
      unreachable
    )
  )
  (core module $__init (;5;)
    (type (;0;) (func))
    (type (;1;) (func (param i32)))
    (type (;2;) (func (param i32) (result i32)))
    (type (;3;) (func (param i32) (result i32)))
    (type (;4;) (func (param i32) (result i32)))
    (import "env" "memory" (memory (;0;) 0))
    (import "env" "__indirect_function_table" (table (;0;) 0 funcref))
    (import "env" "foo:memory_base" (global (;0;) i32))
    (import "foo" "what" (global (;1;) i32))
    (import "libdl.so" "_initialize" (func (;0;) (type 0)))
    (import "libdl.so" "__wasm_set_libraries" (func (;1;) (type 1)))
    (import "foo" "bar" (func (;2;) (type 2)))
    (import "foo" "baz" (func (;3;) (type 3)))
    (import "foo" "test:test/test#foo" (func (;4;) (type 4)))
    (start 5)
    (elem (;0;) (i32.const 1) func 2 3 4)
    (elem (;1;) (i32.const 4) func)
    (func (;5;) (type 0)
      i32.const 1048656
      global.get 0
      global.get 1
      i32.add
      i32.store
      call 0
      i32.const 1048676
      call 1
    )
    (data (;0;) (i32.const 1048576) "foo\00bar\00baz\00test:test/test#foo\00\00what\03\00\00\00\04\00\10\00\01\00\00\00\03\00\00\00\08\00\10\00\02\00\00\00\12\00\00\00\0c\00\10\00\03\00\00\00\04\00\00\00 \00\10\00\00\00\00\00\03\00\00\00\00\00\10\00\04\00\00\00$\00\10\00\01\00\00\00T\00\10\00")
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core instance $main (;0;) (instantiate $main))
  (alias core export $main "memory" (core memory $memory (;0;)))
  (alias core export $main "__heap_base" (core global $__heap_base (;0;)))
  (alias core export $main "__heap_end" (core global $__heap_end (;1;)))
  (core instance $GOT.mem (;1;)
    (export "__heap_base" (global $__heap_base))
    (export "__heap_end" (global $__heap_end))
  )
  (core instance $libc.so (;2;) (instantiate $libc.so
      (with "GOT.mem" (instance $GOT.mem))
    )
  )
  (core instance $wit-component:stubs (;3;) (instantiate $wit-component:stubs))
  (alias core export $main "__indirect_function_table" (core table $__indirect_function_table (;0;)))
  (alias core export $main "libdl.so:memory_base" (core global $libdl.so:memory_base (;2;)))
  (alias core export $main "libdl.so:table_base" (core global $libdl.so:table_base (;3;)))
  (alias core export $wit-component:stubs "strlen" (core func $strlen (;0;)))
  (alias core export $wit-component:stubs "memcmp" (core func $memcmp (;1;)))
  (core instance $env (;4;)
    (export "memory" (memory $memory))
    (export "__indirect_function_table" (table $__indirect_function_table))
    (export "__memory_base" (global $libdl.so:memory_base))
    (export "__table_base" (global $libdl.so:table_base))
    (export "strlen" (func $strlen))
    (export "memcmp" (func $memcmp))
  )
  (core instance $libdl.so (;5;) (instantiate $libdl.so
      (with "env" (instance $env))
    )
  )
  (alias export $test:test/test "foo" (func $foo (;0;)))
  (core func $foo (;2;) (canon lower (func $foo)))
  (core instance $test:test/test (;6;)
    (export "foo" (func $foo))
  )
  (alias core export $libdl.so "dlopen" (core func $dlopen (;3;)))
  (core instance $"#core-instance7 env" (@name "env") (;7;)
    (export "dlopen" (func $dlopen))
  )
  (core instance $foo (;8;) (instantiate $foo
      (with "test:test/test" (instance $test:test/test))
      (with "env" (instance $"#core-instance7 env"))
    )
  )
  (core instance $__init (;9;) (instantiate $__init
      (with "env" (instance $main))
      (with "foo" (instance $foo))
      (with "libdl.so" (instance $libdl.so))
    )
  )
  (type (;1;) (func (param "v" s32) (result s32)))
  (alias core export $foo "test:test/test#foo" (core func $test:test/test#foo (;4;)))
  (func $"#func1 foo" (@name "foo") (;1;) (type 1) (canon lift (core func $test:test/test#foo)))
  (component $test:test/test-shim-component (;0;)
    (type (;0;) (func (param "v" s32) (result s32)))
    (import "import-func-foo" (func (;0;) (type 0)))
    (type (;1;) (func (param "v" s32) (result s32)))
    (export (;1;) "foo" (func 0) (func (type 1)))
  )
  (instance $test:test/test-shim-instance (;1;) (instantiate $test:test/test-shim-component
      (with "import-func-foo" (func $"#func1 foo"))
    )
  )
  (export $"#instance2 test:test/test" (@name "test:test/test") (;2;) "test:test/test" (instance $test:test/test-shim-instance))
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
