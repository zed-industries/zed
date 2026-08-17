(component
  (type (;0;)
    (component
      (type (;0;)
        (instance
          (type (;0;) u32)
          (export (;1;) "type" (type (eq 0)))
          (export (;2;) "world" (type (eq 1)))
          (type (;3;) (record (field "variant" 2)))
          (export (;4;) "record" (type (eq 3)))
        )
      )
      (export (;0;) "foo:foo/interface" (instance (type 0)))
    )
  )
  (export (;1;) "interface" (type 0))
  (@custom "package-docs" "\00{}")
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
