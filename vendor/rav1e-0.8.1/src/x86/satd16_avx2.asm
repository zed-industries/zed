; Copyright (c) 2022, The rav1e contributors. All rights reserved
;
; This source code is subject to the terms of the BSD 2 Clause License and
; the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
; was not distributed with this source code in the LICENSE file, you can
; obtain it at www.aomedia.org/license/software. If the Alliance for Open
; Media Patent License 1.0 was not distributed with this source code in the
; PATENTS file, you can obtain it at www.aomedia.org/license/patent.

%include "config.asm"
%include "ext/x86/x86inc.asm"

%define m(x) mangle(private_prefix %+ _ %+ x %+ SUFFIX)

%if ARCH_X86_64

SECTION_RODATA 32

align 32
pw_1x16:   times 16 dw 1

SECTION .text

%macro NORMALIZE4PT 0
    add eax, 2
    shr eax, 2
%endmacro

%macro NORMALIZE8PT 0
    add eax, 4
    shr eax, 3
%endmacro

; Add and subtract registers
;
; Takes m0 and m1 as both input and output.
; Requires m2 as a free register.
;
; If we start with this permutation:
;
; m0    0 1  2  3    4  5  6  7
; m1    8 9 10 11   12 13 14 15
;
; Then the output will be as such:
;
; m0    [0+8][1+9][2+10][3+11] [4+12][5+13][6+14][7+15]
; m1    [0-8][1-9][2-10][3-11] [4-12][5-13][6-14][7-15]
%macro BUTTERFLY 3
    %define BIT_PRECISION %1
    %define VEC_SIZE %2
    ; use alternate registers 3,4,5
    %define USE_ALT %3

    %if USE_ALT == 1
        SWAP 3, 0
        SWAP 4, 1
        SWAP 5, 2
    %endif

    %if VEC_SIZE == 32
        %define V ym
    %elif VEC_SIZE == 16
        %define V xm
    %endif

    ; Use m2 as a temporary register, then swap
    ; so that m0 and m1 contain the output.
    %if BIT_PRECISION == 16
        paddw       V%+ 2, V%+ 0, V%+ 1
        psubw       V%+ 0, V%+ 1
    %elif BIT_PRECISION == 32
        paddd       ym2, ym0, ym1
        psubd       ym0, ym1
    %else
        %error Incorrect precision specified (16 or 32 expected)
    %endif

    SWAP 2, 1, 0

    %if USE_ALT == 1
        SWAP 3, 0
        SWAP 4, 1
        SWAP 5, 2
    %endif
%endmacro

; Interleave packed rows together (in m0 and m1).
; m2 should contain a free register.
;
; Macro argument takes size in bits of each element (where one
; element is the difference between two original source pixels).
;
; If we start with this permutation:
;
; m0    0 1  2  3    4  5  6  7
; m1    8 9 10 11   12 13 14 15
;
; Then, after INTERLEAVE, this will be the permutation:
;
; m0    0  8  1  9   2 10  3 11
; m1    4 12  5 13   6 14  7 15
%macro INTERLEAVE 3
    %define BIT_PRECISION %1
    %define VEC_SIZE %2
    %define USE_ALT %3

    %if USE_ALT == 1
        SWAP 3, 0
        SWAP 4, 1
        SWAP 5, 2
    %endif

    %if VEC_SIZE == 16
        %define V xm
    %elif VEC_SIZE == 32
        %define V ym
    %else
        %error Invalid vector size (expected 16 or 32)
    %endif

    %if BIT_PRECISION == 16
        punpcklwd   V%+ 2, V%+ 0, V%+ 1
        punpckhwd   V%+ 0, V%+ 1
        SWAP 2, 1, 0
    %elif BIT_PRECISION == 32
        punpckldq   ym2, ym0, ym1
        punpckhdq   ym0, ym1
        ; AVX2 shuffles operate over 128-bit halves of the full ymm register
        ; in parallel, so these shuffles are required to fix up the permutation.
        vperm2i128  ym1, ym2, ym0, 0x20
        vperm2i128  ym0, ym2, ym0, 0x31
        SWAP 0, 1
    %else
        %error Incorrect precision specified (16 or 32 expected)
    %endif

    %if USE_ALT == 1
        SWAP 3, 0
        SWAP 4, 1
        SWAP 5, 2
    %endif
%endmacro

; Interleave pairs of 2 elements (in m0 and m1)
; m2 should contain a free register.
%macro INTERLEAVE_PAIRS 3
    %define BIT_PRECISION %1
    %define VEC_SIZE %2
    %define USE_ALT %3

    %if USE_ALT == 1
        SWAP 3, 0
        SWAP 4, 1
        SWAP 5, 2
    %endif

    %if VEC_SIZE == 16
        %define V xm
    %elif VEC_SIZE == 32
        %define V ym
    %else
        %error Invalid vector size (expected 16 or 32)
    %endif

    %if BIT_PRECISION == 16
        punpckldq   V%+ 2, V%+ 0, V%+ 1
        punpckhdq   V%+ 0, V%+ 1
    %elif BIT_PRECISION == 32
        punpcklqdq  ym2, ym0, ym1
        punpckhqdq  ym0, ym1
    %else
        %error Incorrect precision specified (16 or 32 expected)
    %endif
    SWAP 2, 1, 0

    %if USE_ALT == 1
        SWAP 3, 0
        SWAP 4, 1
        SWAP 5, 2
    %endif
