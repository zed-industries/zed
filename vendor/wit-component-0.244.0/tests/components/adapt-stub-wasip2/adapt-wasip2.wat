;; module name: wasi:cli/environment@0.2.0
(module
  (type (func (param i32)))
  (func $get-environment (type 0)
    unreachable
  )
  (export "get-environment" (func $get-environment))
)
