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

l_shuf_v:      times 2 db  0, 32
pw_1:          times 2 dw  1
               times 2 db  4, 36
pw_3:          times 2 dw  3
               times 2 db  8, 40
pw_4:          times 2 dw  4
               times 2 db 12, 44
pw_16:         times 2 dw 16
               times 2 db 16, 48
pw_4096:       times 2 dw 4096
               times 2 db 20, 52
pw_16384:      times 2 dw 16384
               times 2 db 24, 56
pw_32767:      times 2 dw 32767
               times 2 db 28, 60
               times 2 dw  0
filter_mask:   dd  1,  2,  4,  8, 16, 32, 64,128
stride_mul:    dd  0,  1,  8,  9, 16, 17, 24, 25
l_shuf_h:      db  4, -1,  4, -1,  4, -1,  4, -1, 12, -1, 12, -1, 12, -1, 12, -1
clip_max: dw  511,  511,  2047,  2047
clip_min: dw -512, -512, -2048, -2048

SECTION .text

%macro TRANSPOSE8X8W 9 ; src/dst[1-8], tmp
    punpckhwd        m%9, m%5, m%6
    punpcklwd        m%5, m%6
    punpckhwd        m%6, m%1, m%2
    punpcklwd        m%1, m%2
    punpckhwd        m%2, m%7, m%8
    punpcklwd        m%7, m%8
    punpckhwd        m%8, m%3, m%4
    punpcklwd        m%3, m%4
    punpckhdq        m%4, m%1, m%3
    punpckldq        m%1, m%3
    punpckldq        m%3, m%5, m%7
    punpckhdq        m%5, m%7
    punpckhdq        m%7, m%6, m%8
    punpckldq        m%6, m%8
    punpckldq        m%8, m%9, m%2
    punpckhdq        m%9, m%2
    punpckhqdq       m%2, m%1, m%3
    punpcklqdq       m%1, m%3
    punpcklqdq       m%3, m%4, m%5
    punpckhqdq       m%4, m%5
    punpcklqdq       m%5, m%6, m%8
    punpckhqdq       m%6, m%8
    punpckhqdq       m%8, m%7, m%9
    punpcklqdq       m%7, m%9
%endmacro

%macro FILTER 2 ; width [4/6/8/16], dir [h/v]
%ifidn %2, v
%if %1 == 16
    lea             tmpq, [dstq+mstrideq*8]
    mova              m0, [tmpq+strideq*1 ]
    mova              m1, [tmpq+strideq*2 ] ; p5
    mova              m2, [tmpq+stride3q  ] ; p4
    mova              m3, [tmpq+strideq*4 ] ; p3
    mova              m4, [tmpq+stride5q  ] ; p2
%elif %1 == 6 || %1 == 8
    lea             tmpq, [dstq+mstrideq*4]
%if %1 == 8
    mova              m3, [tmpq+strideq*0 ]
%endif
    mova              m4, [tmpq+strideq*1 ]
%endif
    mova              m5, [dstq+mstrideq*2] ; p1
    mova              m6, [dstq+mstrideq*1] ; p0
    mova              m7, [dstq+strideq*0 ] ; q0
    mova              m8, [dstq+strideq*1 ] ; q1
%if %1 != 4
    mova              m9, [dstq+strideq*2 ] ; q2
%endif
%if %1 == 8 || %1 == 16
    mova             m10, [dstq+stride3q  ] ; q3
%endif
%if %1 == 16
    mova             m11, [dstq+strideq*4 ] ; q4
    mova             m22, [dstq+stride5q  ] ; q5
    mova             m23, [dstq+stride3q*2]
