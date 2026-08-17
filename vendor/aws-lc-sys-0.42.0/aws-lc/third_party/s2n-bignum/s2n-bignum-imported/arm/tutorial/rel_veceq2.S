ldp x9, x15, [x1]
ldr q30, [x1]
umull v0.2D, v30.2S, v30.2S
umull2 v2.2D, v30.4S, v30.4S
xtn v24.2S, v30.2D
uzp2 v30.4S, v30.4S, v30.4S
umull v30.2D, v30.2S, v24.2S
mov x7, v0.d[0]
mov x14, v0.d[1]
mov x19, v2.d[0]
mov x22, v2.d[1]
mov x4, v30.d[0]
mov x12, v30.d[1]
adds x21, x7, x4, lsl #33
lsr x4, x4, #31
adc x14, x14, x4
adds x19, x19, x12, lsl #33
lsr x4, x12, #31
adc x22, x22, x4
mul x4, x9, x15
umulh x12, x9, x15
adds x24, x14, x4, lsl #1
extr x4, x12, x4, #63
adcs x19, x19, x4
lsr x4, x12, #63
adc x4, x22, x4 //x21,x24,x19,x4
