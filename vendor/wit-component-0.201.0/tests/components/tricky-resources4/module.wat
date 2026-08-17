(module
  (import "[export]foo:bar/name" "[resource-new]name"
    (func $new (param i32) (result i32)))
  (func (export "foo:bar/name#foo") (result i32)
    (call $new (i32.const 100)))
)
