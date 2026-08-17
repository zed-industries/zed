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

%if ARCH_X86_64

SECTION_RODATA 64

cdef_perm:     db  2, 18, 16, 18, 24, 19,  0, 19, 25, 20,  1, 20, 26, 21,  2, 21
               db  3, 26,  3, 26, 28, 27,  4, 27, 29, 28, -1, 28, 30, 29, -1, 29
               db  0, 34, 17, 34, 16, 35,  8, 35, 17, 36,  9, 36, 18, 37, 10, 37
               db  1, 42, 11, 42, 20, 43, 12, 43, 21, 44, -1, 44, 22, 45, -1, 45
end_perm4:     db  1,  2,  5,  6,  9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30
               db 33, 34, 37, 38, 41, 42, 45, 46, 49, 50, 53, 54, 57, 58, 61, 62
edge_mask4:    dw 0xff99, 0xff88, 0xff11, 0xff00 ; 0100, 0101, 0110, 0111
               dw 0x99ff, 0x88ff, 0x11ff, 0x00ff ; 1000, 1001, 1010, 1011
               dw 0x9999, 0x8888, 0x1111, 0x0000 ; 1100, 1101, 1110, 1111
pri_taps4:     dw 64, 32, 48, 48                 ; left-shifted by 4
cdef_dirs4:    dw  8, 16,  8, 15, -7,-14,  1, -6
               dw  1,  2,  1, 10,  9, 18,  8, 17
               dw  8, 16,  8, 15, -7,-14,  1, -6
deint_shuf:    db  0,  1,  4,  5,  8,  9, 12, 13,  2,  3,  6,  7, 10, 11, 14, 15
cdef_dirs8:    db 32, 64, 32, 62,-30,-60,  2,-28
               db  2,  4,  2, 36, 34, 68, 32, 66
               db 32, 64, 32, 62,-30,-60,  2,-28
pri_taps8:     dw  4,  4,  2,  2,  3,  3,  3,  3
sec_taps4:     dw 32, 16
pw_m16384:     times 2 dw -16384
pw_2048:       times 2 dw 2048
pd_268435568:  dd 268435568                      ; (1 << 28) + (7 << 4)
edge_mask8:    dw 0x2121, 0x2020, 0x0101

SECTION .text

%macro CONSTRAIN 7 ; dst, p, px, zero, tresh, shift, tmp
    psubw           %1, %2, %3
    pabsw           %1, %1
    vpcmpgtw        k1, %3, %2
    vpsrlvw         %7, %1, %6
    psubusw         %7, %5, %7
    pminsw          %1, %7
    vpsubw      %1{k1}, %4, %1
%endmacro

; t0 t1 t2 t3 t4 t5 t6 t7   L4 L5 20 21 22 23 24 25
; T0 T1 T2 T3 T4 T5 T6 T7   L6 L7 30 31 32 33 34 35
; L0 L1 00 01 02 03 04 05   b0 b1 b2 b3 b4 b5 b6 b7
; L2 L3 10 11 12 13 14 15   B0 B1 B2 B3 B4 B5 B6 B7

INIT_ZMM avx512icl
cglobal cdef_filter_4x4_16bpc, 5, 7, 16, dst, stride, left, top, bot, \
                                         pri, sec, dir, damping, edge
%define base r6-cdef_dirs4
    lea             r6, [cdef_dirs4]
    movu           xm3, [dstq+strideq*0]
    vinserti32x4   ym3, [dstq+strideq*1], 1
    mova           xm2, [leftq]
    lea             r2, [dstq+strideq*2]
    vinserti32x4    m3, [r2+strideq*0], 2
    mova            m5, [base+cdef_perm]
    vinserti32x4    m3, [r2+strideq*1], 3
    vpermt2d        m2, m5, m3
    vinserti32x4    m1, m2, [topq+strideq*0-4], 0
    vinserti32x4    m1, [topq+strideq*1-4], 1
    mov            r3d, edgem
    movifnidn     prid, prim
    punpcklwd       m3, m3     ; px
    psrlw           m5, 8
    vpbroadcastd    m0, [base+pd_268435568]
    pxor           m12, m12
    cmp            r3d, 0x0f
    jne .mask_edges
    vinserti32x4    m2, [botq+strideq*0-4], 2
    vinserti32x4    m2, [botq+strideq*1-4], 3