%endmacro

%macro HADAMARD_4X4_PACKED 2
    %define BIT_PRECISION %1
    ; Register size to use (in bytes)
    %define VEC_SIZE %2

    %if VEC_SIZE == 16
        %define V xm
    %elif VEC_SIZE == 32
        %define V ym
    %else
        %error Invalid vector size (expected 16 or 32)
    %endif

    ; Starting registers:

    ; m0    0    1   2   3
    ; m1    4    5   6   7
    ; m2    8    9  10  11
    ; m3    12  13  14  15

    ; Where each number represents an index of the
    ; original block of differences.

    ; Pack rows 0,2 and 1,3 into m0 and m1
    %if BIT_PRECISION == 16
        %if VEC_SIZE == 16
            ; In this case, each row only has 64 bits, so we use
            ; punpcklqdq only. The high 64 bits are always 0.
            punpcklqdq  xm0, xm2
            punpcklqdq  xm1, xm3
        %elif VEC_SIZE == 32
            ; The upper 128 bits of all input registers are zeroed
            punpcklqdq      m4, m0, m2
            punpcklqdq      m5, m1, m3
            punpckhqdq      m0, m0, m2
            punpckhqdq      m1, m1, m3
            vinserti128     m0, m4, xm0, 1
            vinserti128     m1, m5, xm1, 1
        %endif
    %elif BIT_PRECISION == 32
        vinserti128 ym0, ym0, xm2, 1
        vinserti128 ym1, ym1, xm3, 1
    %else
        %error Invalid bit precision (expected 16 or 32)
    %endif

    ; Now that we've packed rows 0-2 and 1-3 together,
    ; this is our permutation:

    ; m0    0 1 2 3   8  9 10 11
    ; m1    4 5 6 7  12 13 14 15

    ; For a 8x4 transform (with 16-bit coefficients), this pattern is
    ; extended for each 128-bit half but for the second block, and thus
    ; all comments also apply to the upper 128-bits for the 8x4 transform.

    BUTTERFLY %1, %2, 0

    ; m0    [0+4][1+5][2+6][3+7] [8+12][9+13][10+14][11+15]
    ; m1    [0-4][1-5][2-6][3-7] [8-12][9-13][10-14][11-15]

    INTERLEAVE %1, %2, 0

    ; m0    [ 0+4][ 0-4][ 1+5][ 1-5] [2 + 6][2 - 6][3 + 7][3 - 7]
    ; m1    [8+12][8-12][9+13][9-13] [10+14][10-14][11+15][11-15]

    BUTTERFLY %1, %2, 0

    ; m0    [0+4+8+12][0-4+8-12][1+5+9+13][1-5+9-13] [2+6+10+14][2-6+10-14][3+7+11+15][3-7+11-15]
    ; m1    [0+4-8-12][0-4-8+12][1+5-9-13][1-5-9+13] [2+6-10-14][2-6-10+14][3+7-11-15][3-7-11+15]

    ; for one row:
    ; [0+1+2+3][0-1+2-3][0+1-2-3][0-1-2+3]
    ; For the vertical transform, these are packed into a new column.

    INTERLEAVE_PAIRS %1, %2, 0

    ;               p0         p1         p2         p3
    ; m0    [0+4+ 8+12][0-4+ 8-12][0+4- 8-12][0-4- 8+12] [1+5+ 9+13][1-5+ 9-13][1+5- 9-13][1-5- 9+13]
    ; m1    [2+6+10+14][2-6+10-14][2+6-10-14][2-6-10+14] [3+7+11+15][3-7+11-15][3+7-11-15][3-7-11+15]

    ; According to this grid:

    ; p0  q0  r0  s0
    ; p1  q1  r1  s1
    ; p2  q2  r2  s2
    ; p3  q3  r3  s3

    ; Horizontal transform; since the output is transposed from the original order,
    ; we can do the same steps as the vertical transform and the result will be the same.
    BUTTERFLY   %1, %2, 0
    INTERLEAVE  %1, %2, 0
    BUTTERFLY   %1, %2, 0

    ; Finished horizontal transform except for the last step (interleaving pairs),
    ; which we skip, because after this we add up the absolute value of the
    ; coefficients, which is a commutative operation (order does not matter).
%endmacro

