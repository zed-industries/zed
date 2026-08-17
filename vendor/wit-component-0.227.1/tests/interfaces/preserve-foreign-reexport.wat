(component
  (type (;0;)
    (component
      (type (;0;)
        (component
          (type (;0;)
            (instance
              (type (;0;) (record (field "f" u8)))
              (export (;1;) "foo" (type (eq 0)))
              (export (;2;) "bar" (type (eq 1)))
            )
          )
          (export (;0;) "foo:my-dep/my-interface" (instance (type 0)))
        )
      )
      (export (;0;) "foo:foo/foo" (component (type 0)))
    )
  )
  (export (;1;) "foo" (type 0))
  (@custom "package-docs" "\00{}")
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