%endif
%else ; h
%if %1 == 16
    movu            ym16, [dstq+strideq*0 -16]
    movu            ym17, [dstq+strideq*1 -16]
    movu            ym18, [dstq+strideq*2 -16]
    movu            ym19, [dstq+stride3q  -16]
    movu            ym20, [dstq+strideq*4 -16]
    movu            ym22, [dstq+stride5q  -16]
    movu            ym23, [dstq+stride3q*2-16]
    movu            ym28, [dstq+stride7q  -16]
    lea             tmpq, [dstq+strideq*8 -16]
    vinserti32x8      m7, m16, [tmpq+strideq*0 ], 1
    vinserti32x8      m8, m17, [tmpq+strideq*1 ], 1
    vinserti32x8      m9, m18, [tmpq+strideq*2 ], 1
    vinserti32x8     m10, m19, [tmpq+stride3q  ], 1
    vinserti32x8     m11, m20, [tmpq+strideq*4 ], 1
    vinserti32x8     m22, m22, [tmpq+stride5q  ], 1
    vinserti32x8     m23, m23, [tmpq+stride3q*2], 1
    vinserti32x8     m28, m28, [tmpq+stride7q  ], 1
    lea             tmpq, [tmpq+strideq*8]
    TRANSPOSE8X8W      7,  8,  9, 10, 11, 22, 23, 28, 27
    movu            ym16, [tmpq+strideq*0 ]
    movu            ym17, [tmpq+strideq*1 ]
    movu            ym18, [tmpq+strideq*2 ]
    movu            ym19, [tmpq+stride3q  ]
    movu            ym24, [tmpq+strideq*4 ]
    movu            ym25, [tmpq+stride5q  ]
    movu            ym26, [tmpq+stride3q*2]
    movu            ym20, [tmpq+stride7q  ]
    lea             tmpq, [tmpq+strideq*8]
    vinserti32x8      m0, m16, [tmpq+strideq*0 ], 1
    vinserti32x8      m1, m17, [tmpq+strideq*1 ], 1
    vinserti32x8      m2, m18, [tmpq+strideq*2 ], 1
    vinserti32x8      m3, m19, [tmpq+stride3q  ], 1
    vinserti32x8      m4, m24, [tmpq+strideq*4 ], 1
    vinserti32x8      m5, m25, [tmpq+stride5q  ], 1
    vinserti32x8      m6, m26, [tmpq+stride3q*2], 1
    vinserti32x8     m20, m20, [tmpq+stride7q  ], 1
    TRANSPOSE8X8W      0,  1,  2,  3,  4,  5,  6, 20, 27
    vshufi32x4       m27, m7, m0, q2020
    vshufi32x4        m7, m0, q3131
    vshufi32x4        m0, m8, m1, q2020
    vshufi32x4        m8, m1, q3131
    vshufi32x4        m1, m9, m2, q2020
    vshufi32x4        m9, m2, q3131
    vshufi32x4        m2, m10, m3, q2020
    vshufi32x4       m10, m3, q3131
    vshufi32x4        m3, m11, m4, q2020
    vshufi32x4       m11, m4, q3131
    vshufi32x4        m4, m22, m5, q2020
    vshufi32x4       m22, m5, q3131
    vshufi32x4        m5, m23, m6, q2020
    vshufi32x4       m23, m6, q3131
    vshufi32x4        m6, m28, m20, q2020
    vshufi32x4       m28, m20, q3131
%elif %1 == 6 || %1 == 8
%if %1 == 8
    sub             dstq, 8
    movu            xm16, [dstq+strideq*0 ]
    movu            xm17, [dstq+strideq*1 ]
    movu            xm18, [dstq+strideq*2 ]
    movu            xm19, [dstq+stride3q  ]
    movu            xm24, [dstq+strideq*4 ]
    movu            xm25, [dstq+stride5q  ]
    movu            xm26, [dstq+stride3q*2]
    movu            xm27, [dstq+stride7q  ]
    lea             tmpq, [dstq+strideq*8 ]
    vinserti128     ym16, [tmpq+strideq*0 ], 1
    vinserti128     ym17, [tmpq+strideq*1 ], 1
    vinserti128     ym18, [tmpq+strideq*2 ], 1
    vinserti128     ym19, [tmpq+stride3q  ], 1
    vinserti128     ym24, [tmpq+strideq*4 ], 1
    vinserti128     ym25, [tmpq+stride5q  ], 1
    vinserti128     ym26, [tmpq+stride3q*2], 1
    vinserti128     ym27, [tmpq+stride7q  ], 1
    lea             tmpq, [tmpq+strideq*8 ]
    vinserti32x4     m10, m16, [tmpq+strideq*0 ], 2
    vinserti32x4      m8, m17, [tmpq+strideq*1 ], 2
    vinserti32x4      m5, m18, [tmpq+strideq*2 ], 2
    vinserti32x4      m7, m19, [tmpq+stride3q  ], 2
    vinserti32x4      m2, m24, [tmpq+strideq*4 ], 2
    vinserti32x4      m9, m25, [tmpq+stride5q  ], 2
    vinserti32x4      m3, m26, [tmpq+stride3q*2], 2
    vinserti32x4      m4, m27, [tmpq+stride7q  ], 2
    lea             tmpq, [tmpq+strideq*8 ]
    vinserti32x4     m10, [tmpq+strideq*0 ], 3
    vinserti32x4      m8, [tmpq+strideq*1 ], 3
    vinserti32x4      m5, [tmpq+strideq*2 ], 3
    vinserti32x4      m7, [tmpq+stride3q  ], 3
    vinserti32x4      m2, [tmpq+strideq*4 ], 3
    vinserti32x4      m9, [tmpq+stride5q  ], 3
    vinserti32x4      m3, [tmpq+stride3q*2], 3
    vinserti32x4      m4, [tmpq+stride7q  ], 3
