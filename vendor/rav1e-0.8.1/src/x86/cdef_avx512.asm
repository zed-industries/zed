; Copyright © 2020, VideoLAN and dav1d authors
; Copyright © 2020, Two Orioles, LLC
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

%macro DUP4 1-*
    %rep %0
        times 4 db %1
        %rotate 1
    %endrep
%endmacro

%macro DIRS 16 ; cdef_directions[]
    %rep 4 + 16 + 4 ; 6 7   0 1 2 3 4 5 6 7   0 1
        ; masking away unused bits allows us to use a single vpaddd {1to16}
        ; instruction instead of having to do vpbroadcastd + paddb
        db %13 & 0x3f, -%13 & 0x3f
        %rotate 1
    %endrep
%endmacro

SECTION_RODATA 64

lut_perm_4x4:  db 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79
               db 16, 17,  0,  1,  2,  3,  4,  5, 18, 19,  8,  9, 10, 11, 12, 13
               db 20, 21, 80, 81, 82, 83, 84, 85, 22, 23, 32, 33, 34, 35, 36, 37
               db 98, 99,100,101,102,103,104,105, 50, 51, 52, 53, 54, 55, 56, 57
lut_perm_4x8a: db 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79
               db 96, 97,  0,  1,  2,  3,  4,  5, 98, 99,  8,  9, 10, 11, 12, 13
lut_perm_4x8b:db 100,101, 16, 17, 18, 19, 20, 21,102,103, 24, 25, 26, 27, 28, 29
              db 104,105, 32, 33, 34, 35, 36, 37,106,107, 40, 41, 42, 43, 44, 45
              db 108,109, 48, 49, 50, 51, 52, 53,110,111, 56, 57, 58, 59, 60, 61
               db 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95
pd_01234567:   dd  0,  1,  2,  3,  4,  5,  6,  7
lut_perm_8x8a: db 32, 33, 34, 35, 36, 37, 38, 39, 48, 49, 50, 51, 52, 53, 54, 55
               db 36, 37, 38, 39, 40, 41, 42, 43, 52, 53, 54, 55, 56, 57, 58, 59
lut_perm_8x8b: db 12, 13,  0,  1,  2,  3,  4,  5, 14, 15, 16, 17, 18, 19, 20, 21
               db  2,  3,  4,  5,  6,  7,  8,  9, 18, 19, 20, 21, 22, 23, 24, 25
               db 28, 29, 32, 33, 34, 35, 36, 37, 30, 31, 48, 49, 50, 51, 52, 53
               db 34, 35, 36, 37, 38, 39, 40, 41, 50, 51, 52, 53, 54, 55, 56, 57
end_perm:      db  1,  5,  9, 13, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61
               db  3,  7, 11, 15, 19, 23, 27, 31, 35, 39, 43, 47, 51, 55, 59, 63
end_perm_clip: db  0,  4,  8, 12,  2,  6, 10, 14, 16, 20, 24, 28, 18, 22, 26, 30
               db 32, 36, 40, 44, 34, 38, 42, 46, 48, 52, 56, 60, 50, 54, 58, 62
               db  1,  5,  9, 13,  3,  7, 11, 15, 17, 21, 25, 29, 19, 23, 27, 31
               db 33, 37, 41, 45, 35, 39, 43, 47, 49, 53, 57, 61, 51, 55, 59, 63
edge_mask:     dq 0x00003c3c3c3c0000, 0x00003f3f3f3f0000 ; 0000, 0001
               dq 0x0000fcfcfcfc0000, 0x0000ffffffff0000 ; 0010, 0011
               dq 0x00003c3c3c3c3c3c, 0x00003f3f3f3f3f3f ; 0100, 0101
               dq 0x0000fcfcfcfcfcfc, 0x0000ffffffffffff ; 0110, 0111
               dq 0x3c3c3c3c3c3c0000, 0x3f3f3f3f3f3f0000 ; 1000, 1001
               dq 0xfcfcfcfcfcfc0000, 0xffffffffffff0000 ; 1010, 1011
               dq 0x3c3c3c3c3c3c3c3c, 0x3f3f3f3f3f3f3f3f ; 1100, 1101
               dq 0xfcfcfcfcfcfcfcfc, 0xffffffffffffffff ; 1110, 1111
px_idx:      DUP4 18, 19, 20, 21, 26, 27, 28, 29, 34, 35, 36, 37, 42, 43, 44, 45
cdef_dirs:   DIRS -7,-14,  1, -6,  1,  2,  1, 10,  9, 18,  8, 17,  8, 16,  8, 15
gf_shr:        dq 0x0102040810204080, 0x0102040810204080 ; >> 0, >> 0
               dq 0x0204081020408000, 0x0408102040800000 ; >> 1, >> 2
               dq 0x0810204080000000, 0x1020408000000000 ; >> 3, >> 4
               dq 0x2040800000000000, 0x4080000000000000 ; >> 5, >> 6
pri_tap:       db 64, 64, 32, 32, 48, 48, 48, 48         ; left-shifted by 4
sec_tap:       db 32, 32, 16, 16
pd_268435568:  dd 268435568

SECTION .text

%if WIN64
DECLARE_REG_TMP 4
%else
DECLARE_REG_TMP 8
%endif

