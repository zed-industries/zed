(module
  (import "foo" "[constructor]a" (func (param i32)))
  (import "foo" "[resource-drop]a" (func (param i32)))
  (memory (export "memory") 1)
)
