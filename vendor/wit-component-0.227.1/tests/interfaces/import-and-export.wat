(component
  (type (;0;)
    (component
      (type (;0;)
        (instance
          (type (;0;) (func))
          (export (;0;) "foo" (func (type 0)))
        )
      )
      (export (;0;) "foo:foo/foo" (instance (type 0)))
    )
  )
  (export (;1;) "foo" (type 0))
  (type (;2;)
    (component
      (type (;0;)
        (instance
          (type (;0;) (func))
          (export (;0;) "bar" (func (type 0)))
        )
      )
      (export (;0;) "foo:foo/bar" (instance (type 0)))
    )
  )
  (export (;3;) "bar" (type 2))
  (type (;4;)
    (component
      (type (;0;)
        (component
          (type (;0;)
            (instance
              (type (;0;) (func))
              (export (;0;) "foo" (func (type 0)))
            )
          )
          (import "foo:foo/foo" (instance (;0;) (type 0)))
          (type (;1;)
            (instance
              (type (;0;) (func))
              (export (;0;) "bar" (func (type 0)))
            )
          )
          (export (;1;) "foo:foo/bar" (instance (type 1)))
        )
      )
      (export (;0;) "foo:foo/import-and-export" (component (type 0)))
    )
  )
  (export (;5;) "import-and-export" (type 4))
  (@custom "package-docs" "\00{}")
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