; lut:
; t0 t1 t2 t3 t4 t5 t6 t7
; T0 T1 T2 T3 T4 T5 T6 T7
; L0 L1 00 01 02 03 04 05
; L2 L3 10 11 12 13 14 15
; L4 L5 20 21 22 23 24 25
; L6 L7 30 31 32 33 34 35
; b0 b1 b2 b3 b4 b5 b6 b7
; B0 B1 B2 B3 B4 B5 B6 B7

INIT_ZMM avx512icl
cglobal cdef_filter_4x4_8bpc, 5, 8, 13, dst, stride, left, top, bot, \
                                        pri, sec, dir, damping, edge
%define base r7-edge_mask
    movq         xmm0, [dstq+strideq*0]
    movhps       xmm0, [dstq+strideq*1]
    lea            r7, [edge_mask]
    movq         xmm1, [topq+strideq*0-2]
    movhps       xmm1, [topq+strideq*1-2]
    mov           r6d, edgem
    vinserti32x4  ym0, ymm0, [leftq], 1
    lea            r2, [strideq*3]
    vinserti32x4  ym1, ymm1, [dstq+strideq*2], 1
    mova           m5, [base+lut_perm_4x4]
    vinserti32x4   m0, [dstq+r2], 2
    test          r6b, 0x08      ; avoid buffer overread
    jz .main
    vinserti32x4   m1, [botq+strideq*0-4], 2
    vinserti32x4   m0, [botq+strideq*1-4], 3
.main:
    movifnidn    prid, prim
    mov           t0d, dirm
    mova           m3, [base+px_idx]
    mov           r3d, dampingm
    vpermi2b       m5, m0, m1    ; lut
    vpbroadcastd   m0, [base+pd_268435568] ; (1 << 28) + (7 << 4)
    pxor           m7, m7
    lea            r3, [r7+r3*8] ; gf_shr + (damping - 30) * 8
    vpermb         m6, m3, m5    ; px
    cmp           r6d, 0x0f
    jne .mask_edges              ; mask edges only if required
    test         prid, prid
    jz .sec_only
    vpaddd         m1, m3, [base+cdef_dirs+(t0+2)*4] {1to16} ; dir
    vpermb         m1, m1, m5    ; k0p0 k0p1 k1p0 k1p1
%macro CDEF_FILTER_4x4_PRI 0
    vpcmpub        k1, m6, m1, 6 ; px > pN
    psubb          m2, m1, m6
    lzcnt         r6d, prid
    vpsubb     m2{k1}, m6, m1    ; abs(diff)
    vpbroadcastb   m4, prid
    and          prid, 1
    vgf2p8affineqb m9, m2, [r3+r6*8] {1to8}, 0 ; abs(diff) >> shift
    movifnidn    secd, secm
    vpbroadcastd  m10, [base+pri_tap+priq*4]
    vpsubb    m10{k1}, m7, m10   ; apply_sign(pri_tap)
    psubusb        m4, m9        ; imax(0, pri_strength - (abs(diff) >> shift)))
    pminub         m2, m4
    vpdpbusd       m0, m2, m10   ; sum
%endmacro
    CDEF_FILTER_4x4_PRI
    test         secd, secd
    jz .end_no_clip
    call .sec
.end_clip:
    pminub         m4, m6, m1
    pmaxub         m1, m6
    pminub         m5, m2, m3
    pmaxub         m2, m3
    pminub         m4, m5
    pmaxub         m2, m1
    psrldq         m1, m4, 2
    psrldq         m3, m2, 2
    pminub         m1, m4
    vpcmpw         k1, m0, m7, 1
    vpshldd        m6, m0, 8
    pmaxub         m2, m3
    pslldq         m3, m1, 1
    psubw          m7, m0
    paddusw        m0, m6     ; clip >0xff
    vpsubusw   m0{k1}, m6, m7 ; clip <0x00
    pslldq         m4, m2, 1
    pminub         m1, m3
    pmaxub         m2, m4
    pmaxub         m0, m1
    pminub         m0, m2
    jmp .end
.sec_only:
    movifnidn    secd, secm
    call .sec
.end_no_clip:
    vpshldd        m6, m0, 8  ; (px << 8) + ((sum > -8) << 4)
    paddw          m0, m6     ; (px << 8) + ((sum + (sum > -8) + 7) << 4)
.end:
    mova          xm1, [base+end_perm]
    vpermb         m0, m1, m0 ; output in bits 8-15 of each dword
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    pextrd [dstq+strideq*2], xm0, 2
    pextrd [dstq+r2       ], xm0, 3
    RET
.mask_edges_sec_only:
    movifnidn    secd, secm
    call .mask_edges_sec
    jmp .end_no_clip
ALIGN function_align
.mask_edges:
    vpbroadcastq   m8, [base+edge_mask+r6*8]
    test         prid, prid
    jz .mask_edges_sec_only
    vpaddd         m2, m3, [base+cdef_dirs+(t0+2)*4] {1to16}
    vpshufbitqmb   k1, m8, m2 ; index in-range
    mova           m1, m6
    vpermb     m1{k1}, m2, m5
    CDEF_FILTER_4x4_PRI
    test         secd, secd
    jz .end_no_clip
    call .mask_edges_sec
    jmp .end_clip
