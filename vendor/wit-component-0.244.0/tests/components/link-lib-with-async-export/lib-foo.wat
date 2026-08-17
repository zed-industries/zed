(module
  (@dylink.0
    (mem-info (memory 4 4))
    (needed "c")
  )

  (import "[export]$root" "[task-return]f" (func (param i32)))

  (func (export "foo"))
  (func (export "[async-lift]f") (result i32) unreachable)
  (func (export "[callback][async-lift]f") (param i32 i32 i32) (result i32) unreachable)
)