.main:
    test          prid, prid
    jz .sec_only
    lzcnt          r4d, prid
    rorx           r3d, prid, 2
    vpbroadcastw   m13, prim
    cmp     dword r10m, 0xfff  ; if (bpc == 12)
    cmove         prid, r3d    ;     pri >>= 2
    mov            r3d, dampingm
    and           prid, 4
    sub            r3d, 31
    vpbroadcastd   m15, [base+pri_taps4+priq]
    xor           prid, prid
    add            r4d, r3d
    cmovns        prid, r4d    ; pri_shift
    mov            r4d, dirm
    vpbroadcastw   m14, prid
    mov            r5d, secm
    vpbroadcastd    m9, [base+cdef_dirs4+(r4+2)*4]
    call .constrain
    test           r5d, r5d
    jz .end_no_clip
    lzcnt          r5d, r5d
    vpbroadcastw   m13, secm
    add            r3d, r5d
    pminuw          m6, m3, m8
    pmaxsw          m7, m3, m8
    pminuw          m6, m9
    pmaxsw          m7, m9
    call .constrain_sec
    pminuw          m6, m8
    pmaxsw          m7, m8
    pminuw          m6, m9
    pmaxsw          m7, m9
    vpbroadcastd    m9, [base+cdef_dirs4+(r4+0)*4]
    call .constrain
    pminuw          m6, m8
    pmaxsw          m7, m8
    pminuw          m6, m9
    pmaxsw          m7, m9
    psrldq          m8, m6, 2
    vpshldd         m3, m0, 8
    psrldq          m9, m7, 2
    paddd           m0, m3
    pminuw          m6, m8
    psrldq          m0, 1
    pmaxsw          m7, m9
    pmaxsw          m0, m6
    pminsw          m0, m7
    vpmovdw        ym0, m0
    jmp .end
.sec_only:
    tzcnt          r5d, secm
    mov            r3d, dampingm
    vpbroadcastw   m13, secm
    mov            r4d, dirm
    sub            r3d, r5d    ; sec_shift
    call .constrain_sec
    vpbroadcastd    m9, [base+cdef_dirs4+(r4+0)*4]
    call .constrain
.end_no_clip:
    mova           ym1, [base+end_perm4]
    vpshldd         m3, m0, 8  ; (px << 8) + ((sum > -8) << 4)
    paddd           m0, m3     ; (px << 8) + ((sum + (sum > -8) + 7) << 4)
    vpermb          m0, m1, m0
.end:
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    vextracti32x4  xm0, ym0, 1
    movq   [r2+strideq*0], xm0
    movhps [r2+strideq*1], xm0
    RET
.mask_edges:
    vpbroadcastd    m6, [base+pw_m16384]
    test           r3b, 0x08
    jz .mask_edges_no_bottom  ; avoid buffer overread
    vinserti32x4    m2, [botq+strideq*0-4], 2
    vinserti32x4    m2, [botq+strideq*1-4], 3
    kmovw           k1, [base+edge_mask4-8+r3*2]
    jmp .mask_edges_main
.mask_edges_no_bottom:
    kmovw           k1, [base+edge_mask4+8+r3*2]
.mask_edges_main:
    or             r3d, 0x04
    vmovdqa32   m1{k1}, m6     ; edge pixels = -16384
    kmovw           k1, [base+edge_mask4-8+r3*2]
    vmovdqa32   m2{k1}, m6
    jmp .main
.constrain_sec:
    vpbroadcastd    m9, [base+cdef_dirs4+(r4+4)*4]
    vpbroadcastw   m14, r3d
    vpbroadcastd   m15, [base+sec_taps4]
.constrain:
    paddw           m8, m5, m9
    vpermi2w        m8, m1, m2 ; k0p0 k1p0
    psubw           m9, m5, m9
    vpermi2w        m9, m1, m2 ; k0p1 k1p1
    CONSTRAIN      m10, m8, m3, m12, m13, m14, m11
    vpdpwssd        m0, m10, m15
    CONSTRAIN      m10, m9, m3, m12, m13, m14, m11
    vpdpwssd        m0, m10, m15
    ret

; t0 t1 t2 t3 t4 t5 t6 t7   L4 L5 20 21 22 23 24 25   Lc Ld 60 61 62 63 64 65
; T0 T1 T2 T3 T4 T5 T6 T7   L6 L7 30 31 32 33 34 35   Le Lf 70 71 72 73 74 75
; L0 L1 00 01 02 03 04 05   L8 L9 40 41 42 43 44 45   b0 b1 b2 b3 b4 b5 b6 b7
; L2 L3 10 11 12 13 14 15   La Lb 50 51 52 53 54 55   B0 B1 B2 B3 B4 B5 B6 B7

