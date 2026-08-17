(component
  (core module (;0;)
    (type (;0;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;1;) (func (param i32 i32 i32)))
    (func (;0;) (type 0) (param i32 i32 i32 i32) (result i32)
      unreachable
    )
    (func (;1;) (type 1) (param i32 i32 i32)
      unreachable
    )
    (memory (;0;) 1)
    (export "canonical_abi_realloc" (func 0))
    (export "canonical_abi_free" (func 1))
    (export "memory" (memory 0))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (core module (;1;)
    (type (;0;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;1;) (func (param i32 i32 i32)))
    (type (;2;) (func (result i32)))
    (type (;3;) (func (param i32)))
    (type (;4;) (func))
    (import "__main_module__" "canonical_abi_realloc" (func $realloc (;0;) (type 0)))
    (import "__main_module__" "canonical_abi_free" (func $free (;1;) (type 1)))
    (func (;2;) (type 2) (result i32)
      call $allocate_stack
      global.get $__stack_pointer
      global.get $allocation_state
      i32.const 0
      i32.const 0
      i32.const 0
      i32.const 0
      call $realloc
      unreachable
    )
    (func (;3;) (type 3) (param i32)
      call $allocate_stack
      i32.const 0
      i32.const 0
      i32.const 0
      call $free
      unreachable
    )
    (func $allocate_stack (;4;) (type 4)
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
        call $realloc
        i32.const 65536
        i32.add
        global.set $__stack_pointer
        i32.const 2
        global.set $allocation_state
      end
    )
    (global $__stack_pointer (;0;) (mut i32) i32.const 0)
    (global $allocation_state (;1;) (mut i32) i32.const 0)
    (export "foo:foo/new#foo" (func 2))
    (export "cabi_post_foo:foo/new#foo" (func 3))
  )
  (core instance (;0;) (instantiate 0))
  (alias core export 0 "memory" (core memory (;0;)))
  (alias core export 0 "canonical_abi_realloc" (core func (;0;)))
  (alias core export 0 "canonical_abi_realloc" (core func (;1;)))
  (alias core export 0 "canonical_abi_free" (core func (;2;)))
  (core instance (;1;)
    (export "canonical_abi_realloc" (func 1))
    (export "canonical_abi_free" (func 2))
  )
  (core instance (;2;) (instantiate 1
      (with "__main_module__" (instance 1))
    )
  )
  (type (;0;) (func (result string)))
  (alias core export 2 "foo:foo/new#foo" (core func (;3;)))
  (alias core export 2 "cabi_post_foo:foo/new#foo" (core func (;4;)))
  (func (;0;) (type 0) (canon lift (core func 3) (memory 0) string-encoding=utf8 (post-return 4)))
  (component (;0;)
    (type (;0;) (func (result string)))
    (import "import-func-foo" (func (;0;) (type 0)))
    (type (;1;) (func (result string)))
    (export (;1;) "foo" (func 0) (func (type 1)))
  )
  (instance (;0;) (instantiate 0
      (with "import-func-foo" (func 0))
    )
  )
  (export (;1;) "foo:foo/new" (instance 0))
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
