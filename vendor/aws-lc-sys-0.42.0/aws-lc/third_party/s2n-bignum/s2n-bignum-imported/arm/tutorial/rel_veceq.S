#define a0 x10
#define a1 x11
#define res0 x20
#define res1 x21
#define res2 x22
#define res3 x23
#define res1t x27
#define res2t x28
#define res3t x29
#define a0a0_hi x12
#define a0a1_lo x13
#define a0a1_hi x14
#define a1a1_lo x15
#define a1a1_hi x16

ldp a0, a1, [x1]
mul   res0,    a0, a0
umulh a0a0_hi, a0, a0
mul   a0a1_lo, a0, a1
umulh a0a1_hi, a0, a1
mul   a1a1_lo, a1, a1
umulh a1a1_hi, a1, a1
adds  res1t,   a0a0_hi, a0a1_lo
adcs  res2t,   a1a1_lo, a0a1_hi
adc   res3t,   a1a1_hi, xzr
adds  res1,    res1t,   a0a1_lo
adcs  res2,    res2t,   a0a1_hi
adc   res3,    res3t,   xzr