.mask_edges_sec:
    vpaddd         m4, m3, [base+cdef_dirs+(t0+4)*4] {1to16}
    vpaddd         m9, m3, [base+cdef_dirs+(t0+0)*4] {1to16}
    vpshufbitqmb   k1, m8, m4
    mova           m2, m6
    vpermb     m2{k1}, m4, m5
    vpshufbitqmb   k1, m8, m9
    mova           m3, m6
    vpermb     m3{k1}, m9, m5
    jmp .sec_main
ALIGN function_align
.sec:
    vpaddd         m2, m3, [base+cdef_dirs+(t0+4)*4] {1to16} ; dir + 2
    vpaddd         m3,     [base+cdef_dirs+(t0+0)*4] {1to16} ; dir - 2
    vpermb         m2, m2, m5 ; k0s0 k0s1 k1s0 k1s1
    vpermb         m3, m3, m5 ; k0s2 k0s3 k1s2 k1s3
.sec_main:
    vpbroadcastd   m8, [base+sec_tap]
    vpcmpub        k1, m6, m2, 6
    psubb          m4, m2, m6
    vpbroadcastb  m12, secd
    lzcnt        secd, secd
    vpsubb     m4{k1}, m6, m2
    vpcmpub        k2, m6, m3, 6
    vpbroadcastq  m11, [r3+secq*8]
    gf2p8affineqb m10, m4, m11, 0
    psubb          m5, m3, m6
    mova           m9, m8
    vpsubb     m8{k1}, m7, m8
    psubusb       m10, m12, m10
    vpsubb     m5{k2}, m6, m3
    pminub         m4, m10
    vpdpbusd       m0, m4, m8
    gf2p8affineqb m11, m5, m11, 0
    vpsubb     m9{k2}, m7, m9
    psubusb       m12, m11
    pminub         m5, m12
    vpdpbusd       m0, m5, m9
    ret

DECLARE_REG_TMP 2, 7

;         lut top                lut bottom
; t0 t1 t2 t3 t4 t5 t6 t7  L4 L5 20 21 22 23 24 25
; T0 T1 T2 T3 T4 T5 T6 T7  L6 L7 30 31 32 33 34 35
; L0 L1 00 01 02 03 04 05  L8 L9 40 41 42 43 44 45
; L2 L3 10 11 12 13 14 15  La Lb 50 51 52 53 54 55
; L4 L5 20 21 22 23 24 25  Lc Ld 60 61 62 63 64 65
; L6 L7 30 31 32 33 34 35  Le Lf 70 71 72 73 74 75
; L8 L9 40 41 42 43 44 45  b0 b1 b2 b3 b4 b5 b6 b7
; La Lb 50 51 52 53 54 55  B0 B1 B2 B3 B4 B5 B6 B7

cglobal cdef_filter_4x8_8bpc, 5, 9, 22, dst, stride, left, top, bot, \
                                        pri, sec, dir, damping, edge
%define base r8-edge_mask
    vpbroadcastd ym21, strided
    mov           r6d, edgem
    lea            r8, [edge_mask]
    movq          xm1, [topq+strideq*0-2]
    pmulld       ym21, [base+pd_01234567]
    kxnorb         k1, k1, k1
    movq          xm2, [topq+strideq*1-2]
    vpgatherdq m0{k1}, [dstq+ym21]  ; +0+1 +2+3 +4+5 +6+7
    mova          m14, [base+lut_perm_4x8a]
    movu          m15, [base+lut_perm_4x8b]
    test          r6b, 0x08         ; avoid buffer overread
    jz .main
    vinserti32x4  ym1, [botq+strideq*0-2], 1
    vinserti32x4  ym2, [botq+strideq*1-2], 1
.main:
    punpcklqdq    ym1, ym2
    vinserti32x4   m1, [leftq], 2   ; -2-1 +8+9 left ____
    movifnidn    prid, prim
    mov           t0d, dirm
    mova          m16, [base+px_idx]
    mov           r3d, dampingm
    vpermi2b      m14, m0, m1    ; lut top
    vpermi2b      m15, m0, m1    ; lut bottom
    vpbroadcastd   m0, [base+pd_268435568] ; (1 << 28) + (7 << 4)
    pxor          m20, m20
    lea            r3, [r8+r3*8] ; gf_shr + (damping - 30) * 8
    vpermb         m2, m16, m14  ; pxt
    vpermb         m3, m16, m15  ; pxb
    mova           m1, m0
    cmp           r6b, 0x0f
    jne .mask_edges              ; mask edges only if required
    test         prid, prid
    jz .sec_only
    vpaddd         m6, m16, [base+cdef_dirs+(t0+2)*4] {1to16} ; dir
    vpermb         m4, m6, m14   ; pNt k0p0 k0p1 k1p0 k1p1
    vpermb         m5, m6, m15   ; pNb
