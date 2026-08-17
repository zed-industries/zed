// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

#include <openssl/asm_base.h>
// Auto-added during import for AWS-LC symbol prefixing support
#include "_internal_s2n_bignum.h"

        .arch	armv8-a+crypto
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(aes_hw_xts_decrypt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(aes_hw_xts_decrypt)
        .text
        .balign 4


#define STACK_SIZE_GPRS  32 //2*16
#define STACK_SIZE_VREGS 64 //4*16
#define STACK_SIZE  (STACK_SIZE_GPRS + STACK_SIZE_VREGS)

#define STACK_BASE_GPRS  0
#define STACK_BASE_VREGS STACK_SIZE_GPRS

#define save_regs \
        stp  x19, x20, [sp, #(STACK_BASE_GPRS + 16*0)] __LF \
     	stp  x21, x22, [sp, #(STACK_BASE_GPRS + 16*1)]

#define restore_regs \
        ldp  x19, x20, [sp, #(STACK_BASE_GPRS + 16*0)] __LF \
        ldp  x21, x22, [sp, #(STACK_BASE_GPRS + 16*1)]

#define save_vregs \
    	stp  d8,  d9, [sp, #(STACK_BASE_VREGS + 16*0)] __LF \
    	stp d10, d11, [sp, #(STACK_BASE_VREGS + 16*1)] __LF \
    	stp d12, d13, [sp, #(STACK_BASE_VREGS + 16*2)] __LF \
        stp d14, d15, [sp, #(STACK_BASE_VREGS + 16*3)]

#define restore_vregs \
        ldp  d8,  d9, [sp, #(STACK_BASE_VREGS + 16*0)] __LF \
        ldp d10, d11, [sp, #(STACK_BASE_VREGS + 16*1)] __LF \
        ldp d12, d13, [sp, #(STACK_BASE_VREGS + 16*2)] __LF \
        ldp d14, d15, [sp, #(STACK_BASE_VREGS + 16*3)]

// A single AES round
// Prevent SLOTHY from unfolding because uArchs tend to fuse AESMC+AESE
#define aesr(data, key) \
        aese  data, key __LF \
        aesmc data, data

// A single AESD round
// Prevent SLOTHY from unfolding because uArchs tend to fuse AESIMC+AESD
#define aesdr(data, key) \
        aesd  data, key __LF \
        aesimc data, data

#define tweak(lo, hi) \
        extr  x22, x10, x10, #32 __LF \
        extr  x10, x10,  x9, #63 __LF \
        and   w11, w19, w22, asr#31 __LF \
        eor   x9,  x11,  x9, lsl#1 __LF \
        fmov  lo,  x9 __LF \
        fmov  hi, x10

#define udiv_by_80(src, dst) \
        mov   dst, #-0x3333333333333334 __LF \
        movk  dst, #0xcccd __LF \
        umulh dst, src, dst __LF \
        lsr   dst, dst, #6

S2N_BN_SYMBOL(aes_hw_xts_decrypt):
        AARCH64_VALID_CALL_TARGET
        sub sp, sp, #STACK_SIZE
        save_vregs
        save_regs

        cmp    x2,#16
        // Original input data size bigger than 16, jump to big size processing.
        b.lt    .Lxts_dec_abort

.align 5
.Lxts_dec_big_size:  // decrypt input size >= 16 bytes
        and  x21, x2, #0xf    // store the tail value of length%16
        and  x2, x2, #-16    // len &= 0x1..110000, now divisible by 16

        // Firstly, encrypt the iv with key2, as the first iv of XEX.
        ldr  w6, [x4,#240]
        ld1  {v0.4s}, [x4], #16
        ld1  {v6.16b}, [x5]
        sub  w6, w6, #2
        ld1  {v1.4s}, [x4], #16

.Loop_dec_iv_enc:
        aesr(v6.16b, v0.16b)
        ld1  {v0.4s}, [x4], #16
        subs w6, w6, #2
        aesr(v6.16b, v1.16b)
        ld1  {v1.4s}, [x4], #16
        b.gt .Loop_dec_iv_enc

        aesr(v6.16b, v0.16b)
        ld1  {v0.4s}, [x4]
        aese v6.16b, v1.16b
        eor  v6.16b, v6.16b, v0.16b

        // The iv for second block
        // x9- iv(low), x10 - iv(high)
        // the five ivs stored into, v6.16b,v8.16b,v9.16b,v10.16b,v11.16b
        fmov  x9, d6
        fmov  x10, v6.d[1]
        mov   w19, #0x87
        tweak(d8, v8.d[1])

        mov x7, x3
        ld1 {v16.4s,v17.4s},[x7], #32         // load key schedule
        ld1 {v12.4s,v13.4s},[x7], #32
        ld1 {v14.4s,v15.4s},[x7], #32
        ld1 {v4.4s,v5.4s},  [x7], #32
        ld1 {v18.4s,v19.4s},[x7], #32
        ld1 {v20.4s,v21.4s},[x7], #32
        ld1 {v22.4s,v23.4s},[x7], #32
        ld1 {v7.4s},        [x7]

// decryption
.Lxts_dec:
        tst x21,#0xf
        b.eq .Lxts_dec_begin
        subs x2,x2,#16
        cmp x2, #16
        b.lt .Lxts_dec_done   // There is one block and a tail, go directy to cipher-stealing

.Lxts_dec_begin:
        udiv_by_80(x2, x8) // Number of 5x-unrolled iterations

        cmp x2, #0x20
        b.lo .Lxts_dec_tail1x   // when input = 1 with another block and a tail

        cmp  x2,#0x30         // bias
        b.lo  .Lxts_dec_tail2x    // when input size  = 2 with another block and a tail

        // The iv for third block
        tweak(d9,v9.d[1])

        cmp  x2,#0x40
        b.lo  .Lxts_dec_tail3x // when input size  = 3 with another block and a tail

        // The iv for fourth block
        tweak(d10,v10.d[1])

        cmp x2,#0x50
        b.lo  .Lxts_dec_tail4x // when input size  = 4 with another block and a tail

        // The iv for fifth block
        tweak(d11,v11.d[1])

.align  4
.Loop5x_xts_dec:
        ldp q0, q1, [x0], #0x50
        ldp q24, q25, [x0, #-0x30]
        ldr q26, [x0, #-0x10]

        eor  v0.16b,v0.16b,v6.16b
        eor  v1.16b,v1.16b,v8.16b
        eor  v24.16b,v24.16b,v9.16b
        eor  v25.16b,v25.16b,v10.16b
        eor  v26.16b,v26.16b,v11.16b

        aesdr(v0.16b, v16.16b)
        aesdr(v1.16b, v16.16b)
        aesdr(v24.16b, v16.16b)
        aesdr(v25.16b, v16.16b)
        aesdr(v26.16b, v16.16b)

        aesdr(v0.16b, v17.16b)
        aesdr(v1.16b, v17.16b)
        aesdr(v24.16b, v17.16b)
        aesdr(v25.16b, v17.16b)
        aesdr(v26.16b, v17.16b)

        aesdr(v0.16b, v12.16b)
        aesdr(v1.16b, v12.16b)
        aesdr(v24.16b, v12.16b)
        aesdr(v25.16b, v12.16b)
        aesdr(v26.16b, v12.16b)

        aesdr(v0.16b, v13.16b)
        aesdr(v1.16b, v13.16b)
        aesdr(v24.16b, v13.16b)
        aesdr(v25.16b, v13.16b)
        aesdr(v26.16b, v13.16b)

        aesdr(v0.16b, v14.16b)
        aesdr(v1.16b, v14.16b)
        aesdr(v24.16b, v14.16b)
        aesdr(v25.16b, v14.16b)
        aesdr(v26.16b, v14.16b)

        aesdr(v0.16b, v15.16b)
        aesdr(v1.16b, v15.16b)
        aesdr(v24.16b, v15.16b)
        aesdr(v25.16b, v15.16b)
        aesdr(v26.16b, v15.16b)

        aesdr(v0.16b, v4.16b)
        aesdr(v1.16b, v4.16b)
        aesdr(v24.16b, v4.16b)
        aesdr(v25.16b, v4.16b)
        aesdr(v26.16b, v4.16b)

        aesdr(v0.16b, v5.16b)
        aesdr(v1.16b, v5.16b)
        aesdr(v24.16b, v5.16b)
        aesdr(v25.16b, v5.16b)
        aesdr(v26.16b, v5.16b)

        aesdr(v0.16b, v18.16b)
        aesdr(v1.16b, v18.16b)
        aesdr(v24.16b, v18.16b)
        aesdr(v25.16b, v18.16b)
        aesdr(v26.16b, v18.16b)

        aesdr(v0.16b, v19.16b)
        aesdr(v1.16b, v19.16b)
        aesdr(v24.16b, v19.16b)
        aesdr(v25.16b, v19.16b)
        aesdr(v26.16b, v19.16b)

        aesdr(v0.16b, v20.16b)
        aesdr(v1.16b, v20.16b)
        aesdr(v24.16b, v20.16b)
        aesdr(v25.16b, v20.16b)
        aesdr(v26.16b, v20.16b)

        aesdr(v0.16b, v21.16b)
        aesdr(v1.16b, v21.16b)
        aesdr(v24.16b, v21.16b)
        aesdr(v25.16b, v21.16b)
        aesdr(v26.16b, v21.16b)

        aesdr(v0.16b, v22.16b)
        aesdr(v1.16b, v22.16b)
        aesdr(v24.16b, v22.16b)
        aesdr(v25.16b, v22.16b)
        aesdr(v26.16b, v22.16b)

        aesd  v0.16b,v23.16b
        aesd  v1.16b,v23.16b
        aesd  v24.16b,v23.16b
        aesd  v25.16b,v23.16b
        aesd  v26.16b,v23.16b

        eor  v0.16b,v0.16b,v7.16b
        eor  v0.16b,v0.16b,v6.16b
        // The iv for first block of one iteration
        tweak(d6,v6.d[1])
        eor  v1.16b,v1.16b,v7.16b
        eor  v1.16b,v1.16b,v8.16b

        // The iv for second block
        tweak(d8,v8.d[1])
        eor  v24.16b,v24.16b,v7.16b
        eor  v24.16b,v24.16b,v9.16b

        // The iv for third block
        tweak(d9,v9.d[1])
        eor  v25.16b,v25.16b,v7.16b
        eor  v25.16b,v25.16b,v10.16b

        // The iv for fourth block
        tweak(d10,v10.d[1])
        eor  v26.16b,v26.16b,v7.16b
        eor  v26.16b,v26.16b,v11.16b

        // The iv for fifth block
        tweak(d11,v11.d[1])

        stp q0, q1, [x1], #0x50
        stp q24, q25, [x1, #-0x30]
        str  q26, [x1, #-0x10]

        subs x2,x2,#0x50
        subs x8,x8,#1
        cbnz x8, .Loop5x_xts_dec

.Loop5x_dec_after:
        cmp  x2,#0x40
        b.eq  .Lxts_dec_tail4x  // 4 blocks left + 1block and a tail

        cmp  x2,#0x30
        b.eq  .Lxts_dec_tail3x    // 3 blocks left + 1block and a tail

        cmp  x2,#0x20
        b.eq  .Lxts_dec_tail2x    // 2 block left + 1block and a tail

        cmp  x2,#0x10
        b.eq  .Lxts_dec_tail1x  // 1 blocks left + 1block and a tail

        b  .Lxts_dec_done  // no blocks left + 1block and a tail

.align  4
.Lxts_dec_tail4x:
        ld1  {v0.16b,v1.16b}, [x0],#32              // the first and second blocks
        ld1  {v24.16b,v25.16b}, [x0],#32           // the third and fourth blocks

        eor  v0.16b,v0.16b,v6.16b
        eor  v1.16b,v1.16b,v8.16b
        eor  v24.16b,v24.16b,v9.16b
        eor  v25.16b,v25.16b,v10.16b

        aesdr(v0.16b, v16.16b)
        aesdr(v1.16b, v16.16b)
        aesdr(v24.16b, v16.16b)
        aesdr(v25.16b, v16.16b)

        aesdr(v0.16b, v17.16b)
        aesdr(v1.16b, v17.16b)
        aesdr(v24.16b, v17.16b)
        aesdr(v25.16b, v17.16b)

        aesdr(v0.16b, v12.16b)
        aesdr(v1.16b, v12.16b)
        aesdr(v24.16b, v12.16b)
        aesdr(v25.16b, v12.16b)

        aesdr(v0.16b, v13.16b)
        aesdr(v1.16b, v13.16b)
        aesdr(v24.16b, v13.16b)
        aesdr(v25.16b, v13.16b)

        aesdr(v0.16b, v14.16b)
        aesdr(v1.16b, v14.16b)
        aesdr(v24.16b, v14.16b)
        aesdr(v25.16b, v14.16b)

        aesdr(v0.16b, v15.16b)
        aesdr(v1.16b, v15.16b)
        aesdr(v24.16b, v15.16b)
        aesdr(v25.16b, v15.16b)

        aesdr(v0.16b, v4.16b)
        aesdr(v1.16b, v4.16b)
        aesdr(v24.16b, v4.16b)
        aesdr(v25.16b, v4.16b)

        aesdr(v0.16b, v5.16b)
        aesdr(v1.16b, v5.16b)
        aesdr(v24.16b, v5.16b)
        aesdr(v25.16b, v5.16b)

        aesdr(v0.16b, v18.16b)
        aesdr(v1.16b, v18.16b)
        aesdr(v24.16b, v18.16b)
        aesdr(v25.16b, v18.16b)

        aesdr(v0.16b, v19.16b)
        aesdr(v1.16b, v19.16b)
        aesdr(v24.16b, v19.16b)
        aesdr(v25.16b, v19.16b)

        aesdr(v0.16b, v20.16b)
        aesdr(v1.16b, v20.16b)
        aesdr(v24.16b, v20.16b)
        aesdr(v25.16b, v20.16b)

        aesdr(v0.16b, v21.16b)
        aesdr(v1.16b, v21.16b)
        aesdr(v24.16b, v21.16b)
        aesdr(v25.16b, v21.16b)

        aesdr(v0.16b, v22.16b)
        aesdr(v1.16b, v22.16b)
        aesdr(v24.16b, v22.16b)
        aesdr(v25.16b, v22.16b)

        aesd  v0.16b,v23.16b
        aesd  v1.16b,v23.16b
        aesd  v24.16b,v23.16b
        aesd  v25.16b,v23.16b

        eor  v0.16b,v0.16b,v7.16b
        eor  v0.16b,v0.16b,v6.16b

        eor  v1.16b,v1.16b,v7.16b
        eor  v1.16b,v1.16b,v8.16b

        eor  v24.16b,v24.16b,v7.16b
        eor  v24.16b,v24.16b,v9.16b

        eor  v25.16b,v25.16b,v7.16b
        eor  v25.16b,v25.16b,v10.16b
        // The iv for tail
        fmov  x9,d10
        fmov  x10,v10.d[1]
        tweak(d6,v6.d[1])

        st1  {v0.16b,v1.16b},[x1],#32
        st1  {v24.16b,v25.16b},[x1],#32

        b  .Lxts_dec_done

.align  4
.Lxts_dec_tail3x:
        ld1  {v0.16b,v1.16b}, [x0],#32
        ld1  {v24.16b}, [x0],#16

        eor  v0.16b,v0.16b,v6.16b
        eor  v1.16b,v1.16b,v8.16b
        eor  v24.16b,v24.16b,v9.16b

        aesdr(v0.16b, v16.16b)
        aesdr(v1.16b, v16.16b)
        aesdr(v24.16b, v16.16b)

        aesdr(v0.16b, v17.16b)
        aesdr(v1.16b, v17.16b)
        aesdr(v24.16b, v17.16b)

        aesdr(v0.16b, v12.16b)
        aesdr(v1.16b, v12.16b)
        aesdr(v24.16b, v12.16b)

        aesdr(v0.16b, v13.16b)
        aesdr(v1.16b, v13.16b)
        aesdr(v24.16b, v13.16b)

        aesdr(v0.16b, v14.16b)
        aesdr(v1.16b, v14.16b)
        aesdr(v24.16b, v14.16b)

        aesdr(v0.16b, v15.16b)
        aesdr(v1.16b, v15.16b)
        aesdr(v24.16b, v15.16b)

        aesdr(v0.16b, v4.16b)
        aesdr(v1.16b, v4.16b)
        aesdr(v24.16b, v4.16b)

        aesdr(v0.16b, v5.16b)
        aesdr(v1.16b, v5.16b)
        aesdr(v24.16b, v5.16b)

        aesdr(v0.16b, v18.16b)
        aesdr(v1.16b, v18.16b)
        aesdr(v24.16b, v18.16b)

        aesdr(v0.16b, v19.16b)
        aesdr(v1.16b, v19.16b)
        aesdr(v24.16b, v19.16b)

        aesdr(v0.16b, v20.16b)
        aesdr(v1.16b, v20.16b)
        aesdr(v24.16b, v20.16b)

        aesdr(v0.16b, v21.16b)
        aesdr(v1.16b, v21.16b)
        aesdr(v24.16b, v21.16b)

        aesdr(v0.16b, v22.16b)
        aesdr(v1.16b, v22.16b)
        aesdr(v24.16b, v22.16b)

        aesd  v0.16b,v23.16b
        aesd  v1.16b,v23.16b
        aesd  v24.16b,v23.16b

        eor  v0.16b,v0.16b,v7.16b
        eor  v0.16b,v0.16b,v6.16b
        eor  v1.16b,v1.16b,v7.16b
        eor  v1.16b,v1.16b,v8.16b
        eor  v24.16b,v24.16b,v7.16b
        eor  v24.16b,v24.16b,v9.16b

        // The iv for tail
        fmov  x9,d9
        fmov  x10,v9.d[1]
        tweak(d6,v6.d[1])

        st1  {v0.16b,v1.16b},[x1],#32
        st1  {v24.16b},[x1],#16
        b .Lxts_dec_done  // done processing three blocks

.Lxts_dec_tail2x:
        ld1  {v0.16b,v1.16b},[x0],#32  // the first block

        eor  v0.16b,v0.16b,v6.16b
        eor  v1.16b,v1.16b,v8.16b

        aesdr(v0.16b, v16.16b)
        aesdr(v1.16b, v16.16b)

        aesdr(v0.16b, v17.16b)
        aesdr(v1.16b, v17.16b)

        aesdr(v0.16b, v12.16b)
        aesdr(v1.16b, v12.16b)


        aesdr(v0.16b, v13.16b)
        aesdr(v1.16b, v13.16b)

        aesdr(v0.16b, v14.16b)
        aesdr(v1.16b, v14.16b)

        aesdr(v0.16b, v15.16b)
        aesdr(v1.16b, v15.16b)

        aesdr(v0.16b, v4.16b)
        aesdr(v1.16b, v4.16b)

        aesdr(v0.16b, v5.16b)
        aesdr(v1.16b, v5.16b)

        aesdr(v0.16b, v18.16b)
        aesdr(v1.16b, v18.16b)

        aesdr(v0.16b, v19.16b)
        aesdr(v1.16b, v19.16b)

        aesdr(v0.16b, v20.16b)
        aesdr(v1.16b, v20.16b)

        aesdr(v0.16b, v21.16b)
        aesdr(v1.16b, v21.16b)

        aesdr(v0.16b, v22.16b)
        aesdr(v1.16b, v22.16b)
        aesd  v0.16b,v23.16b
        aesd  v1.16b,v23.16b

        eor  v0.16b,v0.16b,v7.16b
        eor  v0.16b,v0.16b,v6.16b
        eor  v1.16b,v1.16b,v7.16b
        eor  v1.16b,v1.16b,v8.16b

        st1  {v0.16b,v1.16b},[x1],#32

        // The iv for tail
        fmov  x9,d8
        fmov  x10,v8.d[1]
        tweak(d6,v6.d[1])
        b  .Lxts_dec_done

.Lxts_dec_tail1x:
        ld1  {v0.16b}, [x0],#16  // the first block

        eor  v0.16b,v0.16b,v6.16b

        aesdr(v0.16b, v16.16b)

        aesdr(v0.16b, v17.16b)

        aesdr(v0.16b, v12.16b)

        aesdr(v0.16b, v13.16b)

        aesdr(v0.16b, v14.16b)

        aesdr(v0.16b, v15.16b)

        aesdr(v0.16b, v4.16b)

        aesdr(v0.16b, v5.16b)

        aesdr(v0.16b, v18.16b)

        aesdr(v0.16b, v19.16b)

        aesdr(v0.16b, v20.16b)
        aesdr(v0.16b, v21.16b)
        aesdr(v0.16b, v22.16b)
        aesd  v0.16b,v23.16b

        eor  v0.16b,v0.16b,v7.16b
        eor  v0.16b,v0.16b,v6.16b
        st1  {v0.16b},[x1],#16
        // tweak for tail
        fmov  x9,d6
        fmov  x10,v6.d[1]
        tweak(d6,v6.d[1])
        b  .Lxts_dec_done

.Lxts_dec_done:
        tst    x21,#0xf
        b.eq    .Lxts_dec_abort
        // Processing the last 1 block and a tail with cipher stealing.
        // At this point, we know there is 1 block+tail
        // We need two tweaks, one is already calculated and stored in v6
        // We use it to calculate v8
        mov    x7,x3

        fmov  x9,d6
        fmov  x10,v6.d[1]
        tweak(d8,v8.d[1])

        ld1    {v0.4s},[x0],#16

// Decrypt the second to last block
.Lxts_dec_1st_done:
        eor    v26.16b,v0.16b,v8.16b
     
        aesdr(v26.16b, v16.16b)
        aesdr(v26.16b, v17.16b)
        aesdr(v26.16b, v12.16b)
        aesdr(v26.16b, v13.16b)
        aesdr(v26.16b, v14.16b)
        aesdr(v26.16b, v15.16b)
        aesdr(v26.16b, v4.16b)
        aesdr(v26.16b, v5.16b)
        aesdr(v26.16b, v18.16b)
        aesdr(v26.16b, v19.16b)
        aesdr(v26.16b, v20.16b)
        aesdr(v26.16b, v21.16b)
        aesdr(v26.16b, v22.16b)
        aesd  v26.16b,v23.16b
        eor  v26.16b,v26.16b,v7.16b

        eor    v26.16b,v26.16b,v8.16b
        st1    {v26.16b},[x1]

        mov    x20,x0
        add    x13,x1,#16

        // Composite the tailcnt "16 byte not aligned block" into the last second plain blocks
        // to get the last encrypted block.
.composite_dec_loop:
        subs    x21,x21,#1
        ldrb    w15,[x1,x21]
        ldrb    w14,[x20,x21]
        strb    w15,[x13,x21]
        strb    w14,[x1,x21]
        b.gt    .composite_dec_loop
.Lxts_dec_load_done:
        ld1    {v26.16b},[x1]
        eor    v26.16b,v26.16b,v6.16b

        // Decrypt the composite block to get the last second plain text block
        aesdr(v26.16b, v16.16b)
        aesdr(v26.16b, v17.16b)
        aesdr(v26.16b, v12.16b)
        aesdr(v26.16b, v13.16b)
        aesdr(v26.16b, v14.16b)
        aesdr(v26.16b, v15.16b)
        aesdr(v26.16b, v4.16b)
        aesdr(v26.16b, v5.16b)
        aesdr(v26.16b, v18.16b)
        aesdr(v26.16b, v19.16b)
        aesdr(v26.16b, v20.16b)
        aesdr(v26.16b, v21.16b)
        aesdr(v26.16b, v22.16b)
        aesd  v26.16b,v23.16b
        eor  v26.16b,v26.16b,v7.16b

        eor    v26.16b,v26.16b,v6.16b
        st1    {v26.16b},[x1]

.Lxts_dec_abort:
        restore_vregs
        restore_regs
        add sp, sp, #STACK_SIZE
        ret

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
