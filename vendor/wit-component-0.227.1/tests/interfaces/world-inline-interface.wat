(component
  (type (;0;)
    (component
      (type (;0;)
        (component
          (type (;0;)
            (instance)
          )
          (import "foo" (instance (;0;) (type 0)))
          (type (;1;)
            (instance)
          )
          (export (;1;) "bar" (instance (type 1)))
        )
      )
      (export (;0;) "foo:foo/has-inline" (component (type 0)))
    )
  )
  (export (;1;) "has-inline" (type 0))
  (@custom "package-docs" "\00{}")
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