; Horizontal sum of mm register
;
; Inputs:
; %1 = Element size in bits (16 or 32)
; %2 = Size of input register in bytes (16 or 32)
;      You can e.g. pass 16 for this argument if you
;      only want to sum up the bottom 128-bits of a
;      ymm register.
; %3 = Input register number
; %4 = Temporary register number
; %5 = Output register (e.g., eax)
%macro HSUM 5
    %define E_SIZE %1
    %define REG_SIZE %2
    %define INPUT %3
    %define TMP %4
    %define OUTPUT %5

    %if REG_SIZE == 16
    %define V xm
    %elif REG_SIZE == 32
    %define V ym
    %else
        %error Invalid register size (expected 16 or 32)
    %endif

    %if E_SIZE == 16
        ; Add adjacent pairs of 16-bit elements to produce 32-bit results,
        ; then proceed with 32-bit sum
        pmaddwd     V%+INPUT, [pw_1x16]
    %endif

    %if mmsize == 32 && REG_SIZE == 32
        ; Add upper half of ymm to xmm
        vextracti128    xm%+TMP,   ym%+INPUT, 1
        paddd           xm%+INPUT, xm%+TMP
    %endif

    ; Reduce 32-bit results
    pshufd      xm%+TMP,     xm%+INPUT, q2323
    paddd       xm%+INPUT,   xm%+TMP
    pshufd      xm%+TMP,     xm%+INPUT, q1111
    paddd       xm%+INPUT,   xm%+TMP
    movd        OUTPUT,      xm%+INPUT
%endmacro

; given m0-7, do butterfly as follows:
; (m0, m1) = butterfly(m0, m1)
; (m2, m3) = butterfly(m2, m3)
; (m4, m5) = butterfly(m4, m5)
; (m6, m7) = butterfly(m6, m7)
%macro BUTTERFLY_8X8 0
    ; m8 is free
    paddd   m8, m0, m1
    psubd   m0, m1
    SWAP    8, 1, 0

    ; m8 is free
    paddd   m8, m2, m3
    psubd   m2, m3
    SWAP    8, 3, 2

    paddd   m8, m4, m5
    psubd   m4, m5
    SWAP    8, 5, 4

    paddd   m8, m6, m7
    psubd   m6, m7
    SWAP    8, 7, 6
%endmacro

%macro HADAMARD_8X8_VERTICAL 0
    BUTTERFLY_8X8
    ; m0-7 contain a0-7

    SWAP 2, 1
    SWAP 6, 5

    BUTTERFLY_8X8

    SWAP 1, 4
    SWAP 3, 6

    BUTTERFLY_8X8

    SWAP 2, 1
    SWAP 2, 4
    SWAP 3, 6
    SWAP 5, 6
%endmacro

; Transpose rows m0-7.
; Output is also contained in m0-7.
;
; Uses m8, m10-15 as temporary registers (i.e. m9 is left unchanged.)
%macro TRANSPOSE8X8D 0
    SWAP   9, 0
    SWAP  10, 1
    SWAP  11, 2
    SWAP  12, 3
    SWAP  13, 4
    SWAP  14, 5
    SWAP  15, 6
    SWAP   2, 7

    punpckldq    m6,  m9,  m10
    punpckldq    m1,  m11, m12
    punpckhdq    m8,  m9,  m10
    punpckldq    m4,  m13, m14
    punpckldq    m9,  m15, m2
    vshufps      m3,  m6,  m1,  0x4e
    vpblendd     m10, m6,  m3,  0xcc
    vshufps      m6,  m4,  m9,  0x4e
    punpckhdq    m7,  m11, m12
    vpblendd     m11, m4,  m6,  0xcc
    vpblendd     m12, m3,  m1,  0xcc
    vperm2i128   m3,  m10, m11, 0x20
    punpckhdq    m5,  m13, m14
    vpblendd     m13, m6,  m9,  0xcc
    punpckhdq    m4,  m15, m2
    vperm2i128   m2,  m12, m13, 0x20
    vshufps      m14, m8,  m7,  0x4e
    vpblendd     m15, m14, m7,  0xcc
    vshufps      m7,  m5,  m4,  0x4e
    vpblendd     m8,  m8,  m14, 0xcc
    vpblendd     m5,  m5,  m7,  0xcc
    vperm2i128   m6,  m8,  m5,  0x20
    vpblendd     m4,  m7,  m4,  0xcc
    vperm2i128   m7,  m15, m4,  0x20
    vperm2i128   m1,  m10, m11, 0x31
    vperm2i128   m9,  m12, m13, 0x31
    vperm2i128   m5,  m8,  m5,  0x31
    vperm2i128   m4,  m15, m4,  0x31

    SWAP 0, 9

    ; Output order is as follows:
    ; 3 2 6 7 1 0 5 4

    ; sort rows
    SWAP 3, 0
    ; 0 2 6 7 1 3 5 4
    SWAP 1, 2
    ; 0 1 6 7 2 3 5 4
    SWAP 6, 2
    ; 0 1 2 7 6 3 5 4
    SWAP 7, 3
    ; 0 1 2 3 6 7 5 4
    SWAP 6, 4
    ; 0 1 2 3 4 7 5 6
    SWAP 7, 5
    ; 0 1 2 3 4 5 7 6
    SWAP 6, 7
    ; 0 1 2 3 4 5 6 7
%endmacro

; m0-7 as input; add coefficients to ymm9.
INIT_YMM avx2
cglobal satd_8x8_hbd_internal, 0, 0, 0, src, src_stride, dst, dst_stride, bdmax, \
                                        src_stride3, dst_stride3
    HADAMARD_8X8_VERTICAL

    TRANSPOSE8X8D

    HADAMARD_8X8_VERTICAL

    REPX    {pabsd x, x}, m0, m1, m2, m3, m4, m5, m6, m7

    ; Add m0-7
    paddd   m0, m4
    paddd   m1, m5
    paddd   m2, m6
    paddd   m3, m7

    paddd   m0, m2
    paddd   m1, m3

    paddd   m0, m1
    paddd ymm9, m0
    ret

