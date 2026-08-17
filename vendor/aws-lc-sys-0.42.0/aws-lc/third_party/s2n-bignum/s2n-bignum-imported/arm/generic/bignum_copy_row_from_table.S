// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Given table: uint64_t[height*width], copy table[idx*width...(idx+1)*width-1]
// into z[0..width-1].
// This function is constant-time with respect to the value of `idx`. This is
// achieved by reading the whole table and using the bit-masking to get the
// `idx`-th row.
//
//    extern void bignum_copy_row_from_table
//     (uint64_t *z, const uint64_t *table, uint64_t height, uint64_t width,
//      uint64_t idx);
//
// Standard ARM ABI: X0 = z, X1 = table, X2 = height, X3 = width, X4 = idx
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_copy_row_from_table)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_copy_row_from_table)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_copy_row_from_table)
        .text
        .balign 4

#define z x0
#define table x1
#define height x2
#define width x3
#define idx x4

#define i x5
#define mask x6
#define j x7

S2N_BN_SYMBOL(bignum_copy_row_from_table):
        CFI_START

        cbz     height, Lbignum_copy_row_from_table_end
        cbz     width,  Lbignum_copy_row_from_table_end
        mov     i, width
        mov     x6, z

Lbignum_copy_row_from_table_initzero:
        str     xzr, [x6]
        add     x6, x6, #8
        subs    i, i, #1
        bne     Lbignum_copy_row_from_table_initzero

        mov     i, xzr
        mov     x8, table

Lbignum_copy_row_from_table_outerloop:

        cmp     i, idx
        csetm   mask, eq

        mov     j, width
        mov     x9, z

Lbignum_copy_row_from_table_innerloop:

        ldr     x10, [x8]
        ldr     x11, [x9]
        and     x10, x10, mask
        orr     x11, x11, x10
        str     x11, [x9]

        add     x8, x8, #8
        add     x9, x9, #8
        subs    j, j, #1
        bne     Lbignum_copy_row_from_table_innerloop

Lbignum_copy_row_from_table_innerloop_done:
        add     i, i, #1
        cmp     i, height
        bne     Lbignum_copy_row_from_table_outerloop

Lbignum_copy_row_from_table_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_copy_row_from_table)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