%macro CDEF_FILTER_4x8_PRI 0
    vpcmpub        k1, m2, m4, 6 ; pxt > pNt
    vpcmpub        k2, m3, m5, 6 ; pxb > pNb
    psubb          m6, m4, m2
    psubb          m7, m5, m3
    lzcnt         r6d, prid
    vpsubb     m6{k1}, m2, m4    ; abs(diff_top)
    vpsubb     m7{k2}, m3, m5    ; abs(diff_bottom)
    vpbroadcastb  m13, prid
    vpbroadcastq   m9, [r3+r6*8]
    and          prid, 1
    vpbroadcastd  m11, [base+pri_tap+priq*4]
    vgf2p8affineqb m8, m6, m9, 0 ; abs(dt) >> shift
    vgf2p8affineqb m9, m7, m9, 0 ; abs(db) >> shift
    mova          m10, m11
    movifnidn     t1d, secm
    vpsubb    m10{k1}, m20, m11  ; apply_sign(pri_tap_top)
    vpsubb    m11{k2}, m20, m11  ; apply_sign(pri_tap_bottom)
    psubusb       m12, m13, m8   ; imax(0, pri_strength - (abs(dt) >> shift)))
    psubusb       m13, m13, m9   ; imax(0, pri_strength - (abs(db) >> shift)))
    pminub         m6, m12
    pminub         m7, m13
    vpdpbusd       m0, m6, m10   ; sum top
    vpdpbusd       m1, m7, m11   ; sum bottom
%endmacro
    CDEF_FILTER_4x8_PRI
    test          t1d, t1d       ; sec
    jz .end_no_clip
    call .sec
.end_clip:
    pminub        m10, m4, m2
    pminub        m12, m6, m8
    pminub        m11, m5, m3
    pminub        m13, m7, m9
    pmaxub         m4, m2
    pmaxub         m6, m8
    pmaxub         m5, m3
    pmaxub         m7, m9
    pminub        m10, m12
    pminub        m11, m13
    pmaxub         m4, m6
    pmaxub         m5, m7
    mov           r2d, 0xAAAAAAAA
    kmovd          k1, r2d
    kxnorb         k2, k2, k2       ;   hw   lw
    vpshrdd       m12, m0, m1, 16   ;  m1lw m0hw
    vpshrdd        m6, m10, m11, 16 ; m11lw m10hw
    vpshrdd        m8, m4, m5, 16   ;  m5lw m4hw
    vpblendmw  m7{k1}, m10, m11     ; m11hw m10lw
    vpblendmw  m9{k1}, m4, m5       ;  m5hw m4lw
    vpblendmw  m4{k1}, m0, m12      ;  m1lw m0lw
    vpblendmw  m5{k1}, m12, m1      ;  m1hw m0hw
    vpshrdd        m2, m3, 16
    pminub         m6, m7
    pmaxub         m8, m9
    mova         ym14, [base+end_perm]
    vpcmpw         k1, m4, m20, 1
    vpshldw        m2, m5, 8
    pslldq         m7, m6, 1
    pslldq         m9, m8, 1
    psubw          m5, m20, m4
    paddusw        m0, m4, m2 ; clip >0xff
    pminub         m6, m7
    pmaxub         m8, m9
    psubusw    m0{k1}, m2, m5 ; clip <0x00
    pmaxub         m0, m6
    pminub         m0, m8
    vpermb         m0, m14, m0
    vpscatterdd [dstq+ym21]{k2}, ym0
    RET
.sec_only:
    movifnidn     t1d, secm
    call .sec
.end_no_clip:
    mova          ym4, [base+end_perm]
    kxnorb         k1, k1, k1
    vpshldd        m2, m0, 8  ; (px << 8) + ((sum > -8) << 4)
    vpshldd        m3, m1, 8
    paddw          m0, m2     ; (px << 8) + ((sum + (sum > -8) + 7) << 4)
    paddw          m1, m3
    pslld          m0, 16
    vpshrdd        m0, m1, 16
    vpermb         m0, m4, m0 ; output in bits 8-15 of each word
    vpscatterdd [dstq+ym21]{k1}, ym0
    RET
.mask_edges_sec_only:
    movifnidn     t1d, secm
    call .mask_edges_sec
    jmp .end_no_clip
ALIGN function_align
.mask_edges:
    mov           t1d, r6d
    or            r6d, 8 ; top 4x4 has bottom
    or            t1d, 4 ; bottom 4x4 has top
    vpbroadcastq  m17, [base+edge_mask+r6*8]
    vpbroadcastq  m18, [base+edge_mask+t1*8]
    test         prid, prid
    jz .mask_edges_sec_only
    vpaddd         m6, m16, [base+cdef_dirs+(t0+2)*4] {1to16}
    vpshufbitqmb   k1, m17, m6 ; index in-range
    vpshufbitqmb   k2, m18, m6
    mova           m4, m2
    mova           m5, m3
    vpermb     m4{k1}, m6, m14
    vpermb     m5{k2}, m6, m15
    CDEF_FILTER_4x8_PRI
    test          t1d, t1d
    jz .end_no_clip
    call .mask_edges_sec
    jmp .end_clip
.mask_edges_sec:
    vpaddd        m10, m16, [base+cdef_dirs+(t0+4)*4] {1to16}
    vpaddd        m11, m16, [base+cdef_dirs+(t0+0)*4] {1to16}
    vpshufbitqmb   k1, m17, m10
    vpshufbitqmb   k2, m18, m10
    vpshufbitqmb   k3, m17, m11
    vpshufbitqmb   k4, m18, m11
    mova           m6, m2
    mova           m7, m3
    mova           m8, m2
    mova           m9, m3
    vpermb     m6{k1}, m10, m14
    vpermb     m7{k2}, m10, m15
    vpermb     m8{k3}, m11, m14
    vpermb     m9{k4}, m11, m15
    jmp .sec_main