%macro LOAD_DIFF_8X8 0
    movu        xm0, [srcq + 0*src_strideq]
    movu        xm1, [srcq + 1*src_strideq]
    movu        xm2, [srcq + 2*src_strideq]
    movu        xm3, [srcq + src_stride3q ]
    lea        srcq, [srcq + 4*src_strideq]
    movu        xm4, [srcq + 0*src_strideq]
    movu        xm5, [srcq + 1*src_strideq]
    movu        xm6, [srcq + 2*src_strideq]
    movu        xm7, [srcq + src_stride3q ]

    psubw       xm0, [dstq + 0*dst_strideq]
    psubw       xm1, [dstq + 1*dst_strideq]
    psubw       xm2, [dstq + 2*dst_strideq]
    psubw       xm3, [dstq + dst_stride3q ]
    lea        dstq, [dstq + 4*dst_strideq]
    psubw       xm4, [dstq + 0*dst_strideq]
    psubw       xm5, [dstq + 1*dst_strideq]
    psubw       xm6, [dstq + 2*dst_strideq]
    psubw       xm7, [dstq + dst_stride3q ]

    pmovsxwd    m0, xm0
    pmovsxwd    m1, xm1
    pmovsxwd    m2, xm2
    pmovsxwd    m3, xm3
    pmovsxwd    m4, xm4
    pmovsxwd    m5, xm5
    pmovsxwd    m6, xm6
    pmovsxwd    m7, xm7
%endmacro

INIT_YMM avx2
cglobal satd_8x8_hbd, 5, 7, 16, src, src_stride, dst, dst_stride, bdmax, \
                                src_stride3, dst_stride3
    lea         src_stride3q, [3*src_strideq]
    lea         dst_stride3q, [3*dst_strideq]

    LOAD_DIFF_8X8

    ; m0-7 contain rows of 8x8 block to transform
    ; with 32-bit coefficients

    HADAMARD_8X8_VERTICAL

    TRANSPOSE8X8D

    HADAMARD_8X8_VERTICAL

    REPX    {pabsd x, x}, m0, m1, m2, m3, m4, m5, m6, m7

    ; Add m0-7
    paddd   m0, m4
    paddd   m1, m5
    paddd   m2, m6
    paddd   m3, m7

    paddd   m0, m2
    paddd   m1, m3

    paddd   m0, m1

    HSUM 32, 32, 0, 1, eax
    NORMALIZE8PT
    RET

INIT_YMM avx2
cglobal satd_4x4_hbd, 5, 7, 8, src, src_stride, dst, dst_stride, bdmax, \
                               src_stride3, dst_stride3
    lea         src_stride3q, [3*src_strideq]
    lea         dst_stride3q, [3*dst_strideq]

    cmp         bdmaxd, (1 << 10) - 1
    jne         .12bpc

    ; Load src rows
    movq        xm0, [srcq + 0*src_strideq]
    movq        xm1, [srcq + 1*src_strideq]
    movq        xm2, [srcq + 2*src_strideq]
    movq        xm3, [srcq + src_stride3q ]

    ; src -= dst
    psubw       xm0, [dstq + 0*dst_strideq]
    psubw       xm1, [dstq + 1*dst_strideq]
    psubw       xm2, [dstq + 2*dst_strideq]
    psubw       xm3, [dstq + dst_stride3q ]

    HADAMARD_4X4_PACKED 16, 16

    ; Sum up absolute value of transform coefficients
    pabsw       xm0, xm0
    pabsw       xm1, xm1
    paddw       xm0, xm1
    HSUM 16, 16, 0, 1, eax
    NORMALIZE4PT
    RET
.12bpc:
    ; this gives a nicer disassembly
    RESET_MM_PERMUTATION

    ; Load src rows
    pmovzxwd    xm0, [srcq + 0*src_strideq]
    pmovzxwd    xm1, [srcq + 1*src_strideq]
    pmovzxwd    xm2, [srcq + 2*src_strideq]
    pmovzxwd    xm3, [srcq + src_stride3q ]

    ; Load dst rows
    pmovzxwd    xm4, [dstq + 0*dst_strideq]
    pmovzxwd    xm5, [dstq + 1*dst_strideq]
    pmovzxwd    xm6, [dstq + 2*dst_strideq]
    pmovzxwd    xm7, [dstq + dst_stride3q ]

    ; src -= dst
    psubd       xm0, xm4
    psubd       xm1, xm5
    psubd       xm2, xm6
    psubd       xm3, xm7

    HADAMARD_4X4_PACKED 32, 32

    pabsd       m0, m0
    pabsd       m1, m1
    paddd       m0, m1
    HSUM 32, 32, 0, 1, eax
    NORMALIZE4PT
    RET