%else ; %1 == 6
    movu            xm16, [dstq+strideq*0-8]
    movu            xm17, [dstq+strideq*1-8]
    movu            xm18, [dstq+strideq*2-8]
    movu            xm19, [dstq+stride3q -8]
    lea             tmpq, [dstq+strideq*4-8]
    movu             xm2, [tmpq+strideq*0]
    movu             xm9, [tmpq+strideq*1]
    movu             xm3, [tmpq+strideq*2]
    movu             xm4, [tmpq+stride3q ]
    lea             tmpq, [tmpq+strideq*4]
    vinserti128     ym16, [tmpq+strideq*0], 1
    vinserti128     ym17, [tmpq+strideq*1], 1
    vinserti128     ym18, [tmpq+strideq*2], 1
    vinserti128     ym19, [tmpq+stride3q ], 1
    lea             tmpq, [tmpq+strideq*4]
    vinserti128      ym2, [tmpq+strideq*0], 1
    vinserti128      ym9, [tmpq+strideq*1], 1
    vinserti128      ym3, [tmpq+strideq*2], 1
    vinserti128      ym4, [tmpq+stride3q ], 1
    lea             tmpq, [tmpq+strideq*4]
    vinserti32x4     m10, m16, [tmpq+strideq*0], 2
    vinserti32x4      m8, m17, [tmpq+strideq*1], 2
    vinserti32x4      m5, m18, [tmpq+strideq*2], 2
    vinserti32x4      m7, m19, [tmpq+stride3q ], 2
    lea             tmpq, [tmpq+strideq*4]
    vinserti32x4      m2, [tmpq+strideq*0], 2
    vinserti32x4      m9, [tmpq+strideq*1], 2
    vinserti32x4      m3, [tmpq+strideq*2], 2
    vinserti32x4      m4, [tmpq+stride3q ], 2
    lea             tmpq, [tmpq+strideq*4]
    vinserti32x4     m10, [tmpq+strideq*0], 3
    vinserti32x4      m8, [tmpq+strideq*1], 3
    vinserti32x4      m5, [tmpq+strideq*2], 3
    vinserti32x4      m7, [tmpq+stride3q ], 3
    lea             tmpq, [tmpq+strideq*4]
    vinserti32x4      m2, [tmpq+strideq*0], 3
    vinserti32x4      m9, [tmpq+strideq*1], 3
    vinserti32x4      m3, [tmpq+strideq*2], 3
    vinserti32x4      m4, [tmpq+stride3q ], 3
%endif
    punpcklwd         m6, m10, m8
    punpckhwd        m10, m8
    punpcklwd         m8, m5, m7
    punpckhwd         m5, m7
    punpcklwd         m7, m2, m9
    punpckhwd         m2, m9
    punpcklwd         m9, m3, m4
    punpckhwd         m3, m4
    punpckldq         m4, m6, m8
    punpckhdq         m6, m8
    punpckldq         m8, m10, m5
    punpckhdq        m10, m5
    punpckldq         m5, m7, m9
    punpckhdq         m7, m9
    punpckldq         m9, m2, m3
    punpckhdq         m2, m3
%if %1 == 8
    punpcklqdq        m3, m4, m5
%endif
    punpckhqdq        m4, m5
    punpcklqdq        m5, m6, m7
    punpckhqdq        m6, m7
    punpcklqdq        m7, m8, m9
    punpckhqdq        m8, m9
    punpcklqdq        m9, m10, m2
%if %1 == 8
    punpckhqdq       m10, m2
%endif
%else ; %1 == 4
    kxnorb            k1, k1, k1
    kmovb             k2, k1
    vpgatherdq    m7{k1}, [dstq+ym12-4]
    lea             tmpq, [dstq+strideq*2-4]
    kmovb             k1, k2
    vpgatherdq    m4{k2}, [tmpq+ym12]
    lea             tmpq, [tmpq+strideq*2]
    kmovb             k2, k1
    vpgatherdq    m5{k1}, [tmpq+ym12]
    lea             tmpq, [tmpq+strideq*2]
    vpgatherdq    m6{k2}, [tmpq+ym12]
    punpcklwd         m8, m7, m4
    punpckhwd         m7, m4
    punpcklwd         m4, m5, m6
    punpckhwd         m5, m6
    punpcklwd         m6, m8, m7
    punpckhwd         m8, m7
    punpcklwd         m7, m4, m5
    punpckhwd         m4, m5
    punpcklqdq        m5, m6, m7
    punpckhqdq        m6, m7
    punpcklqdq        m7, m8, m4
    punpckhqdq        m8, m4
%endif
%endif

    ; load L/E/I/H
%ifidn %2, v
    movu            ym16, [lq+l_strideq*1]
    movsldup         m17, [l_shuf_v]
    vptestnmb         k1, ym16, ym16
    vmovdqu8    ym16{k1}, [lq+l_strideq*0]  ; l[x][] ? l[x][] : l[x-stride][]
    vpermb           m16, m17, m16          ; l[x][1]
%else
    movq            xm16, [lq+l_strideq*0]
    movq            xm17, [lq+l_strideq*1]
    vinserti128     ym16, [lq+l_strideq*2], 1
    vinserti128     ym17, [lq+l_stride3q ], 1
    lea             tmpq, [lq+l_strideq*4]
    vinserti32x4     m16, [tmpq+l_strideq*0], 2
    vinserti32x4     m17, [tmpq+l_strideq*1], 2
    vinserti32x4     m16, [tmpq+l_strideq*2], 3
    vinserti32x4     m17, [tmpq+l_stride3q ], 3
    punpcklqdq       m16, m17
    vbroadcasti32x4  m17, [l_shuf_h]
    vptestnmb         k1, m16, m16
    vpalignr     m16{k1}, m16, 12
    pshufb           m16, m17               ; l[x][1]
%endif
    vpbroadcastd     m20, [pw_32767]
    psubw            m17, m5, m6            ; p1-p0
    psubw            m18, m7, m8            ; q1-q0
    vptestmw          k1, m16, m16          ; L
    pabsw            m17, m17
    pabsw            m18, m18
    vpmaxuw      m20{k1}, m17, m18
    vpbroadcastw     m17, [lutq+136]
    psrlw            m18, m16, [lutq+128]
    vpbroadcastd     m19, [pw_1]
    pminuw           m18, m17
    psrlw            m17, m16, 4            ; H
    paddw            m16, m16
    pmaxuw           m18, m19               ; I
    vpaddd           m16, [pw_4] {1to16}
    paddw            m16, m18               ; E
    REPX {pmullw x, m13}, m17, m18, m16
    vpcmpw            k4, m20, m17, 6       ; hev