ALIGN function_align
.sec:
    vpaddd         m8, m16, [base+cdef_dirs+(t0+4)*4] {1to16} ; dir + 2
    vpaddd         m9, m16, [base+cdef_dirs+(t0+0)*4] {1to16} ; dir - 2
    vpermb         m6, m8, m14 ; pNt k0s0 k0s1 k1s0 k1s1
    vpermb         m7, m8, m15 ; pNb
    vpermb         m8, m9, m14 ; pNt k0s2 k0s3 k1s2 k1s3
    vpermb         m9, m9, m15 ; pNb
.sec_main:
    vpbroadcastb  m18, t1d
    lzcnt         t1d, t1d
    vpcmpub        k1, m2, m6, 6
    vpcmpub        k2, m3, m7, 6
    vpcmpub        k3, m2, m8, 6
    vpcmpub        k4, m3, m9, 6
    vpbroadcastq  m17, [r3+t1*8]
    psubb         m10, m6, m2
    psubb         m11, m7, m3
    psubb         m12, m8, m2
    psubb         m13, m9, m3
    vpsubb    m10{k1}, m2, m6      ; abs(dt0)
    vpsubb    m11{k2}, m3, m7      ; abs(db0)
    vpsubb    m12{k3}, m2, m8      ; abs(dt1)
    vpsubb    m13{k4}, m3, m9      ; abs(db1)
    vpbroadcastd  m19, [base+sec_tap]
    gf2p8affineqb m14, m10, m17, 0 ; abs(dt0) >> shift
    gf2p8affineqb m15, m11, m17, 0 ; abs(db0) >> shift
    gf2p8affineqb m16, m12, m17, 0 ; abs(dt1) >> shift
    gf2p8affineqb m17, m13, m17, 0 ; abs(db1) >> shift
    psubusb       m14, m18, m14    ; imax(0, sec_strength - (abs(dt0) >> shift)))
    psubusb       m15, m18, m15    ; imax(0, sec_strength - (abs(db0) >> shift)))
    psubusb       m16, m18, m16    ; imax(0, sec_strength - (abs(dt1) >> shift)))
    psubusb       m17, m18, m17    ; imax(0, sec_strength - (abs(db1) >> shift)))
    pminub        m10, m14
    pminub        m11, m15
    pminub        m12, m16
    pminub        m13, m17
    mova          m14, m19
    mova          m15, m19
    mova          m16, m19
    vpsubb    m14{k1}, m20, m19    ; apply_sign(sec_tap_top_0)
    vpsubb    m15{k2}, m20, m19    ; apply_sign(sec_tap_bottom_0)
    vpsubb    m16{k3}, m20, m19    ; apply_sign(sec_tap_top_1)
    vpsubb    m19{k4}, m20, m19    ; apply_sign(sec_tap_bottom_1)
    vpdpbusd       m0, m10, m14
    vpdpbusd       m1, m11, m15
    vpdpbusd       m0, m12, m16
    vpdpbusd       m1, m13, m19
    ret

;         lut tl                   lut tr
; t0 t1 t2 t3 t4 t5 t6 t7  t4 t5 t6 t7 t8 t9 ta tb
; T0 T1 T2 T3 T4 T5 T6 T7  T4 T5 T6 T7 T8 T9 Ta Tb
; L0 L1 00 01 02 03 04 05  02 03 04 05 06 07 08 09
; L2 L3 10 11 12 13 14 15  12 13 14 15 16 17 18 19
; L4 L5 20 21 22 23 24 25  22 23 24 25 26 27 28 29
; L6 L7 30 31 32 33 34 35  32 33 34 35 36 37 38 39
; L8 L9 40 41 42 43 44 45  42 43 44 45 46 47 48 49
; La Lb 50 51 52 53 54 55  52 53 54 55 56 57 58 59
;         lut bl                   lut br
; L4 L5 20 21 22 23 24 25  22 23 24 25 26 27 28 29
; L6 L7 30 31 32 33 34 35  32 33 34 35 36 37 38 39
; L8 L9 40 41 42 43 44 45  42 43 44 45 46 47 48 49
; La Lb 50 51 52 53 54 55  52 53 54 55 56 57 58 59
; Lc Ld 60 61 62 63 64 65  62 63 64 65 66 67 68 69
; Le Lf 70 71 72 73 74 75  72 73 74 75 76 77 78 79
; b0 b1 b2 b3 b4 b5 b6 b7  b4 b5 b6 b7 b8 b9 ba bb
; B0 B1 B2 B3 B4 B5 B6 B7  B4 B5 B6 B7 B8 B9 Ba Bb

cglobal cdef_filter_8x8_8bpc, 5, 11, 32, 4*64, dst, stride, left, top, bot, \
                                               pri, sec, dir, damping, edge