cglobal cdef_filter_4x8_16bpc, 5, 7, 22, dst, stride, left, top, bot, \
                                         pri, sec, dir, damping, edge
    lea             r6, [cdef_dirs4]
    movu          xm18, [dstq+strideq*0]
    vinserti128   ym18, [dstq+strideq*1], 1
    mova           xm1, [leftq+16*0]
    mova           xm2, [leftq+16*1]
    lea             r2, [strideq*3]
    vinserti32x4   m18, [dstq+strideq*2], 2
    mova            m5, [base+cdef_perm]
    vinserti32x4   m18, [dstq+r2       ], 3
    vpermt2d        m1, m5, m18
    vinserti32x4    m0, m1, [topq+strideq*0-4], 0
    vinserti32x4    m0, [topq+strideq*1-4], 1
    lea             r3, [dstq+strideq*4]
    movu          xm19, [r3+strideq*0]
    vinserti128   ym19, [r3+strideq*1], 1
    vinserti32x4   m19, [r3+strideq*2], 2
    vinserti32x4   m19, [r3+r2       ], 3
    mov            r3d, edgem
    movifnidn     prid, prim
    vpermt2d        m2, m5, m19
    vpbroadcastd   m16, [base+pd_268435568]
    pxor           m12, m12
    punpcklwd      m18, m18    ; px (top)
    psrlw           m5, 8
    punpcklwd      m19, m19    ; px (bottom)
    mova           m17, m16
    vshufi32x4      m1, m2, q3210
    cmp            r3d, 0x0f
    jne .mask_edges
    vinserti32x4    m2, [botq+strideq*0-4], 2
    vinserti32x4    m2, [botq+strideq*1-4], 3
.main:
    test          prid, prid
    jz .sec_only
    lzcnt          r4d, prid
    rorx           r3d, prid, 2
    vpbroadcastw   m13, prim
    cmp     dword r10m, 0xfff  ; if (bpc == 12)
    cmove         prid, r3d    ;     pri >>= 2
    mov            r3d, dampingm
    and           prid, 4
    sub            r3d, 31
    vpbroadcastd   m15, [base+pri_taps4+priq]
    xor           prid, prid
    add            r4d, r3d
    cmovns        prid, r4d    ; pri_shift
    mov            r4d, dirm
    vpbroadcastw   m14, prid
    mov            r5d, secm
    vpbroadcastd    m9, [base+cdef_dirs4+(r4+2)*4]
    call .constrain
    test           r5d, r5d
    jz .end_no_clip
    lzcnt          r5d, r5d
    vpbroadcastw   m13, secm
    add            r3d, r5d
    pminuw          m3, m18, m6
    pmaxsw          m4, m18, m6
    pminuw         m20, m19, m7
    pmaxsw         m21, m19, m7
    pminuw          m3, m8
    pmaxsw          m4, m8
    pminuw         m20, m9
    pmaxsw         m21, m9
    call .constrain_sec
    pminuw          m3, m6
    pmaxsw          m4, m6
    pminuw         m20, m7
    pmaxsw         m21, m7
    pminuw          m3, m8
    pmaxsw          m4, m8
    pminuw         m20, m9
    pmaxsw         m21, m9
    vpbroadcastd    m9, [base+cdef_dirs4+(r4+0)*4]
    call .constrain
    pminuw          m3, m6
    pmaxsw          m4, m6
    mov             r3, 0xcccccccccccccccc
    pminuw         m20, m7
    pmaxsw         m21, m7
    kmovq           k1, r3
    pminuw          m3, m8
    pmaxsw          m4, m8
    pminuw         m20, m9
    pmaxsw         m21, m9
    vbroadcasti32x4 m0, [base+deint_shuf]
    vpshldd         m6, m20, m3, 16
    vmovdqu8    m3{k1}, m20
    vpshldd        m18, m16, 8
    vpshldd         m7, m21, m4, 16
    vmovdqu8    m4{k1}, m21
    vpshldd        m19, m17, 8
    pminuw          m3, m6
    paddd          m16, m18
    pmaxsw          m4, m7
    paddd          m17, m19
    psrldq         m16, 1
    palignr    m16{k1}, m17, m17, 15
    lea             r6, [dstq+strideq*4]
    pmaxsw         m16, m3
    pminsw         m16, m4
    pshufb         m16, m0
    movq   [dstq+strideq*0], xm16
    movhps [r6  +strideq*0], xm16
    vextracti128  xm17, ym16, 1
    movq   [dstq+strideq*1], xm17
    movhps [r6  +strideq*1], xm17
    vextracti32x4  xm17, m16, 2
    movq   [dstq+strideq*2], xm17
    movhps [r6  +strideq*2], xm17
    vextracti32x4  xm16, m16, 3
    movq   [dstq+r2       ], xm16
    movhps [r6  +r2       ], xm16
    RET