%if %1 != 4
    psubw            m19, m4, m5            ; p2-p1
    pabsw            m19, m19
%if %1 == 8 || %1 == 16
    psubw            m17, m3, m4            ; p3-p2
    pabsw            m17, m17
    pmaxuw           m19, m17
    psubw            m17, m9, m10           ; q3-q2
    pabsw            m17, m17
    pmaxuw           m19, m17
%endif
    psubw            m17, m9, m8            ; q2-q1
    pabsw            m17, m17
    pmaxuw           m19, m17
%if %1 == 16
    vpbroadcastd    ym17, [maskq+4]
    vpord           ym17, [maskq+8] {1to8}
    vptestmd          k1, ym17, ym21
%else
    vptestmd          k1, ym21, [maskq+4] {1to8}
%endif
    pmaxuw           m19, m20
    psubw            m17, m4, m6            ; p2-p0
    pabsw            m17, m17
    pmaxuw           m17, m20
    vmovdqa64    m20{k1}, m19               ; only apply fm-wide to wd>4 blocks
%if %1 == 8 || %1 == 16
    psubw            m19, m3, m6            ; p3-p0
    pabsw            m19, m19
    pmaxuw           m17, m19
    psubw            m19, m7, m10           ; q3-q0
    pabsw            m19, m19
    pmaxuw           m17, m19
%endif
    psubw            m19, m7, m9            ; q2-q0
    pabsw            m19, m19
    pmaxuw           m17, m19
%endif
    vpcmpw            k1, m20, m18, 2
    psubw            m18, m5, m8            ; p1-q1
    psubw            m19, m6, m7            ; p0-q0
    pabsw            m18, m18
    pabsw            m19, m19
    psrlw            m18, 1
    paddw            m19, m19
    paddw            m18, m19               ; abs(p0-q0)*2+(abs(p1-q1)>>1)
    vpcmpw        k1{k1}, m18, m16, 2       ; abs(p0-q0)*2+(abs(p1-q1)>>1) <= E
%if %1 != 4
    vpcmpw        k2{k1}, m17, m13, 2       ; flat8in
%endif
%if %1 == 16
    psubw            m20, m0, m6
    psubw            m16, m1, m6
    pabsw            m20, m20
    psubw            m17, m2, m6
    pabsw            m16, m16
    psubw            m18, m11, m7
    pabsw            m17, m17
    psubw            m19, m22, m7
    pabsw            m18, m18
    pmaxuw           m20, m16
    psubw            m16, m23, m7
    pabsw            m19, m19
    pmaxuw           m17, m18
    pabsw            m16, m16
    vpandd          ym18, ym21, [maskq+8] {1to8}
    pmaxuw           m20, m17
    pmaxuw           m19, m16
    pcmpeqd         ym16, ym21, ym18
    vpternlogd      ym18, ym21, [maskq+4] {1to8}, 0xc8
    pmaxuw           m20, m19
    pcmpeqd         ym17, ym21, ym18
    vpternlogd      ym18, ym21, [maskq+0] {1to8}, 0xc8
    vpcmpw        k3{k2}, m20, m13, 2       ; flat8in & flat8out
    pcmpeqd         ym18, ym21
    vptestmb      k3{k3}, ym16, ym16        ; flat8 & fm
    vptestmb      k2{k2}, ym17, ym17        ; flat8in
    vptestmb      k1{k1}, ym18, ym18
    kandnd            k1, k2, k1            ; fm & !flat8 & !flat16
    kandnd            k2, k3, k2            ; flat8 & !flat16
%elif %1 == 6 || %1 == 8
    vpandd          ym17, ym21, [maskq+4] {1to8}
    pcmpeqd         ym16, ym21, ym17
    vpternlogd      ym17, ym21, [maskq+0] {1to8}, 0xc8
    pcmpeqd         ym17, ym21
    vptestmb      k2{k2}, ym16, ym16        ; flat8 & fm
    vptestmb      k1{k1}, ym17, ym17
    kandnd            k1, k2, k1            ; fm & !flat8
%else ; %1 == 4
    vpandd          ym16, ym21, [maskq+0] {1to8}
    pcmpeqd         ym16, ym21
    vptestmb      k1{k1}, ym16, ym16
%endif

    ; short filter
    psubw            m16, m7, m6
    vpbroadcastd     m17, [pw_3]
    paddw            m18, m16, m16
    paddw            m18, m16
    psubw            m16, m5, m8            ; iclip_diff(p1-q1)
    pminsw           m16, m14
    vpmaxsw   m16{k4}{z}, m15               ; f=iclip_diff(p1-q1)&hev
    knotd             k4, k4                ; !hev
    paddw            m16, m18               ; f=iclip_diff(3*(q0-p0)+f)
    vpbroadcastd     m18, [pw_4]
    pminsw           m16, m14
    vpmaxsw   m16{k1}{z}, m15               ; f&=fm
    paddw            m17, m16
    paddw            m16, m18
    vpbroadcastd     m18, [pw_16384]
    pminsw           m17, m14
    pminsw           m16, m14
    psraw            m17, 3                 ; f2
    psraw            m16, 3                 ; f1
    paddw             m6, m17
    psubw             m7, m16
    vpmulhrsw m16{k4}{z}, m18               ; (f=(f1+1)>>1) & !hev
    psubw            m17, m14, m15          ; 1023 or 4095
    pxor             m18, m18
    paddw             m5, m16
    psubw             m8, m16
    REPX {pminsw x, m17}, m6, m7, m5, m8
    REPX {pmaxsw x, m18}, m6, m7, m5, m8