%define base r8-edge_mask
    movu         xm16, [dstq+strideq*0]
    pinsrd       xm16, [leftq+4*0], 3
    mov           r6d, edgem
    vinserti128  ym16, [dstq+strideq*1], 1
    lea           r10, [dstq+strideq*4]
    movu         xm17, [dstq+strideq*2]
    vinserti32x4  m16, [topq+strideq*0-2], 2
    lea            r9, [strideq*3]
    pinsrd       xm17, [leftq+4*1], 3
    vinserti32x4  m16, [topq+strideq*1-2], 3 ; 0 1 t T
    lea            r8, [edge_mask]
    vinserti128  ym17, [dstq+r9       ], 1
    vpbroadcastd ym18, [leftq+4*2]
    vpblendd     ym17, ym18, 0x80
    movu         xm18, [r10 +strideq*2]
    vinserti32x4  m17, [r10 +strideq*0], 2
    pinsrd       xm18, [leftq+4*3], 3
    vinserti32x4  m17, [r10 +strideq*1], 3   ; 2 3 4 5
    vinserti128  ym18, [r10 +r9       ], 1
    test          r6b, 0x08       ; avoid buffer overread
    jz .main
    vinserti32x4  m18, [botq+strideq*0-2], 2
    vinserti32x4  m18, [botq+strideq*1-2], 3 ; 6 7 b B
.main:
    mova           m0, [base+lut_perm_8x8a]
    movu           m1, [base+lut_perm_8x8b]
    mova          m30, [base+px_idx]
    vpermb        m16, m0, m16
    movifnidn    prid, prim
    vpermb        m17, m1, m17
    mov           t0d, dirm
    vpermb        m18, m0, m18
    mov           r3d, dampingm
    vshufi32x4    m12, m16, m17, q2020 ; lut tl
    vshufi32x4    m13, m16, m17, q3131 ; lut tr
    vshufi32x4    m14, m17, m18, q0220 ; lut bl
    vshufi32x4    m15, m17, m18, q1331 ; lut br
    vpbroadcastd   m0, [base+pd_268435568] ; (1 << 28) + (7 << 4)
    pxor          m31, m31
    lea            r3, [r8+r3*8]  ; gf_shr + (damping - 30) * 8
    vpermb         m4, m30, m12   ; pxtl
    mova           m1, m0
    vpermb         m5, m30, m13   ; pxtr
    mova           m2, m0
    vpermb         m6, m30, m14   ; pxbl
    mova           m3, m0
    vpermb         m7, m30, m15   ; pxbr
    cmp           r6b, 0x0f
    jne .mask_edges               ; mask edges only if required
    test         prid, prid
    jz .sec_only
    vpaddd        m11, m30, [base+cdef_dirs+(t0+2)*4] {1to16} ; dir
    vpermb         m8, m11, m12   ; pNtl k0p0 k0p1 k1p0 k1p1
    vpermb         m9, m11, m13   ; pNtr
    vpermb        m10, m11, m14   ; pNbl
    vpermb        m11, m11, m15   ; pNbr
%macro CDEF_FILTER_8x8_PRI 0
    vpcmpub        k1, m4, m8, 6  ; pxtl > pNtl
    vpcmpub        k2, m5, m9, 6  ; pxtr > pNtr
    vpcmpub        k3, m6, m10, 6 ; pxbl > pNbl
    vpcmpub        k4, m7, m11, 6 ; pxbr > pNbr
    psubb         m16, m8, m4
    psubb         m17, m9, m5
    psubb         m18, m10, m6
    psubb         m19, m11, m7
    lzcnt         r6d, prid
    vpsubb    m16{k1}, m4, m8     ; abs(diff_tl)
    vpsubb    m17{k2}, m5, m9     ; abs(diff_tr)
    vpsubb    m18{k3}, m6, m10    ; abs(diff_bl)
    vpsubb    m19{k4}, m7, m11    ; abs(diff_br)
    vpbroadcastq  m28, [r3+r6*8]
    vpbroadcastb  m29, prid
    and          prid, 1
    vpbroadcastd  m27, [base+pri_tap+priq*4]
    vgf2p8affineqb m20, m16, m28, 0 ; abs(dtl) >> shift
    vgf2p8affineqb m21, m17, m28, 0 ; abs(dtr) >> shift
    vgf2p8affineqb m22, m18, m28, 0 ; abs(dbl) >> shift
    vgf2p8affineqb m23, m19, m28, 0 ; abs(dbl) >> shift
    mova          m24, m27
    mova          m25, m27
    mova          m26, m27
    movifnidn     t1d, secm
    vpsubb    m24{k1}, m31, m27   ; apply_sign(pri_tap_tl)
    vpsubb    m25{k2}, m31, m27   ; apply_sign(pri_tap_tr)
    vpsubb    m26{k3}, m31, m27   ; apply_sign(pri_tap_tl)
    vpsubb    m27{k4}, m31, m27   ; apply_sign(pri_tap_tr)
    psubusb       m20, m29, m20   ; imax(0, pri_strength - (abs(dtl) >> shift)))
    psubusb       m21, m29, m21   ; imax(0, pri_strength - (abs(dtr) >> shift)))
    psubusb       m22, m29, m22   ; imax(0, pri_strength - (abs(dbl) >> shift)))
    psubusb       m23, m29, m23   ; imax(0, pri_strength - (abs(dbr) >> shift)))
    pminub        m16, m20
    pminub        m17, m21
    pminub        m18, m22
    pminub        m19, m23
    vpdpbusd       m0, m16, m24   ; sum tl
    vpdpbusd       m1, m17, m25   ; sum tr
    vpdpbusd       m2, m18, m26   ; sum bl
    vpdpbusd       m3, m19, m27   ; sum br
%endmacro
    CDEF_FILTER_8x8_PRI
    test          t1d, t1d        ; sec
    jz .end_no_clip
    call .sec