; 32-bit input rows are in m0-3; result is in m0.
; Uses m0-5 as temporary registers.
%macro HADAMARD_8X4_12BPC 0
    vperm2i128      m4, m0, m2, 0x31
    vperm2i128      m5, m1, m3, 0x31
    vinserti128     m0, m0, xm2, 1
    vinserti128     m1, m1, xm3, 1

    ; Swap so m3,m4 are used as inputs.
    SWAP 3, 4, 5

    ; instead of using HADAMARD_4X4_PACKED twice, we interleave
    ; 2 transforms operating over different registers for more
    ; opportunity for instruction level parallelism.

    BUTTERFLY           32, 32, 0
    BUTTERFLY           32, 32, 1
    INTERLEAVE          32, 32, 0
    INTERLEAVE          32, 32, 1
    BUTTERFLY           32, 32, 0
    BUTTERFLY           32, 32, 1
    INTERLEAVE_PAIRS    32, 32, 0
    INTERLEAVE_PAIRS    32, 32, 1
    BUTTERFLY           32, 32, 0
    BUTTERFLY           32, 32, 1
    INTERLEAVE          32, 32, 0
    INTERLEAVE          32, 32, 1
    BUTTERFLY           32, 32, 0
    BUTTERFLY           32, 32, 1

    pabsd       m0, m0
    pabsd       m1, m1
    pabsd       m3, m3
    pabsd       m4, m4

    paddd       m0, m1
    paddd       m3, m4
    paddd       m0, m3
%endmacro

INIT_YMM avx2
cglobal satd_16x4_hbd, 5, 7, 12, src, src_stride, dst, dst_stride, bdmax, \
                               src_stride3, dst_stride3
    lea         src_stride3q, [3*src_strideq]
    lea         dst_stride3q, [3*dst_strideq]

    cmp         bdmaxd, (1 << 10) - 1
    jne         .12bpc

    ; Load src rows
    movu         m0, [srcq + 0*src_strideq]
    movu         m1, [srcq + 1*src_strideq]
    movu         m2, [srcq + 2*src_strideq]
    movu         m3, [srcq + src_stride3q ]

    ; src -= dst
    psubw        m0, [dstq + 0*dst_strideq]
    psubw        m1, [dstq + 1*dst_strideq]
    psubw        m2, [dstq + 2*dst_strideq]
    psubw        m3, [dstq + dst_stride3q ]

.10bpc_main:

    ; Original permutation
    ; m0    0   1   2   3      4   5   6   7      8   9  10  11     12  13  14  15
    ; m1   16  17  18  19     20  21  22  23     24  25  26  27     28  29  30  31
    ; m2   32  33  34  35     36  37  38  39     40  41  42  43     44  45  46  47
    ; m3   48  49  50  51     52  53  54  55     56  57  58  59     60  61  62  63

    ; Two registers perform 2 4x4 transforms in parallel

    punpcklqdq  m4, m0, m2
    punpcklqdq  m5, m1, m3

    punpckhqdq  m0, m0, m2
    punpckhqdq  m1, m1, m3

    SWAP 4, 3
    SWAP 5, 4

    ; New permutation
    ; m0    0   1   2   3     32  33  34  35      8   9  10  11     40  41  42  43
    ; m1   16  17  18  19     48  49  50  51     24  25  26  27     56  57  58  59
    ; m3    4   5   6   7     36  37  38  39     12  13  14  15     44  45  46  47
    ; m4   20  21  22  23     52  53  54  55     28  29  30  31     60  61  62  63

    BUTTERFLY           16, 32, 0
    BUTTERFLY           16, 32, 1
    INTERLEAVE          16, 32, 0
    INTERLEAVE          16, 32, 1
    BUTTERFLY           16, 32, 0
    BUTTERFLY           16, 32, 1
    INTERLEAVE_PAIRS    16, 32, 0
    INTERLEAVE_PAIRS    16, 32, 1
    BUTTERFLY           16, 32, 0
    BUTTERFLY           16, 32, 1
    INTERLEAVE          16, 32, 0
    INTERLEAVE          16, 32, 1
    BUTTERFLY           16, 32, 0
    BUTTERFLY           16, 32, 1

    pabsw       m0, m0
    pabsw       m1, m1
    pabsw       m3, m3
    pabsw       m4, m4

    paddw       m0, m1
    paddw       m3, m4
    paddw       m0, m3

    HSUM 16, 32, 0, 1, eax
    NORMALIZE4PT
    RET
.12bpc:
    RESET_MM_PERMUTATION

    mov        bdmaxd, 2
    pxor       m6, m6
.12bpc_loop:
    movu       xm0, [srcq + 0*src_strideq]
    movu       xm1, [srcq + 1*src_strideq]
    movu       xm2, [srcq + 2*src_strideq]
    movu       xm3, [srcq + src_stride3q ]

    psubw      xm0, [dstq + 0*dst_strideq]
    psubw      xm1, [dstq + 1*dst_strideq]
    psubw      xm2, [dstq + 2*dst_strideq]
    psubw      xm3, [dstq + dst_stride3q ]

    pmovsxwd    m0, xm0
    pmovsxwd    m1, xm1
    pmovsxwd    m2, xm2
    pmovsxwd    m3, xm3

    add     srcq, 16
    add     dstq, 16

    HADAMARD_8X4_12BPC
    paddd   m6, m0
    dec     bdmaxd
    jnz     .12bpc_loop

    HSUM 32, 32, 6, 1, eax
    NORMALIZE4PT
    RET

