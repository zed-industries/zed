/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2020, Martin Storsjo
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#include "src/arm/asm.S"
#include "util.S"

.macro dir_table w, stride
const directions\w
        .byte           -1 * \stride + 1, -2 * \stride + 2
        .byte            0 * \stride + 1, -1 * \stride + 2
        .byte            0 * \stride + 1,  0 * \stride + 2
        .byte            0 * \stride + 1,  1 * \stride + 2
        .byte            1 * \stride + 1,  2 * \stride + 2
        .byte            1 * \stride + 0,  2 * \stride + 1
        .byte            1 * \stride + 0,  2 * \stride + 0
        .byte            1 * \stride + 0,  2 * \stride - 1
// Repeated, to avoid & 7
        .byte           -1 * \stride + 1, -2 * \stride + 2
        .byte            0 * \stride + 1, -1 * \stride + 2
        .byte            0 * \stride + 1,  0 * \stride + 2
        .byte            0 * \stride + 1,  1 * \stride + 2
        .byte            1 * \stride + 1,  2 * \stride + 2
        .byte            1 * \stride + 0,  2 * \stride + 1
endconst
.endm

.macro tables
dir_table 8, 16
dir_table 4, 8

const pri_taps
        .byte           4, 2, 3, 3
endconst
.endm

.macro load_px d11, d12, d21, d22, w
.if \w == 8
        add             r6,  r2,  r9, lsl #1 // x + off
        sub             r9,  r2,  r9, lsl #1 // x - off
        vld1.16         {\d11,\d12}, [r6]    // p0
        vld1.16         {\d21,\d22}, [r9]    // p1
.else
        add             r6,  r2,  r9, lsl #1 // x + off
        sub             r9,  r2,  r9, lsl #1 // x - off
        vld1.16         {\d11}, [r6]         // p0
        add             r6,  r6,  #2*8       // += stride
        vld1.16         {\d21}, [r9]         // p1
        add             r9,  r9,  #2*8       // += stride
        vld1.16         {\d12}, [r6]         // p0
        vld1.16         {\d22}, [r9]         // p1
.endif
.endm
.macro handle_pixel s1, s2, thresh_vec, shift, tap, min
.if \min
        vmin.u16        q2,  q2,  \s1
        vmax.s16        q3,  q3,  \s1
        vmin.u16        q2,  q2,  \s2
        vmax.s16        q3,  q3,  \s2
.endif
        vabd.u16        q8,  q0,  \s1        // abs(diff)
        vabd.u16        q11, q0,  \s2        // abs(diff)
        vshl.u16        q9,  q8,  \shift     // abs(diff) >> shift
        vshl.u16        q12, q11, \shift     // abs(diff) >> shift
        vqsub.u16       q9,  \thresh_vec, q9 // clip = imax(0, threshold - (abs(diff) >> shift))
        vqsub.u16       q12, \thresh_vec, q12// clip = imax(0, threshold - (abs(diff) >> shift))
        vsub.i16        q10, \s1, q0         // diff = p0 - px
        vsub.i16        q13, \s2, q0         // diff = p1 - px
        vneg.s16        q8,  q9              // -clip
        vneg.s16        q11, q12             // -clip
        vmin.s16        q10, q10, q9         // imin(diff, clip)
        vmin.s16        q13, q13, q12        // imin(diff, clip)
        vdup.16         q9,  \tap            // taps[k]
        vmax.s16        q10, q10, q8         // constrain() = imax(imin(diff, clip), -clip)
        vmax.s16        q13, q13, q11        // constrain() = imax(imin(diff, clip), -clip)
        vmla.i16        q1,  q10, q9         // sum += taps[k] * constrain()
        vmla.i16        q1,  q13, q9         // sum += taps[k] * constrain()
.endm

// void dav1d_cdef_filterX_Ybpc_neon(pixel *dst, ptrdiff_t dst_stride,
//                                   const uint16_t *tmp, int pri_strength,
//                                   int sec_strength, int dir, int damping,
//                                   int h, size_t edges);
.macro filter_func w, bpc, pri, sec, min, suffix
function cdef_filter\w\suffix\()_\bpc\()bpc_neon
.if \bpc == 8
        cmp             r8,  #0xf
        beq             cdef_filter\w\suffix\()_edged_neon