.sec_only:
    mov            r4d, dirm
    tzcnt          r5d, secm
    mov            r3d, dampingm
    vpbroadcastw   m13, secm
    sub            r3d, r5d    ; sec_shift
    call .constrain_sec
    vpbroadcastd    m9, [base+cdef_dirs4+(r4+0)*4]
    call .constrain
.end_no_clip:
    mova          ym20, [base+end_perm4]
    vpshldd        m18, m16, 8 ; (px << 8) + ((sum > -8) << 4)
    vpshldd        m19, m17, 8
    paddd          m16, m18    ; (px << 8) + ((sum + (sum > -8) + 7) << 4)
    paddd          m17, m19
    vpermb         m16, m20, m16
    vpermb         m17, m20, m17
    movq   [dstq+strideq*0], xm16
    movhps [dstq+strideq*1], xm16
    vextracti128  xm16, ym16, 1
    movq   [dstq+strideq*2], xm16
    movhps [dstq+r2       ], xm16
    lea           dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm17
    movhps [dstq+strideq*1], xm17
    vextracti128  xm17, ym17, 1
    movq   [dstq+strideq*2], xm17
    movhps [dstq+r2       ], xm17
    RET
.mask_edges:
    vpbroadcastd    m6, [base+pw_m16384]
    test           r3b, 0x08
    jz .mask_edges_no_bottom   ; avoid buffer overread
    vinserti32x4    m2, [botq+strideq*0-4], 2
    vinserti32x4    m2, [botq+strideq*1-4], 3
    kmovw           k1, [base+edge_mask4-8+r3*2]
    jmp .mask_edges_main
.mask_edges_no_bottom:
    kmovw           k1, [base+edge_mask4+8+r3*2]
.mask_edges_main:
    mov            r4d, r3d
    or             r3d, 0x0c
    vmovdqa32   m0{k1}, m6     ; edge pixels = -16384
    kmovw           k1, [base+edge_mask4-8+r3*2]
    or             r4d, 0x04
    vmovdqa32   m1{k1}, m6
    kmovw           k1, [base+edge_mask4-8+r4*2]
    vmovdqa32   m2{k1}, m6
    jmp .main
.constrain_sec:
    vpbroadcastd    m9, [base+cdef_dirs4+(r4+4)*4]
    vpbroadcastw   m14, r3d
    vpbroadcastd   m15, [base+sec_taps4]
.constrain:
    paddw           m7, m5, m9
    mova            m6, m0
    vpermt2w        m6, m7, m1 ; k0p0 k1p0 (top)
    psubw           m9, m5, m9
    mova            m8, m0
    vpermi2w        m7, m1, m2 ; k0p0 k1p0 (bottom)
    CONSTRAIN      m10, m6, m18, m12, m13, m14, m11
    vpermt2w        m8, m9, m1 ; k0p1 k1p1 (top)
    vpdpwssd       m16, m10, m15
    CONSTRAIN      m10, m7, m19, m12, m13, m14, m11
    vpermi2w        m9, m1, m2 ; k0p1 k1p1 (bottom)
    vpdpwssd       m17, m10, m15
    CONSTRAIN      m10, m8, m18, m12, m13, m14, m11
    vpdpwssd       m16, m10, m15
    CONSTRAIN      m10, m9, m19, m12, m13, m14, m11
    vpdpwssd       m17, m10, m15
    ret

cglobal cdef_filter_8x8_16bpc, 5, 7, 22, 64*6, dst, stride, left, top, bot, \
                                               pri, sec, dir, damping, edge