INIT_YMM avx2
cglobal satd_4x16_hbd, 5, 7, 12, src, src_stride, dst, dst_stride, bdmax, \
                               src_stride3, dst_stride3
    lea         src_stride3q, [3*src_strideq]
    lea         dst_stride3q, [3*dst_strideq]

    cmp         bdmaxd, (1 << 10) - 1
    jne         .12bpc

    ; BLOCK 1
    movq        xm0, [srcq + 0*src_strideq]
    movq        xm1, [srcq + 1*src_strideq]
    movq        xm2, [srcq + 2*src_strideq]
    movq        xm3, [srcq + src_stride3q ]
    lea        srcq, [srcq + 4*src_strideq]

    psubw       xm0, [dstq + 0*dst_strideq]
    psubw       xm1, [dstq + 1*dst_strideq]
    psubw       xm2, [dstq + 2*dst_strideq]
    psubw       xm3, [dstq + dst_stride3q ]
    lea        dstq, [dstq + 4*dst_strideq]

    ; BLOCK 2
    movq        xm4, [srcq + 0*src_strideq]
    movq        xm5, [srcq + 1*src_strideq]
    movq        xm6, [srcq + 2*src_strideq]
    movq        xm7, [srcq + src_stride3q ]
    lea        srcq, [srcq + 4*src_strideq]

    psubw       xm4, [dstq + 0*dst_strideq]
    psubw       xm5, [dstq + 1*dst_strideq]
    psubw       xm6, [dstq + 2*dst_strideq]
    psubw       xm7, [dstq + dst_stride3q ]
    lea        dstq, [dstq + 4*dst_strideq]

    vinserti128 m0, m0, xm4, 1
    vinserti128 m1, m1, xm5, 1
    vinserti128 m2, m2, xm6, 1
    vinserti128 m3, m3, xm7, 1

    ; BLOCK 3
    movq        xm4, [srcq + 0*src_strideq]
    movq        xm5, [srcq + 1*src_strideq]
    movq        xm6, [srcq + 2*src_strideq]
    movq        xm7, [srcq + src_stride3q ]
    lea        srcq, [srcq + 4*src_strideq]

    psubw       xm4, [dstq + 0*dst_strideq]
    psubw       xm5, [dstq + 1*dst_strideq]
    psubw       xm6, [dstq + 2*dst_strideq]
    psubw       xm7, [dstq + dst_stride3q ]
    lea        dstq, [dstq + 4*dst_strideq]

    ; BLOCK 4
    movq        xm8, [srcq + 0*src_strideq]
    movq        xm9, [srcq + 1*src_strideq]
    movq       xm10, [srcq + 2*src_strideq]
    movq       xm11, [srcq + src_stride3q ]

    psubw       xm8, [dstq + 0*dst_strideq]
    psubw       xm9, [dstq + 1*dst_strideq]
    psubw      xm10, [dstq + 2*dst_strideq]
    psubw      xm11, [dstq + dst_stride3q ]

    vinserti128  m4, m4, xm8,  1
    vinserti128  m5, m5, xm9,  1
    vinserti128  m6, m6, xm10, 1
    vinserti128  m7, m7, xm11, 1

    punpcklqdq   m0, m0, m4
    punpcklqdq   m1, m1, m5
    punpcklqdq   m2, m2, m6
    punpcklqdq   m3, m3, m7

    jmp     m(satd_16x4_hbd).10bpc_main

.12bpc:
    mov     bdmaxd, 2
    pxor    m8, m8

.12bpc_loop:
    ; BLOCK 1
    movq       xm0, [srcq + 0*src_strideq]
    movq       xm1, [srcq + 1*src_strideq]
    movq       xm2, [srcq + 2*src_strideq]
    movq       xm3, [srcq + src_stride3q ]
    lea       srcq, [srcq + 4*src_strideq]

    psubw      xm0, [dstq + 0*dst_strideq]
    psubw      xm1, [dstq + 1*dst_strideq]
    psubw      xm2, [dstq + 2*dst_strideq]
    psubw      xm3, [dstq + dst_stride3q ]
    lea       dstq, [dstq + 4*dst_strideq]

    pmovsxwd    xm0, xm0
    pmovsxwd    xm1, xm1
    pmovsxwd    xm2, xm2
    pmovsxwd    xm3, xm3

    ; BLOCK 2
    movq       xm4, [srcq + 0*src_strideq]
    movq       xm5, [srcq + 1*src_strideq]
    movq       xm6, [srcq + 2*src_strideq]
    movq       xm7, [srcq + src_stride3q ]
    lea       srcq, [srcq + 4*src_strideq]

    psubw      xm4, [dstq + 0*dst_strideq]
    psubw      xm5, [dstq + 1*dst_strideq]
    psubw      xm6, [dstq + 2*dst_strideq]
    psubw      xm7, [dstq + dst_stride3q ]
    lea       dstq, [dstq + 4*dst_strideq]

    pmovsxwd    xm4, xm4
    pmovsxwd    xm5, xm5
    pmovsxwd    xm6, xm6
    pmovsxwd    xm7, xm7

    vinserti128 m0, m0, xm4, 1
    vinserti128 m1, m1, xm5, 1
    vinserti128 m2, m2, xm6, 1
    vinserti128 m3, m3, xm7, 1

    HADAMARD_8X4_12BPC
    paddd   m8, m0
    dec     bdmaxd
    jnz     .12bpc_loop

    HSUM 32, 32, 8, 0, eax
    NORMALIZE4PT
    RET

