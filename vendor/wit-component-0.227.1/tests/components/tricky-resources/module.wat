(module
  (import "[export]foo:bar/a" "[resource-drop]r" (func (param i32)))
  (import "[export]foo:bar/a" "[resource-rep]r" (func (param i32) (result i32)))

  (func (export "some-name#f") (result i32)
    unreachable)
)
