/* automatically generated from ./x86/regs.dat - do not edit */

#ifndef NASM_REGS_H
#define NASM_REGS_H

#define EXPR_REG_START 1

enum reg_enum {
    R_zero = 0,
    R_none = -1,
    R_AH = EXPR_REG_START,
    R_AL,
    R_AX,
    R_BH,
    R_BL,
    R_BND0,
    R_BND1,
    R_BND2,
    R_BND3,
    R_BP,
    R_BPL,
    R_BX,
    R_CH,
    R_CL,
    R_CR0,
    R_CR1,
    R_CR10,
    R_CR11,
    R_CR12,
    R_CR13,
    R_CR14,
    R_CR15,
    R_CR2,
    R_CR3,
    R_CR4,
    R_CR5,
    R_CR6,
    R_CR7,
    R_CR8,
    R_CR9,
    R_CS,
    R_CX,
    R_DH,
    R_DI,
    R_DIL,
    R_DL,
    R_DR0,
    R_DR1,
    R_DR10,
    R_DR11,
    R_DR12,
    R_DR13,
    R_DR14,
    R_DR15,
    R_DR2,
    R_DR3,
    R_DR4,
    R_DR5,
    R_DR6,
    R_DR7,
    R_DR8,
    R_DR9,
    R_DS,
    R_DX,
    R_EAX,
    R_EBP,
    R_EBX,
    R_ECX,
    R_EDI,
    R_EDX,
    R_ES,
    R_ESI,
    R_ESP,
    R_FS,
    R_GS,
    R_K0,
    R_K1,
    R_K2,
    R_K3,
    R_K4,
    R_K5,
    R_K6,
    R_K7,
    R_MM0,
    R_MM1,
    R_MM2,
    R_MM3,
    R_MM4,
    R_MM5,
    R_MM6,
    R_MM7,
    R_R10,
    R_R10B,
    R_R10D,
    R_R10W,
    R_R11,
    R_R11B,
    R_R11D,
    R_R11W,
    R_R12,
    R_R12B,
    R_R12D,
    R_R12W,
    R_R13,
    R_R13B,
    R_R13D,
    R_R13W,
    R_R14,
    R_R14B,
    R_R14D,
    R_R14W,
    R_R15,
    R_R15B,
    R_R15D,
    R_R15W,
    R_R8,
    R_R8B,
    R_R8D,
    R_R8W,
    R_R9,
    R_R9B,
    R_R9D,
    R_R9W,
    R_RAX,
    R_RBP,
    R_RBX,
    R_RCX,
    R_RDI,
    R_RDX,
    R_RSI,
    R_RSP,
    R_SEGR6,
    R_SEGR7,
    R_SI,
    R_SIL,
    R_SP,
    R_SPL,
    R_SS,
    R_ST0,
    R_ST1,
    R_ST2,
    R_ST3,
    R_ST4,
    R_ST5,
    R_ST6,
    R_ST7,
    R_TMM0,
    R_TMM1,
    R_TMM2,
    R_TMM3,
    R_TMM4,
    R_TMM5,
    R_TMM6,
    R_TMM7,
    R_TR0,
    R_TR1,
    R_TR2,
    R_TR3,
    R_TR4,
    R_TR5,
    R_TR6,
    R_TR7,
    R_XMM0,
    R_XMM1,
    R_XMM10,
    R_XMM11,
    R_XMM12,
    R_XMM13,
    R_XMM14,
    R_XMM15,
    R_XMM16,
    R_XMM17,
    R_XMM18,
    R_XMM19,
    R_XMM2,
    R_XMM20,
    R_XMM21,
    R_XMM22,
    R_XMM23,
    R_XMM24,
    R_XMM25,
    R_XMM26,
    R_XMM27,
    R_XMM28,
    R_XMM29,
    R_XMM3,
    R_XMM30,
    R_XMM31,
    R_XMM4,
    R_XMM5,
    R_XMM6,
    R_XMM7,
    R_XMM8,
    R_XMM9,
    R_YMM0,
    R_YMM1,
    R_YMM10,
    R_YMM11,
    R_YMM12,
    R_YMM13,
    R_YMM14,
    R_YMM15,
    R_YMM16,
    R_YMM17,
    R_YMM18,
    R_YMM19,
    R_YMM2,
    R_YMM20,
    R_YMM21,
    R_YMM22,
    R_YMM23,
    R_YMM24,
    R_YMM25,
    R_YMM26,
    R_YMM27,
    R_YMM28,
    R_YMM29,
    R_YMM3,
    R_YMM30,
    R_YMM31,
    R_YMM4,
    R_YMM5,
    R_YMM6,
    R_YMM7,
    R_YMM8,
    R_YMM9,
    R_ZMM0,
    R_ZMM1,
    R_ZMM10,
    R_ZMM11,
    R_ZMM12,
    R_ZMM13,
    R_ZMM14,
    R_ZMM15,
    R_ZMM16,
    R_ZMM17,
    R_ZMM18,
    R_ZMM19,
    R_ZMM2,
    R_ZMM20,
    R_ZMM21,
    R_ZMM22,
    R_ZMM23,
    R_ZMM24,
    R_ZMM25,
    R_ZMM26,
    R_ZMM27,
    R_ZMM28,
    R_ZMM29,
    R_ZMM3,
    R_ZMM30,
    R_ZMM31,
    R_ZMM4,
    R_ZMM5,
    R_ZMM6,
    R_ZMM7,
    R_ZMM8,
    R_ZMM9,
    REG_ENUM_LIMIT
};

