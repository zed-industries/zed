(component
  (type $ty-foo:foo/bar (;0;)
    (instance
      (type (;0;) (func (param "s" string) (result string)))
      (export (;0;) "foo" (func (type 0)))
    )
  )
  (import "foo:foo/bar" (instance $foo:foo/bar (;0;) (type $ty-foo:foo/bar)))
  (type (;1;) (func (param "s" string) (result string)))
  (import "foo" (func $foo (;0;) (type 1)))
  (core module $main (;0;)
    (type (;0;) (func (param i32 i32 i32) (result i32)))
    (type (;1;) (func (param i32 i32 i32)))
    (type (;2;) (func (param i32 i32 i32 i32) (result i32)))
    (import "$root" "[async-lower]foo" (func (;0;) (type 0)))
    (import "foo:foo/bar" "[async-lower]foo" (func (;1;) (type 0)))
    (import "$root" "foo" (func (;2;) (type 1)))
    (import "foo:foo/bar" "foo" (func (;3;) (type 1)))
    (memory (;0;) 1)
    (export "memory" (memory 0))
    (export "cabi_realloc" (func 4))
    (func (;4;) (type 2) (param i32 i32 i32 i32) (result i32)
      unreachable
    )
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (core module $wit-component-shim-module (;1;)
    (type (;0;) (func (param i32 i32 i32) (result i32)))
    (type (;1;) (func (param i32 i32 i32)))
    (table (;0;) 4 4 funcref)
    (export "0" (func $"indirect-$root-[async-lower]foo"))
    (export "1" (func $indirect-$root-foo))
    (export "2" (func $"indirect-foo:foo/bar-[async-lower]foo"))
    (export "3" (func $indirect-foo:foo/bar-foo))
    (export "$imports" (table 0))
    (func $"indirect-$root-[async-lower]foo" (;0;) (type 0) (param i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      i32.const 0
      call_indirect (type 0)
    )
    (func $indirect-$root-foo (;1;) (type 1) (param i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      i32.const 1
      call_indirect (type 1)
    )
    (func $"indirect-foo:foo/bar-[async-lower]foo" (;2;) (type 0) (param i32 i32 i32) (result i32)
      local.get 0
      local.get 1
      local.get 2
      i32.const 2
      call_indirect (type 0)
    )
    (func $indirect-foo:foo/bar-foo (;3;) (type 1) (param i32 i32 i32)
      local.get 0
      local.get 1
      local.get 2
      i32.const 3
      call_indirect (type 1)
    )
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core module $wit-component-fixup (;2;)
    (type (;0;) (func (param i32 i32 i32) (result i32)))
    (type (;1;) (func (param i32 i32 i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "1" (func (;1;) (type 1)))
    (import "" "2" (func (;2;) (type 0)))
    (import "" "3" (func (;3;) (type 1)))
    (import "" "$imports" (table (;0;) 4 4 funcref))
    (elem (;0;) (i32.const 0) func 0 1 2 3)
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core instance $wit-component-shim-instance (;0;) (instantiate $wit-component-shim-module))
  (alias core export $wit-component-shim-instance "0" (core func $"indirect-$root-[async-lower]foo" (;0;)))
  (alias core export $wit-component-shim-instance "1" (core func $indirect-$root-foo (;1;)))
  (core instance $$root (;1;)
    (export "[async-lower]foo" (func $"indirect-$root-[async-lower]foo"))
    (export "foo" (func $indirect-$root-foo))
  )
  (alias core export $wit-component-shim-instance "2" (core func $"indirect-foo:foo/bar-[async-lower]foo" (;2;)))
  (alias core export $wit-component-shim-instance "3" (core func $indirect-foo:foo/bar-foo (;3;)))
  (core instance $foo:foo/bar (;2;)
    (export "[async-lower]foo" (func $"indirect-foo:foo/bar-[async-lower]foo"))
    (export "foo" (func $indirect-foo:foo/bar-foo))
  )
  (core instance $main (;3;) (instantiate $main
      (with "$root" (instance $$root))
      (with "foo:foo/bar" (instance $foo:foo/bar))
    )
  )
  (alias core export $main "memory" (core memory $memory (;0;)))
  (alias core export $wit-component-shim-instance "$imports" (core table $"shim table" (;0;)))
  (alias core export $main "cabi_realloc" (core func $realloc (;4;)))
  (core func $"#core-func5 indirect-$root-[async-lower]foo" (@name "indirect-$root-[async-lower]foo") (;5;) (canon lower (func $foo) (memory $memory) (realloc $realloc) string-encoding=utf8 async))
  (core func $"#core-func6 indirect-$root-foo" (@name "indirect-$root-foo") (;6;) (canon lower (func $foo) (memory $memory) (realloc $realloc) string-encoding=utf8))
  (alias export $foo:foo/bar "foo" (func $"#func1 foo" (@name "foo") (;1;)))
  (core func $"#core-func7 indirect-foo:foo/bar-[async-lower]foo" (@name "indirect-foo:foo/bar-[async-lower]foo") (;7;) (canon lower (func $"#func1 foo") (memory $memory) (realloc $realloc) string-encoding=utf8 async))
  (alias export $foo:foo/bar "foo" (func $"#func2 foo" (@name "foo") (;2;)))
  (core func $"#core-func8 indirect-foo:foo/bar-foo" (@name "indirect-foo:foo/bar-foo") (;8;) (canon lower (func $"#func2 foo") (memory $memory) (realloc $realloc) string-encoding=utf8))
  (core instance $fixup-args (;4;)
    (export "$imports" (table $"shim table"))
    (export "0" (func $"#core-func5 indirect-$root-[async-lower]foo"))
    (export "1" (func $"#core-func6 indirect-$root-foo"))
    (export "2" (func $"#core-func7 indirect-foo:foo/bar-[async-lower]foo"))
    (export "3" (func $"#core-func8 indirect-foo:foo/bar-foo"))
  )
  (core instance $fixup (;5;) (instantiate $wit-component-fixup
      (with "" (instance $fixup-args))
    )
  )
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