%if %1 == 16 ; flat16 filter
    vpaddd           m19, m0, [pw_1] {1to16}
    paddw            m16, m1, m2            ; p5+p4
    paddw            m26, m1, m6            ; p5+p0
    paddw            m24, m2, m7            ; p4+q0
    paddw            m16, m4                ; p5+p4+p3
    paddw            m17, m3, m5            ; p2+p1
    psllw            m19, 3
    paddw            m16, m26               ; p5*2+p4+p3+p0
    paddw            m17, m24               ; p4+p2+p1+q0
    psubw            m19, m0                ; p6*7+8
    paddw            m16, m17               ; p5*2+p4*2+p3+p2+p1+q0
    paddw            m18, m3, m8
    paddw            m19, m16               ; p6*7+p5+p4*2+p3+p2+p1+p0+q0
    paddw            m25, m1, m0
    paddw            m16, m0, m0
    psrlw         m1{k3}, m19, 4
    paddw            m19, m18
    psubw            m19, m16               ; +p3+q1-p6*2
    paddw            m16, m2, m0
    psrlw         m2{k3}, m19, 4
    psubw            m19, m25
    paddw            m25, m4, m9
    paddw            m20, m10, m5
    paddw            m19, m25               ; +p2+q2-p6-p5
    paddw            m17, m0, m3
    psubw            m16, m20, m16
    psrlw         m3{k3}, m19, 4
    paddw            m19, m16               ; +p1+q3-p6-p4
    paddw            m16, m11, m6
    psubw            m16, m17
    paddw            m17, m0, m4
    psrlw         m4{k3}, m19, 4
    paddw            m19, m16               ; +p0+q4-p6-p3
    paddw            m16, m22, m7
    psubw            m16, m17
    paddw            m17, m0, m5
    psrlw         m5{k3}, m19, 4
    paddw            m19, m16               ; +q0+q5-p6-p2
    paddw            m16, m23, m8
    psrlw         m6{k3}, m19, 4
    psubw            m16, m17
    paddw            m19, m16               ; +q1+q6-p6-p1
    paddw            m16, m23, m9
    psrlw         m7{k3}, m19, 4
    psubw            m16, m26
    paddw            m19, m16               ; +q2+q6-p5-p0
    paddw            m16, m23, m10
    psrlw         m8{k3}, m19, 4
    psubw            m16, m24
    paddw            m19, m16               ; +q3+q6-p4-p0
    paddw            m16, m23, m11
    psrlw         m9{k3}, m19, 4
    psubw            m16, m18
    paddw            m19, m16               ; +q4+q6-p3-q1
    paddw            m16, m23, m22
    psrlw        m10{k3}, m19, 4
    psubw            m16, m25
    paddw            m19, m16               ; +q5+q6-p2-q2
    paddw            m16, m23, m23
    psrlw        m11{k3}, m19, 4
    psubw            m16, m20
    paddw            m19, m16               ; +q6*2-p1-q3
    psrlw        m22{k3}, m19, 4
%endif
%if %1 == 8 || %1 == 16 ; flat8 filter
    vpbroadcastd     m20, [pw_4096]
    paddw            m16, m3, m4            ; p3+p2
    paddw            m19, m5, m6            ; p1+p0
    paddw            m17, m16, m16          ; 2*(p3+p2)
    paddw            m19, m3                ; p1+p0+p3
    paddw            m17, m7                ; 2*(p3+p2)+q0
    paddw            m19, m17               ; 3*p3+2*p2+p1+p0+q0
    paddw            m18, m4, m7
    pmulhrsw      m4{k2}, m19, m20
    psubw            m19, m16
    paddw            m17, m5, m8
    paddw            m16, m3, m5
    paddw            m19, m17
    pmulhrsw      m5{k2}, m19, m20
    psubw            m19, m16
    paddw            m16, m6, m9
    paddw            m19, m16
    paddw            m16, m3, m6
    pmulhrsw      m6{k2}, m19, m20
    paddw            m19, m10
    psubw            m16, m7, m16
    paddw            m19, m16
    psubw            m16, m10, m18
    pmulhrsw      m7{k2}, m19, m20
    paddw            m16, m8
    paddw            m19, m16
    psubw            m16, m10, m17
    pmulhrsw      m8{k2}, m19, m20
    paddw            m16, m9
    paddw            m19, m16
    pmulhrsw      m9{k2}, m19, m20
