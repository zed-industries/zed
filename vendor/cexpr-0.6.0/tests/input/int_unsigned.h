#define Int_456 456
#define Int_0 0
#define Int_1 0b1
#define Int_2 0x2
#define Int_3 3L
#define Int_4 0X4
#define Int_5 0B101
#define Int_63 077
#define Int_123 123
#define Int_124 124u
#define Int_125 125uL
#define Int_126 126LuL
#define Int_16 (((1)<<4ULL))/*comment*/ 
#define Int_13 1|8^6&2<<1

#define Int_47 32|15
#define Int_38 (32|15)^9
#define Int_6 ((32|15)^9)&7
#define Int_12 (((32|15)^9)&7)<<1
#define Int_17 ((((32|15)^9)&7)<<1)+5
#define Int_15 (((((32|15)^9)&7)<<1)+5)-2
#define Int_60 ((((((32|15)^9)&7)<<1)+5)-2)*4
#define Int_30 (((((((32|15)^9)&7)<<1)+5)-2)*4)/2
#define Int_39 32|15^9&7<<1+5-2*4/2

#define Int_n1 18446744073709551615 /*2^64-1*/
#define Int_n9223372036854775808 9223372036854775808

#define Fn_Int_9(_3) _3*3
