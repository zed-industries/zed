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
pb_0to63:      db  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15
               db 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
               db 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47
               db 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
scale_mask:    db -1, -1,  0, -1, -1, -1,  4, -1, -1, -1,  8, -1, -1, -1, 12, -1
scale_shift:           dw   7,   7,   6,   6,   5,   5,   4,   4
pw_27_17_17_27:        dw 108,  68,  68, 108,  27,  17,  17,  27
pw_23_22:              dw  92,  88,   0, 128,  23,  22,   0,  32
fg_min:        times 2 dw 0
               times 2 dw 64
               times 2 dw 256
fg_max:        times 2 dw 1023
               times 2 dw 4095
               times 2 dw 960
               times 2 dw 3840
               times 2 dw 940
               times 2 dw 3760
scale_rnd:             dd 64
                       dd 16
uv_offset_mul:         dd 256
                       dd 1024
pb_8_9_0_1:            db 8, 9, 0, 1

SECTION .text

INIT_ZMM avx512icl
cglobal fgy_32x32xn_16bpc, 6, 15, 21, dst, src, stride, fg_data, w, scaling, \
                                      grain_lut, offx, sby, see, offy, src_bak
%define base r11-fg_min
    lea             r11, [fg_min]
    mov             r6d, r9m    ; bdmax
    mov             r9d, [fg_dataq+FGData.clip_to_restricted_range]
    mov             r7d, [fg_dataq+FGData.scaling_shift]
    mov            sbyd, sbym
    vpbroadcastd     m6, r9m
    shr             r6d, 11     ; is_12bpc
    vbroadcasti32x4  m7, [base+scale_mask]
    shlx           r10d, r9d, r6d
    vpbroadcastd    m10, [base+scale_shift+r7*4-32]
    lea             r9d, [r6+r9*4]
    vpbroadcastd     m8, [base+fg_min+r10*4]
    kxnorw           k1, k1, k1 ; 0xffff
    vpbroadcastd     m9, [base+fg_max+r9*4]
    mov             r12, 0xeeeeeeeeeeeeeeee
    vpbroadcastd    m19, [base+scale_rnd+r6*4]
    kshiftrb         k2, k1, 4  ; 0xf
    vpbroadcastq   xm20, [base+pw_27_17_17_27+r6*8]
    kmovq            k3, r12
    vpbroadcastd    m11, [base+scale_shift+r6*8+4]
    test           sbyd, sbyd
    setnz           r7b
    vpbroadcastd    m12, [base+pw_27_17_17_27+r6*8+0]
    vpbroadcastd    m13, [base+pw_27_17_17_27+r6*8+4]
    test            r7b, [fg_dataq+FGData.overlap_flag]
    jnz .v_overlap

    imul           seed, sbyd, (173 << 24) | 37
    add            seed, (105 << 24) | 178
    rorx           seed, seed, 24
    movzx          seed, seew
    xor            seed, [fg_dataq+FGData.seed]
    lea        src_bakq, [srcq+wq*2]
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
    lea           offyd, [offyq+offxq*2+747] ; offy*stride+offx

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, h, \
                sby, see, offxy, src_bak

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
.loop_y:
    movu             m4, [grain_lutq+offxyq*2+82*0]
    movu             m5, [grain_lutq+offxyq*2+82*2]
    call .add_noise
    sub              hb, 2
    jg .loop_y
    add              wq, 32
    jge .end
    lea            srcq, [src_bakq+wq*2]
    cmp byte [fg_dataq+FGData.overlap_flag], 0
    je .loop_x
    test           sbyd, sbyd
    jnz .hv_overlap

    ; horizontal overlap (without vertical overlap)
