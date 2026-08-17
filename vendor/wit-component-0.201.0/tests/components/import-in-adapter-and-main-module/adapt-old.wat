(module
  (import "foo:shared-dependency/doc" "f1" (func $f1))
  (import "foo:shared-dependency/doc" "f3" (func $f3))

  (import "foo:shared-dependency/doc" "g1" (func $g1 (param i32)))
  (import "foo:shared-dependency/doc" "g3" (func $g3 (param i32)))

  (import "foo:shared-dependency/doc" "unused-in-adapter" (func))
  (import "env" "memory" (memory 0))

  (import "adapter-dep" "foo" (func $foo (result i32)))

  (func (export "adapter-f1")
    call $f1
    call $f3
    (call $g1 (i32.const 0))
    (call $g3 (i32.const 0))
    (drop (call $foo))
  )

  (func (export "cabi_import_realloc") (param i32 i32 i32 i32) (result i32)
    unreachable)
)