%define base r6-cdef_dirs8
    lea             r6, [cdef_dirs8]
    movu          ym17, [dstq+strideq*0]
    vinserti32x8   m17, [dstq+strideq*1], 1
    movq           xm4, [leftq+8*0]
    movq           xm5, [leftq+8*1]
    psrld           m2, [base+cdef_perm], 16
    movq           xm6, [leftq+8*2]
    movq           xm7, [leftq+8*3]
    lea             r2, [strideq*3]
    movu          ym16, [topq+strideq*0-4]
    vinserti32x8   m16, [topq+strideq*1-4], 1
    lea             r3, [dstq+strideq*4]
    movu          ym18, [dstq+strideq*2]
    vinserti32x8   m18, [dstq+r2       ], 1
    movu          ym19, [r3+strideq*0]
    vinserti32x8   m19, [r3+strideq*1], 1
    movu          ym20, [r3+strideq*2]
    vinserti32x8   m20, [r3+r2       ], 1
    vshufi32x4      m0, m17, m18, q2020 ; px (top)
    mov            r3d, edgem
    vshufi32x4      m1, m19, m20, q2020 ; px (bottom)
    movifnidn     prid, prim
    vpermt2d       m17, m2, m4
    vpermt2d       m18, m2, m5
    pxor           m12, m12
    vpermt2d       m19, m2, m6
    vpermt2d       m20, m2, m7
    cmp            r3d, 0x0f
    jne .mask_edges
    movu          ym21, [botq+strideq*0-4]
    vinserti32x8   m21, [botq+strideq*1-4], 1
.main:
    mova    [rsp+64*0], m16    ; top
    mova    [rsp+64*1], m17    ; 0 1
    mova    [rsp+64*2], m18    ; 2 3
    mova    [rsp+64*3], m19    ; 4 5
    mova    [rsp+64*4], m20    ; 6 7
    mova    [rsp+64*5], m21    ; bottom
    test          prid, prid
    jz .sec_only
    lzcnt          r4d, prid
    rorx           r3d, prid, 2
    vpbroadcastw   m13, prim
    cmp     dword r10m, 0xfff  ; if (bpc == 12)
    cmove         prid, r3d    ;     pri >>= 2
    mov            r3d, dampingm
    and           prid, 4
    sub            r3d, 31
    add            r4d, r3d    ; pri_shift
    vpbroadcastw   m14, r4d
    mov            r4d, dirm
    vpbroadcastd    m2, [base+pri_taps8+priq*2+0]
    vpbroadcastd    m3, [base+pri_taps8+priq*2+4]
    movsx           r5, byte [base+cdef_dirs8+(r4+2)*2+0] ; k0off1
    pmaxsw         m14, m12
    call .constrain
    mov            r5d, secm
    pmullw         m16, m8, m2
    pmullw         m17, m9, m2
    test           r5d, r5d
    jnz .pri_sec
    movsx           r5, byte [base+cdef_dirs8+(r4+2)*2+1] ; k1off1
    call .constrain
    pmullw          m8, m3
    pmullw          m9, m3
    jmp .end_no_clip
.pri_sec:
    lzcnt          r5d, r5d
    add            r3d, r5d    ; sec_shift
    movsx           r5, byte [base+cdef_dirs8+(r4+2)*2+1] ; k1off1
    pminuw         m18, m0, m4
    pmaxsw         m19, m0, m4
    pminuw         m20, m1, m5
    pmaxsw         m21, m1, m5
    call .min_max_constrain2
    movsx           r5, byte [base+cdef_dirs8+(r4+0)*2+0] ; k0off2
    pmullw          m8, m3
    pmullw          m9, m3
    vpbroadcastw   m13, secm
    vpbroadcastw   m14, r3d
    paddw          m16, m8
    paddw          m17, m9
    call .min_max_constrain
    movsx           r5, byte [base+cdef_dirs8+(r4+4)*2+0] ; k0off3
    mova            m2, m8
    mova            m3, m9
    call .min_max_constrain
    movsx           r5, byte [base+cdef_dirs8+(r4+0)*2+1] ; k1off2
    paddw           m2, m8
    paddw           m3, m9
    call .min_max_constrain
    movsx           r5, byte [base+cdef_dirs8+(r4+4)*2+1] ; k1off3
    paddw           m2, m2
    paddw           m3, m3
    paddw          m16, m8
    paddw          m17, m9
    call .min_max_constrain
    vpbroadcastd   m10, [base+pw_2048]
    paddw          m16, m2
    paddw          m17, m3
    paddw          m16, m8
    paddw          m17, m9
    psraw           m8, m16, 15
    psraw           m9, m17, 15
    paddw          m16, m8
    paddw          m17, m9
    pmulhrsw       m16, m10
    pmulhrsw       m17, m10
    pminuw         m18, m4
    pmaxsw         m19, m4
    pminuw         m20, m5
    pmaxsw         m21, m5
    pminuw         m18, m6
    pmaxsw         m19, m6
    pminuw         m20, m7
    pmaxsw         m21, m7
    paddw          m16, m0
    paddw          m17, m1
    pmaxsw         m16, m18
    pmaxsw         m17, m20
    pminsw         m16, m19
    pminsw         m17, m21
    jmp .end
