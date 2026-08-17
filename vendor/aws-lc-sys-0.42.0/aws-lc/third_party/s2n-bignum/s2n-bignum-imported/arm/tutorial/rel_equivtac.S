ldp x11, x10, [x0]
add x12, x10, #1
mul x12, x11, x12 // x11 * (x10 + 1)
str x12, [x1]
