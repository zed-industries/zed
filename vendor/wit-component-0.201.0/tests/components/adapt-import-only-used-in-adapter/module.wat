(module
  (import "$root" "foo" (func (param i32 i32)))
  (func (export "bar") unreachable)
  (memory (export "memory") 1)
)
