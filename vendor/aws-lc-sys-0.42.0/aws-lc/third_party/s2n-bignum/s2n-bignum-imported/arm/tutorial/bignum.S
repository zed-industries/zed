  ldp x2, x3, [x0]
  ldp x4, x5, [x1]
  cmp x2, x4
  bne bb_false
  cmp x3, x5
  bne bb_false
  mov x0, #1
  ret
bb_false:
  mov x0, xzr
  ret