.loop_x_h_overlap:
    rorx             r6, seeq, 1
    or             seed, 0xeff4
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d                 ; updated seed

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, offx, \
                sby, see, offy, src_bak, left_offxy

    lea     left_offxyd, [offyq+73]          ; previous column's offy*stride+offx
    rorx          offyd, seed, 8
    rorx          offxq, seeq, 12
    and           offyd, 0xf
    imul          offyd, 164
    lea           offyd, [offyq+offxq*2+747] ; offy*stride+offx

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, h, \
                sby, see, offxy, src_bak, left_offxy

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
.loop_y_h_overlap:
    movu             m4, [grain_lutq+offxyq*2+82*0]
    movu             m5, [grain_lutq+offxyq*2+82*2]
    movd           xm17, [grain_lutq+left_offxyq*2-82*1]
    pinsrd         xm17, [grain_lutq+left_offxyq*2+82*1], 1
    punpckldq      xm16, xm4, xm5
    punpcklwd      xm17, xm16
    mova           xm16, xm19
    vpdpwssd       xm16, xm20, xm17
    psrad          xm16, 1
    packssdw       xm16, xm16
    vpsravw        xm16, xm11
    vmovdqu8     m4{k2}, m16
    vpalignr     m5{k2}, m16, m16, 4
    call .add_noise
    sub              hb, 2
    jg .loop_y_h_overlap
    add              wq, 32
    jge .end
    lea            srcq, [src_bakq+wq*2]
    test           sbyd, sbyd
    jnz .hv_overlap
    jmp .loop_x_h_overlap

