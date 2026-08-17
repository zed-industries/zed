(component
  (type (;0;)
    (component
      (type (;0;)
        (instance)
      )
      (export (;0;) "foo:foo/bar" (instance (type 0)))
    )
  )
  (export (;1;) "bar" (type 0))
  (type (;2;)
    (component
      (type (;0;)
        (instance)
      )
      (export (;0;) "foo:foo/foo" (instance (type 0)))
    )
  )
  (export (;3;) "foo" (type 2))
  (@custom "package-docs" "\00{}")
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
