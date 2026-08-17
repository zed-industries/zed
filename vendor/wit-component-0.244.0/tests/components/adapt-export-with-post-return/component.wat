(component
  (core module $main (;0;)
    (type (;0;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;1;) (func (param i32 i32 i32)))
    (memory (;0;) 1)
    (export "canonical_abi_realloc" (func 0))
    (export "canonical_abi_free" (func 1))
    (export "memory" (memory 0))
    (func (;0;) (type 0) (param i32 i32 i32 i32) (result i32)
      unreachable
    )
    (func (;1;) (type 1) (param i32 i32 i32)
      unreachable
    )
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (core module $wit-component:adapter:old (;1;)
    (type (;0;) (func (param i32 i32 i32 i32) (result i32)))
    (type (;1;) (func (param i32 i32 i32)))
    (type (;2;) (func (result i32)))
    (type (;3;) (func (param i32)))
    (type (;4;) (func))
    (import "__main_module__" "canonical_abi_realloc" (func $realloc (;0;) (type 0)))
    (import "__main_module__" "canonical_abi_free" (func $free (;1;) (type 1)))
    (global $__stack_pointer (;0;) (mut i32) i32.const 0)
    (global $allocation_state (;1;) (mut i32) i32.const 0)
    (export "foo:foo/new#foo" (func 2))
    (export "cabi_post_foo:foo/new#foo" (func 3))
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
  )
  (core instance $main (;0;) (instantiate $main))
  (alias core export $main "memory" (core memory $memory (;0;)))
  (alias core export $main "canonical_abi_realloc" (core func $canonical_abi_realloc (;0;)))
  (alias core export $main "canonical_abi_free" (core func $canonical_abi_free (;1;)))
  (core instance $__main_module__ (;1;)
    (export "canonical_abi_realloc" (func $canonical_abi_realloc))
    (export "canonical_abi_free" (func $canonical_abi_free))
  )
  (core instance $old (;2;) (instantiate $wit-component:adapter:old
      (with "__main_module__" (instance $__main_module__))
    )
  )
  (type (;0;) (func (result string)))
  (alias core export $old "foo:foo/new#foo" (core func $foo:foo/new#foo (;2;)))
  (alias core export $old "cabi_post_foo:foo/new#foo" (core func $cabi_post_foo:foo/new#foo (;3;)))
  (func $foo (;0;) (type 0) (canon lift (core func $foo:foo/new#foo) (memory $memory) string-encoding=utf8 (post-return $cabi_post_foo:foo/new#foo)))
  (component $foo:foo/new-shim-component (;0;)
    (type (;0;) (func (result string)))
    (import "import-func-foo" (func (;0;) (type 0)))
    (type (;1;) (func (result string)))
    (export (;1;) "foo" (func 0) (func (type 1)))
  )
  (instance $foo:foo/new-shim-instance (;0;) (instantiate $foo:foo/new-shim-component
      (with "import-func-foo" (func $foo))
    )
  )
  (export $foo:foo/new (;1;) "foo:foo/new" (instance $foo:foo/new-shim-instance))
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
