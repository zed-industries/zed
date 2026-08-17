(component
  (type (;0;)
    (component
      (type (;0;)
        (instance
          (type (;0;) (record (field "a" u32)))
          (export (;1;) "my-struct" (type (eq 0)))
          (type (;2;) (func (param "a" 1) (result string)))
          (export (;0;) "my-function" (func (type 2)))
        )
      )
      (export (;0;) "foo:foo/foo" (instance (type 0)))
    )
  )
  (export (;1;) "foo" (type 0))
  (type (;2;)
    (component
      (type (;0;)
        (component
          (type (;0;)
            (instance
              (type (;0;) (record (field "a" u32)))
              (export (;1;) "my-struct" (type (eq 0)))
              (type (;2;) (func (param "a" 1) (result string)))
              (export (;0;) "my-function" (func (type 2)))
            )
          )
          (export (;0;) "foo:foo/foo" (instance (type 0)))
        )
      )
      (export (;0;) "foo:foo/export-foo" (component (type 0)))
    )
  )
  (export (;3;) "export-foo" (type 2))
  (@custom "package-docs" "\00{}")
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