.endif
.if \pri
.if \bpc == 16
        clz             r9,  r9
        sub             r9,  r9,  #24        // -bitdepth_min_8
        neg             r9,  r9              // bitdepth_min_8
.endif
        movrel_local    r8,  pri_taps
.if \bpc == 16
        lsr             r9,  r3,  r9         // pri_strength >> bitdepth_min_8
        and             r9,  r9,  #1         // (pri_strength >> bitdepth_min_8) & 1
.else
        and             r9,  r3,  #1
.endif
        add             r8,  r8,  r9, lsl #1
.endif
        movrel_local    r9,  directions\w
        add             r5,  r9,  r5, lsl #1
        vmov.u16        d17, #15
        vdup.16         d16, r6              // damping

.if \pri
        vdup.16         q5,  r3              // threshold
.endif
.if \sec
        vdup.16         q7,  r4              // threshold
.endif
        vmov.16         d8[0], r3
        vmov.16         d8[1], r4
        vclz.i16        d8,  d8              // clz(threshold)
        vsub.i16        d8,  d17, d8         // ulog2(threshold)
        vqsub.u16       d8,  d16, d8         // shift = imax(0, damping - ulog2(threshold))
        vneg.s16        d8,  d8              // -shift
.if \sec
        vdup.16         q6,  d8[1]
.endif
.if \pri
        vdup.16         q4,  d8[0]
.endif

1:
.if \w == 8
        vld1.16         {q0},  [r2, :128]    // px
.else
        add             r12, r2,  #2*8
        vld1.16         {d0},  [r2,  :64]    // px
        vld1.16         {d1},  [r12, :64]    // px
.endif

        vmov.u16        q1,  #0              // sum
.if \min
        vmov.u16        q2,  q0              // min
        vmov.u16        q3,  q0              // max
.endif

        // Instead of loading sec_taps 2, 1 from memory, just set it
        // to 2 initially and decrease for the second round.
        // This is also used as loop counter.
        mov             lr,  #2              // sec_taps[0]

2:
.if \pri
        ldrsb           r9,  [r5]            // off1

        load_px         d28, d29, d30, d31, \w
.endif

.if \sec
        add             r5,  r5,  #4         // +2*2
        ldrsb           r9,  [r5]            // off2
.endif

.if \pri
        ldrb            r12, [r8]            // *pri_taps

        handle_pixel    q14, q15, q5,  q4,  r12, \min
.endif

.if \sec
        load_px         d28, d29, d30, d31, \w

        add             r5,  r5,  #8         // +2*4
        ldrsb           r9,  [r5]            // off3

        handle_pixel    q14, q15, q7,  q6,  lr, \min

        load_px         d28, d29, d30, d31, \w

        handle_pixel    q14, q15, q7,  q6,  lr, \min

        sub             r5,  r5,  #11        // r5 -= 2*(2+4); r5 += 1;
.else
        add             r5,  r5,  #1         // r5 += 1
.endif
        subs            lr,  lr,  #1         // sec_tap-- (value)
.if \pri
        add             r8,  r8,  #1         // pri_taps++ (pointer)
.endif
        bne             2b

        vshr.s16        q14, q1,  #15        // -(sum < 0)
        vadd.i16        q1,  q1,  q14        // sum - (sum < 0)
        vrshr.s16       q1,  q1,  #4         // (8 + sum - (sum < 0)) >> 4
        vadd.i16        q0,  q0,  q1         // px + (8 + sum ...) >> 4
.if \min
        vmin.s16        q0,  q0,  q3
        vmax.s16        q0,  q0,  q2         // iclip(px + .., min, max)
.endif
.if \bpc == 8
        vmovn.u16       d0,  q0
.endif
.if \w == 8
        add             r2,  r2,  #2*16      // tmp += tmp_stride
        subs            r7,  r7,  #1         // h--
