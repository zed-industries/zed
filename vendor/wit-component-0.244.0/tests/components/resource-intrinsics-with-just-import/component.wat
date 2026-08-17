(component
  (type $ty-foo (;0;)
    (instance
      (export (;0;) "a" (type (sub resource)))
    )
  )
  (import "foo" (instance $foo (;0;) (type $ty-foo)))
  (core module $main (;0;)
    (type (;0;) (func (param i32)))
    (import "foo" "[resource-drop]a" (func (;0;) (type 0)))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (alias export $foo "a" (type $a (;1;)))
  (core func $resource.drop (;0;) (canon resource.drop $a))
  (core instance $foo (;0;)
    (export "[resource-drop]a" (func $resource.drop))
  )
  (core instance $main (;1;) (instantiate $main
      (with "foo" (instance $foo))
    )
  )
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
