// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Negated modular inverse, z := (-1/x) mod 2^{64k}
// Input x[k]; output z[k]
//
//    extern void bignum_negmodinv(uint64_t k, uint64_t *z, const uint64_t *x);
//
// Assuming x is odd (otherwise nothing makes sense) the result satisfies
//
//       x * z + 1 == 0 (mod 2^{64 * k})
//
// but is not necessarily reduced mod x.
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_negmodinv)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_negmodinv)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_negmodinv)
        .text
        .balign 4

#define k x0
#define z x1
#define x x2

#define w x3
#define a x4
#define m x5
#define h x6
#define l x7
#define e x8
#define i x9



S2N_BN_SYMBOL(bignum_negmodinv):
        CFI_START

// If k = 0 do nothing

        cbz     k, Lbignum_negmodinv_end

// Compute word-level negated modular inverse w for x[0].

        ldr     a, [x]
        lsl     w, a, #2
        sub     w, a, w
        eor     w, w, #2
        mov     h, #1
        madd    h, a, w, h
        mul     l, h, h
        madd    w, h, w, w
        mul     h, l, l
        madd    w, l, w, w
        mul     l, h, h
        madd    w, h, w, w
        madd    w, l, w, w

// Write that as lowest word of the output, then if k = 1 we're finished

        str     w, [z]
        cmp     k, #1
        beq     Lbignum_negmodinv_end

// Otherwise compute and write the other digits (1..k-1) of w * x + 1.
// Note that at this point CF was set by the comparison (subtraction) "k - 1".
// Since k >= 2 if we got here, this subtraction didn't carry; allowing
// for the inverted carry on ARM that means that CF is guaranteed to be set.
// This allows us to ignore the nominal "a * w + 1" from adding the low
// part of the product, since its only contribution is to set the carry
// flag. Thus, we only calculate the high part of a * w explicitly.

        umulh   h, a, w
        mov     i, #1
Lbignum_negmodinv_initloop:
        ldr     a, [x, i, lsl #3]
        mul     l, a, w
        adcs    l, l, h
        umulh   h, a, w
        str     l, [z, i, lsl #3]
        add     i, i, #1
        sub     a, k, i
        cbnz    a, Lbignum_negmodinv_initloop

// For simpler indexing, z := z + 8 and k := k - 1 per outer iteration
// Then we can use the same index for x and for z and effective size k.
//
// But we also offset k by 1 so the "real" size is k + 1, which is why the
// test at the end of the inner loop is i < k <=> i' = i + 1 < k + 1.
// This lets us avoid some special cases inside the loop at the cost
// of needing the additional "finale" tail for the final iteration
// since we do one outer loop iteration too few.

        subs    k, k, #2
        beq     Lbignum_negmodinv_finale

Lbignum_negmodinv_outerloop:
        add     z, z, #8
        ldr     e, [z]
        mul     m, e, w
        str     m, [z]
        ldr     a, [x]
        umulh   h, a, m
        subs    xzr, e, #1         // Effective carry from a * m + e
        mov     i, #1
Lbignum_negmodinv_innerloop:
        ldr     a, [x, i, lsl #3]
        ldr     e, [z, i, lsl #3]
        mul     l, a, m
        adcs    e, e, h
        umulh   h, a, m
        adc     h, h, xzr
        adds    e, e, l
        str     e, [z, i, lsl #3]
        sub     a, i, k
        add     i, i, #1
        cbnz    a, Lbignum_negmodinv_innerloop

        subs    k, k, #1
        bne     Lbignum_negmodinv_outerloop

Lbignum_negmodinv_finale:
        ldr     e, [z, #8]
        mul     m, e, w
        str     m, [z, #8]

Lbignum_negmodinv_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_negmodinv)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