.sec_only:
    tzcnt          r5d, secm
    mov            r4d, dirm
    mov            r3d, dampingm
    vpbroadcastw   m13, secm
    sub            r3d, r5d
    movsx           r5, byte [base+cdef_dirs8+(r4+0)*2+0]
    vpbroadcastw   m14, r3d
    call .constrain
    movsx           r5, byte [base+cdef_dirs8+(r4+4)*2+0]
    mova           m16, m8
    mova           m17, m9
    call .constrain
    movsx           r5, byte [base+cdef_dirs8+(r4+0)*2+1]
    paddw          m16, m8
    paddw          m17, m9
    call .constrain
    movsx           r5, byte [base+cdef_dirs8+(r4+4)*2+1]
    paddw          m16, m16
    paddw          m17, m17
    paddw          m16, m8
    paddw          m17, m9
    call .constrain
.end_no_clip:
    vpbroadcastd   m10, [base+pw_2048]
    paddw          m16, m8
    paddw          m17, m9
    psraw           m8, m16, 15
    psraw           m9, m17, 15
    paddw          m16, m8
    paddw          m17, m9
    pmulhrsw       m16, m10
    pmulhrsw       m17, m10
    paddw          m16, m0
    paddw          m17, m1
.end:
    mova          [dstq+strideq*0], xm16
    vextracti128  [dstq+strideq*1], ym16, 1
    vextracti32x4 [dstq+strideq*2], m16, 2
    vextracti32x4 [dstq+r2       ], m16, 3
    lea           dstq, [dstq+strideq*4]
    mova          [dstq+strideq*0], xm17
    vextracti128  [dstq+strideq*1], ym17, 1
    vextracti32x4 [dstq+strideq*2], m17, 2
    vextracti32x4 [dstq+r2       ], m17, 3
    RET
.mask_edges:
    vpbroadcastd    m2, [base+pw_m16384]
    test           r3b, 0x08
    jz .mask_edges_no_bottom  ; avoid buffer overread
    movu          ym21, [botq+strideq*0-4]
    vinserti32x8   m21, [botq+strideq*1-4], 1
    jmp .mask_edges_top
.mask_edges_no_bottom:
    mova           m21, m2
.mask_edges_top:
    test           r3b, 0x04
    jnz .mask_edges_main
    mova           m16, m2
.mask_edges_main:
    and            r3d, 0x03
    cmp            r3d, 0x03
    je .main
    kmovw           k1, [base+edge_mask8+r3*2]
    vmovdqa32  m16{k1}, m2     ; edge pixels = -16384
    vmovdqa32  m17{k1}, m2
    vmovdqa32  m18{k1}, m2
    vmovdqa32  m19{k1}, m2
    vmovdqa32  m20{k1}, m2
    vmovdqa32  m21{k1}, m2
    jmp .main
ALIGN function_align
.min_max_constrain:
    pminuw         m18, m4
    pmaxsw         m19, m4
    pminuw         m20, m5
    pmaxsw         m21, m5
.min_max_constrain2:
    pminuw         m18, m6
    pmaxsw         m19, m6
    pminuw         m20, m7
    pmaxsw         m21, m7
.constrain:
    %define        tmp  rsp+gprsize+68
    movu            m4, [tmp+r5+64*0]
    vshufi32x4      m4, [tmp+r5+64*1], q2020 ; k0p0 (top)
    movu            m5, [tmp+r5+64*2]
    vshufi32x4      m5, [tmp+r5+64*3], q2020 ; k0p0 (bottom)
    neg             r5
    movu            m6, [tmp+r5+64*0]
    vshufi32x4      m6, [tmp+r5+64*1], q2020 ; k0p1 (top)
    movu            m7, [tmp+r5+64*2]
    vshufi32x4      m7, [tmp+r5+64*3], q2020 ; k0p1 (bottom)
    CONSTRAIN       m8, m4, m0, m12, m13, m14, m15
    CONSTRAIN       m9, m5, m1, m12, m13, m14, m15
    CONSTRAIN      m10, m6, m0, m12, m13, m14, m15
    CONSTRAIN      m11, m7, m1, m12, m13, m14, m15
    paddw           m8, m10
    paddw           m9, m11
    ret

%endif ; ARCH_X86_64
