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

.macro load_px d1, d2, w
.if \w == 8
        add             x6,  x2,  w9, sxtb #1       // x + off
        sub             x9,  x2,  w9, sxtb #1       // x - off
        ld1             {\d1\().8h}, [x6]           // p0
        ld1             {\d2\().8h}, [x9]           // p1
.else
        add             x6,  x2,  w9, sxtb #1       // x + off
        sub             x9,  x2,  w9, sxtb #1       // x - off
        ld1             {\d1\().4h}, [x6]           // p0
        add             x6,  x6,  #2*8              // += stride
        ld1             {\d2\().4h}, [x9]           // p1
        add             x9,  x9,  #2*8              // += stride
        ld1             {\d1\().d}[1], [x6]         // p0
        ld1             {\d2\().d}[1], [x9]         // p1
.endif
.endm
.macro handle_pixel s1, s2, thresh_vec, shift, tap, min
.if \min
        umin            v2.8h,   v2.8h,  \s1\().8h
        smax            v3.8h,   v3.8h,  \s1\().8h
        umin            v2.8h,   v2.8h,  \s2\().8h
        smax            v3.8h,   v3.8h,  \s2\().8h
.endif
        uabd            v16.8h, v0.8h,  \s1\().8h   // abs(diff)
        uabd            v20.8h, v0.8h,  \s2\().8h   // abs(diff)
        ushl            v17.8h, v16.8h, \shift      // abs(diff) >> shift
        ushl            v21.8h, v20.8h, \shift      // abs(diff) >> shift
        uqsub           v17.8h, \thresh_vec, v17.8h // clip = imax(0, threshold - (abs(diff) >> shift))
        uqsub           v21.8h, \thresh_vec, v21.8h // clip = imax(0, threshold - (abs(diff) >> shift))
        sub             v18.8h, \s1\().8h,  v0.8h   // diff = p0 - px
        sub             v22.8h, \s2\().8h,  v0.8h   // diff = p1 - px
        neg             v16.8h, v17.8h              // -clip
        neg             v20.8h, v21.8h              // -clip
        smin            v18.8h, v18.8h, v17.8h      // imin(diff, clip)
        smin            v22.8h, v22.8h, v21.8h      // imin(diff, clip)
        dup             v19.8h, \tap                // taps[k]
        smax            v18.8h, v18.8h, v16.8h      // constrain() = imax(imin(diff, clip), -clip)
        smax            v22.8h, v22.8h, v20.8h      // constrain() = imax(imin(diff, clip), -clip)
        mla             v1.8h,  v18.8h, v19.8h      // sum += taps[k] * constrain()
        mla             v1.8h,  v22.8h, v19.8h      // sum += taps[k] * constrain()
.endm

// void dav1d_cdef_filterX_Ybpc_neon(pixel *dst, ptrdiff_t dst_stride,
//                                   const uint16_t *tmp, int pri_strength,
//                                   int sec_strength, int dir, int damping,
//                                   int h, size_t edges);
.macro filter_func w, bpc, pri, sec, min, suffix
function cdef_filter\w\suffix\()_\bpc\()bpc_neon
.if \bpc == 8
        ldr             w8,  [sp]                   // edges
        cmp             w8,  #0xf
        b.eq            cdef_filter\w\suffix\()_edged_8bpc_neon
