(module
  (@dylink.0 (needed "bar"))
  (import "env" "tag1" (tag))
  (import "env" "tag2" (tag))
  (import "GOT.func" "bar" (global (mut i32)))
  (func (export "run") unreachable)
)
