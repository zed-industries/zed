(component
  (type $ty-foo:foo/foo (;0;)
    (instance
      (type (;0;) (func))
      (export (;0;) "the-func" (func (type 0)))
    )
  )
  (import "foo:foo/foo" (instance $foo:foo/foo (;0;) (type $ty-foo:foo/foo)))
  (core module $main (;0;)
    (type (;0;) (func))
    (import "foo:foo/foo" "the-func" (func (;0;) (type 0)))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (alias export $foo:foo/foo "the-func" (func $the-func (;0;)))
  (core func $the-func (;0;) (canon lower (func $the-func)))
  (core instance $foo:foo/foo (;0;)
    (export "the-func" (func $the-func))
  )
  (core instance $main (;1;) (instantiate $main
      (with "foo:foo/foo" (instance $foo:foo/foo))
    )
  )
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
