; Copyright © 2022, VideoLAN and dav1d authors
; Copyright © 2022, Two Orioles, LLC
; All rights reserved.
;
; Redistribution and use in source and binary forms, with or without
; modification, are permitted provided that the following conditions are met:
;
; 1. Redistributions of source code must retain the above copyright notice, this
;    list of conditions and the following disclaimer.
;
; 2. Redistributions in binary form must reproduce the above copyright notice,
;    this list of conditions and the following disclaimer in the documentation
;    and/or other materials provided with the distribution.
;
; THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
; ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
; WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
; DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
; ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
; (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
; ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
; (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
; SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

%include "config.asm"
%include "ext/x86/x86inc.asm"
%include "x86/filmgrain_common.asm"

%if ARCH_X86_64

SECTION_RODATA 64

pb_even:       db  0,  2,  4,  6,  8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30
               db 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62
               db 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94
               db 96, 98,100,102,104,106,108,110,112,114,116,118,120,122,124,126
pb_odd:        db  1,  3,  5,  7,  9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31
               db 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59, 61, 63
               db 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 89, 91, 93, 95
               db 97, 99,101,103,105,107,109,111,113,115,117,119,121,123,125,127
interleave_hl: db  8,  0,  9,  1, 10,  2, 11,  3, 12,  4, 13,  5, 14,  6, 15,  7
pb_27_17_17_27:        db 27, 17, 17, 27,  0, 32,  0, 32
pb_23_22_0_32:         db 23, 22,  0, 32,  0, 32,  0, 32
pb_27_17:      times 2 db 27, 17
pb_23_22:      times 2 db 23, 22
pw_8:          times 2 dw 8
pw_1024:       times 2 dw 1024
pb_17_27:      times 2 db 17, 27
fg_max:        times 4 db 255
               times 4 db 240
               times 4 db 235
fg_min:        times 4 db 0
               times 4 db 16
noise_rnd:     times 2 dw 128
               times 2 dw 64
               times 2 dw 32
               times 2 dw 16

SECTION .text

INIT_ZMM avx512icl
cglobal fgy_32x32xn_8bpc, 6, 13, 22, dst, src, stride, fg_data, w, scaling, \
                                     grain_lut, h, sby, see, overlap
%define base r11-fg_min
    lea             r11, [fg_min]
    mov             r6d, [fg_dataq+FGData.scaling_shift]
    mov             r7d, [fg_dataq+FGData.clip_to_restricted_range]
    mov            sbyd, sbym
    mov        overlapd, [fg_dataq+FGData.overlap_flag]
    mov             r12, 0x0000000f0000000f ; h_overlap mask
    mova             m0, [scalingq+64*0]
    mova             m1, [scalingq+64*1]
    mova             m2, [scalingq+64*2]
    mova             m3, [scalingq+64*3]
    kmovq            k1, r12
    vbroadcasti32x4  m4, [base+interleave_hl]
    vpbroadcastd   ym16, [base+pb_27_17]
    vpbroadcastd    m12, [base+pb_17_27]
    vpbroadcastd     m6, [base+noise_rnd+r6*4-32]
    test           sbyd, sbyd
    setnz           r6b
    vpbroadcastd     m7, [base+fg_min+r7*4]
    vpbroadcastd     m8, [base+fg_max+r7*8]
    pxor             m5, m5
    vpbroadcastd     m9, [base+pw_1024]
    vpbroadcastq    m10, [base+pb_27_17_17_27]
    vmovdqa64   m12{k1}, m16
    test            r6b, overlapb
    jnz .v_overlap

    imul           seed, sbyd, (173 << 24) | 37
    add            seed, (105 << 24) | 178
    rorx           seed, seed, 24
    movzx          seed, seew
    xor            seed, [fg_dataq+FGData.seed]

    DEFINE_ARGS dst, src, stride, src_bak, w, offx, offy, \
                h, sby, see, overlap

    lea        src_bakq, [srcq+wq]
    neg              wq
    sub            dstq, srcq
.loop_x:
    rorx             r6, seeq, 1
    or             seed, 0xeff4
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d                 ; updated seed
    rorx          offyd, seed, 8
    rorx          offxq, seeq, 12
    and           offyd, 0xf
    imul          offyd, 164
    lea           offxd, [offyq+offxq*2+829] ; offy*stride+offx

    DEFINE_ARGS dst, src, stride, src_bak, w, offxy, grain_lut, \
                h, sby, see, overlap

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
.loop_y:
    movu           ym21, [grain_lutq+offxyq-82]
    vinserti32x8    m21, [grain_lutq+offxyq+ 0], 1
    call .add_noise
    sub              hb, 2
    jg .loop_y
    add              wq, 32
    jge .end
    lea            srcq, [src_bakq+wq]
    test       overlapd, overlapd
    jz .loop_x
    test           sbyd, sbyd
    jnz .hv_overlap

.loop_x_h_overlap:
    rorx             r6, seeq, 1
    or             seed, 0xeff4
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d                 ; updated seed

    DEFINE_ARGS dst, src, stride, src_bak, w, offx, offy, \
                h, sby, see, left_offxy

    rorx          offyd, seed, 8
    mov     left_offxyd, offxd               ; previous column's offy*stride
    rorx          offxq, seeq, 12
    and           offyd, 0xf
    imul          offyd, 164
    lea           offxd, [offyq+offxq*2+829] ; offy*stride+offx

    DEFINE_ARGS dst, src, stride, src_bak, w, offxy, grain_lut, \
                h, sby, see, left_offxy

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
.loop_y_h_overlap:
    movu           ym20, [grain_lutq+offxyq-82]
    vinserti32x8    m20, [grain_lutq+offxyq+ 0], 1
    movd           xm19, [grain_lutq+left_offxyq-50]
    vinserti32x4    m19, [grain_lutq+left_offxyq+32], 2
    punpcklbw       m19, m20
    pmaddubsw       m19, m10, m19
    pmulhrsw        m19, m9
    punpckhbw       m21, m20, m5
    packsswb    m20{k1}, m19, m19
    punpcklbw       m20, m5, m20
    call .add_noise_h
    sub              hb, 2
    jg .loop_y_h_overlap
    add              wq, 32
    jge .end
    lea            srcq, [src_bakq+wq]
    test           sbyd, sbyd
    jnz .hv_overlap
    jmp .loop_x_h_overlap

.v_overlap:
    DEFINE_ARGS dst, src, stride, fg_data, w, offy, offx, \
                h, sby, see, overlap

    movzx           r6d, sbyb
    imul           seed, [fg_dataq+FGData.seed], 0x00010001
    imul            r7d, r6d, 173 * 0x00010001
    imul            r6d, 37 * 0x01000100
    add             r7d, (105 << 16) | 188
    add             r6d, (178 << 24) | (141 << 8)
    and             r7d, 0x00ff00ff
    and             r6d, 0xff00ff00
    xor            seed, r7d
    xor            seed, r6d     ; (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, src_bak, w, offx, offy, \
                h, sby, see, overlap

    lea        src_bakq, [srcq+wq]
    neg              wq
    sub            dstq, srcq

    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            r7b          ; parity of top_seed
    shr            seed, 16
    shl             r7d, 16
    test           seeb, seeh
    setp            r7b          ; parity of cur_seed
    or              r6d, 0x00010001
    xor             r7d, r6d
    rorx           seed, r7d, 1  ; updated (cur_seed << 16) | top_seed
    rorx          offyd, seed, 8
    rorx          offxd, seed, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offxd, [offyq+offxq*2+0x10001*829+32*82]

    DEFINE_ARGS dst, src, stride, src_bak, w, offxy, grain_lut, \
                h, sby, see, overlap, top_offxy

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
    movzx    top_offxyd, offxyw
    shr          offxyd, 16
    movu           ym19, [grain_lutq+offxyq-82]
    vinserti32x8    m19, [grain_lutq+offxyq+ 0], 1
    movu           ym21, [grain_lutq+top_offxyq-82]
    vinserti32x8    m21, [grain_lutq+top_offxyq+ 0], 1
    punpckhbw       m20, m21, m19
    punpcklbw       m21, m19
    call .add_noise_v
    sub              hb, 2
    jg .loop_y
    add              wq, 32
    jge .end
    lea            srcq, [src_bakq+wq]

    ; since fg_dataq.overlap is guaranteed to be set, we never jump back
    ; to .v_overlap, and instead always fall-through to h+v overlap
.hv_overlap:
    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            r7b          ; parity of top_seed
    shr            seed, 16
    shl             r7d, 16
    test           seeb, seeh
    setp            r7b          ; parity of cur_seed
    or              r6d, 0x00010001
    xor             r7d, r6d
    rorx           seed, r7d, 1  ; updated (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, src_bak, w, offx, offy, \
                h, sby, see, left_offxy, top_offxy, topleft_offxy

    mov  topleft_offxyd, top_offxyd
    rorx          offyd, seed, 8
    mov     left_offxyd, offxd
    rorx          offxd, seed, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offxd, [offyq+offxq*2+0x10001*829+32*82]

    DEFINE_ARGS dst, src, stride, src_bak, w, offxy, grain_lut, \
                h, sby, see, left_offxy, top_offxy, topleft_offxy

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
    movzx    top_offxyd, offxyw
    shr          offxyd, 16
    movu           ym19, [grain_lutq+offxyq-82]
    vinserti32x8    m19, [grain_lutq+offxyq+ 0], 1
    movd           xm16, [grain_lutq+left_offxyq-50]
    vinserti32x4    m16, [grain_lutq+left_offxyq+32], 2
    movu           ym21, [grain_lutq+top_offxyq-82]
    vinserti32x8    m21, [grain_lutq+top_offxyq+ 0], 1
    movd           xm17, [grain_lutq+topleft_offxyq-50]
    vinserti32x4    m17, [grain_lutq+topleft_offxyq+32], 2
    ; do h interpolation first (so top | top/left -> top, left | cur -> cur)
    punpcklbw       m16, m19
    pmaddubsw       m16, m10, m16
    punpcklbw       m17, m21
    pmaddubsw       m17, m10, m17
    punpckhbw       m20, m21, m19
    pmulhrsw        m16, m9
    pmulhrsw        m17, m9
    packsswb    m19{k1}, m16, m16
    packsswb    m21{k1}, m17, m17
    ; followed by v interpolation (top | cur -> cur)
    punpcklbw       m21, m19
    call .add_noise_v
    sub              hb, 2
    jg .loop_y_h_overlap
    add              wq, 32
    lea            srcq, [src_bakq+wq]
    jl .hv_overlap
.end:
    RET
ALIGN function_align
.add_noise_v:
    pmaddubsw       m20, m12, m20
    pmaddubsw       m21, m12, m21
    pmulhrsw        m20, m9
    pmulhrsw        m21, m9
    packsswb        m21, m20
.add_noise:
    punpcklbw       m20, m5, m21
    punpckhbw       m21, m5
.add_noise_h:
    mova           ym18, [srcq+strideq*0]
    vinserti32x8    m18, [srcq+strideq*1], 1
    mova            m19, m0
    punpcklbw       m16, m18, m5
    vpermt2b        m19, m18, m1 ; scaling[  0..127]
    vpmovb2m         k2, m18
    punpckhbw       m17, m18, m5
    vpermi2b        m18, m2, m3  ; scaling[128..255]
    vmovdqu8    m19{k2}, m18     ; scaling[src]
    pshufb          m19, m4
    pmaddubsw       m18, m19, m20
    pmaddubsw       m19, m21
    add      grain_lutq, 82*2
    pmulhrsw        m18, m6      ; noise
    pmulhrsw        m19, m6
    paddw           m16, m18
    paddw           m17, m19
    packuswb        m16, m17
    pmaxub          m16, m7
    pminub          m16, m8
    mova    [dstq+srcq], ym16
    add            srcq, strideq
    vextracti32x8 [dstq+srcq], m16, 1
    add            srcq, strideq
    ret

%macro FGUV_FN 3 ; name, ss_hor, ss_ver
cglobal fguv_32x32xn_i%1_8bpc, 6, 14+%2, 22, dst, src, stride, fg_data, w, \
                                             scaling, grain_lut, h, sby, luma, \
                                             overlap, uv_pl, is_id, _, stride3
    lea             r11, [fg_min]
    mov             r6d, [fg_dataq+FGData.scaling_shift]
    mov             r7d, [fg_dataq+FGData.clip_to_restricted_range]
    mov             r9d, is_idm
    mov            sbyd, sbym
    mov        overlapd, [fg_dataq+FGData.overlap_flag]
%if %2
    mov             r12, 0x000f000f000f000f ; h_overlap mask
    vpbroadcastq    m10, [base+pb_23_22_0_32]
    lea        stride3q, [strideq*3]
%else
    mov             r12, 0x0000000f0000000f
    vpbroadcastq    m10, [base+pb_27_17_17_27]
%endif
    mova             m0, [scalingq+64*0]
    mova             m1, [scalingq+64*1]
    mova             m2, [scalingq+64*2]
    mova             m3, [scalingq+64*3]
    kmovq            k1, r12
    vbroadcasti32x4  m4, [base+interleave_hl]
    vpbroadcastd     m6, [base+noise_rnd+r6*4-32]
    vpbroadcastd     m7, [base+fg_min+r7*4]
    shlx            r7d, r7d, r9d
    vpbroadcastd     m8, [base+fg_max+r7*4]
    test           sbyd, sbyd
    setnz           r7b
    vpbroadcastd     m9, [base+pw_1024]
    mova            m11, [base+pb_even]
    mova            m12, [base+pb_odd]
    pxor             m5, m5
    mov              r5, r10mp      ; lstride
    cmp byte [fg_dataq+FGData.chroma_scaling_from_luma], 0
    jne .csfl

%macro %%FGUV_32x32xN_LOOP 3 ; not-csfl, ss_hor, ss_ver
    DEFINE_ARGS dst, src, stride, fg_data, w, lstride, grain_lut, \
                h, sby, see, overlap, uv_pl, _, _, stride3
%if %1
    mov             r6d, uv_plm
    vpbroadcastd    m16, [base+pw_8]
    vbroadcasti32x4 m14, [fg_dataq+FGData.uv_mult+r6*4]
    vpbroadcastw    m15, [fg_dataq+FGData.uv_offset+r6*4]
    pshufb          m14, m16     ; uv_luma_mult, uv_mult
%endif
    test            r7b, overlapb
    jnz %%v_overlap

    imul           seed, sbyd, (173 << 24) | 37
    add            seed, (105 << 24) | 178
    rorx           seed, seed, 24
    movzx          seed, seew
    xor            seed, [fg_dataq+FGData.seed]

    DEFINE_ARGS dst, src, stride, luma, w, lstride, grain_lut, \
                offx, offy, see, overlap, _, _, _, stride3

    mov           lumaq, r9mp
    lea             r11, [srcq+wq]
    lea             r12, [dstq+wq]
    lea             r13, [lumaq+wq*(1+%2)]
    mov           r11mp, r11
    mov           r12mp, r12
    neg              wq

%%loop_x:
    rorx             r6, seeq, 1
    or             seed, 0xeff4
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d     ; updated seed
    rorx          offyd, seed, 8
    rorx          offxq, seeq, 12
    and           offyd, 0xf
    imul          offyd, 164>>%3
    lea           offyd, [offyq+offxq*(2-%2)+(3+(6>>%3))*82+3+(6>>%2)] ; offy*stride+offx

    DEFINE_ARGS dst, src, stride, luma, w, lstride, grain_lut, \
                h, offxy, see, overlap, _, _, _, stride3

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
%%loop_y:
%if %2
    movu           xm21, [grain_lutq+offxyq+82*0]
    vinserti128    ym21, [grain_lutq+offxyq+82*1], 1
    vinserti32x4    m21, [grain_lutq+offxyq+82*2], 2
    vinserti32x4    m21, [grain_lutq+offxyq+82*3], 3
%else
    movu           ym21, [grain_lutq+offxyq+82*0]
    vinserti32x8    m21, [grain_lutq+offxyq+82*1], 1
%endif
    call %%add_noise
    sub              hb, 2<<%2
    jg %%loop_y
    add              wq, 32>>%2
    jge .end
    mov            srcq, r11mp
    mov            dstq, r12mp
    lea           lumaq, [r13+wq*(1<<%2)]
    add            srcq, wq
    add            dstq, wq
    test       overlapd, overlapd
    jz %%loop_x
    cmp       dword r8m, 0       ; sby
    jne %%hv_overlap

    ; horizontal overlap (without vertical overlap)
%%loop_x_h_overlap:
    rorx             r6, seeq, 1
    or             seed, 0xeff4
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d     ; updated seed

    DEFINE_ARGS dst, src, stride, luma, w, lstride, grain_lut, \
                offx, offy, see, left_offxy, _, _, _, stride3

    lea     left_offxyd, [offyq+(32>>%2)]         ; previous column's offy*stride+offx
    rorx          offyd, seed, 8
    rorx          offxq, seeq, 12
    and           offyd, 0xf
    imul          offyd, 164>>%3
    lea           offyd, [offyq+offxq*(2-%2)+(3+(6>>%3))*82+3+(6>>%2)] ; offy*stride+offx

    DEFINE_ARGS dst, src, stride, luma, w, lstride, grain_lut, \
                h, offxy, see, left_offxy, _, _, _, stride3

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
%%loop_y_h_overlap:
%if %2
    movu           xm20, [grain_lutq+offxyq     +82*0]
    movd           xm19, [grain_lutq+left_offxyq+82*0]
    vinserti32x4   ym20, [grain_lutq+offxyq     +82*1], 1
    vinserti32x4   ym19, [grain_lutq+left_offxyq+82*1], 1
    vinserti32x4    m20, [grain_lutq+offxyq     +82*2], 2
    vinserti32x4    m19, [grain_lutq+left_offxyq+82*2], 2
    vinserti32x4    m20, [grain_lutq+offxyq     +82*3], 3
    vinserti32x4    m19, [grain_lutq+left_offxyq+82*3], 3
%else
    movu           ym20, [grain_lutq+offxyq     + 0]
    movd           xm19, [grain_lutq+left_offxyq+ 0]
    vinserti32x8    m20, [grain_lutq+offxyq     +82], 1
    vinserti32x4    m19, [grain_lutq+left_offxyq+82], 2
%endif
    punpcklbw       m19, m20
    pmaddubsw       m19, m10, m19
    punpckhbw       m21, m20, m5
    pmulhrsw        m19, m9
    vpacksswb   m20{k1}, m19, m19
    punpcklbw       m20, m5, m20
    call %%add_noise_h
    sub              hb, 2<<%2
    jg %%loop_y_h_overlap
    add              wq, 32>>%2
    jge .end
    mov            srcq, r11mp
    mov            dstq, r12mp
    lea           lumaq, [r13+wq*(1<<%2)]
    add            srcq, wq
    add            dstq, wq
    cmp       dword r8m, 0       ; sby
    jne %%hv_overlap
    jmp %%loop_x_h_overlap

%%v_overlap:
    DEFINE_ARGS dst, src, stride, fg_data, w, lstride, grain_lut, \
                _, sby, see, overlap, _, _, _, stride3

    movzx          sbyd, sbyb
    imul           seed, [fg_dataq+FGData.seed], 0x00010001
    imul            r7d, sbyd, 173 * 0x00010001
    imul           sbyd, 37 * 0x01000100
    add             r7d, (105 << 16) | 188
    add            sbyd, (178 << 24) | (141 << 8)
    and             r7d, 0x00ff00ff
    and            sbyd, 0xff00ff00
    xor            seed, r7d
    xor            seed, sbyd    ; (cur_seed << 16) | top_seed

%if %3
    vpbroadcastd    m13, [base+pb_23_22]
    kxnorw           k3, k3, k3  ; v_overlap mask
%elif %2
    vbroadcasti32x8 m13, [base+pb_27_17]
    kxnord           k3, k3, k3
    pshufd          m13, m13, q0000 ; 8x27_17, 8x17_27
%else
    vpbroadcastd   ym16, [base+pb_27_17]
    vpbroadcastd    m13, [base+pb_17_27]
    vmovdqa64   m13{k1}, m16
%endif

    DEFINE_ARGS dst, src, stride, luma, w, lstride, grain_lut, \
                offx, offy, see, overlap, top_offxy, _, _, stride3

    mov           lumaq, r9mp
    lea             r11, [srcq+wq]
    lea             r12, [dstq+wq]
    lea             r13, [lumaq+wq*(1<<%2)]
    mov           r11mp, r11
    mov           r12mp, r12
    neg              wq

    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            r7b          ; parity of top_seed
    shr            seed, 16
    shl             r7d, 16
    test           seeb, seeh
    setp            r7b          ; parity of cur_seed
    or              r6d, 0x00010001
    xor             r7d, r6d
    rorx           seed, r7d, 1  ; updated (cur_seed << 16) | top_seed
    rorx          offyd, seed, 8
    rorx          offxd, seed, 12
    and           offyd, 0x000f000f
    and           offxd, 0x000f000f
    imul          offyd, 164>>%3
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyd, [offyq+offxq*(2-%2)+0x10001*((3+(6>>%3))*82+3+(6>>%2))+(32>>%3)*82]

    DEFINE_ARGS dst, src, stride, luma, w, lstride, grain_lut, \
                h, offxy, see, overlap, top_offxy, _, _, stride3

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
    movzx    top_offxyd, offxyw
    shr          offxyd, 16

%if %3
    movu           xm18, [grain_lutq+offxyq+82*0]
    movu           xm20, [grain_lutq+top_offxyq+82*0]
    ; only interpolate first line, insert remaining line unmodified
    vbroadcasti128 ym21, [grain_lutq+offxyq+82*1]
    vinserti32x4    m21, [grain_lutq+offxyq+82*2], 2
    vinserti32x4    m21, [grain_lutq+offxyq+82*3], 3
    punpcklbw      xm19, xm20, xm18
    punpckhbw      xm20, xm18
%elif %2
    movu           xm18, [grain_lutq+offxyq+82*0]
    vinserti128    ym18, [grain_lutq+offxyq+82*1], 1
    movu           xm20, [grain_lutq+top_offxyq+82*0]
    vinserti32x4   ym20, [grain_lutq+top_offxyq+82*1], 1
    vbroadcasti32x4 m21, [grain_lutq+offxyq+82*2]
    vinserti32x4    m21, [grain_lutq+offxyq+82*3], 3
    punpcklbw      ym19, ym20, ym18
    punpckhbw      ym20, ym18
%else
    movu           ym21, [grain_lutq+offxyq+82*0]
    vinserti32x8    m21, [grain_lutq+offxyq+82*1], 1
    movu           ym20, [grain_lutq+top_offxyq+82*0]
    vinserti32x8    m20, [grain_lutq+top_offxyq+82*1], 1
%endif
    call %%add_noise_v
    sub              hb, 2<<%2
    jg %%loop_y
    add              wq, 32>>%2
    jge .end
    mov            srcq, r11mp
    mov            dstq, r12mp
    lea           lumaq, [r13+wq*(1<<%2)]
    add            srcq, wq
    add            dstq, wq

%%hv_overlap:
    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            r7b          ; parity of top_seed
    shr            seed, 16
    shl             r7d, 16
    test           seeb, seeh
    setp            r7b          ; parity of cur_seed
    or              r6d, 0x00010001
    xor             r7d, r6d
    rorx           seed, r7d, 1  ; updated (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, luma, w, lstride, grain_lut, \
                offx, offy, see, left_offxy, top_offxy, topleft_offxy, _, stride3

    lea  topleft_offxyd, [top_offxyq+(32>>%2)]
    lea     left_offxyd, [offyq+(32>>%2)]
    rorx          offyd, seed, 8
    rorx          offxd, seed, 12
    and           offyd, 0x000f000f
    and           offxd, 0x000f000f
    imul          offyd, 164>>%3
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyd, [offyq+offxq*(2-%2)+0x10001*((3+(6>>%3))*82+3+(6>>%2))+(32>>%3)*82]

    DEFINE_ARGS dst, src, stride, luma, w, lstride, grain_lut, \
                h, offxy, see, left_offxy, top_offxy, topleft_offxy, _, stride3

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
    movzx    top_offxyd, offxyw
    shr          offxyd, 16

%if %2
    movu           xm21, [grain_lutq+offxyq+82*0]
    movd           xm16, [grain_lutq+left_offxyq+82*0]
    vinserti128    ym21, [grain_lutq+offxyq+82*1], 1
    vinserti128    ym16, [grain_lutq+left_offxyq+82*1], 1
    vinserti32x4    m21, [grain_lutq+offxyq+82*2], 2
    vinserti32x4    m16, [grain_lutq+left_offxyq+82*2], 2
    vinserti32x4    m21, [grain_lutq+offxyq+82*3], 3
    vinserti32x4    m16, [grain_lutq+left_offxyq+82*3], 3
    movd           xm18, [grain_lutq+topleft_offxyq+82*0]
    movu           xm20, [grain_lutq+top_offxyq]
    ; do h interpolation first (so top | top/left -> top, left | cur -> cur)
    punpcklbw       m16, m21
%if %3
    punpcklbw      xm18, xm20
%else
    vinserti128    ym18, [grain_lutq+topleft_offxyq+82*1], 1
    vinserti128    ym20, [grain_lutq+top_offxyq+82*1], 1
    punpcklbw      ym18, ym20
%endif
    punpcklqdq      m16, m18
    pmaddubsw       m16, m10, m16
    pmulhrsw        m16, m9
    packsswb        m16, m16
    vmovdqu8    m21{k1}, m16
%if %3
    vpalignr   xm20{k1}, xm16, xm16, 4
    punpcklbw      xm19, xm20, xm21
    punpckhbw      xm20, xm21
%else
    vpalignr   ym20{k1}, ym16, ym16, 4
    punpcklbw      ym19, ym20, ym21
    punpckhbw      ym20, ym21
%endif
%else
    movu           ym21, [grain_lutq+offxyq+82*0]
    vinserti32x8    m21, [grain_lutq+offxyq+82*1], 1
    movd           xm16, [grain_lutq+left_offxyq+82*0]
    vinserti32x4    m16, [grain_lutq+left_offxyq+82*1], 2
    movu           ym20, [grain_lutq+top_offxyq+82*0]
    vinserti32x8    m20, [grain_lutq+top_offxyq+82*1], 1
    movd           xm18, [grain_lutq+topleft_offxyq+82*0]
    vinserti32x4    m18, [grain_lutq+topleft_offxyq+82*1], 2
    punpcklbw       m16, m21
    punpcklbw       m18, m20
    punpcklqdq      m16, m18
    pmaddubsw       m16, m10, m16
    pmulhrsw        m16, m9
    packsswb        m16, m16
    vpalignr    m20{k1}, m16, m16, 4
    vmovdqu8    m21{k1}, m16
%endif
    call %%add_noise_v
    sub              hb, 2<<%2
    jg %%loop_y_h_overlap
    add              wq, 32>>%2
    jge .end
    mov            srcq, r11mp
    mov            dstq, r12mp
    lea           lumaq, [r13+wq*(1<<%2)]
    add            srcq, wq
    add            dstq, wq
    jmp %%hv_overlap
ALIGN function_align
%%add_noise_v:
%if %3
    pmaddubsw      xm19, xm13, xm19
    pmaddubsw      xm20, xm13, xm20
    pmulhrsw       xm19, xm9
    pmulhrsw       xm20, xm9
    vpacksswb   m21{k3}, m19, m20
%elif %2
    pmaddubsw      ym19, ym13, ym19
    pmaddubsw      ym20, ym13, ym20
    pmulhrsw       ym19, ym9
    pmulhrsw       ym20, ym9
    vpacksswb   m21{k3}, m19, m20
%else
    punpcklbw       m19, m20, m21
    punpckhbw       m20, m21
    pmaddubsw       m19, m13, m19
    pmaddubsw       m20, m13, m20
    pmulhrsw        m19, m9
    pmulhrsw        m20, m9
    packsswb        m21, m19, m20
%endif
%%add_noise:
    punpcklbw       m20, m5, m21
    punpckhbw       m21, m5
%%add_noise_h:
    mova           ym18, [lumaq+lstrideq*(0<<%3)]
    vinserti32x8    m18, [lumaq+lstrideq*(1<<%3)], 1
%if %2
    lea           lumaq, [lumaq+lstrideq*(2<<%3)]
    mova           ym16, [lumaq+lstrideq*(0<<%3)]
    vinserti32x8    m16, [lumaq+lstrideq*(1<<%3)], 1
    mova           xm17, [srcq+strideq*0]
    mova            m19, m11
    vpermi2b        m19, m18, m16
    vinserti128    ym17, [srcq+strideq*1], 1
    vpermt2b        m18, m12, m16
    vinserti32x4    m17, [srcq+strideq*2], 2
    pavgb           m18, m19
    vinserti32x4    m17, [srcq+stride3q ], 3
%else
    mova           ym17, [srcq+strideq*0]
    vinserti32x8    m17, [srcq+strideq*1], 1
%endif
%if %1
    punpckhbw       m19, m18, m17
    punpcklbw       m18, m17     ; { luma, chroma }
    pmaddubsw       m19, m14
    pmaddubsw       m18, m14
    psraw           m19, 6
    psraw           m18, 6
    paddw           m19, m15
    paddw           m18, m15
    packuswb        m18, m19
.add_noise_main:
    mova            m19, m0
    vpermt2b        m19, m18, m1 ; scaling[  0..127]
    vpmovb2m         k2, m18
    vpermi2b        m18, m2, m3  ; scaling[128..255]
    vmovdqu8    m19{k2}, m18     ; scaling[src]
    pshufb          m19, m4
    pmaddubsw       m18, m19, m20
    pmaddubsw       m19, m21
    add      grain_lutq, 82*2<<%2
    lea           lumaq, [lumaq+lstrideq*(2<<%3)]
    lea            srcq, [srcq+strideq*(2<<%2)]
    pmulhrsw        m18, m6      ; noise
    pmulhrsw        m19, m6
    punpcklbw       m16, m17, m5 ; chroma
    punpckhbw       m17, m5
    paddw           m16, m18
    paddw           m17, m19
    packuswb        m16, m17
    pmaxub          m16, m7
    pminub          m16, m8
%if %2
    mova          [dstq+strideq*0], xm16
    vextracti128  [dstq+strideq*1], ym16, 1
    vextracti32x4 [dstq+strideq*2], m16, 2
    vextracti32x4 [dstq+stride3q ], m16, 3
%else
    mova          [dstq+strideq*0], ym16
    vextracti32x8 [dstq+strideq*1], m16, 1
%endif
    lea            dstq, [dstq+strideq*(2<<%2)]
    ret
%else
    jmp .add_noise_main
%endif
%endmacro

    %%FGUV_32x32xN_LOOP 1, %2, %3
.csfl:
    %%FGUV_32x32xN_LOOP 0, %2, %3
.end:
    RET
%endmacro

FGUV_FN 420, 1, 1
FGUV_FN 422, 1, 0
FGUV_FN 444, 0, 0

%endif ; ARCH_X86_64
