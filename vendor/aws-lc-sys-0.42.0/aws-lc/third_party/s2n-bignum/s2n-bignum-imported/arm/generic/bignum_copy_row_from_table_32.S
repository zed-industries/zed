// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Given table: uint64_t[height*32], copy table[idx*32...(idx+1)*32-1]
// into z[0..row-1].
// This function is constant-time with respect to the value of `idx`. This is
// achieved by reading the whole table and using the bit-masking to get the
// `idx`-th row.
//
//    extern void bignum_copy_row_from_table_32
//     (uint64_t *z, const uint64_t *table, uint64_t height, uint64_t idx);
//
// Initial version written by Hanno Becker
// Standard ARM ABI: X0 = z, X1 = table, X2 = height, X3 = idx
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_arm.h"

        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_copy_row_from_table_32)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_copy_row_from_table_32)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_copy_row_from_table_32)
        .text
        .balign 4


// *****************************************************
// Main code
// *****************************************************

#define z      x0
#define tbl    x1
#define height x2
#define idx    x3

#define mask  x5
#define cnt   x6

#define ventry0 v20
#define qentry0 q20
#define ventry1 v21
#define qentry1 q21
#define ventry2 v22
#define qentry2 q22
#define ventry3 v23
#define qentry3 q23
#define ventry4 v24
#define qentry4 q24
#define ventry5 v25
#define qentry5 q25
#define ventry6 v26
#define qentry6 q26
#define ventry7 v27
#define qentry7 q27
#define ventry8 v28
#define qentry8 q28
#define ventry9 v29
#define qentry9 q29
#define ventry10 v30
#define qentry10 q30
#define ventry11 v31
#define qentry11 q31
#define ventry12 v0
#define qentry12 q0
#define ventry13 v1
#define qentry13 q1
#define ventry14 v2
#define qentry14 q2
#define ventry15 v3
#define qentry15 q3

#define vtmp     v16
#define qtmp     q16

#define vmask    v17

S2N_BN_SYMBOL(bignum_copy_row_from_table_32):
        CFI_START

    // Clear accumulator
    // Zeroing can be done via xor, but xor isn't formalized yet.
    dup ventry0.2d, xzr
    mov ventry1.16b,  ventry0.16b
    mov ventry2.16b,  ventry0.16b
    mov ventry3.16b,  ventry0.16b
    mov ventry4.16b,  ventry0.16b
    mov ventry5.16b,  ventry0.16b
    mov ventry6.16b,  ventry0.16b
    mov ventry7.16b,  ventry0.16b
    mov ventry8.16b,  ventry0.16b
    mov ventry9.16b,  ventry0.16b
    mov ventry10.16b,  ventry0.16b
    mov ventry11.16b,  ventry0.16b
    mov ventry12.16b,  ventry0.16b
    mov ventry13.16b,  ventry0.16b
    mov ventry14.16b,  ventry0.16b
    mov ventry15.16b,  ventry0.16b

    mov cnt, #0
Lbignum_copy_row_from_table_32_loop:

    // Compute mask: Check if current index matches target index
    subs xzr, cnt, idx
    cinv mask, xzr, eq
    dup vmask.2d, mask

    ldr qtmp, [tbl, #16*0]
    bit ventry0.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*1]
    bit ventry1.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*2]
    bit ventry2.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*3]
    bit ventry3.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*4]
    bit ventry4.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*5]
    bit ventry5.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*6]
    bit ventry6.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*7]
    bit ventry7.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*8]
    bit ventry8.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*9]
    bit ventry9.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*10]
    bit ventry10.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*11]
    bit ventry11.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*12]
    bit ventry12.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*13]
    bit ventry13.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*14]
    bit ventry14.16b, vtmp.16b, vmask.16b

    ldr qtmp, [tbl, #16*15]
    bit ventry15.16b, vtmp.16b, vmask.16b

    add tbl, tbl, #32*8

    add cnt, cnt, #1
    subs xzr, height, cnt
    b.ne Lbignum_copy_row_from_table_32_loop

Lbignum_copy_row_from_table_32_end:

    str qentry0,  [z, #16*0]
    str qentry1,  [z, #16*1]
    str qentry2,  [z, #16*2]
    str qentry3,  [z, #16*3]
    str qentry4,  [z, #16*4]
    str qentry5,  [z, #16*5]
    str qentry6,  [z, #16*6]
    str qentry7,  [z, #16*7]
    str qentry8,  [z, #16*8]
    str qentry9,  [z, #16*9]
    str qentry10,  [z, #16*10]
    str qentry11,  [z, #16*11]
    str qentry12,  [z, #16*12]
    str qentry13,  [z, #16*13]
    str qentry14,  [z, #16*14]
    str qentry15,  [z, #16*15]

    CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_copy_row_from_table_32)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
