  cmp x1, x2
  b.hi BB2
  mov x0, x2
  ret
BB2:
  mov x0, x1
  ret
