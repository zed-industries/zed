(component
  (type $ty-foo:foo/my-wasi (;0;)
    (instance
      (type (;0;) (func (param "code" u32)))
      (export (;0;) "proc-exit" (func (type 0)))
    )
  )
  (import "foo:foo/my-wasi" (instance $foo:foo/my-wasi (;0;) (type $ty-foo:foo/my-wasi)))
  (type (;1;) (func))
  (import "f" (func $f (;0;) (type 1)))
  (type (;2;) (func (result u32)))
  (import "f2" (func $f2 (;1;) (type 2)))
  (import "g" (func $g (;2;) (type 1)))
  (import "g2" (func $g2 (;3;) (type 2)))
  (core module $main (;0;)
    (type (;0;) (func (param i32)))
    (type (;1;) (func))
    (type (;2;) (func (result i32)))
    (import "wasi-snapshot-preview1" "proc_exit" (func $exit1 (;0;) (type 0)))
    (import "wasi-snapshot-preview1" "proc_exit [v2]" (func $exit2 (;1;) (type 0)))
    (import "cm32p2" "f" (func $f_v1 (;2;) (type 1)))
    (import "cm32p2" "f2" (func $f2 (;3;) (type 2)))
    (import "cm32p2" "f [v2]" (func $f_v2 (;4;) (type 1)))
    (import "cm32p2" "f [v3]" (func $f_v3 (;5;) (type 1)))
    (import "cm32p2" "g" (func $g_v1 (;6;) (type 1)))
    (import "cm32p2" "g [v2]" (func $g_v2 (;7;) (type 1)))
    (import "cm32p2" "g2" (func $g2 (;8;) (type 2)))
    (memory (;0;) 1)
    (export "memory" (memory 0))
    (func (;9;) (type 1)
      call $f_v1
      call $f_v2
      call $f_v3
      call $f2
      drop
      call $g_v1
      call $g_v2
      call $g2
      drop
      i32.const 42
      call $exit1
      i32.const 42
      call $exit2
    )
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (core module $wit-component:adapter:wasi-snapshot-preview1 (;1;)
    (type (;0;) (func (param i32)))
    (import "foo:foo/my-wasi" "proc-exit" (func $proc_exit (;0;) (type 0)))
    (export "proc_exit" (func 1))
    (func (;1;) (type 0) (param i32)
      local.get 0
      call $proc_exit
    )
  )
  (core module $wit-component-shim-module (;2;)
    (type (;0;) (func (param i32)))
    (table (;0;) 1 1 funcref)
    (export "0" (func $adapt-wasi-snapshot-preview1-proc_exit))
    (export "$imports" (table 0))
    (func $adapt-wasi-snapshot-preview1-proc_exit (;0;) (type 0) (param i32)
      local.get 0
      i32.const 0
      call_indirect (type 0)
    )
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core module $wit-component-fixup (;3;)
    (type (;0;) (func (param i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "$imports" (table (;0;) 1 1 funcref))
    (elem (;0;) (i32.const 0) func 0)
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core instance $wit-component-shim-instance (;0;) (instantiate $wit-component-shim-module))
  (alias core export $wit-component-shim-instance "0" (core func $adapt-wasi-snapshot-preview1-proc_exit (;0;)))
  (core instance $wasi-snapshot-preview1 (;1;)
    (export "proc_exit" (func $adapt-wasi-snapshot-preview1-proc_exit))
    (export "proc_exit [v2]" (func $adapt-wasi-snapshot-preview1-proc_exit))
  )
  (core func $f (;1;) (canon lower (func $f)))
  (core func $f2 (;2;) (canon lower (func $f2)))
  (core func $"#core-func3 f" (@name "f") (;3;) (canon lower (func $f)))
  (core func $"#core-func4 f" (@name "f") (;4;) (canon lower (func $f)))
  (core func $g (;5;) (canon lower (func $g)))
  (core func $"#core-func6 g" (@name "g") (;6;) (canon lower (func $g)))
  (core func $g2 (;7;) (canon lower (func $g2)))
  (core instance $cm32p2 (;2;)
    (export "f" (func $f))
    (export "f2" (func $f2))
    (export "f [v2]" (func $"#core-func3 f"))
    (export "f [v3]" (func $"#core-func4 f"))
    (export "g" (func $g))
    (export "g [v2]" (func $"#core-func6 g"))
    (export "g2" (func $g2))
  )
  (core instance $main (;3;) (instantiate $main
      (with "wasi-snapshot-preview1" (instance $wasi-snapshot-preview1))
      (with "cm32p2" (instance $cm32p2))
    )
  )
  (alias core export $main "memory" (core memory $memory (;0;)))
  (alias export $foo:foo/my-wasi "proc-exit" (func $proc-exit (;4;)))
  (core func $proc-exit (;8;) (canon lower (func $proc-exit)))
  (core instance $foo:foo/my-wasi (;4;)
    (export "proc-exit" (func $proc-exit))
  )
  (core instance $"#core-instance5 wasi-snapshot-preview1" (@name "wasi-snapshot-preview1") (;5;) (instantiate $wit-component:adapter:wasi-snapshot-preview1
      (with "foo:foo/my-wasi" (instance $foo:foo/my-wasi))
    )
  )
  (alias core export $wit-component-shim-instance "$imports" (core table $"shim table" (;0;)))
  (alias core export $"#core-instance5 wasi-snapshot-preview1" "proc_exit" (core func $proc_exit (;9;)))
  (core instance $fixup-args (;6;)
    (export "$imports" (table $"shim table"))
    (export "0" (func $proc_exit))
  )
  (core instance $fixup (;7;) (instantiate $wit-component-fixup
      (with "" (instance $fixup-args))
    )
  )
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