#define EXPR_REG_END 248

#define REG_NUM_AH       4
#define REG_NUM_AL       0
#define REG_NUM_AX       0
#define REG_NUM_BH       7
#define REG_NUM_BL       3
#define REG_NUM_BND0     0
#define REG_NUM_BND1     1
#define REG_NUM_BND2     2
#define REG_NUM_BND3     3
#define REG_NUM_BP       5
#define REG_NUM_BPL      5
#define REG_NUM_BX       3
#define REG_NUM_CH       5
#define REG_NUM_CL       1
#define REG_NUM_CR0      0
#define REG_NUM_CR1      1
#define REG_NUM_CR10    10
#define REG_NUM_CR11    11
#define REG_NUM_CR12    12
#define REG_NUM_CR13    13
#define REG_NUM_CR14    14
#define REG_NUM_CR15    15
#define REG_NUM_CR2      2
#define REG_NUM_CR3      3
#define REG_NUM_CR4      4
#define REG_NUM_CR5      5
#define REG_NUM_CR6      6
#define REG_NUM_CR7      7
#define REG_NUM_CR8      8
#define REG_NUM_CR9      9
#define REG_NUM_CS       1
#define REG_NUM_CX       1
#define REG_NUM_DH       6
#define REG_NUM_DI       7
#define REG_NUM_DIL      7
#define REG_NUM_DL       2
#define REG_NUM_DR0      0
#define REG_NUM_DR1      1
#define REG_NUM_DR10    10
#define REG_NUM_DR11    11
#define REG_NUM_DR12    12
#define REG_NUM_DR13    13
#define REG_NUM_DR14    14
#define REG_NUM_DR15    15
#define REG_NUM_DR2      2
#define REG_NUM_DR3      3
#define REG_NUM_DR4      4
#define REG_NUM_DR5      5
#define REG_NUM_DR6      6
#define REG_NUM_DR7      7
#define REG_NUM_DR8      8
#define REG_NUM_DR9      9
#define REG_NUM_DS       3
#define REG_NUM_DX       2
#define REG_NUM_EAX      0
#define REG_NUM_EBP      5
#define REG_NUM_EBX      3
#define REG_NUM_ECX      1
#define REG_NUM_EDI      7
#define REG_NUM_EDX      2
#define REG_NUM_ES       0
#define REG_NUM_ESI      6
#define REG_NUM_ESP      4
#define REG_NUM_FS       4
#define REG_NUM_GS       5
#define REG_NUM_K0       0
#define REG_NUM_K1       1
#define REG_NUM_K2       2
#define REG_NUM_K3       3
#define REG_NUM_K4       4
#define REG_NUM_K5       5
#define REG_NUM_K6       6
#define REG_NUM_K7       7
#define REG_NUM_MM0      0
#define REG_NUM_MM1      1
#define REG_NUM_MM2      2
#define REG_NUM_MM3      3
#define REG_NUM_MM4      4
#define REG_NUM_MM5      5
#define REG_NUM_MM6      6
#define REG_NUM_MM7      7
#define REG_NUM_R10     10
#define REG_NUM_R10B    10
#define REG_NUM_R10D    10
#define REG_NUM_R10W    10
#define REG_NUM_R11     11
#define REG_NUM_R11B    11
#define REG_NUM_R11D    11
#define REG_NUM_R11W    11
#define REG_NUM_R12     12
#define REG_NUM_R12B    12
#define REG_NUM_R12D    12
#define REG_NUM_R12W    12
#define REG_NUM_R13     13
#define REG_NUM_R13B    13
#define REG_NUM_R13D    13
#define REG_NUM_R13W    13
#define REG_NUM_R14     14
#define REG_NUM_R14B    14
#define REG_NUM_R14D    14
#define REG_NUM_R14W    14
#define REG_NUM_R15     15
#define REG_NUM_R15B    15
#define REG_NUM_R15D    15
#define REG_NUM_R15W    15
#define REG_NUM_R8       8
#define REG_NUM_R8B      8
#define REG_NUM_R8D      8
#define REG_NUM_R8W      8
#define REG_NUM_R9       9
#define REG_NUM_R9B      9
#define REG_NUM_R9D      9
#define REG_NUM_R9W      9
#define REG_NUM_RAX      0
#define REG_NUM_RBP      5
#define REG_NUM_RBX      3
#define REG_NUM_RCX      1
#define REG_NUM_RDI      7
#define REG_NUM_RDX      2
#define REG_NUM_RSI      6
#define REG_NUM_RSP      4
#define REG_NUM_SEGR6    6
#define REG_NUM_SEGR7    7
#define REG_NUM_SI       6
#define REG_NUM_SIL      6
#define REG_NUM_SP       4
#define REG_NUM_SPL      4
#define REG_NUM_SS       2
#define REG_NUM_ST0      0
#define REG_NUM_ST1      1
#define REG_NUM_ST2      2
#define REG_NUM_ST3      3
#define REG_NUM_ST4      4
#define REG_NUM_ST5      5
#define REG_NUM_ST6      6
#define REG_NUM_ST7      7
#define REG_NUM_TMM0     0
#define REG_NUM_TMM1     1
#define REG_NUM_TMM2     2
#define REG_NUM_TMM3     3
#define REG_NUM_TMM4     4
#define REG_NUM_TMM5     5
#define REG_NUM_TMM6     6
#define REG_NUM_TMM7     7
#define REG_NUM_TR0      0
#define REG_NUM_TR1      1
#define REG_NUM_TR2      2
#define REG_NUM_TR3      3
#define REG_NUM_TR4      4
#define REG_NUM_TR5      5
#define REG_NUM_TR6      6
#define REG_NUM_TR7      7
#define REG_NUM_XMM0     0
#define REG_NUM_XMM1     1
#define REG_NUM_XMM10   10
#define REG_NUM_XMM11   11
#define REG_NUM_XMM12   12
#define REG_NUM_XMM13   13
#define REG_NUM_XMM14   14
#define REG_NUM_XMM15   15
#define REG_NUM_XMM16   16
#define REG_NUM_XMM17   17
#define REG_NUM_XMM18   18
#define REG_NUM_XMM19   19
#define REG_NUM_XMM2     2
#define REG_NUM_XMM20   20
#define REG_NUM_XMM21   21
#define REG_NUM_XMM22   22
#define REG_NUM_XMM23   23
#define REG_NUM_XMM24   24
#define REG_NUM_XMM25   25
#define REG_NUM_XMM26   26
#define REG_NUM_XMM27   27
#define REG_NUM_XMM28   28
#define REG_NUM_XMM29   29
#define REG_NUM_XMM3     3
#define REG_NUM_XMM30   30
#define REG_NUM_XMM31   31
#define REG_NUM_XMM4     4
#define REG_NUM_XMM5     5
#define REG_NUM_XMM6     6
#define REG_NUM_XMM7     7
#define REG_NUM_XMM8     8
#define REG_NUM_XMM9     9
#define REG_NUM_YMM0     0
#define REG_NUM_YMM1     1
#define REG_NUM_YMM10   10
#define REG_NUM_YMM11   11
#define REG_NUM_YMM12   12
#define REG_NUM_YMM13   13
#define REG_NUM_YMM14   14
#define REG_NUM_YMM15   15
#define REG_NUM_YMM16   16
#define REG_NUM_YMM17   17
#define REG_NUM_YMM18   18
#define REG_NUM_YMM19   19
#define REG_NUM_YMM2     2
#define REG_NUM_YMM20   20
#define REG_NUM_YMM21   21
#define REG_NUM_YMM22   22
#define REG_NUM_YMM23   23
#define REG_NUM_YMM24   24
#define REG_NUM_YMM25   25
#define REG_NUM_YMM26   26
#define REG_NUM_YMM27   27
#define REG_NUM_YMM28   28
#define REG_NUM_YMM29   29
#define REG_NUM_YMM3     3
#define REG_NUM_YMM30   30
#define REG_NUM_YMM31   31
#define REG_NUM_YMM4     4
#define REG_NUM_YMM5     5
#define REG_NUM_YMM6     6
#define REG_NUM_YMM7     7
#define REG_NUM_YMM8     8
#define REG_NUM_YMM9     9
#define REG_NUM_ZMM0     0
#define REG_NUM_ZMM1     1
#define REG_NUM_ZMM10   10
#define REG_NUM_ZMM11   11
#define REG_NUM_ZMM12   12
#define REG_NUM_ZMM13   13
#define REG_NUM_ZMM14   14
#define REG_NUM_ZMM15   15
#define REG_NUM_ZMM16   16
#define REG_NUM_ZMM17   17
#define REG_NUM_ZMM18   18
#define REG_NUM_ZMM19   19
#define REG_NUM_ZMM2     2
#define REG_NUM_ZMM20   20
#define REG_NUM_ZMM21   21
#define REG_NUM_ZMM22   22
#define REG_NUM_ZMM23   23
#define REG_NUM_ZMM24   24
#define REG_NUM_ZMM25   25
#define REG_NUM_ZMM26   26
#define REG_NUM_ZMM27   27
#define REG_NUM_ZMM28   28
#define REG_NUM_ZMM29   29
#define REG_NUM_ZMM3     3
#define REG_NUM_ZMM30   30
#define REG_NUM_ZMM31   31
#define REG_NUM_ZMM4     4
#define REG_NUM_ZMM5     5
#define REG_NUM_ZMM6     6
#define REG_NUM_ZMM7     7
#define REG_NUM_ZMM8     8
#define REG_NUM_ZMM9     9


#endif /* NASM_REGS_H */
