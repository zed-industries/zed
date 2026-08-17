(component
  (type $ty-foo:foo/foo (;0;)
    (instance
      (type (;0;) (func (param "x" bool)))
      (export (;0;) "name" (func (type 0)))
    )
  )
  (import "foo:foo/foo" (instance $foo:foo/foo (;0;) (type $ty-foo:foo/foo)))
  (core module $main (;0;)
    (type (;0;) (func (param i32)))
    (import "foo:foo/foo" "name" (func (;0;) (type 0)))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (alias export $foo:foo/foo "name" (func $name (;0;)))
  (core func $name (;0;) (canon lower (func $name)))
  (core instance $foo:foo/foo (;0;)
    (export "name" (func $name))
  )
  (core instance $main (;1;) (instantiate $main
      (with "foo:foo/foo" (instance $foo:foo/foo))
    )
  )
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