%elif %1 == 6 ; flat6 filter
    vpbroadcastd     m10, [pw_4096]
    paddw             m2, m5, m6
    paddw             m0, m4, m7
    paddw             m1, m2, m4            ; p2+p1+p0
    paddw             m3, m4, m4
    paddw             m1, m1
    paddw             m4, m5
    paddw             m1, m0                ; p2+2*(p2+p1+p0)+q0
    psubw             m3, m7, m3
    pmulhrsw      m5{k2}, m1, m10
    paddw             m3, m8
    psubw             m4, m8, m4
    paddw             m1, m3
    pmulhrsw      m6{k2}, m1, m10
    paddw             m4, m9
    paddw             m9, m9
    paddw             m1, m4
    pmulhrsw      m7{k2}, m1, m10
    psubw             m9, m2
    paddw             m1, m9
    pmulhrsw      m8{k2}, m1, m10
%endif

%ifidn %2, v
%if %1 == 16
    mova [tmpq+strideq*2 ], m1              ; p5
    mova [tmpq+stride3q  ], m2              ; p4
    mova [tmpq+strideq*4 ], m3              ; p3
    mova [tmpq+stride5q  ], m4              ; p2
%elif %1 == 8
    mova [tmpq+strideq*1 ], m4              ; p2
%endif
    mova [dstq+mstrideq*2], m5              ; p1
    mova [dstq+mstrideq  ], m6              ; p0
    mova [dstq+strideq*0 ], m7              ; q0
    mova [dstq+strideq*1 ], m8              ; q1
%if %1 == 8 || %1 == 16
    mova [dstq+strideq*2 ], m9              ; q2
%endif
%if %1 == 16
    mova [dstq+stride3q  ], m10             ; q3
    mova [dstq+strideq*4 ], m11             ; q4
    mova [dstq+stride5q  ], m22             ; q5
%endif
%else
%if %1 == 16
    TRANSPOSE8X8W     27,  0,  1,  2,  3,  4,  5,  6, 20
    TRANSPOSE8X8W      7,  8,  9, 10, 11, 22, 23, 28, 20
    mova          [dstq+strideq*0 -16], xm27
    mova          [dstq+strideq*0    ], xm7
    mova          [dstq+strideq*1 -16], xm0
    mova          [dstq+strideq*1    ], xm8
    mova          [dstq+strideq*2 -16], xm1
    mova          [dstq+strideq*2    ], xm9
    mova          [dstq+stride3q  -16], xm2
    mova          [dstq+stride3q     ], xm10
    mova          [dstq+strideq*4 -16], xm3
    mova          [dstq+strideq*4    ], xm11
    mova          [dstq+stride5q  -16], xm4
    mova          [dstq+stride5q     ], xm22
    mova          [dstq+stride3q*2-16], xm5
    mova          [dstq+stride3q*2   ], xm23
    mova          [dstq+stride7q  -16], xm6
    mova          [dstq+stride7q     ], xm28
    lea             dstq, [dstq+strideq*8]
    vextracti128  [dstq+strideq*0 -16], ym27, 1
    vextracti128  [dstq+strideq*0    ], ym7, 1
    vextracti128  [dstq+strideq*1 -16], ym0, 1
    vextracti128  [dstq+strideq*1    ], ym8, 1
    vextracti128  [dstq+strideq*2 -16], ym1, 1
    vextracti128  [dstq+strideq*2    ], ym9, 1
    vextracti128  [dstq+stride3q  -16], ym2, 1
    vextracti128  [dstq+stride3q     ], ym10, 1
    vextracti128  [dstq+strideq*4 -16], ym3, 1
    vextracti128  [dstq+strideq*4    ], ym11, 1
    vextracti128  [dstq+stride5q  -16], ym4, 1
    vextracti128  [dstq+stride5q     ], ym22, 1
    vextracti128  [dstq+stride3q*2-16], ym5, 1
    vextracti128  [dstq+stride3q*2   ], ym23, 1
    vextracti128  [dstq+stride7q  -16], ym6, 1
    vextracti128  [dstq+stride7q     ], ym28, 1
    lea             dstq, [dstq+strideq*8]
    vextracti32x4 [dstq+strideq*0 -16], m27, 2
    vextracti32x4 [dstq+strideq*0    ], m7, 2
    vextracti32x4 [dstq+strideq*1 -16], m0, 2
    vextracti32x4 [dstq+strideq*1    ], m8, 2
    vextracti32x4 [dstq+strideq*2 -16], m1, 2
    vextracti32x4 [dstq+strideq*2    ], m9, 2
    vextracti32x4 [dstq+stride3q  -16], m2, 2
    vextracti32x4 [dstq+stride3q     ], m10, 2
    vextracti32x4 [dstq+strideq*4 -16], m3, 2
    vextracti32x4 [dstq+strideq*4    ], m11, 2
    vextracti32x4 [dstq+stride5q  -16], m4, 2
    vextracti32x4 [dstq+stride5q     ], m22, 2
    vextracti32x4 [dstq+stride3q*2-16], m5, 2
    vextracti32x4 [dstq+stride3q*2   ], m23, 2
    vextracti32x4 [dstq+stride7q  -16], m6, 2
    vextracti32x4 [dstq+stride7q     ], m28, 2
    lea             dstq, [dstq+strideq*8]
    vextracti32x4 [dstq+strideq*0 -16], m27, 3
    vextracti32x4 [dstq+strideq*0    ], m7, 3
    vextracti32x4 [dstq+strideq*1 -16], m0, 3
    vextracti32x4 [dstq+strideq*1    ], m8, 3
    vextracti32x4 [dstq+strideq*2 -16], m1, 3
    vextracti32x4 [dstq+strideq*2    ], m9, 3
    vextracti32x4 [dstq+stride3q  -16], m2, 3
    vextracti32x4 [dstq+stride3q     ], m10, 3
    vextracti32x4 [dstq+strideq*4 -16], m3, 3
    vextracti32x4 [dstq+strideq*4    ], m11, 3
    vextracti32x4 [dstq+stride5q  -16], m4, 3
    vextracti32x4 [dstq+stride5q     ], m22, 3
    vextracti32x4 [dstq+stride3q*2-16], m5, 3
    vextracti32x4 [dstq+stride3q*2   ], m23, 3
    vextracti32x4 [dstq+stride7q  -16], m6, 3
    vextracti32x4 [dstq+stride7q     ], m28, 3