INIT_YMM avx2
cglobal satd_8x4_hbd, 5, 7, 12, src, src_stride, dst, dst_stride, bdmax, \
                               src_stride3, dst_stride3
    lea         src_stride3q, [3*src_strideq]
    lea         dst_stride3q, [3*dst_strideq]

    cmp         bdmaxd, (1 << 10) - 1
    jne         .12bpc

    ; Load src rows
    movu        xm0, [srcq + 0*src_strideq]
    movu        xm1, [srcq + 1*src_strideq]
    movu        xm2, [srcq + 2*src_strideq]
    movu        xm3, [srcq + src_stride3q ]

    ; src -= dst
    psubw       xm0, [dstq + 0*dst_strideq]
    psubw       xm1, [dstq + 1*dst_strideq]
    psubw       xm2, [dstq + 2*dst_strideq]
    psubw       xm3, [dstq + dst_stride3q ]

.10bpc_main:
    HADAMARD_4X4_PACKED 16, 32

    pabsw   m0, m0
    pabsw   m1, m1
    paddw   m0, m1
    HSUM    16, 32, 0, 1, eax
    NORMALIZE4PT
    RET
.12bpc:
    RESET_MM_PERMUTATION

    pmovzxwd    m0, [srcq + 0*src_strideq]
    pmovzxwd    m1, [srcq + 1*src_strideq]
    pmovzxwd    m2, [srcq + 2*src_strideq]
    pmovzxwd    m3, [srcq + src_stride3q ]

    pmovzxwd    m4, [dstq + 0*dst_strideq]
    pmovzxwd    m5, [dstq + 1*dst_strideq]
    pmovzxwd    m6, [dstq + 2*dst_strideq]
    pmovzxwd    m7, [dstq + dst_stride3q ]

    ; src -= dst
    psubd       m0, m4
    psubd       m1, m5
    psubd       m2, m6
    psubd       m3, m7

.12bpc_main:
    HADAMARD_8X4_12BPC
    HSUM 32, 32, 0, 1, eax
    NORMALIZE4PT
    RET

INIT_YMM avx2
cglobal satd_4x8_hbd, 5, 7, 12, src, src_stride, dst, dst_stride, bdmax, \
                               src_stride3, dst_stride3
    lea         src_stride3q, [3*src_strideq]
    lea         dst_stride3q, [3*dst_strideq]

    cmp         bdmaxd, (1 << 10) - 1
    jne         .12bpc

    movq        xm0, [srcq + 0*src_strideq]
    movq        xm1, [srcq + 1*src_strideq]
    movq        xm2, [srcq + 2*src_strideq]
    movq        xm3, [srcq + src_stride3q ]
    lea        srcq, [srcq + 4*src_strideq]
    movq        xm4, [srcq + 0*src_strideq]
    movq        xm5, [srcq + 1*src_strideq]
    movq        xm6, [srcq + 2*src_strideq]
    movq        xm7, [srcq + src_stride3q ]

    ; This loads past the number of elements we are technically supposed
    ; to read, however, this should still be safe, since at least one
    ; valid element is in the memory address.
    psubw       xm0, [dstq + 0*dst_strideq]
    psubw       xm1, [dstq + 1*dst_strideq]
    psubw       xm2, [dstq + 2*dst_strideq]
    psubw       xm3, [dstq + dst_stride3q ]
    lea        dstq, [dstq + 4*dst_strideq]
    psubw       xm4, [dstq + 0*dst_strideq]
    psubw       xm5, [dstq + 1*dst_strideq]
    psubw       xm6, [dstq + 2*dst_strideq]
    psubw       xm7, [dstq + dst_stride3q ]

    punpcklqdq  xm0, xm0, xm4
    punpcklqdq  xm1, xm1, xm5
    punpcklqdq  xm2, xm2, xm6
    punpcklqdq  xm3, xm3, xm7

    ; Jump to HADAMARD_4X4_PACKED in 8x4 satd, this saves us some binary size
    ; by deduplicating the shared code.
    jmp m(satd_8x4_hbd).10bpc_main
    ; No return; we return in the other function.