.end_clip:
    pminub        m20, m8, m4
    pminub        m24, m12, m16
    pminub        m21, m9, m5
    pminub        m25, m13, m17
    pminub        m22, m10, m6
    pminub        m26, m14, m18
    pminub        m23, m11, m7
    pminub        m27, m15, m19
    pmaxub         m8, m4
    pmaxub        m12, m16
    pmaxub         m9, m5
    pmaxub        m13, m17
    pmaxub        m10, m6
    pmaxub        m14, m18
    pmaxub        m11, m7
    pmaxub        m15, m19
    pminub        m20, m24
    pminub        m21, m25
    pminub        m22, m26
    pminub        m23, m27
    pmaxub         m8, m12
    pmaxub         m9, m13
    pmaxub        m10, m14
    pmaxub        m11, m15
    mov           r2d, 0xAAAAAAAA
    kmovd          k1, r2d
    vpshrdd       m24,  m0,  m1, 16
    vpshrdd       m25,  m2,  m3, 16
    vpshrdd       m12, m20, m21, 16
    vpshrdd       m14, m22, m23, 16
    vpshrdd       m16,  m8,  m9, 16
    vpshrdd       m18, m10, m11, 16
    vpblendmw m13{k1}, m20, m21
    vpblendmw m15{k1}, m22, m23
    vpblendmw m17{k1},  m8, m9
    vpblendmw m19{k1}, m10, m11
    vpblendmw m20{k1},  m0, m24
    vpblendmw m21{k1}, m24, m1
    vpblendmw m22{k1},  m2, m25
    vpblendmw m23{k1}, m25, m3
    vpshrdd        m4, m5, 16
    vpshrdd        m6, m7, 16
    pminub        m12, m13
    pminub        m14, m15
    pmaxub        m16, m17
    pmaxub        m18, m19
    mova           m8, [base+end_perm_clip]
    vpcmpw         k2, m20, m31, 1
    vpcmpw         k3, m22, m31, 1
    vpshldw        m4, m21, 8
    vpshldw        m6, m23, 8
    kunpckdq       k1, k1, k1
    kxnorb         k4, k4, k4
    vpshrdw       m11, m12, m14, 8
    vpshrdw       m15, m16, m18, 8
    vpblendmb m13{k1}, m12, m14
    vpblendmb m17{k1}, m16, m18
    psubw         m21, m31, m20
    psubw         m23, m31, m22
    paddusw        m0, m20, m4  ; clip >0xff
    paddusw        m1, m22, m6
    pminub        m11, m13
    pmaxub        m15, m17
    psubusw    m0{k2}, m4, m21  ; clip <0x00
    psubusw    m1{k3}, m6, m23
    psrlw          m0, 8
    vmovdqu8   m0{k1}, m1
    pmaxub         m0, m11
    pminub         m0, m15
    vpermb         m0, m8, m0
    vextracti32x4 xm1, m0, 1
    vextracti32x4 xm2, m0, 2
    vextracti32x4 xm3, m0, 3
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*2], xm1
    movq   [r10 +strideq*0], xm2
    movq   [r10 +strideq*2], xm3
    movhps [dstq+strideq*1], xm0
    movhps [dstq+r9       ], xm1
    movhps [r10 +strideq*1], xm2
    movhps [r10 +r9       ], xm3
    RET
.sec_only:
    movifnidn     t1d, secm
    call .sec
.end_no_clip:
    mova          xm8, [base+end_perm]
    kxnorb         k1, k1, k1
    vpshldd        m4, m0, 8  ; (px << 8) + ((sum > -8) << 4)
    vpshldd        m5, m1, 8
    vpshldd        m6, m2, 8
    vpshldd        m7, m3, 8
    paddw          m0, m4     ; (px << 8) + ((sum + (sum > -8) + 7) << 4)
    paddw          m1, m5
    paddw          m2, m6
    paddw          m3, m7
    vpermb         m0, m8, m0
    vpermb         m1, m8, m1
    vpermb         m2, m8, m2
    vpermb         m3, m8, m3
    punpckldq      m4, m0, m1
    punpckhdq      m0, m1
    punpckldq      m5, m2, m3
    punpckhdq      m2, m3
    movq   [dstq+strideq*0], xm4
    movq   [dstq+strideq*2], xm0
    movq   [r10 +strideq*0], xm5
    movq   [r10 +strideq*2], xm2
    movhps [dstq+strideq*1], xm4
    movhps [dstq+r9       ], xm0
    movhps [r10 +strideq*1], xm5
    movhps [r10 +r9       ], xm2
    RET
.mask_edges_sec_only:
    movifnidn     t1d, secm
    call .mask_edges_sec
    jmp .end_no_clip