%elif %1 == 8
    TRANSPOSE8X8W      3,  4,  5,  6,  7,  8,  9, 10,  2
    movu   [dstq+strideq*0 ], xm3
    movu   [dstq+strideq*1 ], xm4
    movu   [dstq+strideq*2 ], xm5
    movu   [dstq+stride3q  ], xm6
    movu   [dstq+strideq*4 ], xm7
    movu   [dstq+stride5q  ], xm8
    movu   [dstq+stride3q*2], xm9
    movu   [dstq+stride7q  ], xm10
    lea             dstq, [dstq+strideq*8]
    vextracti128  [dstq+strideq*0 ], ym3, 1
    vextracti128  [dstq+strideq*1 ], ym4, 1
    vextracti128  [dstq+strideq*2 ], ym5, 1
    vextracti128  [dstq+stride3q  ], ym6, 1
    vextracti128  [dstq+strideq*4 ], ym7, 1
    vextracti128  [dstq+stride5q  ], ym8, 1
    vextracti128  [dstq+stride3q*2], ym9, 1
    vextracti128  [dstq+stride7q  ], ym10, 1
    lea             dstq, [dstq+strideq*8]
    vextracti32x4 [dstq+strideq*0 ], m3, 2
    vextracti32x4 [dstq+strideq*1 ], m4, 2
    vextracti32x4 [dstq+strideq*2 ], m5, 2
    vextracti32x4 [dstq+stride3q  ], m6, 2
    vextracti32x4 [dstq+strideq*4 ], m7, 2
    vextracti32x4 [dstq+stride5q  ], m8, 2
    vextracti32x4 [dstq+stride3q*2], m9, 2
    vextracti32x4 [dstq+stride7q  ], m10, 2
    lea             dstq, [dstq+strideq*8]
    vextracti32x4 [dstq+strideq*0 ], m3, 3
    vextracti32x4 [dstq+strideq*1 ], m4, 3
    vextracti32x4 [dstq+strideq*2 ], m5, 3
    vextracti32x4 [dstq+stride3q  ], m6, 3
    vextracti32x4 [dstq+strideq*4 ], m7, 3
    vextracti32x4 [dstq+stride5q  ], m8, 3
    vextracti32x4 [dstq+stride3q*2], m9, 3
    vextracti32x4 [dstq+stride7q  ], m10, 3
    lea             dstq, [dstq+strideq*8+8]
%else ; %1 == 4 || %1 == 6
    punpcklwd         m9, m5, m6
    punpckhwd         m5, m6
    kxnorb            k1, k1, k1
    punpcklwd         m6, m7, m8
    punpckhwd         m7, m8
    kmovb             k2, k1
    punpckldq         m8, m9, m6
    vpscatterdq [dstq+ym12-4]{k1}, m8
    punpckhdq         m9, m6
    lea             tmpq, [dstq+strideq*2-4]
    kmovb             k1, k2
    vpscatterdq [tmpq+ym12]{k2}, m9
    punpckldq         m6, m5, m7
    lea             tmpq, [tmpq+strideq*2]
    kmovb             k2, k1
    vpscatterdq [tmpq+ym12]{k1}, m6
    punpckhdq         m5, m7
    lea             tmpq, [tmpq+strideq*2]
    vpscatterdq [tmpq+ym12]{k2}, m5
%endif
%endif
%endmacro

INIT_ZMM avx512icl
cglobal lpf_v_sb_y_16bpc, 6, 12, 26, dst, stride, mask, l, l_stride, \
                                     lut, w, stride3, mstride, tmp, \
                                     mask_bits, stride5
%define base tmpq-filter_mask
    SWAP              12, 26                ; avoids clobbering xmm10 on WIN64
    lea             tmpq, [filter_mask]
    mov              r6d, r7m               ; bitdepth_max
    lea         stride3q, [strideq*3]
    shl        l_strideq, 2
    lea         stride5q, [strideq*5]
    shr              r6d, 11                ; is_12bpc
    mova            ym21, [base+filter_mask]
    mov         mstrideq, strideq
    vpbroadcastd     m13, [base+pw_4+r6*8]
    mov       mask_bitsd, 0xff
    vpbroadcastd     m14, [base+clip_max+r6*4]
    sub               lq, l_strideq
    vpbroadcastd     m15, [base+clip_min+r6*4]
    neg         mstrideq
    mov               wd, wm
.loop:
    test       [maskq+8], mask_bitsd        ; vmask[2]
    jz .no_flat16
    FILTER            16, v
    jmp .end
