(module
  ;; Import an adapter function twice.
  (func $exit1 (import "wasi-snapshot-preview1" "proc_exit") (param i32))
  (func $exit2 (import "wasi-snapshot-preview1" "proc_exit") (param i32))

  ;; define `f2` before the second `f` import shows up
  (import "cm32p2" "f" (func $f_v1))
  (import "cm32p2" "f2" (func $f2 (result i32)))
  (import "cm32p2" "f" (func $f_v2))
  (import "cm32p2" "f" (func $f_v3))

  ;; define two `g` imports before the "real" `g2` import shows up
  (import "cm32p2" "g" (func $g_v1))
  (import "cm32p2" "g" (func $g_v2))
  (import "cm32p2" "g2" (func $g2 (result i32)))

  (func
    ;; Call all the "f" imports and its duplicate copies
    call $f_v1
    call $f_v2
    call $f_v3
    call $f2
    drop

    ;; Call all the "g" imports and its duplicate copies
    call $g_v1
    call $g_v2
    call $g2
    drop

    ;; Call all the "proc_exit" imports and its duplicate copies
    i32.const 42
    call $exit1
    i32.const 42
    call $exit2
  )

  ;; Required by wasi
  (memory (export "memory") 1)
)
