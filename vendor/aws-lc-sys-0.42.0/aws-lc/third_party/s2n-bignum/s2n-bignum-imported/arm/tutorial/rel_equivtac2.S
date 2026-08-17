ldp x21, x20, [x0]
mul x22, x21, x20
add x22, x22, x21 // x21 * x20 + x21
str x22, [x1]