ALIGN function_align
.mask_edges:
    mov           t0d, r6d
    mov           t1d, r6d
    or            t0d, 0xA ; top-left 4x4 has bottom and right
    or            t1d, 0x9 ; top-right 4x4 has bottom and left
    vpbroadcastq  m26, [base+edge_mask+t0*8]
    vpbroadcastq  m27, [base+edge_mask+t1*8]
    mov           t1d, r6d
    or            r6d, 0x6 ; bottom-left 4x4 has top and right
    or            t1d, 0x5 ; bottom-right 4x4 has top and left
    vpbroadcastq  m28, [base+edge_mask+r6*8]
    vpbroadcastq  m29, [base+edge_mask+t1*8]
    mov           t0d, dirm
    test         prid, prid
    jz .mask_edges_sec_only
    vpaddd        m20, m30, [base+cdef_dirs+(t0+2)*4] {1to16}
    vpshufbitqmb   k1, m26, m20 ; index in-range
    vpshufbitqmb   k2, m27, m20
    vpshufbitqmb   k3, m28, m20
    vpshufbitqmb   k4, m29, m20
    mova           m8, m4
    mova           m9, m5
    mova          m10, m6
    mova          m11, m7
    vpermb     m8{k1}, m20, m12
    vpermb     m9{k2}, m20, m13
    vpermb    m10{k3}, m20, m14
    vpermb    m11{k4}, m20, m15
    mova   [rsp+0x00], m26
    mova   [rsp+0x40], m27
    mova   [rsp+0x80], m28
    mova   [rsp+0xC0], m29
    CDEF_FILTER_8x8_PRI
    test          t1d, t1d
    jz .end_no_clip
    mova          m26, [rsp+0x00]
    mova          m27, [rsp+0x40]
    mova          m28, [rsp+0x80]
    mova          m29, [rsp+0xC0]
    call .mask_edges_sec
    jmp .end_clip
.mask_edges_sec:
    vpaddd        m20, m30, [base+cdef_dirs+(t0+4)*4] {1to16}
    vpaddd        m21, m30, [base+cdef_dirs+(t0+0)*4] {1to16}
    vpshufbitqmb   k1, m26, m20
    vpshufbitqmb   k2, m27, m20
    vpshufbitqmb   k3, m28, m20
    vpshufbitqmb   k4, m29, m20
    mova          m16, m4
    mova          m17, m5
    mova          m18, m6
    mova          m19, m7
    vpermb    m16{k1}, m20, m12
    vpermb    m17{k2}, m20, m13
    vpermb    m18{k3}, m20, m14
    vpermb    m19{k4}, m20, m15
    vpshufbitqmb   k1, m26, m21
    vpshufbitqmb   k2, m27, m21
    vpshufbitqmb   k3, m28, m21
    vpshufbitqmb   k4, m29, m21
    vpermb        m12, m21, m12
    vpermb        m13, m21, m13
    vpermb        m14, m21, m14
    vpermb        m15, m21, m15
    vpblendmb m12{k1}, m4, m12
    vpblendmb m13{k2}, m5, m13
    vpblendmb m14{k3}, m6, m14
    vpblendmb m15{k4}, m7, m15
    jmp .sec_main
ALIGN function_align
.sec:
    vpaddd        m20, m30, [base+cdef_dirs+(t0+4)*4] {1to16} ; dir + 2
    vpaddd        m21, m30, [base+cdef_dirs+(t0+0)*4] {1to16} ; dir - 2
    vpermb        m16, m20, m12 ; pNtl k0s0 k0s1 k1s0 k1s1
    vpermb        m17, m20, m13 ; pNtr
    vpermb        m18, m20, m14 ; pNbl
    vpermb        m19, m20, m15 ; pNbr
    vpermb        m12, m21, m12 ; pNtl k0s2 k0s3 k1s2 k1s3
    vpermb        m13, m21, m13 ; pNtr
    vpermb        m14, m21, m14 ; pNbl
    vpermb        m15, m21, m15 ; pNbr
.sec_main:
%macro CDEF_FILTER_8x8_SEC 4-5 0 ; load constants
    vpcmpub        k1, m4, %1, 6
    vpcmpub        k2, m5, %2, 6
    vpcmpub        k3, m6, %3, 6
    vpcmpub        k4, m7, %4, 6
    psubb         m20, %1, m4
    psubb         m21, %2, m5
    psubb         m22, %3, m6
    psubb         m23, %4, m7
%if %5
    vpbroadcastb  m28, t1d
    lzcnt         t1d, t1d
    vpbroadcastq  m29, [r3+t1*8]
%endif
    vpsubb    m20{k1}, m4, %1
    vpsubb    m21{k2}, m5, %2
    vpsubb    m22{k3}, m6, %3
    vpsubb    m23{k4}, m7, %4
    gf2p8affineqb m24, m20, m29, 0
    gf2p8affineqb m25, m21, m29, 0
    gf2p8affineqb m26, m22, m29, 0
    gf2p8affineqb m27, m23, m29, 0
%if %5
    vpbroadcastd  m30, [base+sec_tap]
%endif
    psubusb       m24, m28, m24
    psubusb       m25, m28, m25
    psubusb       m26, m28, m26
    psubusb       m27, m28, m27
    pminub        m20, m24
    pminub        m21, m25
    pminub        m22, m26
    pminub        m23, m27
    mova          m24, m30
    mova          m25, m30
    mova          m26, m30
    mova          m27, m30
    vpsubb    m24{k1}, m31, m30
    vpsubb    m25{k2}, m31, m30
    vpsubb    m26{k3}, m31, m30
    vpsubb    m27{k4}, m31, m30
    vpdpbusd       m0, m20, m24
    vpdpbusd       m1, m21, m25
    vpdpbusd       m2, m22, m26
    vpdpbusd       m3, m23, m27
%endmacro
    CDEF_FILTER_8x8_SEC m16, m17, m18, m19, 1
    CDEF_FILTER_8x8_SEC m12, m13, m14, m15
    ret

%endif ; ARCH_X86_64
