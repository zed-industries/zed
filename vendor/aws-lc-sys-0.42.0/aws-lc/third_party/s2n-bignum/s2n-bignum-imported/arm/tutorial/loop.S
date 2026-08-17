  mov x1, xzr
  mov x0, xzr

loop:
  add x1, x1, #1
  add x0, x0, #2
  cmp x1, #10
  bne loop

  ret
