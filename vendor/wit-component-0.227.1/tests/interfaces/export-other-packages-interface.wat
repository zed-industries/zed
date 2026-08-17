(component
  (type (;0;)
    (component
      (type (;0;)
        (component
          (type (;0;)
            (instance
              (type (;0;) u8)
              (export (;1;) "t" (type (eq 0)))
            )
          )
          (export (;0;) "foo:the-dep/the-interface" (instance (type 0)))
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
