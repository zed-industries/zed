(module
  (import "old" "get_sum" (func (result i32)))
  ;; Minimal realloc which only accepts new, page-sized allocations:
  (func $cabi_realloc (param i32 i32 i32 i32) (result i32)
    (local i32)
    i32.const 0
    local.get 0
    i32.ne
    if
      unreachable
    end
    i32.const 0
    local.get 1
    i32.ne
    if
      unreachable
    end
    i32.const 65536
    local.get 3
    i32.ne
    if
      unreachable
    end
    i32.const 1
    memory.grow
    local.tee 4
    i32.const -1
    i32.eq
    if
      unreachable
    end
    local.get 4
    i32.const 16
    i32.shl
  )
  (memory (export "memory") 1)
  (export "cabi_realloc" (func $cabi_realloc))
)
