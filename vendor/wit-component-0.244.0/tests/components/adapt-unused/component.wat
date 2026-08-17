(component
  (core module $main (;0;)
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (core instance $main (;0;) (instantiate $main))
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
