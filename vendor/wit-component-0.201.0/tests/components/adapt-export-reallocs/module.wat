(module
  (import "old" "read" (func (param i32 i32)))
  (func (export "main") (param $args i32) (param $argv i32)
    ;; ...
  )

  (memory (export "memory") 1)
)