.12bpc:
    RESET_MM_PERMUTATION

    pmovzxwd    xm0, [srcq + 0*src_strideq]
    pmovzxwd    xm1, [srcq + 1*src_strideq]
    pmovzxwd    xm2, [srcq + 2*src_strideq]
    pmovzxwd    xm3, [srcq + src_stride3q ]
    lea        srcq, [srcq + 4*src_strideq]

    pmovzxwd    xm4, [dstq + 0*dst_strideq]
    pmovzxwd    xm5, [dstq + 1*dst_strideq]
    pmovzxwd    xm6, [dstq + 2*dst_strideq]
    pmovzxwd    xm7, [dstq + dst_stride3q ]
    lea        dstq, [dstq + 4*dst_strideq]

    ; src -= dst
    psubd       xm0, xm4
    psubd       xm1, xm5
    psubd       xm2, xm6
    psubd       xm3, xm7

    pmovzxwd    xm4, [srcq + 0*src_strideq]
    pmovzxwd    xm5, [srcq + 1*src_strideq]
    pmovzxwd    xm6, [srcq + 2*src_strideq]
    pmovzxwd    xm7, [srcq + src_stride3q ]

    pmovzxwd    xm8, [dstq + 0*dst_strideq]
    pmovzxwd    xm9, [dstq + 1*dst_strideq]
    pmovzxwd   xm10, [dstq + 2*dst_strideq]
    pmovzxwd   xm11, [dstq + dst_stride3q ]

    ; src -= dst (second block)
    psubd       xm4, xm8
    psubd       xm5, xm9
    psubd       xm6, xm10
    psubd       xm7, xm11

    vinserti128 m0, m0, xm4, 1
    vinserti128 m1, m1, xm5, 1
    vinserti128 m2, m2, xm6, 1
    vinserti128 m3, m3, xm7, 1

    ; Jump to HADAMARD_4X4_PACKED in 8x4 satd, this saves us some binary size
    ; by deduplicating the shared code.
    jmp m(satd_8x4_hbd).12bpc_main
    ; No return; we return in the other function.


; <width>, <height>
%macro SATD_NXM 2

INIT_YMM avx2
cglobal satd_%1x%2_hbd, 5, 10, 16, src, src_stride, dst, dst_stride, bdmax, \
                                 src_stride3, dst_stride3, nsrc_stride4, ndst_stride4, rows
    lea     nsrc_stride4q, [4*src_strideq]
    lea     ndst_stride4q, [4*dst_strideq]

    lea     src_stride3q, [3*src_strideq]
    lea     dst_stride3q, [3*dst_strideq]

    neg     nsrc_stride4q
    neg     ndst_stride4q

    pxor    m9, m9

    ; Height contains the number of rows.
    mov     rowsd, %2/8
.outer:
    mov     bdmaxd, %1/8

; Loop over blocks in same row.
.loop:
    LOAD_DIFF_8X8

    ; Fix up pointers and go to next block in same row.
    lea     srcq, [srcq + nsrc_stride4q + 16]
    lea     dstq, [dstq + ndst_stride4q + 16]

    call    m(satd_8x8_hbd_internal)
    dec     bdmaxd
    jnz     .loop

    lea     srcq, [srcq + 8*src_strideq - (%1*16)/8]
    lea     dstq, [dstq + 8*dst_strideq - (%1*16)/8]

    dec     rowsd
    jnz     .outer

    HSUM 32, 32, 9, 0, eax
    NORMALIZE8PT
    RET
%endmacro

%macro SATD_NX8 1
INIT_YMM avx2
cglobal satd_%1x8_hbd, 5, 9, 16, src, src_stride, dst, dst_stride, bdmax, \
                                 src_stride3, dst_stride3, nsrc_stride4, ndst_stride4
    lea     nsrc_stride4q, [4*src_strideq]
    lea     ndst_stride4q, [4*dst_strideq]

    lea     src_stride3q, [3*src_strideq]
    lea     dst_stride3q, [3*dst_strideq]

    neg     nsrc_stride4q
    neg     ndst_stride4q

    pxor    m9, m9
    mov     bdmaxd, %1/8

.loop:
    LOAD_DIFF_8X8

    lea     srcq, [srcq + nsrc_stride4q + 16]
    lea     dstq, [dstq + ndst_stride4q + 16]

    call    m(satd_8x8_hbd_internal)
    dec     bdmaxd
    jnz     .loop

    HSUM 32, 32, 9, 0, eax
    NORMALIZE8PT
    RET
%endmacro

%macro SATD_8XM 1
INIT_YMM avx2
cglobal satd_8x%1_hbd, 5, 7, 16, src, src_stride, dst, dst_stride, bdmax, \
                                 src_stride3, dst_stride3
    lea     src_stride3q, [3*src_strideq]
    lea     dst_stride3q, [3*dst_strideq]

    pxor    m9, m9
    mov     bdmaxd, %1/8

.loop:
    LOAD_DIFF_8X8

    lea     srcq, [srcq + 4*src_strideq]
    lea     dstq, [dstq + 4*dst_strideq]

    call    m(satd_8x8_hbd_internal)
    dec     bdmaxd
    jnz     .loop

    HSUM 32, 32, 9, 0, eax
    NORMALIZE8PT
    RET
%endmacro

SATD_NXM 16, 16
SATD_NXM 32, 32
SATD_NXM 64, 64
SATD_NXM 128, 128
SATD_NXM 16, 32
SATD_NXM 16, 64
SATD_NXM 32, 16
SATD_NXM 32, 64
SATD_NXM 64, 16
SATD_NXM 64, 32
SATD_NXM 64, 128
SATD_NXM 128, 64
SATD_NX8 16
SATD_NX8 32
SATD_8XM 16
SATD_8XM 32

%endif ; ARCH_X86_64
