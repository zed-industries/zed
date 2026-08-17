(module
  (import "[export]foo" "[resource-new]a" (func $new (param i32) (result i32)))
  (import "[export]foo" "[resource-rep]a" (func (param i32) (result i32)))
  (import "[export]foo" "[resource-drop]a" (func (param i32)))

  (func (export "foo#[constructor]a") (result i32)
    (call $new (i32.const 100))
  )
  (func (export "foo#[static]a.other-new") (result i32)
    (call $new (i32.const 200))
  )
  (func (export "foo#[dtor]a") (param i32)
    ;; ...
  )
)
