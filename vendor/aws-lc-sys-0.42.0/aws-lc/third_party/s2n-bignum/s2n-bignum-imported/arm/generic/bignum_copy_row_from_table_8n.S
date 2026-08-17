// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Given table: uint64_t[height*width], copy table[idx*width...(idx+1)*width-1]
// into z[0..width-1]. width must be a multiple of 8.
// This function is constant-time with respect to the value of `idx`. This is
// achieved by reading the whole table and using the bit-masking to get the
// `idx`-th row.
//
//    extern void bignum_copy_row_from_table_8n
//     (uint64_t *z, const uint64_t *table, uint64_t height, uint64_t width, uint64_t idx);
//
// Standard ARM ABI: X0 = z, X1 = table, X2 = height, X3 = width, X4 = idx
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_copy_row_from_table_8n)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_copy_row_from_table_8n)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_copy_row_from_table_8n)
        .text
        .balign 4


#define z      x0
#define table  x1
#define height x2
#define width  x3
#define idx    x4

#define i x5
#define mask x6
#define j x7

#define vmask v16

S2N_BN_SYMBOL(bignum_copy_row_from_table_8n):
        CFI_START

        cbz     height, Lbignum_copy_row_from_table_8n_end
        cbz     width,  Lbignum_copy_row_from_table_8n_end
        mov     i, width
        mov     x6, z
        dup     v16.2d, xzr

Lbignum_copy_row_from_table_8n_initzero:
        str     q16, [x6]
        str     q16, [x6, #16]
        str     q16, [x6, #32]
        str     q16, [x6, #48]
        add     x6, x6, #64
        subs    i, i, #8
        bne     Lbignum_copy_row_from_table_8n_initzero

        mov     i, xzr
        mov     x8, table

Lbignum_copy_row_from_table_8n_outerloop:

        cmp     i, idx
        csetm   mask, eq
        dup     vmask.2d, mask

        mov     j, width
        mov     x9, z

Lbignum_copy_row_from_table_8n_innerloop:

        ldr     q17, [x8]
        ldr     q18, [x9]
        bit     v18.16b, v17.16b, vmask.16b
        str     q18, [x9]

        ldr     q17, [x8, #16]
        ldr     q18, [x9, #16]
        bit     v18.16b, v17.16b, vmask.16b
        str     q18, [x9, #16]

        ldr     q17, [x8, #32]
        ldr     q18, [x9, #32]
        bit     v18.16b, v17.16b, vmask.16b
        str     q18, [x9, #32]

        ldr     q17, [x8, #48]
        ldr     q18, [x9, #48]
        bit     v18.16b, v17.16b, vmask.16b
        str     q18, [x9, #48]

        add     x8, x8, #64
        add     x9, x9, #64
        subs    j, j, #8
        bne     Lbignum_copy_row_from_table_8n_innerloop

Lbignum_copy_row_from_table_8n_innerloop_done:
        add     i, i, #1
        cmp     i, height
        bne     Lbignum_copy_row_from_table_8n_outerloop

Lbignum_copy_row_from_table_8n_end:
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_copy_row_from_table_8n)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
