;; this is a polyfill module that translates from wasi-preview1 to a different
;; interface

(module
  (import "other1" "foo" (func $foo))
  (import "other2" "bar" (func $bar))

  (func (export "foo") call $foo)
  (func (export "bar") call $bar)
)
