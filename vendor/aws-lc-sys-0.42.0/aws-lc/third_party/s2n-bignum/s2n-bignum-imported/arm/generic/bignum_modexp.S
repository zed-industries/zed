// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Modular exponentiation for arbitrary odd modulus
// Inputs a[k], p[k], m[k]; output z[k], temporary buffer t[>=3*k]
//
//   extern void bignum_modexp
//    (uint64_t k,uint64_t *z, const uint64_t *a,const uint64_t *p,
//     const uint64_t *m,uint64_t *t);
//
// Does z := (a^p) mod m where all numbers are k-digit and m is odd
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = a, X3 = p, X4 = m, X5 = t
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_modexp)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_modexp)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_modexp)

        .text
        .balign 4

// Local variables, all held in extra registers

#define k x19
#define res x20
#define a x21
#define p x22
#define m x23
#define x x24
#define i x25
#define y x, k, lsl #3
#define z x, k, lsl #4

S2N_BN_SYMBOL(bignum_modexp):
        CFI_START

// Save some registers including link register

        CFI_PUSH2(x19,x20)
        CFI_PUSH2(x21,x22)
        CFI_PUSH2(x23,x24)
        CFI_PUSH2(x25,x30)

// If size is zero (which falsifies the oddness condition) do nothing

        cbz     x0, Lbignum_modexp_end

// Move parameters into permanent homes

        mov     k, x0
        mov     res, x1
        mov     a, x2
        mov     p, x3
        mov     m, x4
        mov     x, x5

// Let x == 2^64k * a (mod m) and initialize z == 2^64k * 1 (mod m)

        mov     x0, k
        add     x1, z
        mov     x2, m
        add     x3, y
        CFI_BL(Lbignum_modexp_local_amontifier)

        mov     x0, k
        mov     x1, x
        add     x2, z
        mov     x3, a
        mov     x4, m
        CFI_BL(Lbignum_modexp_local_amontmul)

        mov     x0, k
        add     x1, z
        add     x2, z
        mov     x3, m
        CFI_BL(Lbignum_modexp_local_demont)

// Main loop with z == 2^64k * a^(p >> 2^i) (mod m)

        lsl     i, k, #6

Lbignum_modexp_loop:
        sub     i, i, #1

        mov     x0, k
        add     x1, y
        add     x2, z
        add     x3, z
        mov     x4, m
        CFI_BL(Lbignum_modexp_local_amontmul)

        mov     x0, k
        add     x1, z
        mov     x2, x
        add     x3, y
        mov     x4, m
        CFI_BL(Lbignum_modexp_local_amontmul)

        lsr     x0, i, #6
        ldr     x0, [p, x0, lsl #3]
        lsr     x0, x0, i
        and     x0, x0, #1

        mov     x1, k
        add     x2, z
        add     x3, z
        add     x4, y
        CFI_BL(Lbignum_modexp_local_mux)

        cbnz    i, Lbignum_modexp_loop

// Convert back from Montgomery representation and copy the result
// (via a degenerate case of multiplexing) into the output buffer

        mov     x0, k
        add     x1, z
        add     x2, z
        mov     x3, m
        CFI_BL(Lbignum_modexp_local_demont)

        mov     x0, xzr
        mov     x1, k
        mov     x2, res
        add     x3, z
        add     x4, z
        CFI_BL(Lbignum_modexp_local_mux)

// Restore registers and return

Lbignum_modexp_end:

        CFI_POP2(x25,x30)
        CFI_POP2(x23,x24)
        CFI_POP2(x21,x22)
        CFI_POP2(x19,x20)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_modexp)

// Local copy of bignum_amontifier

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_modexp_local_amontifier)

Lbignum_modexp_local_amontifier:
        CFI_START
        cbz     x0, Lbignum_modexp_amontifend
        mov     x4, xzr