.if \bpc == 8
        vst1.8          {d0}, [r0, :64], r1
.else
        vst1.16         {q0}, [r0, :128], r1
.endif
.else
.if \bpc == 8
        vst1.32         {d0[0]}, [r0, :32], r1
.else
        vst1.16         {d0},    [r0, :64], r1
.endif
        add             r2,  r2,  #2*16      // tmp += 2*tmp_stride
        subs            r7,  r7,  #2         // h -= 2
.if \bpc == 8
        vst1.32         {d0[1]}, [r0, :32], r1
.else
        vst1.16         {d1},    [r0, :64], r1
.endif
.endif

        // Reset pri_taps and directions back to the original point
        sub             r5,  r5,  #2
.if \pri
        sub             r8,  r8,  #2
.endif

        bgt             1b
        vpop            {q4-q7}
        pop             {r4-r9,pc}
endfunc
.endm

.macro filter w, bpc
filter_func \w, \bpc, pri=1, sec=0, min=0, suffix=_pri
filter_func \w, \bpc, pri=0, sec=1, min=0, suffix=_sec
filter_func \w, \bpc, pri=1, sec=1, min=1, suffix=_pri_sec

function cdef_filter\w\()_\bpc\()bpc_neon, export=1
        push            {r4-r9,lr}
        vpush           {q4-q7}
        ldrd            r4,  r5,  [sp, #92]
        ldrd            r6,  r7,  [sp, #100]
.if \bpc == 16
        ldrd            r8,  r9,  [sp, #108]
.else
        ldr             r8,  [sp, #108]
.endif
        cmp             r3,  #0 // pri_strength
        bne             1f
        b               cdef_filter\w\()_sec_\bpc\()bpc_neon // only sec
1:
        cmp             r4,  #0 // sec_strength
        bne             1f
        b               cdef_filter\w\()_pri_\bpc\()bpc_neon // only pri
1:
        b               cdef_filter\w\()_pri_sec_\bpc\()bpc_neon // both pri and sec
endfunc
.endm

const div_table, align=4
        .short         840, 420, 280, 210, 168, 140, 120, 105
endconst

const alt_fact, align=4
        .short         420, 210, 140, 105, 105, 105, 105, 105, 140, 210, 420, 0
endconst

.macro cost_alt dest, s1, s2, s3, s4, s5, s6
        vmull.s16       q1,  \s1, \s1          // sum_alt[n]*sum_alt[n]
        vmull.s16       q2,  \s2, \s2
        vmull.s16       q3,  \s3, \s3
        vmull.s16       q5,  \s4, \s4          // sum_alt[n]*sum_alt[n]
        vmull.s16       q12, \s5, \s5
        vmull.s16       q6,  \s6, \s6          // q6 overlaps the first \s1-\s2 here
        vmul.i32        q1,  q1,  q13          // sum_alt[n]^2*fact
        vmla.i32        q1,  q2,  q14
        vmla.i32        q1,  q3,  q15
        vmul.i32        q5,  q5,  q13          // sum_alt[n]^2*fact
        vmla.i32        q5,  q12, q14
        vmla.i32        q5,  q6,  q15
        vadd.i32        d2,  d2,  d3
        vadd.i32        d3,  d10, d11
        vpadd.i32       \dest, d2, d3          // *cost_ptr
.endm

.macro find_best s1, s2, s3
.ifnb \s2
        vmov.32         lr,  \s2
.endif
        cmp             r12, r1                // cost[n] > best_cost
        itt             gt
        movgt           r0,  r3                // best_dir = n
        movgt           r1,  r12               // best_cost = cost[n]
.ifnb \s2
        add             r3,  r3,  #1           // n++
        cmp             lr,  r1                // cost[n] > best_cost
        vmov.32         r12, \s3
        itt             gt
        movgt           r0,  r3                // best_dir = n
        movgt           r1,  lr                // best_cost = cost[n]
        add             r3,  r3,  #1           // n++
.endif
.endm

// int dav1d_cdef_find_dir_Xbpc_neon(const pixel *img, const ptrdiff_t stride,
//                                   unsigned *const var)
.macro find_dir bpc
function cdef_find_dir_\bpc\()bpc_neon, export=1
        push            {lr}
        vpush           {q4-q7}
.if \bpc == 16
        clz             r3,  r3                // clz(bitdepth_max)
        sub             lr,  r3,  #24          // -bitdepth_min_8
.endif
        sub             sp,  sp,  #32          // cost
        mov             r3,  #8
        vmov.u16        q1,  #0                // q0-q1   sum_diag[0]
        vmov.u16        q3,  #0                // q2-q3   sum_diag[1]
        vmov.u16        q5,  #0                // q4-q5   sum_hv[0-1]
        vmov.u16        q8,  #0                // q6,d16  sum_alt[0]
                                               // q7,d17  sum_alt[1]
        vmov.u16        q9,  #0                // q9,d22  sum_alt[2]
        vmov.u16        q11, #0
        vmov.u16        q10, #0                // q10,d23 sum_alt[3]


.irpc i, 01234567
.if \bpc == 8
        vld1.8          {d30}, [r0, :64], r1
        vmov.u8         d31, #128
        vsubl.u8        q15, d30, d31          // img[x] - 128
.else
        vld1.16         {q15}, [r0, :128], r1
        vdup.16         q14, lr                // -bitdepth_min_8
        vshl.u16        q15, q15, q14
        vmov.u16        q14, #128
        vsub.i16        q15, q15, q14          // img[x] - 128
.endif
        vmov.u16        q14, #0

.if \i == 0
        vmov            q0,  q15               // sum_diag[0]
.else
        vext.8          q12, q14, q15, #(16-2*\i)
        vext.8          q13, q15, q14, #(16-2*\i)
        vadd.i16        q0,  q0,  q12          // sum_diag[0]
        vadd.i16        q1,  q1,  q13          // sum_diag[0]
.endif
        vrev64.16       q13, q15
        vswp            d26, d27               // [-x]
.if \i == 0
        vmov            q2,  q13               // sum_diag[1]
.else
        vext.8          q12, q14, q13, #(16-2*\i)
        vext.8          q13, q13, q14, #(16-2*\i)
        vadd.i16        q2,  q2,  q12          // sum_diag[1]
        vadd.i16        q3,  q3,  q13          // sum_diag[1]
.endif

        vpadd.u16       d26, d30, d31          // [(x >> 1)]
        vmov.u16        d27, #0
        vpadd.u16       d24, d26, d28
        vpadd.u16       d24, d24, d28          // [y]
        vmov.u16        r12, d24[0]
        vadd.i16        q5,  q5,  q15          // sum_hv[1]
.if \i < 4
        vmov.16         d8[\i],   r12          // sum_hv[0]
.else
        vmov.16         d9[\i-4], r12          // sum_hv[0]
.endif

.if \i == 0
        vmov.u16        q6,  q13               // sum_alt[0]
.else
        vext.8          q12, q14, q13, #(16-2*\i)
        vext.8          q14, q13, q14, #(16-2*\i)
        vadd.i16        q6,  q6,  q12          // sum_alt[0]
        vadd.i16        d16, d16, d28          // sum_alt[0]
.endif
        vrev64.16       d26, d26               // [-(x >> 1)]
        vmov.u16        q14, #0
.if \i == 0
        vmov            q7,  q13               // sum_alt[1]
.else
        vext.8          q12, q14, q13, #(16-2*\i)
        vext.8          q13, q13, q14, #(16-2*\i)
        vadd.i16        q7,  q7,  q12          // sum_alt[1]
        vadd.i16        d17, d17, d26          // sum_alt[1]
.endif

.if \i < 6
        vext.8          q12, q14, q15, #(16-2*(3-(\i/2)))
        vext.8          q13, q15, q14, #(16-2*(3-(\i/2)))
        vadd.i16        q9,  q9,  q12          // sum_alt[2]
        vadd.i16        d22, d22, d26          // sum_alt[2]
.else
        vadd.i16        q9,  q9,  q15          // sum_alt[2]
.endif
.if \i == 0
        vmov            q10, q15               // sum_alt[3]
.elseif \i == 1
        vadd.i16        q10, q10, q15          // sum_alt[3]
.else
        vext.8          q12, q14, q15, #(16-2*(\i/2))
        vext.8          q13, q15, q14, #(16-2*(\i/2))
        vadd.i16        q10, q10, q12          // sum_alt[3]
        vadd.i16        d23, d23, d26          // sum_alt[3]
.endif
.endr

        vmov.u32        q15, #105

        vmull.s16       q12, d8,  d8           // sum_hv[0]*sum_hv[0]
        vmlal.s16       q12, d9,  d9
        vmull.s16       q13, d10, d10          // sum_hv[1]*sum_hv[1]
        vmlal.s16       q13, d11, d11
        vadd.s32        d8,  d24, d25
        vadd.s32        d9,  d26, d27
        vpadd.s32       d8,  d8,  d9           // cost[2,6] (s16, s17)
        vmul.i32        d8,  d8,  d30          // cost[2,6] *= 105

        vrev64.16       q1,  q1
        vrev64.16       q3,  q3
        vext.8          q1,  q1,  q1,  #10     // sum_diag[0][14-n]
        vext.8          q3,  q3,  q3,  #10     // sum_diag[1][14-n]

        vstr            s16, [sp, #2*4]        // cost[2]
        vstr            s17, [sp, #6*4]        // cost[6]

        movrel_local    r12, div_table
        vld1.16         {q14}, [r12, :128]

        vmull.s16       q5,  d0,  d0           // sum_diag[0]*sum_diag[0]
        vmull.s16       q12, d1,  d1
        vmlal.s16       q5,  d2,  d2
        vmlal.s16       q12, d3,  d3
        vmull.s16       q0,  d4,  d4           // sum_diag[1]*sum_diag[1]
        vmull.s16       q1,  d5,  d5
        vmlal.s16       q0,  d6,  d6
        vmlal.s16       q1,  d7,  d7
        vmovl.u16       q13, d28               // div_table
        vmovl.u16       q14, d29
        vmul.i32        q5,  q5,  q13          // cost[0]
        vmla.i32        q5,  q12, q14
        vmul.i32        q0,  q0,  q13          // cost[4]
        vmla.i32        q0,  q1,  q14
        vadd.i32        d10, d10, d11
        vadd.i32        d0,  d0,  d1
        vpadd.i32       d0,  d10, d0           // cost[0,4] = s0,s1

        movrel_local    r12, alt_fact
        vld1.16         {d29, d30, d31}, [r12, :64] // div_table[2*m+1] + 105

        vstr            s0,  [sp, #0*4]        // cost[0]
        vstr            s1,  [sp, #4*4]        // cost[4]

        vmovl.u16       q13, d29               // div_table[2*m+1] + 105
        vmovl.u16       q14, d30
        vmovl.u16       q15, d31

        cost_alt        d14, d12, d13, d16, d14, d15, d17 // cost[1], cost[3]
        cost_alt        d15, d18, d19, d22, d20, d21, d23 // cost[5], cost[7]
        vstr            s28, [sp, #1*4]        // cost[1]
        vstr            s29, [sp, #3*4]        // cost[3]

        mov             r0,  #0                // best_dir
        vmov.32         r1,  d0[0]             // best_cost
        mov             r3,  #1                // n

        vstr            s30, [sp, #5*4]        // cost[5]
        vstr            s31, [sp, #7*4]        // cost[7]

        vmov.32         r12, d14[0]

        find_best       d14[0], d8[0], d14[1]
        find_best       d14[1], d0[1], d15[0]
        find_best       d15[0], d8[1], d15[1]
        find_best       d15[1]

        eor             r3,  r0,  #4           // best_dir ^4
        ldr             r12, [sp, r3, lsl #2]
        sub             r1,  r1,  r12          // best_cost - cost[best_dir ^ 4]
        lsr             r1,  r1,  #10
        str             r1,  [r2]              // *var

        add             sp,  sp,  #32
        vpop            {q4-q7}
        pop             {pc}
endfunc
.endm
