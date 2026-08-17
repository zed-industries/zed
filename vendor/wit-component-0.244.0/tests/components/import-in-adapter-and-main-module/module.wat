(module
  (import "foo:shared-dependency/doc" "f1" (func))
  (import "foo:shared-dependency/doc" "f2" (func))

  (import "foo:shared-dependency/doc" "g1" (func (param i32)))
  (import "foo:shared-dependency/doc" "g2" (func (param i32)))

  (import "old" "adapter-f1" (func))

  (import "main-dep" "foo" (func (result i32)))

  (memory (export "memory") 1)
  (func (export "cabi_realloc") (param i32 i32 i32 i32) (result i32)
    unreachable)
)
