;; this is a polyfill module that translates from wasi-preview1 to a different
;; interface

(module
  (import "foo:foo/my-wasi" "proc-exit" (func $proc_exit (param i32)))
  (func (export "proc_exit") (param i32)
    local.get 0
    call $proc_exit
  )
  (func (export "random_get") (param i32 i32) (result i32)
    i32.const 0)
  (func (export "something_else"))
)
