;; This example represents adapting modules which use an early version of the
;; canonical ABI, back when `cabi_realloc` was named `canonical_abi_realloc` and
;; had a friend named `canonical_abi_free`.  Such modules are pretty easy to
;; adapt since the adapter can use the main module's allocator for both lowering
;; and post-return functions.
;;
;; See https://github.com/fermyon/spin-componentize for a real-world example.

(module
  (import "__main_module__" "canonical_abi_realloc" (func $realloc (param i32 i32 i32 i32) (result i32)))
  (import "__main_module__" "canonical_abi_free" (func $free (param i32 i32 i32)))
  (import "env" "memory" (memory 0))
  (global $__stack_pointer (mut i32) i32.const 0)
  (global $allocation_state (mut i32) i32.const 0)

  (func (export "foo:foo/new#foo") (result i32)
    ;; This is a dummy, non-working implementation, just to make gc.rs do what
    ;; we want, which is to treat this adapter as if it uses the main module's
    ;; allocator to allocate and free memory.

    global.get $__stack_pointer
    global.get $allocation_state
    (call $realloc (i32.const 0) (i32.const 0) (i32.const 0) (i32.const 0))
    unreachable
  )

  (func (export "cabi_post_foo:foo/new#foo") (param i32)
    ;; another dummy implementation

    (call $free (i32.const 0) (i32.const 0) (i32.const 0))
    unreachable
  )
)
