;; This test ensures that the `_initialize` function is hooked up to execute
;; when the component is instantiated.
(module
  (func (export "a") unreachable)
  (func (export "_initialize") unreachable)
)