.v_overlap:
    movzx          sbyd, sbyb
    imul           seed, [fg_dataq+FGData.seed], 0x00010001
    imul            r7d, sbyd, 173 * 0x00010001
    imul           sbyd, 37 * 0x01000100
    add             r7d, (105 << 16) | 188
    add            sbyd, (178 << 24) | (141 << 8)
    and             r7d, 0x00ff00ff
    and            sbyd, 0xff00ff00
    xor            seed, r7d
    xor            seed, sbyd               ; (cur_seed << 16) | top_seed
    lea        src_bakq, [srcq+wq*2]
    neg              wq
    sub            dstq, srcq

    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            r7b                     ; parity of top_seed
    shr            seed, 16
    shl             r7d, 16
    test           seeb, seeh
    setp            r7b                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor             r7d, r6d
    rorx           seed, r7d, 1             ; updated (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, offx, \
                sby, see, offy, src_bak, _, top_offxy

    rorx          offyd, seed, 8
    rorx          offxd, seed, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyd, [offyq+offxq*2+0x10001*747+32*82]

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, h, \
                sby, see, offxy, src_bak, _, top_offxy

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
    movzx    top_offxyd, offxyw
    shr          offxyd, 16

    movu            m16, [grain_lutq+offxyq*2+82*0]
    movu             m0, [grain_lutq+top_offxyq*2+82*0]
    movu            m17, [grain_lutq+offxyq*2+82*2]
    movu             m1, [grain_lutq+top_offxyq*2+82*2]
    punpckhwd        m4, m0, m16
    punpcklwd        m0, m16
    punpckhwd        m5, m1, m17
    punpcklwd        m1, m17
    call .add_noise_v
    sub              hb, 2
    jg .loop_y
    add              wq, 32
    jge .end
    lea            srcq, [src_bakq+wq*2]

    ; since fg_dataq.overlap is guaranteed to be set, we never jump back
    ; to .v_overlap, and instead always fall-through to .hv_overlap
.hv_overlap:
    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            r7b                     ; parity of top_seed
    shr            seed, 16
    shl             r7d, 16
    test           seeb, seeh
    setp            r7b                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor             r7d, r6d
    rorx           seed, r7d, 1             ; updated (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, offx, \
                sby, see, offy, src_bak, left_offxy, top_offxy, topleft_offxy

    lea  topleft_offxyd, [top_offxyq+73]
    lea     left_offxyd, [offyq+73]
    rorx          offyd, seed, 8
    rorx          offxd, seed, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyd, [offyq+offxq*2+0x10001*747+32*82]

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, h, \
                sby, see, offxy, src_bak, left_offxy, top_offxy, topleft_offxy

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
    movzx    top_offxyd, offxyw
    shr          offxyd, 16

    movu             m5, [grain_lutq+offxyq*2+82*0]
    movu             m0, [grain_lutq+top_offxyq*2+82*0]
    movd           xm17, [grain_lutq+left_offxyq*2-82*1]
    pinsrd         xm17, [grain_lutq+topleft_offxyq*2-82*1], 1
    movu             m2, [grain_lutq+offxyq*2+82*2]
    movu             m1, [grain_lutq+top_offxyq*2+82*2]
    movd           xm18, [grain_lutq+left_offxyq*2+82*1]
    pinsrd         xm18, [grain_lutq+topleft_offxyq*2+82*1], 1
    punpckldq      xm16, xm5, xm0
    punpcklwd      xm17, xm16
    mova           xm16, xm19
    vpdpwssd       xm16, xm20, xm17
    punpckldq      xm17, xm2, xm1
    punpcklwd      xm18, xm17
    mova           xm17, xm19
    vpdpwssd       xm17, xm20, xm18
    punpckhwd        m4, m0, m5
    punpcklwd        m0, m5
    punpckhwd        m5, m1, m2
    punpcklwd        m1, m2
    psrad          xm16, 1
    psrad          xm17, 1
    packssdw       xm16, xm17
    vpsravw        xm16, xm11
    vpshuflw     m0{k2}, m16, q1302
    punpckhqdq     xm16, xm16
    vpshuflw     m1{k2}, m16, q1302
    call .add_noise_v
    sub              hb, 2
    jg .loop_y_h_overlap
    add              wq, 32
    lea            srcq, [src_bakq+wq*2]
    jl .hv_overlap
.end:
    RET
ALIGN function_align
.add_noise_v:
    mova             m2, m19
    vpdpwssd         m2, m12, m4
    mova             m3, m19
    vpdpwssd         m3, m13, m5
    mova             m4, m19
    vpdpwssd         m4, m12, m0
    mova             m5, m19
    vpdpwssd         m5, m13, m1
    REPX   {psrad x, 1}, m2, m3, m4, m5
    packssdw         m4, m2
    packssdw         m5, m3
    vpsravw          m4, m11
    vpsravw          m5, m11
.add_noise:
    mova             m0, [srcq+strideq*0]
    mova             m1, [srcq+strideq*1]
    kmovw            k4, k1
    pand            m16, m6, m0
    psrld            m3, m0, 16
    vpgatherdd   m2{k4}, [scalingq+m16]
    vpcmpud          k4, m3, m6, 2 ; px <= bdmax
    vpgatherdd  m16{k4}, [scalingq+m3]
    kmovw            k4, k1
    pand            m17, m6, m1
    vpgatherdd   m3{k4}, [scalingq+m17]
    vpshufb      m2{k3}, m16, m7
    psrld           m16, m1, 16
    vpcmpud          k4, m16, m6, 2
    vpgatherdd  m17{k4}, [scalingq+m16]
    vpshufb      m3{k3}, m17, m7
    vpsllvw          m2, m10
    vpsllvw          m3, m10
    pmulhrsw         m4, m2
    pmulhrsw         m5, m3
    add      grain_lutq, 82*4
    paddw            m0, m4
    paddw            m1, m5
    pmaxsw           m0, m8
    pmaxsw           m1, m8
    pminsw           m0, m9
    pminsw           m1, m9
    mova    [dstq+srcq], m0
    add            srcq, strideq
    mova    [dstq+srcq], m1
    add            srcq, strideq
    ret

%macro FGUV_FN 3 ; name, ss_hor, ss_ver
cglobal fguv_32x32xn_i%1_16bpc, 6, 15, 22, dst, src, stride, fg_data, w, scaling, \
                                           grain_lut, h, sby, luma, lstride, uv_pl, is_id
%define base r12-fg_min
    lea             r12, [fg_min]
    mov             r9d, r13m            ; bdmax
    mov             r7d, [fg_dataq+FGData.scaling_shift]
    mov             r6d, [fg_dataq+FGData.clip_to_restricted_range]
    mov            r11d, is_idm
    kxnorw           k1, k1, k1          ; 0xffff
    vpbroadcastd     m5, r13m
    mov             r13, 0xeeeeeeeeeeeeeeee
    vbroadcasti32x4  m6, [base+scale_mask]
    shr             r9d, 11              ; is_12bpc
    vpbroadcastd     m7, [base+scale_shift+r7*4-32]
    shlx           r10d, r6d, r9d
    mov            sbyd, sbym
    shlx            r6d, r6d, r11d
    vpbroadcastd     m8, [base+fg_min+r10*4]
    lea             r6d, [r9+r6*2]
    vpbroadcastd     m9, [base+fg_max+r6*4]
    kmovq            k2, r13
    vpbroadcastd    m20, [base+scale_rnd+r9*4]
    packssdw         m4, m5, m5
    vpbroadcastd    m21, [base+scale_shift+r9*8+4]
%if %2
    mova            m12, [base+pb_0to63] ; pw_even
    mov            r13d, 0x0101
    vpbroadcastq    m10, [base+pw_23_22+r9*8]
    kmovw            k3, r13d
%if %3
    pshufd          m11, m10, q0000
%else
    vpbroadcastd   ym16, [base+pw_27_17_17_27+r9*8+0]
    vpbroadcastd    m11, [base+pw_27_17_17_27+r9*8+4]
    vmovdqu16   m11{k1}, m16
%endif
    psrlw           m13, m12, 8          ; pw_odd
%else
    vpbroadcastq    m10, [base+pw_27_17_17_27+r9*8]
    kshiftrb         k3, k1, 7           ; 0x01
    kshiftrb         k4, k1, 4           ; 0x0f
    pshufd          m11, m10, q0000
%endif
    mov        lstrideq, r10mp
    test           sbyd, sbyd
    setnz           r7b
    cmp byte [fg_dataq+FGData.chroma_scaling_from_luma], 0
    jne .csfl

%macro %%FGUV_32x32xN_LOOP 3 ; not-csfl, ss_hor, ss_ver
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                _, sby, see, lstride

%if %1
    mov             r6d, r11m
    vpbroadcastd     m0, [base+uv_offset_mul+r9*4]
    vpbroadcastd     m1, [base+pb_8_9_0_1]
    vpbroadcastd    m14, [fg_dataq+FGData.uv_offset+r6*4]
    vbroadcasti32x4 m15, [fg_dataq+FGData.uv_mult+r6*4]
    pmaddwd         m14, m0
    pshufb          m15, m1 ; { uv_luma_mult, uv_mult }
%endif
    test            r7b, [fg_dataq+FGData.overlap_flag]
    jnz %%v_overlap

    imul           seed, sbyd, (173 << 24) | 37
    add            seed, (105 << 24) | 178
    rorx           seed, seed, 24
    movzx          seed, seew
    xor            seed, [fg_dataq+FGData.seed]

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                offx, offy, see, lstride, luma

    mov           lumaq, r9mp
    lea             r12, [srcq+wq*2]
    lea             r13, [dstq+wq*2]
    lea             r14, [lumaq+wq*(2<<%2)]
    mov            r9mp, r12
    mov           r10mp, r13
    mov           r11mp, r14
    neg              wq

%%loop_x:
    rorx             r6, seeq, 1
    or             seed, 0xeff4
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d               ; updated seed
    rorx          offyd, seed, 8
    rorx          offxq, seeq, 12
    and           offyd, 0xf
    imul          offyd, 164>>%3
    lea           offyd, [offyq+offxq*(2-%2)+(3+(6>>%3))*82+(3+(6>>%2))] ; offy*stride+offx

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, offxy, see, lstride, luma

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
%%loop_y:
%if %2
    movu           ym18, [grain_lutq+offxyq*2+82*0]
    vinserti32x8    m18, [grain_lutq+offxyq*2+82*2], 1
    movu           ym19, [grain_lutq+offxyq*2+82*4]
    vinserti32x8    m19, [grain_lutq+offxyq*2+82*6], 1
%else
    movu            m18, [grain_lutq+offxyq*2+82*0]
    movu            m19, [grain_lutq+offxyq*2+82*2]
%endif
    call %%add_noise
    sub              hb, 2<<%2
    jg %%loop_y
    add              wq, 32>>%2
    jge .end
    mov            srcq, r9mp
    mov            dstq, r10mp
    mov           lumaq, r11mp
    lea            srcq, [srcq+wq*2]
    lea            dstq, [dstq+wq*2]
    lea           lumaq, [lumaq+wq*(2<<%2)]
    cmp byte [fg_dataq+FGData.overlap_flag], 0
    je %%loop_x
    cmp       dword r8m, 0 ; sby
    jne %%hv_overlap

    ; horizontal overlap (without vertical overlap)
%%loop_x_h_overlap:
    rorx             r6, seeq, 1
    or             seed, 0xEFF4
    test           seeb, seeh
    lea            seed, [r6+0x8000]
    cmovp          seed, r6d               ; updated seed

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                offx, offy, see, lstride, luma, left_offxy

    lea     left_offxyd, [offyq+(32>>%2)]  ; previous column's offy*stride+offx
    rorx          offyd, seed, 8
    rorx          offxq, seeq, 12
    and           offyd, 0xf
    imul          offyd, 164>>%3
    lea           offyd, [offyq+offxq*(2-%2)+(3+(6>>%3))*82+3+(6>>%2)] ; offy*stride+offx

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, offxy, see, lstride, luma, left_offxy

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
%%loop_y_h_overlap:
%if %2
    movu           ym18, [grain_lutq+offxyq*2+82*0]
    vinserti32x8    m18, [grain_lutq+offxyq*2+82*2], 1
    movu           ym19, [grain_lutq+offxyq*2+82*4]
    vinserti32x8    m19, [grain_lutq+offxyq*2+82*6], 1
    movd           xm16, [grain_lutq+left_offxyq*2+82*0]
    vinserti32x4    m16, [grain_lutq+left_offxyq*2+82*2], 2
    movd           xm17, [grain_lutq+left_offxyq*2+82*4]
    vinserti32x4    m17, [grain_lutq+left_offxyq*2+82*6], 2
    punpckldq       m16, m17
    punpckldq       m17, m18, m19
    punpcklwd       m16, m17
    mova            m17, m20
    vpdpwssd        m17, m16, m10
    psrad           m17, 1
    packssdw        m17, m17
    vpsravw         m17, m21
%else
    movu            m18, [grain_lutq+offxyq*2+82*0]
    movu            m19, [grain_lutq+offxyq*2+82*2]
    movd           xm16, [grain_lutq+left_offxyq*2+82*0]
    pinsrd         xm16, [grain_lutq+left_offxyq*2+82*2], 1
    punpckldq      xm17, xm18, xm19
    punpcklwd      xm16, xm17
    mova           xm17, xm20
    vpdpwssd       xm17, xm16, xm10
    psrad          xm17, 1
    packssdw       xm17, xm17
    vpsravw        xm17, xm21
%endif
    vmovdqa32   m18{k3}, m17
    vpshufd     m19{k3}, m17, q0321
    call %%add_noise
    sub              hb, 2<<%2
    jg %%loop_y_h_overlap
    add              wq, 32>>%2
    jge .end
    mov            srcq, r9mp
    mov            dstq, r10mp
    mov           lumaq, r11mp
    lea            srcq, [srcq+wq*2]
    lea            dstq, [dstq+wq*2]
    lea           lumaq, [lumaq+wq*(2<<%2)]
    cmp       dword r8m, 0 ; sby
    jne %%hv_overlap
    jmp %%loop_x_h_overlap

%%v_overlap:
    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                _, sby, see, lstride

    movzx          sbyd, sbyb
    imul           seed, [fg_dataq+FGData.seed], 0x00010001
    imul            r7d, sbyd, 173 * 0x00010001
    imul           sbyd, 37 * 0x01000100
    add             r7d, (105 << 16) | 188
    add            sbyd, (178 << 24) | (141 << 8)
    and             r7d, 0x00ff00ff
    and            sbyd, 0xff00ff00
    xor            seed, r7d
    xor            seed, sbyd               ; (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                offx, offy, see, lstride, luma, _, top_offxy

    mov           lumaq, r9mp
    lea             r12, [srcq+wq*2]
    lea             r13, [dstq+wq*2]
    lea             r14, [lumaq+wq*(2<<%2)]
    mov            r9mp, r12
    mov           r10mp, r13
    mov           r11mp, r14
    neg              wq

    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            r7b                     ; parity of top_seed
    shr            seed, 16
    shl             r7d, 16
    test           seeb, seeh
    setp            r7b                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor             r7d, r6d
    rorx           seed, r7d, 1             ; updated (cur_seed << 16) | top_seed

    rorx          offyd, seed, 8
    rorx          offxd, seed, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164>>%3
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyd, [offyq+offxq*(2-%2)+0x10001*((3+(6>>%3))*82+3+(6>>%2))+(32>>%3)*82]

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, offxy, see, lstride, luma, _, top_offxy

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
    movzx    top_offxyd, offxyw
    shr          offxyd, 16

%if %3
    movu           ym16, [grain_lutq+offxyq*2+82*0]
    movu            ym1, [grain_lutq+top_offxyq*2+82*0]
    vbroadcasti32x8 m18, [grain_lutq+offxyq*2+82*2]
    movu           ym19, [grain_lutq+offxyq*2+82*4]
    vinserti32x8    m19, [grain_lutq+offxyq*2+82*6], 1
    punpcklwd      ym17, ym1, ym16
    punpckhwd       ym1, ym16
%elif %2
    movu           ym18, [grain_lutq+offxyq*2+82*0]
    vinserti32x8    m18, [grain_lutq+offxyq*2+82*2], 1
    movu           ym17, [grain_lutq+top_offxyq*2+82*0]
    vinserti32x8    m17, [grain_lutq+top_offxyq*2+82*2], 1
    movu           ym19, [grain_lutq+offxyq*2+82*4]
    vinserti32x8    m19, [grain_lutq+offxyq*2+82*6], 1
    punpcklwd       m16, m17, m18
    punpckhwd       m17, m18
%else
    movu            m18, [grain_lutq+offxyq*2+82*0]
    movu            m19, [grain_lutq+top_offxyq*2+82*0]
    movu             m2, [grain_lutq+offxyq*2+82*2]
    movu            m16, [grain_lutq+top_offxyq*2+82*2]
    punpckhwd        m1, m19, m18
    punpcklwd       m19, m18
    punpckhwd       m18, m2, m16
    punpcklwd        m2, m16
%endif
    call %%add_noise_v
    sub              hb, 2<<%2
    jg %%loop_y
    add              wq, 32>>%2
    jge .end
    mov            srcq, r9mp
    mov            dstq, r10mp
    mov           lumaq, r11mp
    lea            srcq, [srcq+wq*2]
    lea            dstq, [dstq+wq*2]
    lea           lumaq, [lumaq+wq*(2<<%2)]

    ; since fg_dataq.overlap is guaranteed to be set, we never jump back
    ; to %%v_overlap, and instead always fall-through to %%hv_overlap
%%hv_overlap:
    ; we assume from the block above that bits 8-15 of r7d are zero'ed
    mov             r6d, seed
    or             seed, 0xeff4eff4
    test           seeb, seeh
    setp            r7b                     ; parity of top_seed
    shr            seed, 16
    shl             r7d, 16
    test           seeb, seeh
    setp            r7b                     ; parity of cur_seed
    or              r6d, 0x00010001
    xor             r7d, r6d
    rorx           seed, r7d, 1             ; updated (cur_seed << 16) | top_seed

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                offx, offy, see, lstride, luma, left_offxy, top_offxy, topleft_offxy

    lea  topleft_offxyq, [top_offxyq+(32>>%2)]
    lea     left_offxyq, [offyq+(32>>%2)]
    rorx          offyd, seed, 8
    rorx          offxd, seed, 12
    and           offyd, 0xf000f
    and           offxd, 0xf000f
    imul          offyd, 164>>%3
    ; offxy=offy*stride+offx, (cur_offxy << 16) | top_offxy
    lea           offyd, [offyq+offxq*(2-%2)+0x10001*((3+(6>>%3))*82+3+(6>>%2))+(32>>%3)*82]

    DEFINE_ARGS dst, src, stride, fg_data, w, scaling, grain_lut, \
                h, offxy, see, lstride, luma, left_offxy, top_offxy, topleft_offxy

    mov      grain_lutq, grain_lutmp
    mov              hd, hm
    movzx    top_offxyd, offxyw
    shr          offxyd, 16

    ; grain = grain_lut[offy+y][offx+x]
%if %2
    movd           xm16, [grain_lutq+left_offxyq*2+82*0]
    vinserti32x4    m16, [grain_lutq+left_offxyq*2+82*2], 2
    movd           xm17, [grain_lutq+left_offxyq*2+82*4]
    vinserti32x4    m17, [grain_lutq+left_offxyq*2+82*6], 2
    movu           ym18, [grain_lutq+offxyq*2+82*0]
    vinserti32x8    m18, [grain_lutq+offxyq*2+82*2], 1
    movu           ym19, [grain_lutq+offxyq*2+82*4]
    vinserti32x8    m19, [grain_lutq+offxyq*2+82*6], 1
    punpckldq       m16, m17
    punpckldq       m17, m18, m19
    punpcklwd       m16, m17
    movu            ym1, [grain_lutq+top_offxyq*2+82*0]
    movd           xm17, [grain_lutq+topleft_offxyq*2+82*0]
    mova             m0, m20
    vpdpwssd         m0, m16, m10
%if %3
    punpcklwd      xm17, xm1
    mova           xm16, xm20
    vpdpwssd       xm16, xm17, xm10
    psrad          xm16, 1
%else
    vinserti32x8     m1, [grain_lutq+top_offxyq*2+82*2], 1
    vinserti32x4    m17, [grain_lutq+topleft_offxyq*2+82*2], 2
    punpcklwd       m17, m1
    mova            m16, m20
    vpdpwssd        m16, m17, m10
    psrad           m16, 1
%endif
    psrad            m0, 1
    packssdw         m0, m16
    vpsravw          m0, m21
    vmovdqa32   m18{k3}, m0
    vpshufd     m19{k3}, m0, q0321
%if %3
    vpunpckhdq  ym1{k3}, ym0, ym0
    punpcklwd      ym17, ym1, ym18
    punpckhwd       ym1, ym18
%else
    vpunpckhdq   m1{k3}, m0, m0
    punpcklwd       m16, m1, m18
    punpckhwd       m17, m1, m18
%endif
%else
    movu            m18, [grain_lutq+offxyq*2+82*0]
    movu            m19, [grain_lutq+top_offxyq*2+82*0]
    movd           xm17, [grain_lutq+left_offxyq*2+82*0]
    pinsrd         xm17, [grain_lutq+topleft_offxyq*2+82*0], 1
    punpckldq      xm16, xm18, xm19
    punpcklwd      xm17, xm16
    movu             m2, [grain_lutq+offxyq*2+82*2]
    movu             m0, [grain_lutq+top_offxyq*2+82*2]
    movd           xm16, [grain_lutq+left_offxyq*2+82*2]
    pinsrd         xm16, [grain_lutq+topleft_offxyq*2+82*2], 1
    punpckldq       xm1, xm2, xm0
    punpcklwd       xm1, xm16, xm1
    mova           xm16, xm20
    vpdpwssd       xm16, xm17, xm10
    mova           xm17, xm20
    vpdpwssd       xm17, xm1, xm10
    punpckhwd        m1, m19, m18
    punpcklwd       m19, m18
    punpckhwd       m18, m2, m0
    punpcklwd        m2, m0
    psrad          xm16, 1
    psrad          xm17, 1
    packssdw       xm16, xm17
    vpsravw        xm16, xm21
    vpshuflw    m19{k4}, m16, q1302
    punpckhqdq     xm16, xm16
    vpshuflw     m2{k4}, m16, q3120
%endif
    call %%add_noise_v
    sub              hb, 2<<%2
    jg %%loop_y_h_overlap
    add              wq, 32>>%2
    jge .end
    mov            srcq, r9mp
    mov            dstq, r10mp
    mov           lumaq, r11mp
    lea            srcq, [srcq+wq*2]
    lea            dstq, [dstq+wq*2]
    lea           lumaq, [lumaq+wq*(2<<%2)]
    jmp %%hv_overlap

ALIGN function_align
%%add_noise_v:
%if %3
    mova           ym16, ym20
    vpdpwssd       ym16, ym17, ym11
    mova           ym17, ym20
    vpdpwssd       ym17, ym1, ym11
    psrad          ym16, 1
    psrad          ym17, 1
    packssdw       ym16, ym17
    vpsravw     m18{k1}, m16, m21
%elif %2
    mova            m18, m20
    vpdpwssd        m18, m16, m11
    mova            m16, m20
    vpdpwssd        m16, m17, m11
    psrad           m18, 1
    psrad           m16, 1
    packssdw        m18, m16
    vpsravw         m18, m21
%else
    mova            m16, m20
    vpdpwssd        m16, m1, m11
    mova            m17, m20
    vpdpwssd        m17, m18, m11
    mova            m18, m20
    vpdpwssd        m18, m19, m11
    mova            m19, m20
    vpdpwssd        m19, m2, m11
    REPX   {psrad x, 1}, m16, m17, m18, m19
    packssdw        m18, m16
    packssdw        m19, m17
    vpsravw         m18, m21
    vpsravw         m19, m21
%endif
%%add_noise:
%if %2
    mova             m2, [lumaq+lstrideq*(0<<%3)]
    mova             m0, [lumaq+lstrideq*(1<<%3)]
    lea           lumaq, [lumaq+lstrideq*(2<<%3)]
    mova             m3, [lumaq+lstrideq*(0<<%3)]
    mova             m1, [lumaq+lstrideq*(1<<%3)]
    mova            m16, m12
    vpermi2w        m16, m2, m0
    vpermt2w         m2, m13, m0
    mova            m17, m12
    vpermi2w        m17, m3, m1
    vpermt2w         m3, m13, m1
    pavgw            m2, m16
    pavgw            m3, m17
%elif %1
    mova             m2, [lumaq+lstrideq*0]
    mova             m3, [lumaq+lstrideq*1]
%endif
%if %2
    mova           ym16, [srcq+strideq*0]
    vinserti32x8    m16, [srcq+strideq*1], 1
    lea            srcq, [srcq+strideq*2]
%else
    mova            m16, [srcq+strideq*0]
%endif
%if %1
    punpckhwd       m17, m2, m16
    mova             m0, m14
    vpdpwssd         m0, m17, m15
    punpcklwd       m17, m2, m16
    mova             m2, m14
    vpdpwssd         m2, m17, m15
%endif
%if %2
    mova           ym17, [srcq+strideq*0]
    vinserti32x8    m17, [srcq+strideq*1], 1
%else
    mova            m17, [srcq+strideq*1]
%endif
%if %1
    psrad            m0, 6
    psrad            m2, 6
    packusdw         m2, m0
    punpckhwd        m0, m3, m17
    mova             m1, m14
    vpdpwssd         m1, m15, m0
    punpcklwd        m0, m3, m17
    mova             m3, m14
    vpdpwssd         m3, m15, m0
    psrad            m1, 6
    psrad            m3, 6
    packusdw         m3, m1
    pminuw           m2, m4
    pminuw           m3, m4

.add_noise_main:
    ; scaling[luma_src]
    kmovw            k5, k1
    pand             m1, m5, m2
    vpgatherdd   m0{k5}, [scalingq+m1]
    kmovw            k5, k1
    psrld            m2, 16
    vpgatherdd   m1{k5}, [scalingq+m2]
    vpshufb      m0{k2}, m1, m6
    kmovw            k5, k1
    psrld            m1, m3, 16
    vpgatherdd   m2{k5}, [scalingq+m1]
    kmovw            k5, k1
    pand             m3, m5
    vpgatherdd   m1{k5}, [scalingq+m3]
    vpshufb      m1{k2}, m2, m6

    ; noise = round2(scaling[luma_src] * grain, scaling_shift)
    vpsllvw          m0, m7
    vpsllvw          m1, m7
    pmulhrsw        m18, m0
    pmulhrsw        m19, m1
    add      grain_lutq, 82*(4<<%2)
    lea           lumaq, [lumaq+lstrideq*(2<<%3)]
    lea            srcq, [srcq+strideq*2]
    paddw           m16, m18
    paddw           m17, m19
    pmaxsw          m16, m8
    pmaxsw          m17, m8
    pminsw          m16, m9
    pminsw          m17, m9
%if %2
    mova          [dstq+strideq*0], ym16
    vextracti32x8 [dstq+strideq*1], m16, 1
    lea            dstq, [dstq+strideq*2]
    mova          [dstq+strideq*0], ym17
    vextracti32x8 [dstq+strideq*1], m17, 1
%else
    mova [dstq+strideq*0], m16
    mova [dstq+strideq*1], m17
%endif
    lea            dstq, [dstq+strideq*2]
    ret
%else
%if %2
    pand             m2, m4
    pand             m3, m4
%else
    pand             m2, m4, [lumaq+lstrideq*0]
    pand             m3, m4, [lumaq+lstrideq*1]
%endif
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

%endif