.no_flat16:
    test       [maskq+4], mask_bitsd        ; vmask[1]
    jz .no_flat
    FILTER             8, v
    jmp .end
.no_flat:
    test       [maskq+0], mask_bitsd        ; vmask[0]
    jz .end
    call .v4
.end:
    shl       mask_bitsd, 8
    add             dstq, 64
    pslld           ym21, 8
    add               lq, 32
    sub               wd, 8
    jg .loop
    RET
ALIGN function_align
.v4: ; called by both luma and chroma
    FILTER             4, v
    ret

cglobal lpf_h_sb_y_16bpc, 6, 13, 29, dst, stride, mask, l, l_stride, \
                                     lut, h, stride3, l_stride3, tmp, \
                                     mask_bits, stride5, stride7
    lea             tmpq, [filter_mask]
    mov              r6d, r7m               ; bitdepth_max
    lea         stride3q, [strideq*3]
    vpbroadcastd    ym12, strided
    shl        l_strideq, 2
    lea         stride5q, [strideq*5]
    shr              r6d, 11                ; is_12bpc
    pmulld          ym12, [base+stride_mul]
    lea         stride7q, [strideq+stride3q*2]
    mova            ym21, [base+filter_mask]
    mov       mask_bitsd, 0xff
    vpbroadcastd     m13, [base+pw_4+r6*8]
    sub               lq, 4
    vpbroadcastd     m14, [base+clip_max+r6*4]
    lea       l_stride3q, [l_strideq*3]
    vpbroadcastd     m15, [base+clip_min+r6*4]
    mov               hd, hm
.loop:
    test       [maskq+8], mask_bitsd        ; vmask[2]
    jz .no_flat16
    FILTER            16, h
    jmp .end
.no_flat16:
    test       [maskq+4], mask_bitsd        ; vmask[1]
    jz .no_flat
    FILTER             8, h
    jmp .end2
.no_flat:
    test       [maskq+0], mask_bitsd        ; vmask[0]
    jz .no_filter
    call .h4
.no_filter:
    lea             dstq, [dstq+stride3q*8]
.end:
    lea             dstq, [dstq+strideq*8]
.end2:
    shl       mask_bitsd, 8
    pslld           ym21, 8
    lea               lq, [lq+l_strideq*8]
    sub               hd, 8
    jg .loop
    RET
ALIGN function_align
.h4: ; called by both luma and chroma
    FILTER         4, h
    ret

cglobal lpf_v_sb_uv_16bpc, 6, 11, 22, dst, stride, mask, l, l_stride, lut, \
                                      w, stride3, mstride, tmp, mask_bits
    lea             tmpq, [filter_mask]
    mov              r6d, r7m               ; bitdepth_max
    shl        l_strideq, 2
    lea         stride3q, [strideq*3]
    shr              r6d, 11                ; is_12bpc
    mova            ym21, [base+filter_mask]
    mov        mstrideq, strideq
    vpbroadcastd     m13, [base+pw_4+r6*8]
    mov       mask_bitsd, 0xff
    vpbroadcastd     m14, [base+clip_max+r6*4]
    sub               lq, l_strideq
    vpbroadcastd     m15, [base+clip_min+r6*4]
    neg         mstrideq
    mov               wd, wm
.loop:
    test       [maskq+4], mask_bitsd        ; vmask[1]
    jz .no_flat
    FILTER             6, v
    jmp .end
.no_flat:
    test       [maskq+0], mask_bitsd        ; vmask[0]
    jz .end
    call mangle(private_prefix %+ _lpf_v_sb_y_16bpc_avx512icl).v4
.end:
    shl       mask_bitsd, 8
    add             dstq, 64
    pslld           ym21, 8
    add               lq, 32
    sub               wd, 8
    jg .loop
    RET

cglobal lpf_h_sb_uv_16bpc, 6, 11, 22, dst, stride, mask, l, l_stride, lut, \
                                      h, stride3, l_stride3, tmp, mask_bits
    lea             tmpq, [filter_mask]
    mov              r6d, r7m               ; bitdepth_max
    vpbroadcastd    ym12, strided
    shl        l_strideq, 2
    shr              r6d, 11                ; is_12bpc
    pmulld          ym12, [base+stride_mul]
    lea         stride3q, [strideq*3]
    mova            ym21, [base+filter_mask]
    mov       mask_bitsd, 0xff
    vpbroadcastd     m13, [base+pw_4+r6*8]
    sub               lq, 4
    vpbroadcastd     m14, [base+clip_max+r6*4]
    lea       l_stride3q, [l_strideq*3]
    vpbroadcastd     m15, [base+clip_min+r6*4]
    mov               hd, hm
.loop:
    test       [maskq+4], mask_bitsd        ; vmask[1]
    jz .no_flat
    FILTER             6, h
    jmp .end
.no_flat:
    test       [maskq+0], mask_bitsd        ; vmask[0]
    jz .end
    call mangle(private_prefix %+ _lpf_h_sb_y_16bpc_avx512icl).h4
.end:
    lea             tmpq, [strideq+stride3q]
    shl       mask_bitsd, 8
    pslld           ym21, 8
    lea             dstq, [dstq+tmpq*8]
    lea               lq, [lq+l_strideq*8]
    sub               hd, 8
    jg .loop
    RET

%endif ; ARCH_X86_64