.endif
.if \pri
.if \bpc == 16
        ldr             w9,  [sp, #8]               // bitdepth_max
        clz             w9,  w9
        sub             w9,  w9,  #24               // -bitdepth_min_8
        neg             w9,  w9                     // bitdepth_min_8
.endif
        movrel          x8,  pri_taps
.if \bpc == 16
        lsr             w9,  w3,  w9                // pri_strength >> bitdepth_min_8
        and             w9,  w9,  #1                // (pri_strength >> bitdepth_min_8) & 1
.else
        and             w9,  w3,  #1
.endif
        add             x8,  x8,  w9, uxtw #1
.endif
        movrel          x9,  directions\w
        add             x5,  x9,  w5, uxtw #1
        movi            v30.4h,   #15
        dup             v28.4h,   w6                // damping

.if \pri
        dup             v25.8h, w3                  // threshold
.endif
.if \sec
        dup             v27.8h, w4                  // threshold
.endif
        trn1            v24.4h, v25.4h, v27.4h
        clz             v24.4h, v24.4h              // clz(threshold)
        sub             v24.4h, v30.4h, v24.4h      // ulog2(threshold)
        uqsub           v24.4h, v28.4h, v24.4h      // shift = imax(0, damping - ulog2(threshold))
        neg             v24.4h, v24.4h              // -shift
.if \sec
        dup             v26.8h, v24.h[1]
.endif
.if \pri
        dup             v24.8h, v24.h[0]
.endif

1:
.if \w == 8
        ld1             {v0.8h}, [x2]               // px
.else
        add             x12, x2,  #2*8
        ld1             {v0.4h},   [x2]             // px
        ld1             {v0.d}[1], [x12]            // px
.endif

        movi            v1.8h,  #0                  // sum
.if \min
        mov             v2.16b, v0.16b              // min
        mov             v3.16b, v0.16b              // max
.endif

        // Instead of loading sec_taps 2, 1 from memory, just set it
        // to 2 initially and decrease for the second round.
        // This is also used as loop counter.
        mov             w11, #2                     // sec_taps[0]

2:
.if \pri
        ldrb            w9,  [x5]                   // off1

        load_px         v4,  v5, \w
.endif

.if \sec
        add             x5,  x5,  #4                // +2*2
        ldrb            w9,  [x5]                   // off2
        load_px         v6,  v7,  \w
.endif

.if \pri
        ldrb            w10, [x8]                   // *pri_taps

        handle_pixel    v4,  v5,  v25.8h, v24.8h, w10, \min
.endif

.if \sec
        add             x5,  x5,  #8                // +2*4
        ldrb            w9,  [x5]                   // off3
        load_px         v4,  v5,  \w

        handle_pixel    v6,  v7,  v27.8h, v26.8h, w11, \min

        handle_pixel    v4,  v5,  v27.8h, v26.8h, w11, \min

        sub             x5,  x5,  #11               // x5 -= 2*(2+4); x5 += 1;
.else
        add             x5,  x5,  #1                // x5 += 1
.endif
        subs            w11, w11, #1                // sec_tap-- (value)
.if \pri
        add             x8,  x8,  #1                // pri_taps++ (pointer)
.endif
        b.ne            2b

        cmlt            v4.8h,  v1.8h,  #0          // -(sum < 0)
        add             v1.8h,  v1.8h,  v4.8h       // sum - (sum < 0)
        srshr           v1.8h,  v1.8h,  #4          // (8 + sum - (sum < 0)) >> 4
        add             v0.8h,  v0.8h,  v1.8h       // px + (8 + sum ...) >> 4
.if \min
        smin            v0.8h,  v0.8h,  v3.8h
        smax            v0.8h,  v0.8h,  v2.8h       // iclip(px + .., min, max)
.endif
.if \bpc == 8
        xtn             v0.8b,  v0.8h
.endif
.if \w == 8
        add             x2,  x2,  #2*16             // tmp += tmp_stride
        subs            w7,  w7,  #1                // h--
.if \bpc == 8
        st1             {v0.8b}, [x0], x1
.else
        st1             {v0.8h}, [x0], x1
.endif
.else
.if \bpc == 8
        st1             {v0.s}[0], [x0], x1
.else
        st1             {v0.d}[0], [x0], x1
.endif
        add             x2,  x2,  #2*16             // tmp += 2*tmp_stride
        subs            w7,  w7,  #2                // h -= 2
.if \bpc == 8
        st1             {v0.s}[1], [x0], x1
.else
        st1             {v0.d}[1], [x0], x1
.endif
.endif

        // Reset pri_taps and directions back to the original point
        sub             x5,  x5,  #2
.if \pri
        sub             x8,  x8,  #2
.endif

        b.gt            1b
        ret
endfunc
.endm

.macro filter w, bpc
filter_func \w, \bpc, pri=1, sec=0, min=0, suffix=_pri
filter_func \w, \bpc, pri=0, sec=1, min=0, suffix=_sec
filter_func \w, \bpc, pri=1, sec=1, min=1, suffix=_pri_sec

function cdef_filter\w\()_\bpc\()bpc_neon, export=1
        cbnz            w3,  1f // pri_strength
        b               cdef_filter\w\()_sec_\bpc\()bpc_neon     // only sec
1:
        cbnz            w4,  1f // sec_strength
        b               cdef_filter\w\()_pri_\bpc\()bpc_neon     // only pri
1:
        b               cdef_filter\w\()_pri_sec_\bpc\()bpc_neon // both pri and sec
endfunc
.endm

const div_table
        .short         840, 420, 280, 210, 168, 140, 120, 105
endconst

const alt_fact
        .short         420, 210, 140, 105, 105, 105, 105, 105, 140, 210, 420, 0
endconst

.macro cost_alt d1, d2, s1, s2, s3, s4
        smull           v22.4s,  \s1\().4h, \s1\().4h // sum_alt[n]*sum_alt[n]
        smull2          v23.4s,  \s1\().8h, \s1\().8h
        smull           v24.4s,  \s2\().4h, \s2\().4h
        smull           v25.4s,  \s3\().4h, \s3\().4h // sum_alt[n]*sum_alt[n]
        smull2          v26.4s,  \s3\().8h, \s3\().8h
        smull           v27.4s,  \s4\().4h, \s4\().4h
        mul             v22.4s,  v22.4s,  v29.4s      // sum_alt[n]^2*fact
        mla             v22.4s,  v23.4s,  v30.4s
        mla             v22.4s,  v24.4s,  v31.4s
        mul             v25.4s,  v25.4s,  v29.4s      // sum_alt[n]^2*fact
        mla             v25.4s,  v26.4s,  v30.4s
        mla             v25.4s,  v27.4s,  v31.4s
        addv            \d1, v22.4s                   // *cost_ptr
        addv            \d2, v25.4s                   // *cost_ptr
.endm

.macro find_best s1, s2, s3
.ifnb \s2
        mov             w5,  \s2\().s[0]
.endif
        cmp             w4,  w1                       // cost[n] > best_cost
        csel            w0,  w3,  w0,  gt             // best_dir = n
        csel            w1,  w4,  w1,  gt             // best_cost = cost[n]
.ifnb \s2
        add             w3,  w3,  #1                  // n++
        cmp             w5,  w1                       // cost[n] > best_cost
        mov             w4,  \s3\().s[0]
        csel            w0,  w3,  w0,  gt             // best_dir = n
        csel            w1,  w5,  w1,  gt             // best_cost = cost[n]
        add             w3,  w3,  #1                  // n++
.endif
.endm

// Steps for loading and preparing each row
.macro dir_load_step1 s1, bpc
.if \bpc == 8
        ld1             {\s1\().8b}, [x0], x1
.else
        ld1             {\s1\().8h}, [x0], x1
.endif
.endm

.macro dir_load_step2 s1, bpc
.if \bpc == 8
        usubl           \s1\().8h,  \s1\().8b, v31.8b
.else
        ushl            \s1\().8h,  \s1\().8h, v8.8h
.endif
.endm

.macro dir_load_step3 s1, bpc
// Nothing for \bpc == 8
.if \bpc != 8
        sub             \s1\().8h,  \s1\().8h, v31.8h
.endif
.endm

// int dav1d_cdef_find_dir_Xbpc_neon(const pixel *img, const ptrdiff_t stride,
//                                   unsigned *const var)
.macro find_dir bpc
function cdef_find_dir_\bpc\()bpc_neon, export=1
.if \bpc == 16
        str             d8,  [sp, #-0x10]!
        clz             w3,  w3                       // clz(bitdepth_max)
        sub             w3,  w3,  #24                 // -bitdepth_min_8
        dup             v8.8h,   w3
.endif
        sub             sp,  sp,  #32 // cost
        mov             w3,  #8
.if \bpc == 8
        movi            v31.16b, #128
.else
        movi            v31.8h,  #128
.endif
        movi            v30.16b, #0
        movi            v1.8h,   #0 // v0-v1 sum_diag[0]
        movi            v3.8h,   #0 // v2-v3 sum_diag[1]
        movi            v5.8h,   #0 // v4-v5 sum_hv[0-1]
        movi            v7.8h,   #0 // v6-v7 sum_alt[0]
        dir_load_step1  v26, \bpc       // Setup first row early
        movi            v17.8h,  #0 // v16-v17 sum_alt[1]
        movi            v18.8h,  #0 // v18-v19 sum_alt[2]
        dir_load_step2  v26, \bpc
        movi            v19.8h,  #0
        dir_load_step3  v26, \bpc
        movi            v21.8h,  #0 // v20-v21 sum_alt[3]

.irpc i, 01234567
        addv            h25,     v26.8h               // [y]
        rev64           v27.8h,  v26.8h
        addp            v28.8h,  v26.8h,  v30.8h      // [(x >> 1)]
        add             v5.8h,   v5.8h,   v26.8h      // sum_hv[1]
        ext             v27.16b, v27.16b, v27.16b, #8 // [-x]
        rev64           v29.4h,  v28.4h               // [-(x >> 1)]
        ins             v4.h[\i], v25.h[0]            // sum_hv[0]
.if \i < 6
        ext             v22.16b, v30.16b, v26.16b, #(16-2*(3-(\i/2)))
        ext             v23.16b, v26.16b, v30.16b, #(16-2*(3-(\i/2)))
        add             v18.8h,  v18.8h,  v22.8h      // sum_alt[2]
        add             v19.4h,  v19.4h,  v23.4h      // sum_alt[2]
.else
        add             v18.8h,  v18.8h,  v26.8h      // sum_alt[2]
.endif
.if \i == 0
        mov             v20.16b, v26.16b              // sum_alt[3]
.elseif \i == 1
        add             v20.8h,  v20.8h,  v26.8h      // sum_alt[3]
.else
        ext             v24.16b, v30.16b, v26.16b, #(16-2*(\i/2))
        ext             v25.16b, v26.16b, v30.16b, #(16-2*(\i/2))
        add             v20.8h,  v20.8h,  v24.8h      // sum_alt[3]
        add             v21.4h,  v21.4h,  v25.4h      // sum_alt[3]
.endif
.if \i == 0
        mov             v0.16b,  v26.16b              // sum_diag[0]
        dir_load_step1  v26, \bpc
        mov             v2.16b,  v27.16b              // sum_diag[1]
        dir_load_step2  v26, \bpc
        mov             v6.16b,  v28.16b              // sum_alt[0]
        dir_load_step3  v26, \bpc
        mov             v16.16b, v29.16b              // sum_alt[1]
.else
        ext             v22.16b, v30.16b, v26.16b, #(16-2*\i)
        ext             v23.16b, v26.16b, v30.16b, #(16-2*\i)
        ext             v24.16b, v30.16b, v27.16b, #(16-2*\i)
        ext             v25.16b, v27.16b, v30.16b, #(16-2*\i)
.if \i != 7 // Nothing to load for the final row
        dir_load_step1  v26, \bpc // Start setting up the next row early.
.endif
        add             v0.8h,   v0.8h,   v22.8h      // sum_diag[0]
        add             v1.8h,   v1.8h,   v23.8h      // sum_diag[0]
        add             v2.8h,   v2.8h,   v24.8h      // sum_diag[1]
        add             v3.8h,   v3.8h,   v25.8h      // sum_diag[1]
.if \i != 7
        dir_load_step2  v26, \bpc
.endif
        ext             v22.16b, v30.16b, v28.16b, #(16-2*\i)
        ext             v23.16b, v28.16b, v30.16b, #(16-2*\i)
        ext             v24.16b, v30.16b, v29.16b, #(16-2*\i)
        ext             v25.16b, v29.16b, v30.16b, #(16-2*\i)
.if \i != 7
        dir_load_step3  v26, \bpc
.endif
        add             v6.8h,   v6.8h,   v22.8h      // sum_alt[0]
        add             v7.4h,   v7.4h,   v23.4h      // sum_alt[0]
        add             v16.8h,  v16.8h,  v24.8h      // sum_alt[1]
        add             v17.4h,  v17.4h,  v25.4h      // sum_alt[1]
.endif
.endr

        movi            v31.4s,  #105

        smull           v26.4s,  v4.4h,   v4.4h       // sum_hv[0]*sum_hv[0]
        smlal2          v26.4s,  v4.8h,   v4.8h
        smull           v27.4s,  v5.4h,   v5.4h       // sum_hv[1]*sum_hv[1]
        smlal2          v27.4s,  v5.8h,   v5.8h
        mul             v26.4s,  v26.4s,  v31.4s      // cost[2] *= 105
        mul             v27.4s,  v27.4s,  v31.4s      // cost[6] *= 105
        addv            s4,  v26.4s                   // cost[2]
        addv            s5,  v27.4s                   // cost[6]

        rev64           v1.8h,   v1.8h
        rev64           v3.8h,   v3.8h
        ext             v1.16b,  v1.16b,  v1.16b, #10 // sum_diag[0][14-n]
        ext             v3.16b,  v3.16b,  v3.16b, #10 // sum_diag[1][14-n]

        str             s4,  [sp, #2*4]               // cost[2]
        str             s5,  [sp, #6*4]               // cost[6]

        movrel          x4,  div_table
        ld1             {v31.8h}, [x4]

        smull           v22.4s,  v0.4h,   v0.4h       // sum_diag[0]*sum_diag[0]
        smull2          v23.4s,  v0.8h,   v0.8h
        smlal           v22.4s,  v1.4h,   v1.4h
        smlal2          v23.4s,  v1.8h,   v1.8h
        smull           v24.4s,  v2.4h,   v2.4h       // sum_diag[1]*sum_diag[1]
        smull2          v25.4s,  v2.8h,   v2.8h
        smlal           v24.4s,  v3.4h,   v3.4h
        smlal2          v25.4s,  v3.8h,   v3.8h
        uxtl            v30.4s,  v31.4h               // div_table
        uxtl2           v31.4s,  v31.8h
        mul             v22.4s,  v22.4s,  v30.4s      // cost[0]
        mla             v22.4s,  v23.4s,  v31.4s      // cost[0]
        mul             v24.4s,  v24.4s,  v30.4s      // cost[4]
        mla             v24.4s,  v25.4s,  v31.4s      // cost[4]
        addv            s0,  v22.4s                   // cost[0]
        addv            s2,  v24.4s                   // cost[4]

        movrel          x5,  alt_fact
        ld1             {v29.4h, v30.4h, v31.4h}, [x5]// div_table[2*m+1] + 105

        str             s0,  [sp, #0*4]               // cost[0]
        str             s2,  [sp, #4*4]               // cost[4]

        uxtl            v29.4s,  v29.4h               // div_table[2*m+1] + 105
        uxtl            v30.4s,  v30.4h
        uxtl            v31.4s,  v31.4h

        cost_alt        s6,  s16, v6,  v7,  v16, v17  // cost[1], cost[3]
        cost_alt        s18, s20, v18, v19, v20, v21  // cost[5], cost[7]
        str             s6,  [sp, #1*4]               // cost[1]
        str             s16, [sp, #3*4]               // cost[3]

        mov             w0,  #0                       // best_dir
        mov             w1,  v0.s[0]                  // best_cost
        mov             w3,  #1                       // n

        str             s18, [sp, #5*4]               // cost[5]
        str             s20, [sp, #7*4]               // cost[7]

        mov             w4,  v6.s[0]

        find_best       v6,  v4, v16
        find_best       v16, v2, v18
        find_best       v18, v5, v20
        find_best       v20

        eor             w3,  w0,  #4                  // best_dir ^4
        ldr             w4,  [sp, w3, uxtw #2]
        sub             w1,  w1,  w4                  // best_cost - cost[best_dir ^ 4]
        lsr             w1,  w1,  #10
        str             w1,  [x2]                     // *var

        add             sp,  sp,  #32
.if \bpc == 16
        ldr             d8,  [sp], 0x10
.endif
        ret
endfunc
.endm
