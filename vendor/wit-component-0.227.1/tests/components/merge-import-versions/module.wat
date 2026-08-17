(module
  (import "a:b/c@0.1.0" "[constructor]r" (func (result i32)))
  (import "a:b/c@0.1.0" "[resource-drop]r" (func (param i32)))
  (import "a:b/c@0.1.0" "x" (func (param i32 i32)))
  (import "a:b/c@0.1.0" "x2" (func (param i32 i32)))

  (import "a:b/c@0.1.1" "[constructor]r" (func (result i32)))
  (import "a:b/c@0.1.1" "[resource-drop]r" (func (param i32)))
  (import "a:b/c@0.1.1" "[method]r.x" (func (param i32)))
  (import "a:b/c@0.1.1" "x" (func (param i32 i32)))
  (import "a:b/c@0.1.1" "x2" (func (param i32 i32)))
  (import "a:b/c@0.1.1" "y" (func))

  (memory (export "memory") 1)
)
