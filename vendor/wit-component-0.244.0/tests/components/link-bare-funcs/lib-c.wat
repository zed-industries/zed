(module
  (@dylink.0
    (mem-info (memory 0 4))
  )
  (type (func))
  (type (func (param i32) (result i32)))
  (import "GOT.mem" "__heap_base" (global $__heap_base (mut i32)))
  (import "GOT.mem" "__heap_end" (global $__heap_end (mut i32)))
  (global $heap (mut i32) i32.const 0)
  (func $start (type 0)
    global.get $__heap_base
    global.set $heap
  )
  (func $malloc (type 1) (param i32) (result i32)
    global.get $heap
    global.get $heap
    local.get 0
    i32.add
    global.set $heap
  )
  (func $abort (type 0)
    unreachable
  )
  (export "malloc" (func $malloc))
  (export "abort" (func $abort))
  (start $start)
)
