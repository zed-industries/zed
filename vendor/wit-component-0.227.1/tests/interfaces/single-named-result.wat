(component
  (type (;0;)
    (component
      (type (;0;)
        (instance
          (type (;0;) (func (result "a" u32)))
          (export (;0;) "a" (func (type 0)))
        )
      )
      (export (;0;) "foo:foo/foo" (instance (type 0)))
    )
  )
  (export (;1;) "foo" (type 0))
  (@custom "package-docs" "\00{}")
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
