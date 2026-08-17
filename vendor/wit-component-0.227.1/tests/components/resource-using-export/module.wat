(module
  (import "[export]foo:bar/foo" "[resource-new]r" (func $new (param i32) (result i32)))
  (func (export "anon#f") (result i32)
    (call $new (i32.const 100))
  )
)
