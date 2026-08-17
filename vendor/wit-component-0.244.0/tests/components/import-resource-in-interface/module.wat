(module
  (import "foo" "[constructor]a" (func (result i32)))
  (import "foo" "[static]a.other-new" (func (result i32)))
  (import "foo" "[resource-drop]a" (func (param i32)))
)
