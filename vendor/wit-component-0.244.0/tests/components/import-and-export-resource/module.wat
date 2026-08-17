(module
  (import "foo:bar/foo" "[resource-drop]a" (func (param i32)))
  (import "[export]foo:bar/foo" "[resource-drop]a" (func (param i32)))
  (import "[export]foo:bar/foo" "[resource-rep]a" (func (param i32) (result i32)))
  (import "[export]foo:bar/foo" "[resource-new]a" (func (param i32) (result i32)))
)