Lbignum_modexp_copyinloop:
        ldr     x9, [x2, x4, lsl #3]
        str     x9, [x3, x4, lsl #3]
        add     x4, x4, #0x1
        cmp     x4, x0
        b.cc    Lbignum_modexp_copyinloop
        subs    x4, x0, #0x1
        b.eq    Lbignum_modexp_normalized
Lbignum_modexp_normloop:
        mov     x5, xzr
        cmp     x9, xzr
        mov     x7, xzr
Lbignum_modexp_shufloop:
        mov     x9, x7
        ldr     x7, [x3, x5, lsl #3]
        csel    x9, x9, x7, eq
        str     x9, [x3, x5, lsl #3]
        add     x5, x5, #0x1
        sub     x11, x5, x0
        cbnz    x11, Lbignum_modexp_shufloop
        subs    x4, x4, #0x1
        b.ne    Lbignum_modexp_normloop
Lbignum_modexp_normalized:
        clz     x9, x9
        mov     x10, xzr
        mov     x4, xzr
        tst     x9, #0x3f
        csetm   x8, ne
        neg     x11, x9
Lbignum_modexp_bitloop:
        ldr     x5, [x3, x4, lsl #3]
        lsl     x7, x5, x9
        orr     x7, x7, x10
        lsr     x10, x5, x11
        and     x10, x10, x8
        str     x7, [x3, x4, lsl #3]
        add     x4, x4, #0x1
        cmp     x4, x0
        b.cc    Lbignum_modexp_bitloop
        sub     x6, x0, #0x1
        ldr     x6, [x3, x6, lsl #3]
        mov     x11, #0x1
        neg     x10, x6
        mov     x4, #0x3e
Lbignum_modexp_estloop:
        add     x11, x11, x11
        mov     x7, x6
        sub     x7, x7, x10
        cmp     x10, x7
        csetm   x7, cs
        sub     x11, x11, x7
        add     x10, x10, x10
        and     x7, x7, x6
        sub     x10, x10, x7
        subs    x4, x4, #0x1
        b.ne    Lbignum_modexp_estloop
        cmp     x10, x6
        cinc    x11, x11, eq
        mov     x9, xzr
        adds    x4, xzr, xzr
Lbignum_modexp_mulloop:
        ldr     x7, [x3, x4, lsl #3]
        mul     x8, x11, x7
        adcs    x8, x8, x9
        umulh   x9, x11, x7
        str     x8, [x1, x4, lsl #3]
        add     x4, x4, #0x1
        sub     x7, x4, x0
        cbnz    x7, Lbignum_modexp_mulloop
        adc     x9, x9, xzr
        mov     x7, #0x4000000000000000
        subs    x9, x9, x7
        csetm   x11, cs
        negs    x4, xzr
Lbignum_modexp_remloop:
        ldr     x7, [x3, x4, lsl #3]
        ldr     x10, [x1, x4, lsl #3]
        and     x7, x7, x11
        sbcs    x7, x7, x10
        str     x7, [x1, x4, lsl #3]
        add     x4, x4, #0x1
        sub     x7, x4, x0
        cbnz    x7, Lbignum_modexp_remloop
        mov     x9, xzr
        negs    x5, xzr
Lbignum_modexp_dubloop1:
        ldr     x7, [x1, x5, lsl #3]
        extr    x9, x7, x9, #63
        ldr     x10, [x3, x5, lsl #3]
        sbcs    x9, x9, x10
        str     x9, [x1, x5, lsl #3]
        mov     x9, x7
        add     x5, x5, #0x1
        sub     x7, x5, x0
        cbnz    x7, Lbignum_modexp_dubloop1
        lsr     x9, x9, #63
        sbc     x9, x9, xzr
        adds    x5, xzr, xzr
Lbignum_modexp_corrloop1:
        ldr     x7, [x1, x5, lsl #3]
        ldr     x10, [x3, x5, lsl #3]
        and     x10, x10, x9
        adcs    x7, x7, x10
        str     x7, [x1, x5, lsl #3]
        add     x5, x5, #0x1
        sub     x7, x5, x0
        cbnz    x7, Lbignum_modexp_corrloop1
        mov     x9, xzr
        negs    x5, xzr
Lbignum_modexp_dubloop2:
        ldr     x7, [x1, x5, lsl #3]
        extr    x9, x7, x9, #63
        ldr     x10, [x3, x5, lsl #3]
        sbcs    x9, x9, x10
        str     x9, [x1, x5, lsl #3]
        mov     x9, x7
        add     x5, x5, #0x1
        sub     x7, x5, x0
        cbnz    x7, Lbignum_modexp_dubloop2
        lsr     x9, x9, #63
        sbc     x9, x9, xzr
        adds    x5, xzr, xzr
Lbignum_modexp_corrloop2:
        ldr     x7, [x1, x5, lsl #3]
        ldr     x10, [x3, x5, lsl #3]
        and     x10, x10, x9
        adcs    x7, x7, x10
        str     x7, [x1, x5, lsl #3]
        str     x7, [x3, x5, lsl #3]
        add     x5, x5, #0x1
        sub     x7, x5, x0
        cbnz    x7, Lbignum_modexp_corrloop2
        mov     x6, xzr
        mov     x4, x0
Lbignum_modexp_modloop:
        mov     x5, xzr
        mov     x10, xzr
        adds    x9, xzr, xzr
Lbignum_modexp_cmaloop:
        ldr     x7, [x1, x5, lsl #3]
        mul     x8, x6, x7
        adcs    x10, x10, x9
        umulh   x9, x6, x7
        adc     x9, x9, xzr
        adds    x8, x10, x8
        ldr     x10, [x3, x5, lsl #3]
        str     x8, [x3, x5, lsl #3]
        add     x5, x5, #0x1
        sub     x7, x5, x0
        cbnz    x7, Lbignum_modexp_cmaloop
        adcs    x6, x10, x9
        csetm   x8, cs
        adds    x5, xzr, xzr
Lbignum_modexp_oaloop:
        ldr     x7, [x3, x5, lsl #3]
        ldr     x10, [x1, x5, lsl #3]
        and     x10, x10, x8
        adcs    x7, x7, x10
        str     x7, [x3, x5, lsl #3]
        add     x5, x5, #0x1
        sub     x7, x5, x0
        cbnz    x7, Lbignum_modexp_oaloop
        adc     x6, x6, xzr
        subs    x4, x4, #0x1
        b.ne    Lbignum_modexp_modloop
        ldr     x7, [x2]
        lsl     x11, x7, #2
        sub     x11, x7, x11
        eor     x11, x11, #0x2
        mov     x8, #0x1
        madd    x9, x7, x11, x8
        mul     x10, x9, x9
        madd    x11, x9, x11, x11
        mul     x9, x10, x10
        madd    x11, x10, x11, x11
        mul     x10, x9, x9
        madd    x11, x9, x11, x11
        madd    x11, x10, x11, x11
        ldr     x10, [x3]
        mul     x11, x10, x11
        mul     x8, x11, x7
        umulh   x9, x11, x7
        mov     x5, #0x1
        sub     x7, x0, #0x1
        cmn     x10, x8
        cbz     x7, Lbignum_modexp_montifend
Lbignum_modexp_montifloop:
        ldr     x7, [x2, x5, lsl #3]
        ldr     x10, [x3, x5, lsl #3]
        mul     x8, x11, x7
        adcs    x10, x10, x9
        umulh   x9, x11, x7
        adc     x9, x9, xzr
        adds    x10, x10, x8
        sub     x7, x5, #0x1
        str     x10, [x3, x7, lsl #3]
        add     x5, x5, #0x1
        sub     x7, x5, x0
        cbnz    x7, Lbignum_modexp_montifloop
Lbignum_modexp_montifend:
        adcs    x6, x6, x9
        csetm   x8, cs
        sub     x7, x0, #0x1
        str     x6, [x3, x7, lsl #3]
        negs    x5, xzr
Lbignum_modexp_osloop:
        ldr     x7, [x3, x5, lsl #3]
        ldr     x10, [x2, x5, lsl #3]
        and     x10, x10, x8
        sbcs    x7, x7, x10
        str     x7, [x1, x5, lsl #3]
        add     x5, x5, #0x1
        sub     x7, x5, x0
        cbnz    x7, Lbignum_modexp_osloop
Lbignum_modexp_amontifend:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_modexp_local_amontifier)

// Local copy of bignum_amontmul

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_modexp_local_amontmul)

Lbignum_modexp_local_amontmul:
        CFI_START
        cbz     x0, Lbignum_modexp_amomend
        ldr     x14, [x4]
        lsl     x5, x14, #2
        sub     x5, x14, x5
        eor     x5, x5, #0x2
        mov     x6, #0x1
        madd    x6, x14, x5, x6
        mul     x7, x6, x6
        madd    x5, x6, x5, x5
        mul     x6, x7, x7
        madd    x5, x7, x5, x5
        mul     x7, x6, x6
        madd    x5, x6, x5, x5
        madd    x5, x7, x5, x5
        mov     x8, xzr
Lbignum_modexp_zoop:
        str     xzr, [x1, x8, lsl #3]
        add     x8, x8, #0x1
        cmp     x8, x0
        b.cc    Lbignum_modexp_zoop
        mov     x6, xzr
        mov     x8, xzr
Lbignum_modexp_outerloop:
        ldr     x9, [x2, x8, lsl #3]
        mov     x10, xzr
        adds    x11, xzr, xzr
Lbignum_modexp_maddloop:
        ldr     x14, [x3, x10, lsl #3]
        ldr     x12, [x1, x10, lsl #3]
        mul     x13, x9, x14
        adcs    x12, x12, x11
        umulh   x11, x9, x14
        adc     x11, x11, xzr
        adds    x12, x12, x13
        str     x12, [x1, x10, lsl #3]
        add     x10, x10, #0x1
        sub     x14, x10, x0
        cbnz    x14, Lbignum_modexp_maddloop
        adcs    x6, x6, x11
        adc     x7, xzr, xzr
        ldr     x12, [x1]
        mul     x9, x12, x5
        ldr     x14, [x4]
        mul     x13, x9, x14
        umulh   x11, x9, x14
        adds    x12, x12, x13
        mov     x10, #0x1
        sub     x14, x0, #0x1
        cbz     x14, Lbignum_modexp_montend
Lbignum_modexp_montloop:
        ldr     x14, [x4, x10, lsl #3]
        ldr     x12, [x1, x10, lsl #3]
        mul     x13, x9, x14
        adcs    x12, x12, x11
        umulh   x11, x9, x14
        adc     x11, x11, xzr
        adds    x12, x12, x13
        sub     x13, x10, #0x1
        str     x12, [x1, x13, lsl #3]
        add     x10, x10, #0x1
        sub     x14, x10, x0
        cbnz    x14, Lbignum_modexp_montloop
Lbignum_modexp_montend:
        adcs    x11, x6, x11
        adc     x6, x7, xzr
        sub     x13, x10, #0x1
        str     x11, [x1, x13, lsl #3]
        add     x8, x8, #0x1
        cmp     x8, x0
        b.cc    Lbignum_modexp_outerloop
        neg     x6, x6
        negs    x10, xzr
Lbignum_modexp_corrloop3:
        ldr     x14, [x1, x10, lsl #3]
        ldr     x12, [x4, x10, lsl #3]
        and     x12, x12, x6
        sbcs    x14, x14, x12
        str     x14, [x1, x10, lsl #3]
        add     x10, x10, #0x1
        sub     x14, x10, x0
        cbnz    x14, Lbignum_modexp_corrloop3
Lbignum_modexp_amomend:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_modexp_local_amontmul)

// Local copy of bignum_demont

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_modexp_local_demont)

Lbignum_modexp_local_demont:
        CFI_START
        cbz     x0, Lbignum_modexp_demontend
        ldr     x11, [x3]
        lsl     x4, x11, #2
        sub     x4, x11, x4
        eor     x4, x4, #0x2
        mov     x5, #0x1
        madd    x5, x11, x4, x5
        mul     x6, x5, x5
        madd    x4, x5, x4, x4
        mul     x5, x6, x6
        madd    x4, x6, x4, x4
        mul     x6, x5, x5
        madd    x4, x5, x4, x4
        madd    x4, x6, x4, x4
        mov     x5, xzr
Lbignum_modexp_iloop:
        ldr     x11, [x2, x5, lsl #3]
        str     x11, [x1, x5, lsl #3]
        add     x5, x5, #0x1
        cmp     x5, x0
        b.cc    Lbignum_modexp_iloop
        mov     x5, xzr
Lbignum_modexp_douterloop:
        ldr     x9, [x1]
        mul     x7, x9, x4
        ldr     x11, [x3]
        mul     x10, x7, x11
        umulh   x8, x7, x11
        adds    x9, x9, x10
        mov     x6, #0x1
        sub     x11, x0, #0x1
        cbz     x11, Lbignum_modexp_dmontend
Lbignum_modexp_dmontloop:
        ldr     x11, [x3, x6, lsl #3]
        ldr     x9, [x1, x6, lsl #3]
        mul     x10, x7, x11
        adcs    x9, x9, x8
        umulh   x8, x7, x11
        adc     x8, x8, xzr
        adds    x9, x9, x10
        sub     x10, x6, #0x1
        str     x9, [x1, x10, lsl #3]
        add     x6, x6, #0x1
        sub     x11, x6, x0
        cbnz    x11, Lbignum_modexp_dmontloop
Lbignum_modexp_dmontend:
        adc     x8, xzr, x8
        sub     x10, x6, #0x1
        str     x8, [x1, x10, lsl #3]
        add     x5, x5, #0x1
        cmp     x5, x0
        b.cc    Lbignum_modexp_douterloop
        negs    x6, xzr
Lbignum_modexp_cmploop:
        ldr     x11, [x1, x6, lsl #3]
        ldr     x9, [x3, x6, lsl #3]
        sbcs    xzr, x11, x9
        add     x6, x6, #0x1
        sub     x11, x6, x0
        cbnz    x11, Lbignum_modexp_cmploop
        csetm   x8, cs
        negs    x6, xzr
Lbignum_modexp_corrloop:
        ldr     x11, [x1, x6, lsl #3]
        ldr     x9, [x3, x6, lsl #3]
        and     x9, x9, x8
        sbcs    x11, x11, x9
        str     x11, [x1, x6, lsl #3]
        add     x6, x6, #0x1
        sub     x11, x6, x0
        cbnz    x11, Lbignum_modexp_corrloop
Lbignum_modexp_demontend:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_modexp_local_demont)

// Local copy of bignum_mux

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_modexp_local_mux)

Lbignum_modexp_local_mux:
        CFI_START
        cbz     x1, Lbignum_modexp_muxend
        cmp     x0, #0x0
Lbignum_modexp_muxloop:
        sub     x1, x1, #0x1
        ldr     x5, [x3, x1, lsl #3]
        ldr     x0, [x4, x1, lsl #3]
        csel    x5, x5, x0, ne
        str     x5, [x2, x1, lsl #3]
        cbnz    x1, Lbignum_modexp_muxloop
Lbignum_modexp_muxend:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_modexp_local_mux)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
