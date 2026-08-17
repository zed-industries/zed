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

SECTION_RODATA 64

spel_h_shufA:  db  0,  1,  2,  3,  2,  3,  4,  5,  4,  5,  6,  7,  6,  7,  8,  9
               db 32, 33, 34, 35, 34, 35, 36, 37, 36, 37, 38, 39, 38, 39, 40, 41
spel_h_shufC:  db  8,  9, 10, 11, 10, 11, 12, 13, 12, 13, 14, 15, 14, 15, 16, 17
               db 40, 41, 42, 43, 42, 43, 44, 45, 44, 45, 46, 47, 46, 47, 48, 49
               db 16, 17, 18, 19, 18, 19, 20, 21, 20, 21, 22, 23, 22, 23, 24, 25
               db 48, 49, 50, 51, 50, 51, 52, 53, 52, 53, 54, 55, 54, 55, 56, 57
spel_h_shufB:  db  4,  5,  6,  7,  6,  7,  8,  9,  8,  9, 10, 11, 10, 11, 12, 13
               db 36, 37, 38, 39, 38, 39, 40, 41, 40, 41, 42, 43, 42, 43, 44, 45
spel_h_shufD:  db 12, 13, 14, 15, 14, 15, 16, 17, 16, 17, 18, 19, 18, 19, 20, 21
               db 44, 45, 46, 47, 46, 47, 48, 49, 48, 49, 50, 51, 50, 51, 52, 53
               db 20, 21, 22, 23, 22, 23, 24, 25, 24, 25, 26, 27, 26, 27, 28, 29
               db 52, 53, 54, 55, 54, 55, 56, 57, 56, 57, 58, 59, 58, 59, 60, 61
spel_v_shuf8:  db  0,  1, 16, 17,  2,  3, 18, 19,  4,  5, 20, 21,  6,  7, 22, 23
               db 16, 17, 32, 33, 18, 19, 34, 35, 20, 21, 36, 37, 22, 23, 38, 39
               db  8,  9, 24, 25, 10, 11, 26, 27, 12, 13, 28, 29, 14, 15, 30, 31
               db 24, 25, 40, 41, 26, 27, 42, 43, 28, 29, 44, 45, 30, 31, 46, 47
spel_v_shuf16: db  0,  1, 32, 33,  2,  3, 34, 35,  4,  5, 36, 37,  6,  7, 38, 39
               db  8,  9, 40, 41, 10, 11, 42, 43, 12, 13, 44, 45, 14, 15, 46, 47
               db 16, 17, 48, 49, 18, 19, 50, 51, 20, 21, 52, 53, 22, 23, 54, 55
               db 24, 25, 56, 57, 26, 27, 58, 59, 28, 29, 60, 61, 30, 31, 62, 63
prep_endA:     db  1,  2,  5,  6,  9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30
               db 33, 34, 37, 38, 41, 42, 45, 46, 49, 50, 53, 54, 57, 58, 61, 62
               db 65, 66, 69, 70, 73, 74, 77, 78, 81, 82, 85, 86, 89, 90, 93, 94
               db 97, 98,101,102,105,106,109,110,113,114,117,118,121,122,125,126
prep_endB:     db  1,  2,  5,  6,  9, 10, 13, 14, 33, 34, 37, 38, 41, 42, 45, 46
               db 17, 18, 21, 22, 25, 26, 29, 30, 49, 50, 53, 54, 57, 58, 61, 62
               db 65, 66, 69, 70, 73, 74, 77, 78, 97, 98,101,102,105,106,109,110
               db 81, 82, 85, 86, 89, 90, 93, 94,113,114,117,118,121,122,125,126
prep_endC:     db  1,  2,  5,  6,  9, 10, 13, 14, 65, 66, 69, 70, 73, 74, 77, 78
               db 17, 18, 21, 22, 25, 26, 29, 30, 81, 82, 85, 86, 89, 90, 93, 94
               db 33, 34, 37, 38, 41, 42, 45, 46, 97, 98,101,102,105,106,109,110
               db 49, 50, 53, 54, 57, 58, 61, 62,113,114,117,118,121,122,125,126
spel_shuf4a:   db  1,  2, 17, 18,  5,  6, 21, 22,  9, 10, 25, 26, 13, 14, 29, 30
               db 17, 18, 33, 34, 21, 22, 37, 38, 25, 26, 41, 42, 29, 30, 45, 46
spel_shuf4b:   db 18, 19, 33, 34, 22, 23, 37, 38, 26, 27, 41, 42, 30, 31, 45, 46
               db 33, 34, 49, 50, 37, 38, 53, 54, 41, 42, 57, 58, 45, 46, 61, 62
spel_shuf8a:   db  1,  2, 17, 18,  5,  6, 21, 22,  9, 10, 25, 26, 13, 14, 29, 30
               db 17, 18, 65, 66, 21, 22, 69, 70, 25, 26, 73, 74, 29, 30, 77, 78
               db 33, 34, 49, 50, 37, 38, 53, 54, 41, 42, 57, 58, 45, 46, 61, 62
               db 49, 50, 97, 98, 53, 54,101,102, 57, 58,105,106, 61, 62,109,110
spel_shuf8b:   db 18, 19, 65, 66, 22, 23, 69, 70, 26, 27, 73, 74, 30, 31, 77, 78
               db 65, 66, 81, 82, 69, 70, 85, 86, 73, 74, 89, 90, 77, 78, 93, 94
               db 50, 51, 97, 98, 54, 55,101,102, 58, 59,105,106, 62, 63,109,110
               db 97, 98,113,114,101,102,117,118,105,106,121,122,109,110,125,126
spel_shuf16:   db  1,  2, 33, 34,  5,  6, 37, 38,  9, 10, 41, 42, 13, 14, 45, 46
               db 17, 18, 49, 50, 21, 22, 53, 54, 25, 26, 57, 58, 29, 30, 61, 62
               db 65, 66, 97, 98, 69, 70,101,102, 73, 74,105,106, 77, 78,109,110
               db 81, 82,113,114, 85, 86,117,118, 89, 90,121,122, 93, 94,125,126
spel_shuf32:   db  1,  2, 65, 66,  5,  6, 69, 70,  9, 10, 73, 74, 13, 14, 77, 78
               db 17, 18, 81, 82, 21, 22, 85, 86, 25, 26, 89, 90, 29, 30, 93, 94
               db 33, 34, 97, 98, 37, 38,101,102, 41, 42,105,106, 45, 46,109,110
               db 49, 50,113,114, 53, 54,117,118, 57, 58,121,122, 61, 62,125,126
spel_h_shuf2b: db  1,  2, 17, 18,  5,  6, 21, 22, 17, 18, 33, 34, 21, 22, 37, 38
               db 33, 34, 49, 50, 37, 38, 53, 54, 49, 50,  9, 10, 53, 54, 13, 14
               db  9, 10, 25, 26, 13, 14, 29, 30, 25, 26, 41, 42, 29, 30, 45, 46
spel_shuf2:    db 10, 11, 17, 18, 14, 15, 21, 22, 17, 18, 25, 26, 21, 22, 29, 30
spel_h_shuf2a: db  0,  1,  2,  3,  2,  3,  4,  5, 16, 17, 18, 19, 18, 19, 20, 21
               db  4,  5,  6,  7,  6,  7,  8,  9, 20, 21, 22, 23, 22, 23, 24, 25
w_mask_end42x: db  1,  5,  9, 13, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61
               db 65, 69, 73, 77, 81, 85, 89, 93, 97,101,105,109,113,117,121,125
w_mask_end444: db  0,  2,  4,  6,  8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30
               db 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62
               db 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94
               db 96, 98,100,102,104,106,108,110,112,114,116,118,120,122,124,126
w_mask_shuf4:  db  0,  2,  8, 10,  4,  6, 12, 14, 16, 18, 24, 26, 20, 22, 28, 30
               db 32, 34, 40, 42, 36, 38, 44, 46, 48, 50, 56, 58, 52, 54, 60, 62
               db 64, 66, 72, 74, 68, 70, 76, 78, 80, 82, 88, 90, 84, 86, 92, 94
               db 96, 98,104,106,100,102,108,110,112,114,120,122,116,118,124,126
w_mask_shuf8:  db  0,  2, 16, 18,  4,  6, 20, 22,  8, 10, 24, 26, 12, 14, 28, 30
               db 32, 34, 48, 50, 36, 38, 52, 54, 40, 42, 56, 58, 44, 46, 60, 62
               db 64, 66, 80, 82, 68, 70, 84, 86, 72, 74, 88, 90, 76, 78, 92, 94
               db 96, 98,112,114,100,102,116,118,104,106,120,122,108,110,124,126
w_mask_shuf16: db  0,  2, 32, 34,  4,  6, 36, 38,  8, 10, 40, 42, 12, 14, 44, 46
               db 16, 18, 48, 50, 20, 22, 52, 54, 24, 26, 56, 58, 28, 30, 60, 62
               db 64, 66, 96, 98, 68, 70,100,102, 72, 74,104,106, 76, 78,108,110
               db 80, 82,112,114, 84, 86,116,118, 88, 90,120,122, 92, 94,124,126
warp8x8_permA: db  0,  1,  2,  3, 32, 33, 34, 35,  2,  3,  4,  5, 34, 35, 36, 37
               db  4,  5,  6,  7, 36, 37, 38, 39,  6,  7,  8,  9, 38, 39, 40, 41
               db  8,  9, 10, 11, 40, 41, 42, 43, 10, 11, 12, 13, 42, 43, 44, 45
               db 12, 13, 14, 15, 44, 45, 46, 47, 14, 15, 16, 17, 46, 47, 48, 49
warp8x8_permB: db 12, 13, 14, 15, 44, 45, 46, 47, 14, 15, 16, 17, 46, 47, 48, 49
               db 16, 17, 18, 19, 48, 49, 50, 51, 18, 19, 20, 21, 50, 51, 52, 53
               db 20, 21, 22, 23, 52, 53, 54, 55, 22, 23, 24, 25, 54, 55, 56, 57
               db 24, 25, 26, 27, 56, 57, 58, 59, 26, 27, 28, 29, 58, 59, 60, 61
warp8x8_end:   db  0,  1,  4,  5, 16, 17, 20, 21, 32, 33, 36, 37, 48, 49, 52, 53
               db  2,  3,  6,  7, 18, 19, 22, 23, 34, 35, 38, 39, 50, 51, 54, 55
               db  8,  9, 12, 13, 24, 25, 28, 29, 40, 41, 44, 45, 56, 57, 60, 61
               db 10, 11, 14, 15, 26, 27, 30, 31, 42, 43, 46, 47, 58, 59, 62, 63
deint_q_shuf: ;dq  0,  2,  4,  6,  1,  3,  5,  7
pd_0to7:       dd  0,  1,  2,  3,  4,  5,  6,  7
               dd  1
pw_2048:       times 2 dw 2048
               dd  3
pw_8192:       times 2 dw 8192
avg_shift:     dw  5,  5,  3,  3
pw_27615:      times 2 dw 27615
pw_32766:      times 2 dw 32766
warp8x8_permC: db -1,  0, -1,  1, -1,  8, -1,  9, -1,  4, -1,  5, -1, 12, -1, 13
warp8x8_permD: db -1,  2, -1,  3, -1, 10, -1, 11, -1,  6, -1,  7, -1, 14, -1, 15
warp_shift_h:  db 11, 19, 11, 19, 43, 51, 43, 51, 13, 21, 13, 21, 45, 53, 45, 53
blend_shuf:    db  0,  1,  0,  1,  0,  1,  0,  1,  2,  3,  2,  3,  2,  3,  2,  3
resize_permA:  dd  0,  4,  8, 12,  1,  5,  9, 13, 16, 20, 24, 28, 17, 21, 25, 29
resize_permB:  dd  2,  6, 10, 14,  3,  7, 11, 15, 18, 22, 26, 30, 19, 23, 27, 31
resize_permC:  dq  0,  1,  4,  5,  8,  9, 12, 13
resize_permD:  dq  2,  3,  6,  7, 10, 11, 14, 15
resize_permE:  dq  0,  2,  4,  6
resize_shufA:  db -1,  0, -1,  1, -1,  4, -1,  5, -1,  8, -1,  9, -1, 12, -1, 13
resize_shufB:  db -1,  2, -1,  3, -1,  6, -1,  7, -1, 10, -1, 11, -1, 14, -1, 15
rescale_mul:   dd  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15
resize_shuf:   db  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  2,  3,  4,  5,  6,  7
               db  8,  9, 10, 11, 12, 13, 14, 15, 14, 15, 14, 15, 14, 15, 14, 15

prep_hv_shift:    dq  6,  4
put_bilin_h_rnd:  dw  8,  8, 10, 10
prep_mul:         dw 16, 16,  4,  4
put_8tap_h_rnd:   dd 34, 40
prep_8tap_rnd:    dd 128 - (8192 << 8)
warp_8x8_rnd_h:   dd 512, 2048
warp_8x8_rnd_v:   dd 262144, 65536
warp_8x8t_rnd_v:  dd 16384 - (8192 << 15)
avg_round:        dw -16400, -16400, -16388, -16388
w_avg_round:      dd 128 + (8192 << 4),  32 + (8192 << 4)
mask_round:       dd 512 + (8192 << 6), 128 + (8192 << 6)
w_mask_round:     dd 128, 64
bidir_shift:      dw  6,  6,  4,  4

pb_64:    times 4 db 64
pw_m512:  times 2 dw -512
pw_2:     times 2 dw 2
pw_64:    times 2 dw 64
pd_32:    dd 32
pd_63:    dd 63
pd_128:   dd 128
pd_640:   dd 640
pd_2176:  dd 2176
pd_16384: dd 16384
pd_0_4:   dd 0, 4

%define pw_16 prep_mul
%define pd_512 warp_8x8_rnd_h

%macro BASE_JMP_TABLE 3-*
    %xdefine %1_%2_table (%%table - %3)
    %xdefine %%base %1_%2
    %%table:
    %rep %0 - 2
        dw %%base %+ _w%3 - %%base
        %rotate 1
    %endrep
%endmacro

%macro HV_JMP_TABLE 5-*
    %xdefine %%prefix mangle(private_prefix %+ _%1_%2_16bpc_%3)
    %xdefine %%base %1_%3
    %assign %%types %4
    %if %%types & 1
        %xdefine %1_%2_h_%3_table  (%%h  - %5)
        %%h:
        %rep %0 - 4
            dw %%prefix %+ .h_w%5 - %%base
            %rotate 1
        %endrep
        %rotate 4
    %endif
    %if %%types & 2
        %xdefine %1_%2_v_%3_table  (%%v  - %5)
        %%v:
        %rep %0 - 4
            dw %%prefix %+ .v_w%5 - %%base
            %rotate 1
        %endrep
        %rotate 4
    %endif
    %if %%types & 4
        %xdefine %1_%2_hv_%3_table (%%hv - %5)
        %%hv:
        %rep %0 - 4
            dw %%prefix %+ .hv_w%5 - %%base
            %rotate 1
        %endrep
    %endif
%endmacro

%macro BIDIR_JMP_TABLE 2-*
    %xdefine %1_%2_table (%%table - 2*%3)
    %xdefine %%base %1_%2_table
    %xdefine %%prefix mangle(private_prefix %+ _%1_16bpc_%2)
    %%table:
    %rep %0 - 2
        dd %%prefix %+ .w%3 - %%base
        %rotate 1
    %endrep
%endmacro

%xdefine put_avx512icl mangle(private_prefix %+ _put_bilin_16bpc_avx512icl.put)
%xdefine prep_avx512icl mangle(private_prefix %+ _prep_bilin_16bpc_avx512icl.prep)

BIDIR_JMP_TABLE avg,        avx512icl,       4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_avg,      avx512icl,       4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE mask,       avx512icl,       4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_420, avx512icl,       4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_422, avx512icl,       4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_444, avx512icl,       4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE blend,      avx512icl,       4, 8, 16, 32
BIDIR_JMP_TABLE blend_v,    avx512icl,    2, 4, 8, 16, 32
BIDIR_JMP_TABLE blend_h,    avx512icl,    2, 4, 8, 16, 32, 64, 128
BASE_JMP_TABLE put,         avx512icl,    2, 4, 8, 16, 32, 64, 128
BASE_JMP_TABLE prep,        avx512icl,       4, 8, 16, 32, 64, 128
HV_JMP_TABLE   put,  bilin, avx512icl, 7, 2, 4, 8, 16, 32, 64, 128
HV_JMP_TABLE   prep, bilin, avx512icl, 7,    4, 8, 16, 32, 64, 128
HV_JMP_TABLE   put,  8tap,  avx512icl, 2, 2, 4, 8, 16, 32, 64, 128
HV_JMP_TABLE   prep, 8tap,  avx512icl, 2,    4, 8, 16, 32, 64, 128

%define table_offset(type, fn) type %+ fn %+ SUFFIX %+ _table - type %+ SUFFIX

cextern mc_subpel_filters
%define subpel_filters (mangle(private_prefix %+ _mc_subpel_filters)-8)

cextern mc_warp_filter
cextern obmc_masks_avx2
cextern resize_filter

SECTION .text

%if WIN64
DECLARE_REG_TMP 4
%else
DECLARE_REG_TMP 8
%endif

INIT_ZMM avx512icl
cglobal put_bilin_16bpc, 4, 8, 13, dst, ds, src, ss, w, h, mxy
    mov                mxyd, r6m ; mx
    lea                  r7, [put_avx512icl]
    tzcnt               t0d, wm
    movifnidn            hd, hm
    test               mxyd, mxyd
    jnz .h
    mov                mxyd, r7m ; my
    test               mxyd, mxyd
    jnz .v
.put:
    movzx               t0d, word [r7+t0*2+table_offset(put,)]
    add                  t0, r7
    jmp                  t0
.put_w2:
    mov                 r6d, [srcq+ssq*0]
    mov                 r7d, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r6d
    mov        [dstq+dsq*1], r7d
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w2
    RET
.put_w4:
    mov                  r6, [srcq+ssq*0]
    mov                  r7, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r6
    mov        [dstq+dsq*1], r7
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w4
    RET
.put_w8:
    movu               xmm0, [srcq+ssq*0]
    movu               xmm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova       [dstq+dsq*0], xmm0
    mova       [dstq+dsq*1], xmm1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w8
    RET
.put_w16:
    movu                ym0, [srcq+ssq*0]
    movu                ym1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova       [dstq+dsq*0], ym0
    mova       [dstq+dsq*1], ym1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w16
    RET
.put_w32:
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w32
    RET
.put_w64:
    movu                 m0, [srcq+ssq*0+64*0]
    movu                 m1, [srcq+ssq*0+64*1]
    movu                 m2, [srcq+ssq*1+64*0]
    movu                 m3, [srcq+ssq*1+64*1]
    lea                srcq, [srcq+ssq*2]
    mova  [dstq+dsq*0+64*0], m0
    mova  [dstq+dsq*0+64*1], m1
    mova  [dstq+dsq*1+64*0], m2
    mova  [dstq+dsq*1+64*1], m3
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w64
    RET
.put_w128:
    movu                 m0, [srcq+64*0]
    movu                 m1, [srcq+64*1]
    movu                 m2, [srcq+64*2]
    movu                 m3, [srcq+64*3]
    add                srcq, ssq
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    mova        [dstq+64*2], m2
    mova        [dstq+64*3], m3
    add                dstq, dsq
    dec                  hd
    jg .put_w128
    RET
.h:
    vpbroadcastw         m5, mxyd
    mov                mxyd, r7m ; my
    vpbroadcastd         m4, [pw_16]
    psubw                m4, m5
    test               mxyd, mxyd
    jnz .hv
    ; 12-bit is rounded twice so we can't use the same pmulhrsw approach as .v
    movzx               t0d, word [r7+t0*2+table_offset(put, _bilin_h)]
    mov                 r6d, r8m ; bitdepth_max
    add                  t0, r7
    shr                 r6d, 11
    vpbroadcastd         m6, [r7-put_avx512icl+put_bilin_h_rnd+r6*4]
    jmp                  t0
.h_w2:
    movq               xmm1, [srcq+ssq*0]
    movhps             xmm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmullw             xmm0, xmm1, xm4
    psrlq              xmm1, 16
    pmullw             xmm1, xm5
    paddw              xmm0, xm6
    paddw              xmm0, xmm1
    psrlw              xmm0, 4
    movd       [dstq+dsq*0], xmm0
    pextrd     [dstq+dsq*1], xmm0, 2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w2
    RET
.h_w4:
    movq               xmm0, [srcq+ssq*0+0]
    movhps             xmm0, [srcq+ssq*1+0]
    movq               xmm1, [srcq+ssq*0+2]
    movhps             xmm1, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    pmullw             xmm0, xm4
    pmullw             xmm1, xm5
    paddw              xmm0, xm6
    paddw              xmm0, xmm1
    psrlw              xmm0, 4
    movq       [dstq+dsq*0], xmm0
    movhps     [dstq+dsq*1], xmm0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w4
    RET
.h_w8:
    movu                xm0, [srcq+ssq*0+0]
    vinserti32x4        ym0, [srcq+ssq*1+0], 1
    movu                xm1, [srcq+ssq*0+2]
    vinserti32x4        ym1, [srcq+ssq*1+2], 1
    lea                srcq, [srcq+ssq*2]
    pmullw              ym0, ym4
    pmullw              ym1, ym5
    paddw               ym0, ym6
    paddw               ym0, ym1
    psrlw               ym0, 4
    mova          [dstq+dsq*0], xm0
    vextracti32x4 [dstq+dsq*1], ym0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    movu                ym0, [srcq+ssq*0+0]
    vinserti32x8         m0, [srcq+ssq*1+0], 1
    movu                ym1, [srcq+ssq*0+2]
    vinserti32x8         m1, [srcq+ssq*1+2], 1
    lea                srcq, [srcq+ssq*2]
    pmullw               m0, m4
    pmullw               m1, m5
    paddw                m0, m6
    paddw                m0, m1
    psrlw                m0, 4
    mova          [dstq+dsq*0], ym0
    vextracti32x8 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w16
    RET
.h_w32:
    pmullw               m0, m4, [srcq+ssq*0+0]
    pmullw               m2, m5, [srcq+ssq*0+2]
    pmullw               m1, m4, [srcq+ssq*1+0]
    pmullw               m3, m5, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    paddw                m0, m6
    paddw                m1, m6
    paddw                m0, m2
    paddw                m1, m3
    psrlw                m0, 4
    psrlw                m1, 4
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w32
    RET
.h_w64:
    pmullw               m0, m4, [srcq+64*0+0]
    pmullw               m2, m5, [srcq+64*0+2]
    pmullw               m1, m4, [srcq+64*1+0]
    pmullw               m3, m5, [srcq+64*1+2]
    add                srcq, ssq
    paddw                m0, m6
    paddw                m1, m6
    paddw                m0, m2
    paddw                m1, m3
    psrlw                m0, 4
    psrlw                m1, 4
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    add                dstq, dsq
    dec                  hd
    jg .h_w64
    RET
.h_w128:
    pmullw               m0, m4, [srcq+64*0+0]
    pmullw               m7, m5, [srcq+64*0+2]
    pmullw               m1, m4, [srcq+64*1+0]
    pmullw               m8, m5, [srcq+64*1+2]
    pmullw               m2, m4, [srcq+64*2+0]
    pmullw               m9, m5, [srcq+64*2+2]
    pmullw               m3, m4, [srcq+64*3+0]
    pmullw              m10, m5, [srcq+64*3+2]
    add                srcq, ssq
    REPX      {paddw x, m6}, m0, m1, m2, m3
    paddw                m0, m7
    paddw                m1, m8
    paddw                m2, m9
    paddw                m3, m10
    REPX       {psrlw x, 4}, m0, m1, m2, m3
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    mova        [dstq+64*2], m2
    mova        [dstq+64*3], m3
    add                dstq, dsq
    dec                  hd
    jg .h_w128
    RET
.v:
    movzx               t0d, word [r7+t0*2+table_offset(put, _bilin_v)]
    shl                mxyd, 11
    vpbroadcastw         m8, mxyd
    add                  t0, r7
    jmp                  t0
.v_w2:
    movd               xmm0, [srcq+ssq*0]
.v_w2_loop:
    movd               xmm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpckldq          xmm2, xmm0, xmm1
    movd               xmm0, [srcq+ssq*0]
    punpckldq          xmm1, xmm0
    psubw              xmm1, xmm2
    pmulhrsw           xmm1, xm8
    paddw              xmm1, xmm2
    movd       [dstq+dsq*0], xmm1
    pextrd     [dstq+dsq*1], xmm1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
    movq               xmm0, [srcq+ssq*0]
.v_w4_loop:
    movq               xmm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklqdq         xmm2, xmm0, xmm1
    movq               xmm0, [srcq+ssq*0]
    punpcklqdq         xmm1, xmm0
    psubw              xmm1, xmm2
    pmulhrsw           xmm1, xm8
    paddw              xmm1, xmm2
    movq       [dstq+dsq*0], xmm1
    movhps     [dstq+dsq*1], xmm1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
    RET
.v_w8:
    movu               xmm0, [srcq+ssq*0]
.v_w8_loop:
    vbroadcasti128     ymm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpblendd           ymm2, ymm0, ymm1, 0xf0
    vbroadcasti128     ymm0, [srcq+ssq*0]
    vpblendd           ymm1, ymm0, 0xf0
    psubw              ymm1, ymm2
    pmulhrsw           ymm1, ym8
    paddw              ymm1, ymm2
    mova         [dstq+dsq*0], xmm1
    vextracti128 [dstq+dsq*1], ymm1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w8_loop
    vzeroupper
    RET
.v_w16:
    movu                ym0, [srcq+ssq*0]
.v_w16_loop:
    movu                ym3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    psubw               ym1, ym3, ym0
    pmulhrsw            ym1, ym8
    paddw               ym1, ym0
    movu                ym0, [srcq+ssq*0]
    psubw               ym2, ym0, ym3
    pmulhrsw            ym2, ym8
    paddw               ym2, ym3
    mova       [dstq+dsq*0], ym1
    mova       [dstq+dsq*1], ym2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w16_loop
    RET
.v_w32:
    movu                 m0, [srcq+ssq*0]
.v_w32_loop:
    movu                 m3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    psubw                m1, m3, m0
    pmulhrsw             m1, m8
    paddw                m1, m0
    movu                 m0, [srcq+ssq*0]
    psubw                m2, m0, m3
    pmulhrsw             m2, m8
    paddw                m2, m3
    mova       [dstq+dsq*0], m1
    mova       [dstq+dsq*1], m2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w32_loop
    RET
.v_w64:
    movu                 m0, [srcq+ssq*0+64*0]
    movu                 m1, [srcq+ssq*0+64*1]
.v_w64_loop:
    movu                 m2, [srcq+ssq*1+64*0]
    movu                 m3, [srcq+ssq*1+64*1]
    lea                srcq, [srcq+ssq*2]
    psubw                m4, m2, m0
    pmulhrsw             m4, m8
    paddw                m4, m0
    movu                 m0, [srcq+ssq*0+64*0]
    psubw                m5, m3, m1
    pmulhrsw             m5, m8
    paddw                m5, m1
    movu                 m1, [srcq+ssq*0+64*1]
    psubw                m6, m0, m2
    pmulhrsw             m6, m8
    psubw                m7, m1, m3
    pmulhrsw             m7, m8
    mova  [dstq+dsq*0+64*0], m4
    mova  [dstq+dsq*0+64*1], m5
    paddw                m6, m2
    paddw                m7, m3
    mova  [dstq+dsq*1+64*0], m6
    mova  [dstq+dsq*1+64*1], m7
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w64_loop
    RET
.v_w128:
    movu                 m0, [srcq+ssq*0+64*0]
    movu                 m1, [srcq+ssq*0+64*1]
    movu                 m2, [srcq+ssq*0+64*2]
    movu                 m3, [srcq+ssq*0+64*3]
.v_w128_loop:
    movu                 m4, [srcq+ssq*1+64*0]
    movu                 m5, [srcq+ssq*1+64*1]
    movu                 m6, [srcq+ssq*1+64*2]
    movu                 m7, [srcq+ssq*1+64*3]
    lea                srcq, [srcq+ssq*2]
    psubw                m9, m4, m0
    pmulhrsw             m9, m8
    paddw                m9, m0
    movu                 m0, [srcq+ssq*0+64*0]
    psubw               m10, m5, m1
    pmulhrsw            m10, m8
    paddw               m10, m1
    movu                 m1, [srcq+ssq*0+64*1]
    psubw               m11, m6, m2
    pmulhrsw            m11, m8
    paddw               m11, m2
    movu                 m2, [srcq+ssq*0+64*2]
    psubw               m12, m7, m3
    pmulhrsw            m12, m8
    paddw               m12, m3
    movu                 m3, [srcq+ssq*0+64*3]
    mova  [dstq+dsq*0+64*0], m9
    psubw                m9, m0, m4
    pmulhrsw             m9, m8
    mova  [dstq+dsq*0+64*1], m10
    psubw               m10, m1, m5
    pmulhrsw            m10, m8
    mova  [dstq+dsq*0+64*2], m11
    psubw               m11, m2, m6
    pmulhrsw            m11, m8
    mova  [dstq+dsq*0+64*3], m12
    psubw               m12, m3, m7
    pmulhrsw            m12, m8
    paddw                m9, m4
    paddw               m10, m5
    mova  [dstq+dsq*1+64*0], m9
    mova  [dstq+dsq*1+64*1], m10
    paddw               m11, m6
    paddw               m12, m7
    mova  [dstq+dsq*1+64*2], m11
    mova  [dstq+dsq*1+64*3], m12
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w128_loop
    RET
.hv:
    movzx               t0d, word [r7+t0*2+table_offset(put, _bilin_hv)]
    shl                mxyd, 11
    vpbroadcastd         m6, [pw_2]
    vpbroadcastw         m7, mxyd
    vpbroadcastd         m8, [pw_8192]
    add                  t0, r7
    test          dword r8m, 0x800
    jnz .hv_12bpc
    psllw                m4, 2
    psllw                m5, 2
    vpbroadcastd         m8, [pw_2048]
.hv_12bpc:
    jmp                  t0
.hv_w2:
    vpbroadcastq       xmm1, [srcq+ssq*0]
    pmullw             xmm0, xmm1, xm4
    psrlq              xmm1, 16
    pmullw             xmm1, xm5
    paddw              xmm0, xm6
    paddw              xmm0, xmm1
    psrlw              xmm0, 2
.hv_w2_loop:
    movq               xmm2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movhps             xmm2, [srcq+ssq*0]
    pmullw             xmm1, xmm2, xm4
    psrlq              xmm2, 16
    pmullw             xmm2, xm5
    paddw              xmm1, xm6
    paddw              xmm1, xmm2
    psrlw              xmm1, 2                ; 1 _ 2 _
    shufpd             xmm2, xmm0, xmm1, 0x01 ; 0 _ 1 _
    mova               xmm0, xmm1
    psubw              xmm1, xmm2
    paddw              xmm1, xmm1
    pmulhw             xmm1, xm7
    paddw              xmm1, xmm2
    pmulhrsw           xmm1, xm8
    movd       [dstq+dsq*0], xmm1
    pextrd     [dstq+dsq*1], xmm1, 2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    RET
.hv_w4:
    pmullw             xmm0, xm4, [srcq+ssq*0-8]
    pmullw             xmm1, xm5, [srcq+ssq*0-6]
    paddw              xmm0, xm6
    paddw              xmm0, xmm1
    psrlw              xmm0, 2
.hv_w4_loop:
    movq               xmm1, [srcq+ssq*1+0]
    movq               xmm2, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    movhps             xmm1, [srcq+ssq*0+0]
    movhps             xmm2, [srcq+ssq*0+2]
    pmullw             xmm1, xm4
    pmullw             xmm2, xm5
    paddw              xmm1, xm6
    paddw              xmm1, xmm2
    psrlw              xmm1, 2                ; 1 2
    shufpd             xmm2, xmm0, xmm1, 0x01 ; 0 1
    mova               xmm0, xmm1
    psubw              xmm1, xmm2
    paddw              xmm1, xmm1
    pmulhw             xmm1, xm7
    paddw              xmm1, xmm2
    pmulhrsw           xmm1, xm8
    movq       [dstq+dsq*0], xmm1
    movhps     [dstq+dsq*1], xmm1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
    RET
.hv_w8:
    pmullw             xmm0, xm4, [srcq+ssq*0+0]
    pmullw             xmm1, xm5, [srcq+ssq*0+2]
    paddw              xmm0, xm6
    paddw              xmm0, xmm1
    psrlw              xmm0, 2
    vinserti32x4        ym0, xmm0, 1
.hv_w8_loop:
    movu                xm1, [srcq+ssq*1+0]
    movu                xm2, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    vinserti32x4        ym1, [srcq+ssq*0+0], 1
    vinserti32x4        ym2, [srcq+ssq*0+2], 1
    pmullw              ym1, ym4
    pmullw              ym2, ym5
    paddw               ym1, ym6
    paddw               ym1, ym2
    psrlw               ym1, 2              ; 1 2
    vshufi32x4          ym2, ym0, ym1, 0x01 ; 0 1
    mova                ym0, ym1
    psubw               ym1, ym2
    paddw               ym1, ym1
    pmulhw              ym1, ym7
    paddw               ym1, ym2
    pmulhrsw            ym1, ym8
    mova          [dstq+dsq*0], xm1
    vextracti32x4 [dstq+dsq*1], ym1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w8_loop
    RET
.hv_w16:
    pmullw              ym0, ym4, [srcq+ssq*0+0]
    pmullw              ym1, ym5, [srcq+ssq*0+2]
    paddw               ym0, ym6
    paddw               ym0, ym1
    psrlw               ym0, 2
    vinserti32x8         m0, ym0, 1
.hv_w16_loop:
    movu                ym1, [srcq+ssq*1+0]
    movu                ym2, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    vinserti32x8         m1, [srcq+ssq*0+0], 1
    vinserti32x8         m2, [srcq+ssq*0+2], 1
    pmullw               m1, m4
    pmullw               m2, m5
    paddw                m1, m6
    paddw                m1, m2
    psrlw                m1, 2             ; 1 2
    vshufi32x4           m2, m0, m1, q1032 ; 0 1
    mova                 m0, m1
    psubw                m1, m2
    paddw                m1, m1
    pmulhw               m1, m7
    paddw                m1, m2
    pmulhrsw             m1, m8
    mova          [dstq+dsq*0], ym1
    vextracti32x8 [dstq+dsq*1], m1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w16_loop
    RET
.hv_w32:
.hv_w64:
.hv_w128:
    movifnidn            wd, wm
    lea                 r6d, [hq+wq*8-256]
    mov                  r4, srcq
    mov                  r7, dstq
.hv_w32_loop0:
    pmullw               m0, m4, [srcq+ssq*0+0]
    pmullw               m1, m5, [srcq+ssq*0+2]
    paddw                m0, m6
    paddw                m0, m1
    psrlw                m0, 2
.hv_w32_loop:
    pmullw               m3, m4, [srcq+ssq*1+0]
    pmullw               m1, m5, [srcq+ssq*1+2]
    lea                srcq, [srcq+ssq*2]
    paddw                m3, m6
    paddw                m3, m1
    psrlw                m3, 2
    psubw                m1, m3, m0
    paddw                m1, m1
    pmulhw               m1, m7
    paddw                m1, m0
    pmullw               m0, m4, [srcq+ssq*0+0]
    pmullw               m2, m5, [srcq+ssq*0+2]
    paddw                m0, m6
    paddw                m0, m2
    psrlw                m0, 2
    psubw                m2, m0, m3
    paddw                m2, m2
    pmulhw               m2, m7
    paddw                m2, m3
    pmulhrsw             m1, m8
    pmulhrsw             m2, m8
    mova       [dstq+dsq*0], m1
    mova       [dstq+dsq*1], m2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w32_loop
    add                  r4, 64
    add                  r7, 64
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
    sub                 r6d, 1<<8
    jg .hv_w32_loop0
    RET

cglobal prep_bilin_16bpc, 3, 7, 16, tmp, src, stride, w, h, mxy, stride3
    movifnidn          mxyd, r5m ; mx
    lea                  r6, [prep_avx512icl]
    tzcnt                wd, wm
    movifnidn            hd, hm
    test               mxyd, mxyd
    jnz .h
    mov                mxyd, r6m ; my
    test               mxyd, mxyd
    jnz .v
.prep:
    movzx                wd, word [r6+wq*2+table_offset(prep,)]
    mov                 r5d, r7m ; bitdepth_max
    vpbroadcastd         m5, [r6-prep_avx512icl+pw_8192]
    add                  wq, r6
    shr                 r5d, 11
    vpbroadcastd         m4, [r6-prep_avx512icl+prep_mul+r5*4]
    lea            stride3q, [strideq*3]
    jmp                  wq
.prep_w4:
    movq               xmm0, [srcq+strideq*0]
    movhps             xmm0, [srcq+strideq*1]
    vpbroadcastq       ymm1, [srcq+strideq*2]
    vpbroadcastq       ymm2, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpblendd           ymm0, ymm1, 0x30
    vpblendd           ymm0, ymm2, 0xc0
    pmullw             ymm0, ym4
    psubw              ymm0, ym5
    mova             [tmpq], ymm0
    add                tmpq, 32
    sub                  hd, 4
    jg .prep_w4
    vzeroupper
    RET
.prep_w8:
    movu                xm0, [srcq+strideq*0]
    vinserti32x4        ym0, [srcq+strideq*1], 1
    vinserti32x4         m0, [srcq+strideq*2], 2
    vinserti32x4         m0, [srcq+stride3q ], 3
    lea                srcq, [srcq+strideq*4]
    pmullw               m0, m4
    psubw                m0, m5
    mova             [tmpq], m0
    add                tmpq, 64
    sub                  hd, 4
    jg .prep_w8
    RET
.prep_w16:
    movu                ym0, [srcq+strideq*0]
    vinserti32x8         m0, [srcq+strideq*1], 1
    movu                ym1, [srcq+strideq*2]
    vinserti32x8         m1, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    pmullw               m0, m4
    pmullw               m1, m4
    psubw                m0, m5
    psubw                m1, m5
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    add                tmpq, 64*2
    sub                  hd, 4
    jg .prep_w16
    RET
.prep_w32:
    pmullw               m0, m4, [srcq+strideq*0]
    pmullw               m1, m4, [srcq+strideq*1]
    pmullw               m2, m4, [srcq+strideq*2]
    pmullw               m3, m4, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    REPX      {psubw x, m5}, m0, m1, m2, m3
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    mova        [tmpq+64*2], m2
    mova        [tmpq+64*3], m3
    add                tmpq, 64*4
    sub                  hd, 4
    jg .prep_w32
    RET
.prep_w64:
    pmullw               m0, m4, [srcq+strideq*0+64*0]
    pmullw               m1, m4, [srcq+strideq*0+64*1]
    pmullw               m2, m4, [srcq+strideq*1+64*0]
    pmullw               m3, m4, [srcq+strideq*1+64*1]
    lea                srcq, [srcq+strideq*2]
    REPX      {psubw x, m5}, m0, m1, m2, m3
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    mova        [tmpq+64*2], m2
    mova        [tmpq+64*3], m3
    add                tmpq, 64*4
    sub                  hd, 2
    jg .prep_w64
    RET
.prep_w128:
    pmullw               m0, m4, [srcq+64*0]
    pmullw               m1, m4, [srcq+64*1]
    pmullw               m2, m4, [srcq+64*2]
    pmullw               m3, m4, [srcq+64*3]
    add                srcq, strideq
    REPX      {psubw x, m5}, m0, m1, m2, m3
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    mova        [tmpq+64*2], m2
    mova        [tmpq+64*3], m3
    add                tmpq, 64*4
    dec                  hd
    jg .prep_w128
    RET
.h:
    vpbroadcastw         m5, mxyd
    mov                mxyd, r6m ; my
    vpbroadcastd         m4, [pw_16]
    vpbroadcastd         m6, [pw_32766]
    psubw                m4, m5
    test          dword r7m, 0x800
    jnz .h_12bpc
    psllw                m4, 2
    psllw                m5, 2
.h_12bpc:
    test               mxyd, mxyd
    jnz .hv
    movzx                wd, word [r6+wq*2+table_offset(prep, _bilin_h)]
    add                  wq, r6
    lea            stride3q, [strideq*3]
    jmp                  wq
.h_w4:
    movu                xm1, [srcq+strideq*0]
    vinserti32x4        ym1, [srcq+strideq*2], 1
    movu                xm2, [srcq+strideq*1]
    vinserti32x4        ym2, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    punpcklqdq          ym0, ym1, ym2
    psrldq              ym1, 2
    psrldq              ym2, 2
    pmullw              ym0, ym4
    punpcklqdq          ym1, ym2
    pmullw              ym1, ym5
    psubw               ym0, ym6
    paddw               ym0, ym1
    psraw               ym0, 2
    mova             [tmpq], ym0
    add                tmpq, 32
    sub                  hd, 4
    jg .h_w4
    RET
.h_w8:
    movu                xm0, [srcq+strideq*0+0]
    movu                xm1, [srcq+strideq*0+2]
    vinserti32x4        ym0, [srcq+strideq*1+0], 1
    vinserti32x4        ym1, [srcq+strideq*1+2], 1
    vinserti32x4         m0, [srcq+strideq*2+0], 2
    vinserti32x4         m1, [srcq+strideq*2+2], 2
    vinserti32x4         m0, [srcq+stride3q +0], 3
    vinserti32x4         m1, [srcq+stride3q +2], 3
    lea                srcq, [srcq+strideq*4]
    pmullw               m0, m4
    pmullw               m1, m5
    psubw                m0, m6
    paddw                m0, m1
    psraw                m0, 2
    mova             [tmpq], m0
    add                tmpq, 64
    sub                  hd, 4
    jg .h_w8
    RET
.h_w16:
    movu                ym0, [srcq+strideq*0+0]
    vinserti32x8         m0, [srcq+strideq*1+0], 1
    movu                ym1, [srcq+strideq*0+2]
    vinserti32x8         m1, [srcq+strideq*1+2], 1
    lea                srcq, [srcq+strideq*2]
    pmullw               m0, m4
    pmullw               m1, m5
    psubw                m0, m6
    paddw                m0, m1
    psraw                m0, 2
    mova             [tmpq], m0
    add                tmpq, 64
    sub                  hd, 2
    jg .h_w16
    RET
.h_w32:
    pmullw               m0, m4, [srcq+strideq*0+0]
    pmullw               m2, m5, [srcq+strideq*0+2]
    pmullw               m1, m4, [srcq+strideq*1+0]
    pmullw               m3, m5, [srcq+strideq*1+2]
    lea                srcq, [srcq+strideq*2]
    psubw                m0, m6
    psubw                m1, m6
    paddw                m0, m2
    paddw                m1, m3
    psraw                m0, 2
    psraw                m1, 2
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    add                tmpq, 64*2
    sub                  hd, 2
    jg .h_w32
    RET
.h_w64:
    pmullw               m0, m4, [srcq+ 0]
    pmullw               m2, m5, [srcq+ 2]
    pmullw               m1, m4, [srcq+64]
    pmullw               m3, m5, [srcq+66]
    add                srcq, strideq
    psubw                m0, m6
    psubw                m1, m6
    paddw                m0, m2
    paddw                m1, m3
    psraw                m0, 2
    psraw                m1, 2
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    add                tmpq, 64*2
    dec                  hd
    jg .h_w64
    RET
.h_w128:
    pmullw               m0, m4, [srcq+  0]
    pmullw               m7, m5, [srcq+  2]
    pmullw               m1, m4, [srcq+ 64]
    pmullw               m8, m5, [srcq+ 66]
    pmullw               m2, m4, [srcq+128]
    pmullw               m9, m5, [srcq+130]
    pmullw               m3, m4, [srcq+192]
    pmullw              m10, m5, [srcq+194]
    add                srcq, strideq
    REPX      {psubw x, m6}, m0, m1, m2, m3
    paddw                m0, m7
    paddw                m1, m8
    paddw                m2, m9
    paddw                m3, m10
    REPX       {psraw x, 2}, m0, m1, m2, m3
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    mova        [tmpq+64*2], m2
    mova        [tmpq+64*3], m3
    add                tmpq, 64*4
    dec                  hd
    jg .h_w128
    RET
.v:
    movzx                wd, word [r6+wq*2+table_offset(prep, _bilin_v)]
    vpbroadcastw         m9, mxyd
    vpbroadcastd         m8, [pw_16]
    vpbroadcastd        m10, [pw_32766]
    add                  wq, r6
    lea            stride3q, [strideq*3]
    psubw                m8, m9
    test          dword r7m, 0x800
    jnz .v_12bpc
    psllw                m8, 2
    psllw                m9, 2
.v_12bpc:
    jmp                  wq
.v_w4:
    movq               xmm0, [srcq+strideq*0]
.v_w4_loop:
    vpbroadcastq       xmm2, [srcq+strideq*1]
    vpbroadcastq       ymm1, [srcq+strideq*2]
    vpbroadcastq       ymm3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpblendd           ymm2, ymm1, 0x30
    vpblendd           ymm2, ymm3, 0xc0
    vpblendd           ymm1, ymm2, ymm0, 0x03 ; 0 1 2 3
    movq               xmm0, [srcq+strideq*0]
    valignq            ymm2, ymm0, ymm2, 1    ; 1 2 3 4
    pmullw             ymm1, ym8
    pmullw             ymm2, ym9
    psubw              ymm1, ym10
    paddw              ymm1, ymm2
    psraw              ymm1, 2
    mova             [tmpq], ymm1
    add                tmpq, 32
    sub                  hd, 4
    jg .v_w4_loop
    vzeroupper
    RET
.v_w8:
    movu                xm0, [srcq+strideq*0]
.v_w8_loop:
    vinserti32x4        ym1, ym0, [srcq+strideq*1], 1
    vinserti32x4         m1, [srcq+strideq*2], 2
    vinserti32x4         m1, [srcq+stride3q ], 3 ; 0 1 2 3
    lea                srcq, [srcq+strideq*4]
    movu                xm0, [srcq+strideq*0]
    valignq              m2, m0, m1, 2           ; 1 2 3 4
    pmullw               m1, m8
    pmullw               m2, m9
    psubw                m1, m10
    paddw                m1, m2
    psraw                m1, 2
    mova             [tmpq], m1
    add                tmpq, 64
    sub                  hd, 4
    jg .v_w8_loop
    RET
.v_w16:
    movu                ym0, [srcq+strideq*0]
.v_w16_loop:
    vinserti32x8         m1, m0, [srcq+strideq*1], 1 ; 0 1
    movu                ym3, [srcq+strideq*2]
    vinserti32x8         m2, m3, [srcq+stride3q ], 1 ; 2 3
    lea                srcq, [srcq+strideq*4]
    movu                ym0, [srcq+strideq*0]
    vshufi32x4           m3, m1, m3, q1032           ; 1 2
    vshufi32x4           m4, m2, m0, q1032           ; 3 4
    pmullw               m1, m8
    pmullw               m2, m8
    pmullw               m3, m9
    pmullw               m4, m9
    psubw                m1, m10
    psubw                m2, m10
    paddw                m1, m3
    paddw                m2, m4
    psraw                m1, 2
    psraw                m2, 2
    mova        [tmpq+64*0], m1
    mova        [tmpq+64*1], m2
    add                tmpq, 64*2
    sub                  hd, 4
    jg .v_w16_loop
    RET
.v_w32:
    movu                 m0, [srcq+strideq*0]
.v_w32_loop:
    movu                 m3, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    pmullw               m1, m8, m0
    movu                 m0, [srcq+strideq*0]
    pmullw               m2, m8, m3
    pmullw               m3, m9
    pmullw               m4, m9, m0
    psubw                m1, m10
    psubw                m2, m10
    paddw                m1, m3
    paddw                m2, m4
    psraw                m1, 2
    psraw                m2, 2
    mova        [tmpq+64*0], m1
    mova        [tmpq+64*1], m2
    add                tmpq, 64*2
    sub                  hd, 2
    jg .v_w32_loop
    RET
.v_w64:
    movu                 m0, [srcq+64*0]
    movu                 m1, [srcq+64*1]
.v_w64_loop:
    add                srcq, strideq
    pmullw               m2, m8, m0
    movu                 m0, [srcq+64*0]
    pmullw               m3, m8, m1
    movu                 m1, [srcq+64*1]
    pmullw               m4, m9, m0
    pmullw               m5, m9, m1
    psubw                m2, m10
    psubw                m3, m10
    paddw                m2, m4
    paddw                m3, m5
    psraw                m2, 2
    psraw                m3, 2
    mova        [tmpq+64*0], m2
    mova        [tmpq+64*1], m3
    add                tmpq, 64*2
    dec                  hd
    jg .v_w64_loop
    RET
.v_w128:
    movu                 m0, [srcq+64*0]
    movu                 m1, [srcq+64*1]
    movu                 m2, [srcq+64*2]
    movu                 m3, [srcq+64*3]
.v_w128_loop:
    add                srcq, strideq
    pmullw               m4, m8, m0
    movu                 m0, [srcq+64*0]
    pmullw               m5, m8, m1
    movu                 m1, [srcq+64*1]
    pmullw               m6, m8, m2
    movu                 m2, [srcq+64*2]
    pmullw               m7, m8, m3
    movu                 m3, [srcq+64*3]
    pmullw              m11, m9, m0
    pmullw              m12, m9, m1
    pmullw              m13, m9, m2
    pmullw              m14, m9, m3
    REPX     {psubw x, m10}, m4, m5, m6, m7
    paddw                m4, m11
    paddw                m5, m12
    paddw                m6, m13
    paddw                m7, m14
    REPX       {psraw x, 2}, m4, m5, m6, m7
    mova        [tmpq+64*0], m4
    mova        [tmpq+64*1], m5
    mova        [tmpq+64*2], m6
    mova        [tmpq+64*3], m7
    add                tmpq, 64*4
    dec                  hd
    jg .v_w128_loop
    RET
.hv:
    movzx                wd, word [r6+wq*2+table_offset(prep, _bilin_hv)]
    shl                mxyd, 11
    vpbroadcastw         m7, mxyd
    add                  wq, r6
    lea            stride3q, [strideq*3]
    jmp                  wq
.hv_w4:
    movq               xmm0, [srcq+strideq*0+0]
    movq               xmm1, [srcq+strideq*0+2]
    pmullw             xmm0, xm4
    pmullw             xmm1, xm5
    psubw              xmm0, xm6
    paddw              xmm0, xmm1
    psraw              xmm0, 2
    vpbroadcastq        ym0, xmm0
.hv_w4_loop:
    movu                xm1, [srcq+strideq*1]
    vinserti128         ym1, [srcq+stride3q ], 1
    movu                xm2, [srcq+strideq*2]
    lea                srcq, [srcq+strideq*4]
    vinserti128         ym2, [srcq+strideq*0], 1
    punpcklqdq          ym3, ym1, ym2
    psrldq              ym1, 2
    psrldq              ym2, 2
    pmullw              ym3, ym4
    punpcklqdq          ym1, ym2
    pmullw              ym1, ym5
    psubw               ym3, ym6
    paddw               ym1, ym3
    psraw               ym1, 2           ; 1 2 3 4
    valignq             ym2, ym1, ym0, 3 ; 0 1 2 3
    mova                ym0, ym1
    psubw               ym1, ym2
    pmulhrsw            ym1, ym7
    paddw               ym1, ym2
    mova             [tmpq], ym1
    add                tmpq, 32
    sub                  hd, 4
    jg .hv_w4_loop
    RET
.hv_w8:
    pmullw              xm0, xm4, [srcq+strideq*0+0]
    pmullw              xm1, xm5, [srcq+strideq*0+2]
    psubw               xm0, xm6
    paddw               xm0, xm1
    psraw               xm0, 2
    vinserti32x4         m0, xm0, 3
.hv_w8_loop:
    movu                xm1, [srcq+strideq*1+0]
    movu                xm2, [srcq+strideq*1+2]
    vinserti32x4        ym1, [srcq+strideq*2+0], 1
    vinserti32x4        ym2, [srcq+strideq*2+2], 1
    vinserti32x4         m1, [srcq+stride3q +0], 2
    vinserti32x4         m2, [srcq+stride3q +2], 2
    lea                srcq, [srcq+strideq*4]
    vinserti32x4         m1, [srcq+strideq*0+0], 3
    vinserti32x4         m2, [srcq+strideq*0+2], 3
    pmullw               m1, m4
    pmullw               m2, m5
    psubw                m1, m6
    paddw                m1, m2
    psraw                m1, 2         ; 1 2 3 4
    valignq              m2, m1, m0, 6 ; 0 1 2 3
    mova                 m0, m1
    psubw                m1, m2
    pmulhrsw             m1, m7
    paddw                m1, m2
    mova             [tmpq], m1
    add                tmpq, 64
    sub                  hd, 4
    jg .hv_w8_loop
    RET
.hv_w16:
    pmullw              ym0, ym4, [srcq+strideq*0+0]
    pmullw              ym1, ym5, [srcq+strideq*0+2]
    psubw               ym0, ym6
    paddw               ym0, ym1
    psraw               ym0, 2
    vinserti32x8         m0, ym0, 1
.hv_w16_loop:
    movu                ym1, [srcq+strideq*1+0]
    movu                ym2, [srcq+strideq*1+2]
    lea                srcq, [srcq+strideq*2]
    vinserti32x8         m1, [srcq+strideq*0+0], 1
    vinserti32x8         m2, [srcq+strideq*0+2], 1
    pmullw               m1, m4
    pmullw               m2, m5
    psubw                m1, m6
    paddw                m1, m2
    psraw                m1, 2             ; 1 2
    vshufi32x4           m2, m0, m1, q1032 ; 0 1
    mova                 m0, m1
    psubw                m1, m2
    pmulhrsw             m1, m7
    paddw                m1, m2
    mova             [tmpq], m1
    add                tmpq, 64
    sub                  hd, 2
    jg .hv_w16_loop
    RET
.hv_w32:
    pmullw               m0, m4, [srcq+strideq*0+0]
    pmullw               m1, m5, [srcq+strideq*0+2]
    psubw                m0, m6
    paddw                m0, m1
    psraw                m0, 2
.hv_w32_loop:
    pmullw               m3, m4, [srcq+strideq*1+0]
    pmullw               m1, m5, [srcq+strideq*1+2]
    lea                srcq, [srcq+strideq*2]
    psubw                m3, m6
    paddw                m3, m1
    psraw                m3, 2
    psubw                m1, m3, m0
    pmulhrsw             m1, m7
    paddw                m1, m0
    pmullw               m0, m4, [srcq+strideq*0+0]
    pmullw               m2, m5, [srcq+strideq*0+2]
    psubw                m0, m6
    paddw                m0, m2
    psraw                m0, 2
    psubw                m2, m0, m3
    pmulhrsw             m2, m7
    paddw                m2, m3
    mova        [tmpq+64*0], m1
    mova        [tmpq+64*1], m2
    add                tmpq, 64*2
    sub                  hd, 2
    jg .hv_w32_loop
    RET
.hv_w64:
    pmullw               m0, m4, [srcq+ 0]
    pmullw               m2, m5, [srcq+ 2]
    pmullw               m1, m4, [srcq+64]
    pmullw               m3, m5, [srcq+66]
    psubw                m0, m6
    psubw                m1, m6
    paddw                m0, m2
    paddw                m1, m3
    psraw                m0, 2
    psraw                m1, 2
.hv_w64_loop:
    add                srcq, strideq
    pmullw               m2, m4, [srcq+ 0]
    pmullw               m8, m5, [srcq+ 2]
    pmullw               m3, m4, [srcq+64]
    pmullw               m9, m5, [srcq+66]
    psubw                m2, m6
    psubw                m3, m6
    paddw                m2, m8
    paddw                m3, m9
    psraw                m2, 2
    psraw                m3, 2
    psubw                m8, m2, m0
    psubw                m9, m3, m1
    pmulhrsw             m8, m7
    pmulhrsw             m9, m7
    paddw                m8, m0
    mova                 m0, m2
    paddw                m9, m1
    mova                 m1, m3
    mova        [tmpq+64*0], m8
    mova        [tmpq+64*1], m9
    add                tmpq, 64*2
    dec                  hd
    jg .hv_w64_loop
    RET
.hv_w128:
    pmullw               m0, m4, [srcq+  0]
    pmullw               m8, m5, [srcq+  2]
    pmullw               m1, m4, [srcq+ 64]
    pmullw               m9, m5, [srcq+ 66]
    pmullw               m2, m4, [srcq+128]
    pmullw              m10, m5, [srcq+130]
    pmullw               m3, m4, [srcq+192]
    pmullw              m11, m5, [srcq+194]
    REPX      {psubw x, m6}, m0, m1, m2, m3
    paddw                m0, m8
    paddw                m1, m9
    paddw                m2, m10
    paddw                m3, m11
    REPX       {psraw x, 2}, m0, m1, m2, m3
.hv_w128_loop:
    add                srcq, strideq
    pmullw               m8, m4, [srcq+  0]
    pmullw              m12, m5, [srcq+  2]
    pmullw               m9, m4, [srcq+ 64]
    pmullw              m13, m5, [srcq+ 66]
    pmullw              m10, m4, [srcq+128]
    pmullw              m14, m5, [srcq+130]
    pmullw              m11, m4, [srcq+192]
    pmullw              m15, m5, [srcq+194]
    REPX      {psubw x, m6}, m8, m9, m10, m11
    paddw                m8, m12
    paddw                m9, m13
    paddw               m10, m14
    paddw               m11, m15
    REPX       {psraw x, 2}, m8, m9, m10, m11
    psubw               m12, m8, m0
    psubw               m13, m9, m1
    psubw               m14, m10, m2
    psubw               m15, m11, m3
    REPX   {pmulhrsw x, m7}, m12, m13, m14, m15
    paddw               m12, m0
    mova                 m0, m8
    paddw               m13, m1
    mova                 m1, m9
    mova        [tmpq+64*0], m12
    mova        [tmpq+64*1], m13
    paddw               m14, m2
    mova                 m2, m10
    paddw               m15, m3
    mova                 m3, m11
    mova        [tmpq+64*2], m14
    mova        [tmpq+64*3], m15
    add                tmpq, 64*4
    dec                  hd
    jg .hv_w128_loop
    RET

; int8_t subpel_filters[5][15][8]
%assign FILTER_REGULAR (0*15 << 16) | 3*15
%assign FILTER_SMOOTH  (1*15 << 16) | 4*15
%assign FILTER_SHARP   (2*15 << 16) | 3*15

%macro MC_8TAP_FN 4 ; prefix, type, type_h, type_v
cglobal %1_8tap_%2_16bpc
    mov                 t0d, FILTER_%3
%ifidn %3, %4
    mov                 t1d, t0d
%else
    mov                 t1d, FILTER_%4
%endif
%ifnidn %2, regular ; skip the jump in the last filter
    jmp mangle(private_prefix %+ _%1_8tap_16bpc %+ SUFFIX)
%endif
%endmacro

%if WIN64
DECLARE_REG_TMP 4, 5
%define buf rsp+stack_offset+8 ; shadow space
%else
DECLARE_REG_TMP 7, 8
%define buf rsp-40 ; red zone
%endif

MC_8TAP_FN put, sharp,          SHARP,   SHARP
MC_8TAP_FN put, sharp_smooth,   SHARP,   SMOOTH
MC_8TAP_FN put, smooth_sharp,   SMOOTH,  SHARP
MC_8TAP_FN put, smooth,         SMOOTH,  SMOOTH
MC_8TAP_FN put, sharp_regular,  SHARP,   REGULAR
MC_8TAP_FN put, regular_sharp,  REGULAR, SHARP
MC_8TAP_FN put, smooth_regular, SMOOTH,  REGULAR
MC_8TAP_FN put, regular_smooth, REGULAR, SMOOTH
MC_8TAP_FN put, regular,        REGULAR, REGULAR

cglobal put_8tap_16bpc, 4, 9, 16, dst, ds, src, ss, w, h, mx, my
%define base r8-put_avx512icl
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 8tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 8tap_v, my, 4tap_v
    lea                  r8, [put_avx512icl]
    movifnidn            wd, wm
    movifnidn            hd, hm
    test                mxd, 0xf00
    jnz .h
    test                myd, 0xf00
    jnz .v
    tzcnt                wd, wd
    movzx                wd, word [r8+wq*2+table_offset(put,)]
    add                  wq, r8
%if WIN64
    pop                  r8
%endif
    jmp                  wq
.h_w2:
    movzx               mxd, mxb
    sub                srcq, 2
    mova                ym2, [spel_h_shuf2a]
    pmovsxbw           xmm4, [base+subpel_filters+mxq*8]
    pshufd             xmm3, xmm4, q1111
    pshufd             xmm4, xmm4, q2222
.h_w2_loop:
    movu                xm1, [srcq+ssq*0]
    vinserti32x4        ym1, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    mova               xmm0, xm8
    vpermb              ym1, ym2, ym1
    vpdpwssd           xmm0, xmm3, xm1
    vextracti32x4       xm1, ym1, 1
    vpdpwssd           xmm0, xmm4, xm1
    psrad              xmm0, 6
    packusdw           xmm0, xmm0
    pminsw             xmm0, xm9
    movd       [dstq+dsq*0], xmm0
    pextrd     [dstq+dsq*1], xmm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w2_loop
    RET
.h_w4:
    movzx               mxd, mxb
    sub                srcq, 2
    pmovsxbw           xmm0, [base+subpel_filters+mxq*8]
    vbroadcasti32x4     ym4, [spel_h_shufA]
    vbroadcasti32x4     ym5, [spel_h_shufB]
    pshufd             xmm0, xmm0, q2211
    vpbroadcastq        ym6, xmm0
    vpermq              ym7, ymm0, q1111
.h_w4_loop:
    movu                xm2, [srcq+ssq*0]
    vinserti32x4        ym2, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    mova                ym0, ym8
    pshufb              ym1, ym2, ym4
    vpdpwssd            ym0, ym6, ym1
    pshufb              ym2, ym5
    vpdpwssd            ym0, ym7, ym2
    psrad               ym0, 6
    vextracti32x4       xm1, ym0, 1
    packusdw            xm0, xm1
    pminsw             xmm0, xm0, xm9
    movq       [dstq+dsq*0], xmm0
    movhps     [dstq+dsq*1], xmm0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w4_loop
    RET
.h:
    test                myd, 0xf00
    jnz .hv
    mov                 r7d, r8m
    vpbroadcastw         m9, r8m
    shr                 r7d, 11
    vpbroadcastd         m8, [base+put_8tap_h_rnd+r7*4]
    cmp                  wd, 4
    je .h_w4
    jl .h_w2
    shr                 mxd, 16
    sub                srcq, 6
    pmovsxbw           xmm0, [base+subpel_filters+mxq*8]
    mova              [buf], xmm0
    vpbroadcastd        m10, xmm0
    vpbroadcastd        m11, [buf+ 4]
    vpbroadcastd        m12, [buf+ 8]
    vpbroadcastd        m13, [buf+12]
    sub                  wd, 16
    je .h_w16
    jg .h_w32
.h_w8:
    mova                 m4, [spel_h_shufA]
    movu                 m5, [spel_h_shufB]
    movu                 m6, [spel_h_shufC]
    mova                 m7, [spel_h_shufD]
.h_w8_loop:
    movu                ym2, [srcq+ssq*0]
    vinserti32x8         m2, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    mova                 m0, m8
    vpermb               m1, m4, m2
    vpdpwssd             m0, m10, m1
    vpermb               m1, m5, m2
    vpdpwssd             m0, m11, m1
    vpermb               m1, m6, m2
    vpdpwssd             m0, m12, m1
    vpermb               m1, m7, m2
    vpdpwssd             m0, m13, m1
    psrad                m0, 6
    vextracti32x8       ym1, m0, 1
    packusdw            ym0, ym1
    pminsw              ym0, ym9
    mova          [dstq+dsq*0], xm0
    vextracti32x4 [dstq+dsq*1], ym0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w8_loop
    RET
.h_w16:
    vbroadcasti32x4      m6, [spel_h_shufA]
    vbroadcasti32x4      m7, [spel_h_shufB]
.h_w16_loop:
    movu                ym2, [srcq+ssq*0+ 0]
    vinserti32x8         m2, [srcq+ssq*1+ 0], 1
    movu                ym3, [srcq+ssq*0+16]
    vinserti32x8         m3, [srcq+ssq*1+16], 1
    lea                srcq, [srcq+ssq*2]
    mova                 m0, m8
    mova                 m1, m8
    pshufb               m4, m2, m6
    vpdpwssd             m0, m10, m4 ; a0
    pshufb               m4, m3, m6
    vpdpwssd             m1, m12, m4 ; b2
    pshufb               m4, m2, m7
    vpdpwssd             m0, m11, m4 ; a1
    pshufb               m4, m3, m7
    vpdpwssd             m1, m13, m4 ; b3
    shufpd               m2, m3, 0x55
    pshufb               m4, m2, m6
    vpdpwssd             m0, m12, m4 ; a2
    vpdpwssd             m1, m10, m4 ; b0
    pshufb               m2, m7
    vpdpwssd             m0, m13, m2 ; a3
    vpdpwssd             m1, m11, m2 ; b1
    psrad                m0, 6
    psrad                m1, 6
    packusdw             m0, m1
    pminsw               m0, m9
    mova          [dstq+dsq*0], ym0
    vextracti32x8 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w16_loop
    RET
.h_w32:
    lea                srcq, [srcq+wq*2]
    vbroadcasti32x4      m6, [spel_h_shufA]
    lea                dstq, [dstq+wq*2]
    vbroadcasti32x4      m7, [spel_h_shufB]
    neg                  wq
.h_w32_loop0:
    mov                  r6, wq
.h_w32_loop:
    movu                 m2, [srcq+r6*2+ 0]
    movu                 m3, [srcq+r6*2+ 8]
    mova                 m0, m8
    mova                 m1, m8
    pshufb               m4, m2, m6
    vpdpwssd             m0, m10, m4 ; a0
    pshufb               m4, m3, m6
    vpdpwssd             m1, m10, m4 ; b0
    vpdpwssd             m0, m12, m4 ; a2
    movu                 m4, [srcq+r6*2+16]
    pshufb               m3, m7
    vpdpwssd             m1, m11, m3 ; b1
    vpdpwssd             m0, m13, m3 ; a3
    pshufb               m3, m4, m6
    vpdpwssd             m1, m12, m3 ; b2
    pshufb               m2, m7
    vpdpwssd             m0, m11, m2 ; a1
    pshufb               m4, m7
    vpdpwssd             m1, m13, m4 ; b3
    psrad                m0, 6
    psrad                m1, 6
    packusdw             m0, m1
    pminsw               m0, m9
    mova        [dstq+r6*2], m0
    add                  r6, 32
    jl .h_w32_loop
    add                srcq, ssq
    add                dstq, dsq
    dec                  hd
    jg .h_w32_loop0
    RET
.v:
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    vpbroadcastd        m10, [pd_32]
    pmovsxbw           xmm0, [base+subpel_filters+myq*8]
    tzcnt               r7d, wd
    vpbroadcastw        m11, r8m
    lea                  r6, [ssq*3]
    movzx               r7d, word [r8+r7*2+table_offset(put, _8tap_v)]
    sub                srcq, r6
    mova [rsp+stack_offset+8], xmm0
    vpbroadcastd        m12, xmm0
    add                  r7, r8
    vpbroadcastd        m13, [rsp+stack_offset+12]
    vpbroadcastd        m14, [rsp+stack_offset+16]
    vpbroadcastd        m15, [rsp+stack_offset+20]
    jmp                  r7
.v_w2:
    movd               xmm2, [srcq+ssq*0]
    pinsrd             xmm2, [srcq+ssq*1], 1
    pinsrd             xmm2, [srcq+ssq*2], 2
    add                srcq, r6
    pinsrd             xmm2, [srcq+ssq*0], 3  ; 0 1 2 3
    movd               xmm3, [srcq+ssq*1]
    vpbroadcastd       xmm1, [srcq+ssq*2]
    add                srcq, r6
    vpbroadcastd       xmm0, [srcq+ssq*0]
    vpblendd           xmm3, xmm1, 0x02       ; 4 5
    vpblendd           xmm1, xmm0, 0x02       ; 5 6
    palignr            xmm4, xmm3, xmm2, 4    ; 1 2 3 4
    punpcklwd          xmm3, xmm1             ; 45 56
    punpcklwd          xmm1, xmm2, xmm4       ; 01 12
    punpckhwd          xmm2, xmm4             ; 23 34
.v_w2_loop:
    vpbroadcastd       xmm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova               xmm5, xm10
    vpdpwssd           xmm5, xm12, xmm1       ; a0 b0
    mova               xmm1, xmm2
    vpdpwssd           xmm5, xm13, xmm2       ; a1 b1
    mova               xmm2, xmm3
    vpdpwssd           xmm5, xm14, xmm3       ; a2 b2
    vpblendd           xmm3, xmm0, xmm4, 0x02 ; 6 7
    vpbroadcastd       xmm0, [srcq+ssq*0]
    vpblendd           xmm4, xmm0, 0x02       ; 7 8
    punpcklwd          xmm3, xmm4             ; 67 78
    vpdpwssd           xmm5, xm15, xmm3       ; a3 b3
    psrad              xmm5, 6
    packusdw           xmm5, xmm5
    pminsw             xmm5, xm11
    movd       [dstq+dsq*0], xmm5
    pextrd     [dstq+dsq*1], xmm5, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
    movq               xmm1, [srcq+ssq*0]
    vpbroadcastq       ymm0, [srcq+ssq*1]
    vpbroadcastq       ymm2, [srcq+ssq*2]
    add                srcq, r6
    vpbroadcastq       ymm4, [srcq+ssq*0]
    vpbroadcastq       ymm3, [srcq+ssq*1]
    vpbroadcastq       ymm5, [srcq+ssq*2]
    add                srcq, r6
    vpblendd           ymm1, ymm0, 0x30
    vpblendd           ymm0, ymm2, 0x30
    punpcklwd          ymm1, ymm0       ; 01 12
    vpbroadcastq       ymm0, [srcq+ssq*0]
    vpblendd           ymm2, ymm4, 0x30
    vpblendd           ymm4, ymm3, 0x30
    punpcklwd          ymm2, ymm4       ; 23 34
    vpblendd           ymm3, ymm5, 0x30
    vpblendd           ymm5, ymm0, 0x30
    punpcklwd          ymm3, ymm5       ; 45 56
.v_w4_loop:
    vpbroadcastq       ymm5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova               ymm4, ym10
    vpdpwssd           ymm4, ym12, ymm1 ; a0 b0
    mova               ymm1, ymm2
    vpdpwssd           ymm4, ym13, ymm2 ; a1 b1
    mova               ymm2, ymm3
    vpdpwssd           ymm4, ym14, ymm3 ; a2 b2
    vpblendd           ymm3, ymm0, ymm5, 0x30
    vpbroadcastq       ymm0, [srcq+ssq*0]
    vpblendd           ymm5, ymm0, 0x30
    punpcklwd          ymm3, ymm5       ; 67 78
    vpdpwssd           ymm4, ym15, ymm3 ; a3 b3
    psrad              ymm4, 6
    vextracti128       xmm5, ymm4, 1
    packusdw           xmm4, xmm5
    pminsw             xmm4, xm11
    movq       [dstq+dsq*0], xmm4
    movhps     [dstq+dsq*1], xmm4
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
    vzeroupper
    RET
.v_w8:
    vbroadcasti32x4      m2, [srcq+ssq*2]
    vinserti32x4         m1, m2, [srcq+ssq*0], 0
    vinserti32x4         m1, [srcq+ssq*1], 1 ; 0 1 2
    add                srcq, r6
    vinserti32x4        ym2, [srcq+ssq*0], 1
    vinserti32x4         m2, [srcq+ssq*1], 2 ; 2 3 4
    mova                 m6, [spel_v_shuf8]
    movu                xm0, [srcq+ssq*1]
    vinserti32x4        ym0, [srcq+ssq*2], 1
    add                srcq, r6
    vinserti32x4         m0, [srcq+ssq*0], 2 ; 4 5 6
    vpermb               m1, m6, m1          ; 01 12
    vpermb               m2, m6, m2          ; 23 34
    vpermb               m3, m6, m0          ; 45 56
.v_w8_loop:
    vinserti32x4         m0, [srcq+ssq*1], 3
    lea                srcq, [srcq+ssq*2]
    movu                xm5, [srcq+ssq*0]
    mova                 m4, m10
    vpdpwssd             m4, m12, m1         ; a0 b0
    mova                 m1, m2
    vshufi32x4           m0, m5, q1032       ; 6 7 8
    vpdpwssd             m4, m13, m2         ; a1 b1
    mova                 m2, m3
    vpdpwssd             m4, m14, m3         ; a2 b2
    vpermb               m3, m6, m0          ; 67 78
    vpdpwssd             m4, m15, m3         ; a3 b3
    psrad                m4, 6
    vextracti32x8       ym5, m4, 1
    packusdw            ym4, ym5
    pminsw              ym4, ym11
    mova          [dstq+dsq*0], xm4
    vextracti32x4 [dstq+dsq*1], ym4, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w8_loop
    RET
.v_w16:
    vbroadcasti32x8      m1, [srcq+ssq*1]
    vinserti32x8         m0, m1, [srcq+ssq*0], 0
    vinserti32x8         m1, [srcq+ssq*2], 1
    mova                 m8, [spel_v_shuf16]
    add                srcq, r6
    movu                ym3, [srcq+ssq*0]
    vinserti32x8         m3, [srcq+ssq*1], 1
    movu                ym5, [srcq+ssq*2]
    add                srcq, r6
    vinserti32x8         m5, [srcq+ssq*0], 1
    vpermb               m0, m8, m0     ; 01
    vpermb               m1, m8, m1     ; 12
    vpermb               m3, m8, m3     ; 34
    vpermb               m5, m8, m5     ; 56
    mova                 m9, [deint_q_shuf]
    vpshrdd              m2, m1, m3, 16 ; 23
    vpshrdd              m4, m3, m5, 16 ; 45
.v_w16_loop:
    mova                 m6, m10
    mova                 m7, m10
    vpdpwssd             m6, m12, m0    ; a0
    mova                 m0, m2
    vpdpwssd             m7, m12, m1    ; b0
    mova                 m1, m3
    vpdpwssd             m6, m13, m2    ; a1
    mova                 m2, m4
    vpdpwssd             m7, m13, m3    ; b1
    mova                 m3, m5
    vpdpwssd             m6, m14, m4    ; a2
    mova                 m4, m5
    vpdpwssd             m7, m14, m5    ; b2
    movu                ym5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vinserti32x8         m5, [srcq+ssq*0], 1
    vpermb               m5, m8, m5     ; 78
    vpshrdd              m4, m5, 16     ; 67
    vpdpwssd             m6, m15, m4    ; a3
    vpdpwssd             m7, m15, m5    ; b3
    psrad                m6, 6
    psrad                m7, 6
    packusdw             m6, m7
    pminsw               m6, m11
    vpermq               m6, m9, m6
    mova          [dstq+dsq*0], ym6
    vextracti32x8 [dstq+dsq*1], m6, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w16_loop
    RET
.v_w32:
.v_w64:
.v_w128:
%if WIN64
    movaps [rsp+stack_offset+8], xmm6
%endif
    lea                  wd, [hq+wq*8-256]
    mov                  r7, srcq
    mov                  r8, dstq
.v_w32_loop0:
    movu                m16, [srcq+ssq*0]
    movu                m17, [srcq+ssq*1]
    movu                m18, [srcq+ssq*2]
    add                srcq, r6
    movu                m19, [srcq+ssq*0]
    movu                m20, [srcq+ssq*1]
    movu                m21, [srcq+ssq*2]
    add                srcq, r6
    movu                m22, [srcq+ssq*0]
    punpcklwd            m0, m16, m17 ; 01l
    punpckhwd           m16, m17      ; 01h
    punpcklwd            m1, m17, m18 ; 12l
    punpckhwd           m17, m18      ; 12h
    punpcklwd            m2, m18, m19 ; 23l
    punpckhwd           m18, m19      ; 23h
    punpcklwd            m3, m19, m20 ; 34l
    punpckhwd           m19, m20      ; 34h
    punpcklwd            m4, m20, m21 ; 45l
    punpckhwd           m20, m21      ; 45h
    punpcklwd            m5, m21, m22 ; 56l
    punpckhwd           m21, m22      ; 56h
.v_w32_loop:
    mova                 m6, m10
    vpdpwssd             m6, m12, m0  ; a0l
    mova                 m8, m10
    vpdpwssd             m8, m12, m16 ; a0h
    mova                 m7, m10
    vpdpwssd             m7, m12, m1  ; b0l
    mova                 m9, m10
    vpdpwssd             m9, m12, m17 ; b0h
    mova                 m0, m2
    vpdpwssd             m6, m13, m2  ; a1l
    mova                m16, m18
    vpdpwssd             m8, m13, m18 ; a1h
    mova                 m1, m3
    vpdpwssd             m7, m13, m3  ; b1l
    mova                m17, m19
    vpdpwssd             m9, m13, m19 ; b1h
    mova                 m2, m4
    vpdpwssd             m6, m14, m4  ; a2l
    mova                m18, m20
    vpdpwssd             m8, m14, m20 ; a2h
    mova                 m3, m5
    vpdpwssd             m7, m14, m5  ; b2l
    mova                m19, m21
    vpdpwssd             m9, m14, m21 ; b2h
    movu                m21, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklwd            m4, m22, m21 ; 67l
    punpckhwd           m20, m22, m21 ; 67h
    movu                m22, [srcq+ssq*0]
    vpdpwssd             m6, m15, m4  ; a3l
    vpdpwssd             m8, m15, m20 ; a3h
    punpcklwd            m5, m21, m22 ; 78l
    punpckhwd           m21, m22      ; 78h
    vpdpwssd             m7, m15, m5  ; b3l
    vpdpwssd             m9, m15, m21 ; b3h
    REPX       {psrad x, 6}, m6, m8, m7, m9
    packusdw             m6, m8
    packusdw             m7, m9
    pminsw               m6, m11
    pminsw               m7, m11
    mova       [dstq+dsq*0], m6
    mova       [dstq+dsq*1], m7
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w32_loop
    add                  r7, 64
    add                  r8, 64
    movzx                hd, wb
    mov                srcq, r7
    mov                dstq, r8
    sub                  wd, 1<<8
    jg .v_w32_loop0
%if WIN64
    movaps             xmm6, [rsp+stack_offset+8]
%endif
    vzeroupper
    RET
.hv:
    vpbroadcastw        m11, r8m
    cmp                  wd, 4
    jg .hv_w8
    movzx               mxd, mxb
    pmovsxbw           xmm0, [base+subpel_filters+mxq*8]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    pmovsxbw           xmm1, [base+subpel_filters+myq*8]
    lea                  r6, [ssq*3]
    sub                srcq, 2
    sub                srcq, r6
    test          dword r8m, 0x800
    jnz .hv_12bit
    vpbroadcastd        m10, [pd_2176]
    psllw              xmm0, 6
    jmp .hv_main
.hv_12bit:
    vpbroadcastd        m10, [pd_640]
    psllw              xmm0, 4
    psllw              xmm1, 2
.hv_main:
    mova           [buf+ 0], xmm0
    mova           [buf+16], xmm1
    vpbroadcastd         m8, [buf+ 4]
    vpbroadcastd         m9, [buf+ 8]
    vpbroadcastd       ym12, xmm1
    vpbroadcastd       ym13, [buf+20]
    vpbroadcastd       ym14, [buf+24]
    vpbroadcastd       ym15, [buf+28]
    movu                xm4, [srcq+ssq*0]
    vinserti32x4        ym4, [srcq+ssq*1], 1
    vinserti32x4         m4, [srcq+ssq*2], 2
    add                srcq, r6
    vinserti32x4         m4, [srcq+ssq*0], 3 ; 0 1 2 3
    movu                xm0, [srcq+ssq*1]
    vinserti32x4        ym0, [srcq+ssq*2], 1
    add                srcq, r6
    vinserti32x4         m0, [srcq+ssq*0], 2 ; 4 5 6
    cmp                  wd, 4
    je .hv_w4
    vbroadcasti32x4      m2, [spel_h_shufA]
    mova                 m3, [spel_h_shuf2b]
    mova                ym6, [spel_h_shuf2a]
    mova                xm7, [spel_shuf2]
    mova                 m1, m10
    pshufb               m4, m2
    pshufb               m0, m2
    punpcklqdq           m2, m4, m0
    vpdpwssd             m1, m8, m2    ; 04 15 26 3_
    punpckhqdq           m4, m0
    vpdpwssd             m1, m9, m4
    vpermb               m1, m3, m1    ; 01 12
    vextracti32x4       xm2, ym1, 1    ; 23 34
    vextracti32x4       xm3, m1, 2     ; 45 56
.hv_w2_loop:
    movu                xm5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vinserti32x4        ym5, [srcq+ssq*0], 1
    mova                xm4, xm10
    vpermb              ym5, ym6, ym5
    pmaddwd            xmm0, xm12, xm1 ; a0 b0
    vpdpwssd            xm4, xm8, xm5
    vextracti32x4       xm5, ym5, 1
    mova                xm1, xm2
    vpdpwssd           xmm0, xm13, xm2 ; a1 b1
    vpdpwssd            xm4, xm9, xm5  ; 7 8
    mova                xm2, xm3
    vpdpwssd           xmm0, xm14, xm3 ; a2 b2
    vpermt2b            xm3, xm7, xm4  ; 67 78
    vpdpwssd           xmm0, xm15, xm3 ; a3 b3
    psrad              xmm0, 10
    packusdw           xmm0, xmm0
    pminsw             xmm0, xm11
    movd       [dstq+dsq*0], xmm0
    pextrd     [dstq+dsq*1], xmm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    RET
.hv_w4:
    vbroadcasti32x4     m19, [spel_h_shufA]
    vbroadcasti32x4     m20, [spel_h_shufB]
    mova                ym6, [spel_shuf4a]
    mova                ym7, [spel_shuf4b]
    mova                 m2, m10
    mova                 m3, m10
    pshufb               m1, m4, m19
    vpdpwssd             m2, m8, m1
    pshufb               m1, m0, m19
    vpdpwssd             m3, m8, m1
    pshufb               m4, m20
    vpdpwssd             m2, m9, m4
    pshufb               m0, m20
    vpdpwssd             m3, m9, m0
    vpermb               m1, m6, m2    ; 01 12
    vshufi32x4           m2, m3, q1032
    vpermb               m3, m6, m3    ; 45 56
    vpermb               m2, m6, m2    ; 23 34
.hv_w4_loop:
    movu               xm18, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vinserti128        ym18, [srcq+ssq*0], 1
    mova                ym4, ym10
    pshufb             ym17, ym18, ym19
    pmaddwd            ym16, ym12, ym1 ; a0 b0
    vpdpwssd            ym4, ym8, ym17
    pshufb             ym18, ym20
    mova                ym1, ym2
    vpdpwssd           ym16, ym13, ym2 ; a1 b1
    vpdpwssd            ym4, ym9, ym18 ; 7 8
    mova                ym2, ym3
    vpdpwssd           ym16, ym14, ym3 ; a2 b2
    vpermt2b            ym3, ym7, ym4  ; 67 78
    vpdpwssd           ym16, ym15, ym3 ; a3 b3
    psrad              ym16, 10
    vextracti128       xm17, ym16, 1
    packusdw           xm16, xm17
    pminsw             xm16, xm11
    movq       [dstq+dsq*0], xm16
    movhps     [dstq+dsq*1], xm16
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
    vzeroupper
    RET
.hv_w8:
    shr                 mxd, 16
    pmovsxbw           xmm0, [base+subpel_filters+mxq*8]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    pmovsxbw           xmm1, [base+subpel_filters+myq*8]
    lea                  r6, [ssq*3]
    sub                srcq, 6
    sub                srcq, r6
    test          dword r8m, 0x800
    jnz .hv_w8_12bit
    vpbroadcastd        m10, [pd_2176]
    psllw              xmm0, 6
    jmp .hv_w8_main
.hv_w8_12bit:
    vpbroadcastd        m10, [pd_640]
    psllw              xmm0, 4
    psllw              xmm1, 2
.hv_w8_main:
    mova           [buf+ 0], xmm0
    mova           [buf+16], xmm1
    vpbroadcastd        m12, xmm0
    vpbroadcastd        m13, [buf+ 4]
    vpbroadcastd        m14, [buf+ 8]
    vpbroadcastd        m15, [buf+12]
    vpbroadcastd        m16, xmm1
    vpbroadcastd        m17, [buf+20]
    vpbroadcastd        m18, [buf+24]
    vpbroadcastd        m19, [buf+28]
    cmp                  wd, 16
    je .hv_w16
    jg .hv_w32
    mova                 m5, [spel_h_shufA]
    movu                ym0, [srcq+ssq*0]
    vinserti32x8         m0, [srcq+ssq*1], 1 ; 0 1
    movu                ym9, [srcq+ssq*2]
    add                srcq, r6
    vinserti32x8         m9, [srcq+ssq*0], 1 ; 2 3
    movu               ym20, [srcq+ssq*1]
    vinserti32x8        m20, [srcq+ssq*2], 1 ; 4 5
    add srcq, r6
    movu               ym21, [srcq+ssq*0]    ; 6
    movu                 m6, [spel_h_shufB]
    movu                 m7, [spel_h_shufC]
    vpermb               m8, m5, m0
    mova                 m1, m10
    vpdpwssd             m1, m12, m8  ; a0 b0
    vpermb               m8, m5, m9
    mova                 m2, m10
    vpdpwssd             m2, m12, m8  ; c0 d0
    vpermb               m8, m5, m20
    mova                 m3, m10
    vpdpwssd             m3, m12, m8  ; e0 f0
    vpermb               m8, m5, m21
    mova                 m4, m10
    vpdpwssd             m4, m12, m8  ; g0
    vpermb               m8, m6, m0
    vpdpwssd             m1, m13, m8  ; a1 b1
    vpermb               m8, m6, m9
    vpdpwssd             m2, m13, m8  ; c1 d1
    vpermb               m8, m6, m20
    vpdpwssd             m3, m13, m8  ; e1 f1
    vpermb               m8, m6, m21
    vpdpwssd             m4, m13, m8  ; g1
    vpermb               m8, m7, m0
    vpdpwssd             m1, m14, m8  ; a2 b2
    vpermb               m8, m7, m9
    vpdpwssd             m2, m14, m8  ; c2 d2
    vpermb               m8, m7, m20
    vpdpwssd             m3, m14, m8  ; e2 f2
    vpermb               m8, m7, m21
    vpdpwssd             m4, m14, m8  ; g2
    mova                 m8, [spel_h_shufD]
    vpermb               m0, m8, m0
    vpdpwssd             m1, m15, m0  ; a3 b3
    mova                 m0, [spel_shuf8a]
    vpermb               m9, m8, m9
    vpdpwssd             m2, m15, m9  ; c3 d3
    mova                 m9, [spel_shuf8b]
    vpermb              m20, m8, m20
    vpdpwssd             m3, m15, m20 ; e3 f3
    vpermb              m21, m8, m21
    vpdpwssd             m4, m15, m21 ; g3
    vpermt2b             m1, m0, m2   ; 01 12
    vpermt2b             m2, m0, m3   ; 23 34
    vpermt2b             m3, m0, m4   ; 45 56
.hv_w8_loop:
    movu                ym0, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vinserti32x8         m0, [srcq+ssq*0], 1
    mova                 m4, m10
    vpermb              m21, m5, m0
    vpdpwssd             m4, m12, m21 ; h0 i0
    vpermb              m21, m6, m0
    pmaddwd             m20, m16, m1  ; A0 B0
    vpdpwssd             m4, m13, m21 ; h1 i1
    vpermb              m21, m7, m0
    mova                 m1, m2
    vpdpwssd            m20, m17, m2  ; A1 B1
    vpdpwssd             m4, m14, m21 ; h2 i2
    vpermb              m21, m8, m0
    mova                 m2, m3
    vpdpwssd            m20, m18, m3  ; A2 B2
    vpdpwssd             m4, m15, m21 ; h3 i3
    vpermt2b             m3, m9, m4   ; 67 78
    vpdpwssd            m20, m19, m3  ; A3 B3
    psrad               m20, 10
    vextracti32x8      ym21, m20, 1
    packusdw           ym20, ym21
    pminsw             ym20, ym11
    mova         [dstq+dsq*0], xm20
    vextracti128 [dstq+dsq*1], ym20, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w8_loop
    vzeroupper
    RET
.hv_w16:
    WIN64_SPILL_XMM 26
    vbroadcasti32x8      m5, [srcq+ssq*0+ 8]
    vinserti32x8         m4, m5, [srcq+ssq*0+ 0], 0
    vinserti32x8         m5, [srcq+ssq*0+16], 1 ; 0
    movu                ym6, [srcq+ssq*1+ 0]
    movu                ym7, [srcq+ssq*1+16]
    vinserti32x8         m6, [srcq+ssq*2+ 0], 1
    vinserti32x8         m7, [srcq+ssq*2+16], 1 ; 1 2
    add                srcq, r6
    movu               ym22, [srcq+ssq*0+ 0]
    movu               ym23, [srcq+ssq*0+16]
    vinserti32x8        m22, [srcq+ssq*1+ 0], 1
    vinserti32x8        m23, [srcq+ssq*1+16], 1 ; 3 4
    movu               ym24, [srcq+ssq*2+ 0]
    movu               ym25, [srcq+ssq*2+16]
    add                srcq, r6
    vinserti32x8        m24, [srcq+ssq*0+ 0], 1
    vinserti32x8        m25, [srcq+ssq*0+16], 1 ; 5 6
    vbroadcasti32x4     m20, [spel_h_shufA]
    vbroadcasti32x4     m21, [spel_h_shufB]
    mova                 m9, [spel_shuf16]
    pshufb               m0, m4, m20
    mova                 m1, m10
    vpdpwssd             m1, m12, m0    ; a0
    pshufb               m0, m6, m20
    mova                 m2, m10
    vpdpwssd             m2, m12, m0    ; b0
    pshufb               m0, m7, m20
    mova                 m3, m10
    vpdpwssd             m3, m14, m0    ; c2
    pshufb               m0, m4, m21
    vpdpwssd             m1, m13, m0    ; a1
    pshufb               m0, m6, m21
    vpdpwssd             m2, m13, m0    ; b1
    pshufb               m0, m7, m21
    vpdpwssd             m3, m15, m0    ; c3
    pshufb               m0, m5, m20
    vpdpwssd             m1, m14, m0    ; a2
    shufpd               m6, m7, 0x55
    pshufb               m7, m6, m20
    vpdpwssd             m2, m14, m7    ; b2
    vpdpwssd             m3, m12, m7    ; c0
    pshufb               m5, m21
    vpdpwssd             m1, m15, m5    ; a3
    pshufb               m6, m21
    vpdpwssd             m2, m15, m6    ; b3
    vpdpwssd             m3, m13, m6    ; c1
    pshufb               m0, m22, m20
    mova                 m4, m10
    vpdpwssd             m4, m12, m0    ; d0
    pshufb               m0, m23, m20
    mova                 m5, m10
    vpdpwssd             m5, m14, m0    ; e2
    pshufb               m0, m24, m20
    mova                 m6, m10
    vpdpwssd             m6, m12, m0    ; f0
    pshufb               m0, m25, m20
    mova                 m7, m10
    vpdpwssd             m7, m14, m0    ; g2
    pshufb               m0, m22, m21
    vpdpwssd             m4, m13, m0    ; d1
    pshufb               m0, m23, m21
    vpdpwssd             m5, m15, m0    ; e3
    pshufb               m0, m24, m21
    vpdpwssd             m6, m13, m0    ; f1
    pshufb               m0, m25, m21
    vpdpwssd             m7, m15, m0    ; g3
    shufpd              m22, m23, 0x55
    pshufb              m23, m22, m20
    vpdpwssd             m4, m14, m23   ; d2
    vpdpwssd             m5, m12, m23   ; e0
    shufpd              m24, m25, 0x55
    pshufb              m25, m24, m20
    vpdpwssd             m6, m14, m25   ; f2
    vpdpwssd             m7, m12, m25   ; g0
    pshufb              m22, m21
    vpdpwssd             m4, m15, m22   ; d3
    vpdpwssd             m5, m13, m22   ; e1
    pshufb              m24, m21
    vpdpwssd             m6, m15, m24   ; f3
    vpdpwssd             m7, m13, m24   ; g1
    pslldq               m1, 1
    vpermt2b             m2, m9, m3     ; 12
    vpermt2b             m4, m9, m5     ; 34
    vpermt2b             m6, m9, m7     ; 56
    vpshrdd              m1, m2, 16     ; 01
    vpshrdd              m3, m2, m4, 16 ; 23
    vpshrdd              m5, m4, m6, 16 ; 45
.hv_w16_loop:
    movu               ym24, [srcq+ssq*1+ 0]
    movu               ym25, [srcq+ssq*1+16]
    lea                srcq, [srcq+ssq*2]
    vinserti32x8        m24, [srcq+ssq*0+ 0], 1
    vinserti32x8        m25, [srcq+ssq*0+16], 1
    mova                 m7, m10
    mova                 m8, m10
    pshufb               m0, m24, m20
    vpdpwssd             m7, m12, m0    ; h0
    pshufb               m0, m25, m20
    vpdpwssd             m8, m14, m0    ; i2
    pmaddwd             m22, m16, m1    ; A0
    mova                 m1, m3
    pmaddwd             m23, m16, m2    ; B0
    mova                 m2, m4
    pshufb               m0, m24, m21
    vpdpwssd             m7, m13, m0    ; h1
    pshufb               m0, m25, m21
    vpdpwssd             m8, m15, m0    ; i3
    vpdpwssd            m22, m17, m3    ; A1
    mova                 m3, m5
    vpdpwssd            m23, m17, m4    ; B1
    mova                 m4, m6
    shufpd              m24, m25, 0x55
    pshufb              m25, m24, m20
    vpdpwssd             m7, m14, m25   ; h2
    vpdpwssd             m8, m12, m25   ; i0
    vpdpwssd            m22, m18, m5    ; A2
    vpdpwssd            m23, m18, m6    ; B2
    pshufb              m24, m21
    vpdpwssd             m7, m15, m24   ; h3
    vpdpwssd             m8, m13, m24   ; i1
    vpermt2b             m7, m9, m8     ; 78
    vpshrdd              m5, m6, m7, 16 ; 67
    vpdpwssd            m22, m19, m5    ; A3
    vpdpwssd            m23, m19, m7    ; B3
    mova                 m6, m7
    psrad               m22, 10
    psrad               m23, 10
    vshufi32x4           m0, m22, m23, q3232
    vinserti32x8        m22, ym23, 1
    packusdw            m22, m0
    pminsw              m22, m11
    mova          [dstq+dsq*0], ym22
    vextracti32x8 [dstq+dsq*1], m22, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w16_loop
    RET
.hv_w32:
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM 32
    vbroadcasti32x4     m20, [spel_h_shufA]
    vbroadcasti32x4     m21, [spel_h_shufB]
    mova                m22, [spel_shuf32]
    lea                  wd, [hq+wq*8-256]
    mov                  r7, srcq
    mov                  r8, dstq
.hv_w32_loop0:
    movu                 m6, [srcq+ssq*0+ 0]
    movu                 m7, [srcq+ssq*0+ 8]
    movu                 m8, [srcq+ssq*0+16]
    mova                 m0, m10
    mova                m23, m10
    pshufb               m9, m6, m20
    vpdpwssd             m0, m12, m9      ; a0l
    pshufb               m9, m7, m20
    vpdpwssd            m23, m12, m9      ; a0h
    vpdpwssd             m0, m14, m9      ; a2l
    pshufb               m7, m21
    vpdpwssd            m23, m13, m7      ; a1h
    vpdpwssd             m0, m15, m7      ; a3l
    pshufb               m7, m8, m20
    vpdpwssd            m23, m14, m7      ; a2h
    pshufb               m6, m21
    vpdpwssd             m0, m13, m6      ; a1l
    pshufb               m8, m21
    vpdpwssd            m23, m15, m8      ; a3h
%macro PUT_8TAP_HV_W32 5 ; dst_lo, dst_hi, stride_name, stride[1-2]
    movu                 m6, [srcq+%3*%4+ 0]
    movu                 m7, [srcq+%3*%4+ 8]
    movu                 m8, [srcq+%3*%4+16]
%if %4 == 2
    add                srcq, r6
%endif
    movu                m29, [srcq+%3*%5+ 0]
    movu                m30, [srcq+%3*%5+ 8]
    movu                m31, [srcq+%3*%5+16]
%if %5 == 2
    add                srcq, r6
%endif
    mova                m%1, m10
    mova                 m9, m10
    pshufb              m%2, m6, m20
    vpdpwssd            m%1, m12, m%2     ; x0l
    pshufb              m%2, m29, m20
    vpdpwssd             m9, m12, m%2     ; y0l
    pshufb               m6, m21
    vpdpwssd            m%1, m13, m6      ; x1l
    pshufb              m29, m21
    vpdpwssd             m9, m13, m29     ; y1l
    pshufb               m6, m7, m20
    mova                m%2, m10
    vpdpwssd            m%2, m12, m6      ; x0h
    pshufb              m29, m30, m20
    vpdpwssd            m%1, m14, m6      ; y2l
    mova                 m6, m10
    vpdpwssd             m6, m12, m29     ; x0h
    pshufb               m7, m21
    vpdpwssd             m9, m14, m29     ; y2l
    pshufb              m30, m21
    vpdpwssd            m%2, m13, m7      ; x1h
    vpdpwssd            m%1, m15, m7      ; x3l
    pshufb               m7, m8, m20
    vpdpwssd             m6, m13, m30     ; y1h
    vpdpwssd             m9, m15, m30     ; y3l
    pshufb              m30, m31, m20
    vpdpwssd            m%2, m14, m7      ; x2h
    pshufb               m8, m21
    vpdpwssd             m6, m14, m30     ; y2h
    pshufb              m31, m21
    vpdpwssd            m%2, m15, m8      ; x3h
    vpdpwssd             m6, m15, m31     ; y3h
%if %1 == 1
    vpermt2b             m0, m22, m%1     ; 01l
    vpermt2b            m23, m22, m%2     ; 01h
%endif
    vpermt2b            m%1, m22, m9      ; xyl
    vpermt2b            m%2, m22, m6      ; xyh
%endmacro
    PUT_8TAP_HV_W32       1, 24, ssq, 1, 2 ; 12
    PUT_8TAP_HV_W32       3, 26, ssq, 0, 1 ; 34
    PUT_8TAP_HV_W32       5, 28, ssq, 2, 0 ; 56
    vpshrdd              m2, m1, m3, 16   ; 23l
    vpshrdd             m25, m24, m26, 16 ; 23h
    vpshrdd              m4, m3, m5, 16   ; 45l
    vpshrdd             m27, m26, m28, 16 ; 45h
.hv_w32_loop:
    movu                 m7, [srcq+ssq*1+ 0]
    movu                 m9, [srcq+ssq*2+ 0]
    movu                 m6, [srcq+ssq*1+ 8]
    movu                 m8, [srcq+ssq*2+ 8]
    mova                m29, m10
    mova                m31, m10
    pshufb              m30, m7, m20
    vpdpwssd            m29, m12, m30     ; h0l
    pshufb              m30, m9, m20
    vpdpwssd            m31, m12, m30     ; i0l
    pshufb               m7, m21
    vpdpwssd            m29, m13, m7      ; h1l
    pshufb               m9, m21
    vpdpwssd            m31, m13, m9      ; i1l
    pshufb               m7, m6, m20
    vpdpwssd            m29, m14, m7      ; h2l
    pshufb               m9, m8, m20
    vpdpwssd            m31, m14, m9      ; i2l
    pshufb               m6, m21
    vpdpwssd            m29, m15, m6      ; h3l
    pshufb               m8, m21
    vpdpwssd            m31, m15, m8      ; i3l
    mova                m30, m10
    vpdpwssd            m30, m12, m7      ; h0h
    movu                 m7, [srcq+ssq*1+16]
    lea                srcq, [srcq+ssq*2]
    vpermt2b            m29, m22, m31     ; 78l
    mova                m31, m10
    vpdpwssd            m31, m12, m9      ; i0h
    movu                 m9, [srcq+ssq*0+16]
    vpdpwssd            m30, m13, m6      ; h1h
    pshufb               m6, m7, m20
    vpdpwssd            m31, m13, m8      ; i1h
    pshufb               m8, m9, m20
    vpdpwssd            m30, m14, m6      ; h2h
    pmaddwd              m6, m16, m0      ; A0l
    pshufb               m7, m21
    vpdpwssd            m31, m14, m8      ; i2h
    pmaddwd              m8, m16, m23     ; A0h
    pshufb               m9, m21
    vpdpwssd            m30, m15, m7      ; h3h
    pmaddwd              m7, m16, m1      ; B0l
    vpdpwssd            m31, m15, m9      ; i3h
    pmaddwd              m9, m16, m24     ; B0h
    mova                 m0, m2
    vpdpwssd             m6, m17, m2      ; A1l
    mova                m23, m25
    vpdpwssd             m8, m17, m25     ; A1h
    mova                 m1, m3
    vpdpwssd             m7, m17, m3      ; B1l
    mova                m24, m26
    vpdpwssd             m9, m17, m26     ; B1h
    vpermt2b            m30, m22, m31     ; 78h
    vpdpwssd             m6, m18, m4      ; A2l
    mova                 m2, m4
    vpdpwssd             m8, m18, m27     ; A2h
    mova                m25, m27
    vpdpwssd             m7, m18, m5      ; B2l
    mova                 m3, m5
    vpdpwssd             m9, m18, m28     ; B2h
    mova                m26, m28
    vpshrdd              m4, m5, m29, 16  ; 67l
    vpdpwssd             m6, m19, m4      ; A3l
    vpshrdd             m27, m28, m30, 16 ; 67h
    vpdpwssd             m8, m19, m27     ; A3h
    mova                 m5, m29
    vpdpwssd             m7, m19, m29     ; B3l
    mova                m28, m30
    vpdpwssd             m9, m19, m30     ; B3h
    REPX      {psrad x, 10}, m6, m8, m7, m9
    packusdw             m6, m8
    packusdw             m7, m9
    pminsw               m6, m11
    pminsw               m7, m11
    mova       [dstq+dsq*0], m6
    mova       [dstq+dsq*1], m7
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w32_loop
    add                  r7, 64
    add                  r8, 64
    movzx                hd, wb
    mov                srcq, r7
    mov                dstq, r8
    sub                  wd, 1<<8
    jg .hv_w32_loop0
    RET

%if WIN64
DECLARE_REG_TMP 6, 4
%else
DECLARE_REG_TMP 6, 7
%endif

MC_8TAP_FN prep, sharp,          SHARP,   SHARP
MC_8TAP_FN prep, sharp_smooth,   SHARP,   SMOOTH
MC_8TAP_FN prep, smooth_sharp,   SMOOTH,  SHARP
MC_8TAP_FN prep, smooth,         SMOOTH,  SMOOTH
MC_8TAP_FN prep, sharp_regular,  SHARP,   REGULAR
MC_8TAP_FN prep, regular_sharp,  REGULAR, SHARP
MC_8TAP_FN prep, smooth_regular, SMOOTH,  REGULAR
MC_8TAP_FN prep, regular_smooth, REGULAR, SMOOTH
MC_8TAP_FN prep, regular,        REGULAR, REGULAR

cglobal prep_8tap_16bpc, 3, 8, 16, tmp, src, stride, w, h, mx, my, stride3
%define base r7-prep_avx512icl
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 8tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 8tap_v, my, 4tap_v
    lea                  r7, [prep_avx512icl]
    mov                  wd, wm
    movifnidn            hd, hm
    test                mxd, 0xf00
    jnz .h
    test                myd, 0xf00
    jnz .v
    tzcnt                wd, wd
    mov                 r5d, r7m ; bitdepth_max
    vpbroadcastd         m5, [pw_8192]
    movzx                wd, word [r7+wq*2+table_offset(prep,)]
    shr                 r5d, 11
    vpbroadcastd         m4, [r7-prep_avx512icl+prep_mul+r5*4]
    add                  wq, r7
    lea                  r6, [strideq*3]
%if WIN64
    pop                  r7
%endif
    jmp                  wq
.h_w4:
    movzx               mxd, mxb
    sub                srcq, 2
    pmovsxbw           xmm0, [base+subpel_filters+mxq*8]
    mov                 r5d, r7m
    vbroadcasti32x4      m4, [spel_h_shufA]
    vbroadcasti32x4      m5, [spel_h_shufB]
    shr                 r5d, 11
    mova                ym9, [prep_endA]
    psllw              xmm0, [base+prep_hv_shift+r5*8]
    mova             [tmpq], xmm0
    vpbroadcastd         m6, [tmpq+4]
    vpbroadcastd         m7, [tmpq+8]
.h_w4_loop:
    movu                xm2, [srcq+strideq*0]
    vinserti32x4        ym2, [srcq+strideq*1], 1
    vinserti32x4         m2, [srcq+strideq*2], 2
    vinserti32x4         m2, [srcq+r6       ], 3
    lea                srcq, [srcq+strideq*4]
    mova                 m0, m10
    pshufb               m1, m2, m4
    vpdpwssd             m0, m6, m1
    pshufb               m2, m5
    vpdpwssd             m0, m7, m2
    vpermb               m0, m9, m0
    mova             [tmpq], ym0
    add                tmpq, 32
    sub                  hd, 4
    jg .h_w4_loop
    RET
.h:
    test                myd, 0xf00
    jnz .hv
    vpbroadcastd        m10, [prep_8tap_rnd]
    lea                  r6, [strideq*3]
    cmp                  wd, 4
    je .h_w4
    shr                 mxd, 16
    pmovsxbw           xmm0, [base+subpel_filters+mxq*8]
    mov                 r5d, r7m
    sub                srcq, 6
    shr                 r5d, 11
    psllw              xmm0, [base+prep_hv_shift+r5*8]
    mova             [tmpq], xmm0
    vpbroadcastd        m12, xmm0
    vpbroadcastd        m13, [tmpq+ 4]
    vpbroadcastd        m14, [tmpq+ 8]
    vpbroadcastd        m15, [tmpq+12]
    cmp                  wd, 16
    je .h_w16
    jg .h_w32
.h_w8:
    mova                 m6, [spel_h_shufA]
    movu                 m7, [spel_h_shufB]
    movu                 m8, [spel_h_shufC]
    mova                 m9, [spel_h_shufD]
    mova                m11, [prep_endB]
.h_w8_loop:
    movu                ym4, [srcq+strideq*0]
    vinserti32x8         m4, [srcq+strideq*1], 1
    movu                ym5, [srcq+strideq*2]
    vinserti32x8         m5, [srcq+r6       ], 1
    lea                srcq, [srcq+strideq*4]
    mova                 m0, m10
    mova                 m1, m10
    vpermb               m2, m6, m4
    vpermb               m3, m6, m5
    vpdpwssd             m0, m12, m2
    vpdpwssd             m1, m12, m3
    vpermb               m2, m7, m4
    vpermb               m3, m7, m5
    vpdpwssd             m0, m13, m2
    vpdpwssd             m1, m13, m3
    vpermb               m2, m8, m4
    vpermb               m3, m8, m5
    vpdpwssd             m0, m14, m2
    vpdpwssd             m1, m14, m3
    vpermb               m2, m9, m4
    vpermb               m3, m9, m5
    vpdpwssd             m0, m15, m2
    vpdpwssd             m1, m15, m3
    vpermt2b             m0, m11, m1
    mova             [tmpq], m0
    add                tmpq, 64
    sub                  hd, 4
    jg .h_w8_loop
    RET
.h_w16:
    vbroadcasti32x4      m6, [spel_h_shufA]
    vbroadcasti32x4      m7, [spel_h_shufB]
    mova                m11, [prep_endC]
.h_w16_loop:
    movu                ym2, [srcq+strideq*0+ 0]
    vinserti32x8         m2, [srcq+strideq*1+ 0], 1
    movu                ym3, [srcq+strideq*0+16]
    vinserti32x8         m3, [srcq+strideq*1+16], 1
    lea                srcq, [srcq+strideq*2]
    mova                 m0, m10
    mova                 m1, m10
    pshufb               m4, m2, m6
    vpdpwssd             m0, m12, m4 ; a0
    pshufb               m4, m3, m6
    vpdpwssd             m1, m14, m4 ; b2
    pshufb               m4, m2, m7
    vpdpwssd             m0, m13, m4 ; a1
    pshufb               m4, m3, m7
    vpdpwssd             m1, m15, m4 ; b3
    shufpd               m2, m3, 0x55
    pshufb               m4, m2, m6
    vpdpwssd             m0, m14, m4 ; a2
    vpdpwssd             m1, m12, m4 ; b0
    pshufb               m2, m7
    vpdpwssd             m0, m15, m2 ; a3
    vpdpwssd             m1, m13, m2 ; b1
    vpermt2b             m0, m11, m1
    mova             [tmpq], m0
    add                tmpq, 64
    sub                  hd, 2
    jg .h_w16_loop
    RET
.h_w32:
    vbroadcasti32x4      m6, [spel_h_shufA]
    lea                srcq, [srcq+wq*2]
    vbroadcasti32x4      m7, [spel_h_shufB]
    neg                  wq
    mova                m11, [prep_endC]
.h_w32_loop0:
    mov                  r6, wq
.h_w32_loop:
    movu                 m2, [srcq+r6*2+ 0]
    movu                 m3, [srcq+r6*2+ 8]
    mova                 m0, m10
    mova                 m1, m10
    pshufb               m4, m2, m6
    vpdpwssd             m0, m12, m4 ; a0
    pshufb               m4, m3, m6
    vpdpwssd             m1, m12, m4 ; b0
    vpdpwssd             m0, m14, m4 ; a2
    movu                 m4, [srcq+r6*2+16]
    pshufb               m3, m7
    vpdpwssd             m1, m13, m3 ; b1
    vpdpwssd             m0, m15, m3 ; a3
    pshufb               m3, m4, m6
    vpdpwssd             m1, m14, m3 ; b2
    pshufb               m2, m7
    vpdpwssd             m0, m13, m2 ; a1
    pshufb               m4, m7
    vpdpwssd             m1, m15, m4 ; b3
    vpermt2b             m0, m11, m1
    mova             [tmpq], m0
    add                tmpq, 64
    add                  r6, 32
    jl .h_w32_loop
    add                srcq, strideq
    dec                  hd
    jg .h_w32_loop0
    RET
.v:
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmove               myd, mxd
    mov                 r5d, r7m
    vpbroadcastd        m10, [prep_8tap_rnd]
    pmovsxbw           xmm0, [base+subpel_filters+myq*8]
    tzcnt               r6d, wd
    shr                 r5d, 11
    movzx               r6d, word [r7+r6*2+table_offset(prep, _8tap_v)]
    psllw              xmm0, [base+prep_hv_shift+r5*8]
    add                  r7, r6
    lea                  r6, [strideq*3]
    sub                srcq, r6
    mova             [tmpq], xmm0
    vpbroadcastd        m12, xmm0
    vpbroadcastd        m13, [tmpq+ 4]
    vpbroadcastd        m14, [tmpq+ 8]
    vpbroadcastd        m15, [tmpq+12]
    jmp                  r7
.v_w4:
    movq               xmm1, [srcq+strideq*0]
    vpbroadcastq       ymm0, [srcq+strideq*1]
    vpbroadcastq       ymm2, [srcq+strideq*2]
    add                srcq, r6
    vpbroadcastq       ymm4, [srcq+strideq*0]
    vpbroadcastq       ymm3, [srcq+strideq*1]
    vpbroadcastq       ymm5, [srcq+strideq*2]
    mova               xm11, [prep_endA]
    add                srcq, r6
    vpblendd           ymm1, ymm0, 0x30
    vpblendd           ymm0, ymm2, 0x30
    punpcklwd          ymm1, ymm0       ; 01 12
    vpbroadcastq       ymm0, [srcq+strideq*0]
    vpblendd           ymm2, ymm4, 0x30
    vpblendd           ymm4, ymm3, 0x30
    punpcklwd          ymm2, ymm4       ; 23 34
    vpblendd           ymm3, ymm5, 0x30
    vpblendd           ymm5, ymm0, 0x30
    punpcklwd          ymm3, ymm5       ; 45 56
.v_w4_loop:
    vpbroadcastq       ymm5, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    mova               ymm4, ym10
    vpdpwssd           ymm4, ym12, ymm1 ; a0 b0
    mova               ymm1, ymm2
    vpdpwssd           ymm4, ym13, ymm2 ; a1 b1
    mova               ymm2, ymm3
    vpdpwssd           ymm4, ym14, ymm3 ; a2 b2
    vpblendd           ymm3, ymm0, ymm5, 0x30
    vpbroadcastq       ymm0, [srcq+strideq*0]
    vpblendd           ymm5, ymm0, 0x30
    punpcklwd          ymm3, ymm5       ; 67 78
    vpdpwssd           ymm4, ym15, ymm3 ; a3 b3
    vpermb             ymm4, ym11, ymm4
    mova             [tmpq], xmm4
    add                tmpq, 16
    sub                  hd, 2
    jg .v_w4_loop
    vzeroupper
    RET
.v_w8:
    vbroadcasti32x4      m2, [srcq+strideq*2]
    vinserti32x4         m1, m2, [srcq+strideq*0], 0
    vinserti32x4         m1, [srcq+strideq*1], 1 ; 0 1 2
    add                srcq, r6
    vinserti32x4        ym2, [srcq+strideq*0], 1
    vinserti32x4         m2, [srcq+strideq*1], 2 ; 2 3 4
    mova                 m6, [spel_v_shuf8]
    movu                xm0, [srcq+strideq*1]
    vinserti32x4        ym0, [srcq+strideq*2], 1
    add                srcq, r6
    vinserti32x4         m0, [srcq+strideq*0], 2 ; 4 5 6
    mova               ym11, [prep_endB]
    vpermb               m1, m6, m1          ; 01 12
    vpermb               m2, m6, m2          ; 23 34
    vpermb               m3, m6, m0          ; 45 56
.v_w8_loop:
    vinserti32x4         m0, [srcq+strideq*1], 3
    lea                srcq, [srcq+strideq*2]
    movu                xm5, [srcq+strideq*0]
    mova                 m4, m10
    vpdpwssd             m4, m12, m1         ; a0 b0
    mova                 m1, m2
    vshufi32x4           m0, m5, q1032       ; 6 7 8
    vpdpwssd             m4, m13, m2         ; a1 b1
    mova                 m2, m3
    vpdpwssd             m4, m14, m3         ; a2 b2
    vpermb               m3, m6, m0          ; 67 78
    vpdpwssd             m4, m15, m3         ; a3 b3
    vpermb               m4, m11, m4
    mova             [tmpq], ym4
    add                tmpq, 32
    sub                  hd, 2
    jg .v_w8_loop
    RET
.v_w16:
    vbroadcasti32x8      m1, [srcq+strideq*1]
    vinserti32x8         m0, m1, [srcq+strideq*0], 0
    vinserti32x8         m1, [srcq+strideq*2], 1
    mova                 m8, [spel_v_shuf16]
    add                srcq, r6
    movu                ym3, [srcq+strideq*0]
    vinserti32x8         m3, [srcq+strideq*1], 1
    movu                ym5, [srcq+strideq*2]
    add                srcq, r6
    vinserti32x8         m5, [srcq+strideq*0], 1
    mova                m11, [prep_endA]
    vpermb               m0, m8, m0     ; 01
    vpermb               m1, m8, m1     ; 12
    vpermb               m3, m8, m3     ; 34
    vpermb               m5, m8, m5     ; 56
    vpshrdd              m2, m1, m3, 16 ; 23
    vpshrdd              m4, m3, m5, 16 ; 45
.v_w16_loop:
    mova                 m6, m10
    mova                 m7, m10
    vpdpwssd             m6, m12, m0    ; a0
    mova                 m0, m2
    vpdpwssd             m7, m12, m1    ; b0
    mova                 m1, m3
    vpdpwssd             m6, m13, m2    ; a1
    mova                 m2, m4
    vpdpwssd             m7, m13, m3    ; b1
    mova                 m3, m5
    vpdpwssd             m6, m14, m4    ; a2
    mova                 m4, m5
    vpdpwssd             m7, m14, m5    ; b2
    movu                ym5, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    vinserti32x8         m5, [srcq+strideq*0], 1
    vpermb               m5, m8, m5     ; 78
    vpshrdd              m4, m5, 16     ; 67
    vpdpwssd             m6, m15, m4    ; a3
    vpdpwssd             m7, m15, m5    ; b3
    vpermt2b             m6, m11, m7
    mova             [tmpq], m6
    add                tmpq, 64
    sub                  hd, 2
    jg .v_w16_loop
    RET
.v_w32:
.v_w64:
.v_w128:
%if WIN64
    PUSH                 r8
    movaps [rsp+stack_offset+8], xmm6
%endif
    lea                  r5, [hq+wq*8-256]
    mov                  r7, srcq
    mov                  r8, tmpq
.v_w32_loop0:
    movu                m16, [srcq+strideq*0]
    movu                m17, [srcq+strideq*1]
    movu                m18, [srcq+strideq*2]
    add                srcq, r6
    movu                m19, [srcq+strideq*0]
    movu                m20, [srcq+strideq*1]
    movu                m21, [srcq+strideq*2]
    add                srcq, r6
    movu                m22, [srcq+strideq*0]
    mova                m11, [prep_endC]
    punpcklwd            m0, m16, m17 ; 01l
    punpckhwd           m16, m17      ; 01h
    punpcklwd            m1, m17, m18 ; 12l
    punpckhwd           m17, m18      ; 12h
    punpcklwd            m2, m18, m19 ; 23l
    punpckhwd           m18, m19      ; 23h
    punpcklwd            m3, m19, m20 ; 34l
    punpckhwd           m19, m20      ; 34h
    punpcklwd            m4, m20, m21 ; 45l
    punpckhwd           m20, m21      ; 45h
    punpcklwd            m5, m21, m22 ; 56l
    punpckhwd           m21, m22      ; 56h
.v_w32_loop:
    mova                 m6, m10
    vpdpwssd             m6, m12, m0  ; a0l
    mova                 m8, m10
    vpdpwssd             m8, m12, m16 ; a0h
    mova                 m7, m10
    vpdpwssd             m7, m12, m1  ; b0l
    mova                 m9, m10
    vpdpwssd             m9, m12, m17 ; b0h
    mova                 m0, m2
    vpdpwssd             m6, m13, m2  ; a1l
    mova                m16, m18
    vpdpwssd             m8, m13, m18 ; a1h
    mova                 m1, m3
    vpdpwssd             m7, m13, m3  ; b1l
    mova                m17, m19
    vpdpwssd             m9, m13, m19 ; b1h
    mova                 m2, m4
    vpdpwssd             m6, m14, m4  ; a2l
    mova                m18, m20
    vpdpwssd             m8, m14, m20 ; a2h
    mova                 m3, m5
    vpdpwssd             m7, m14, m5  ; b2l
    mova                m19, m21
    vpdpwssd             m9, m14, m21 ; b2h
    movu                m21, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    punpcklwd            m4, m22, m21 ; 67l
    punpckhwd           m20, m22, m21 ; 67h
    movu                m22, [srcq+strideq*0]
    vpdpwssd             m6, m15, m4  ; a3l
    vpdpwssd             m8, m15, m20 ; a3h
    punpcklwd            m5, m21, m22 ; 78l
    punpckhwd           m21, m22      ; 78h
    vpdpwssd             m7, m15, m5  ; b3l
    vpdpwssd             m9, m15, m21 ; b3h
    vpermt2b             m6, m11, m8
    vpermt2b             m7, m11, m9
    mova        [tmpq+wq*0], m6
    mova        [tmpq+wq*2], m7
    lea                tmpq, [tmpq+wq*4]
    sub                  hd, 2
    jg .v_w32_loop
    add                  r7, 64
    add                  r8, 64
    movzx                hd, r5b
    mov                srcq, r7
    mov                tmpq, r8
    sub                 r5d, 1<<8
    jg .v_w32_loop0
%if WIN64
    movaps             xmm6, [rsp+stack_offset+8]
    POP                  r8
%endif
    vzeroupper
    RET
.hv:
    cmp                  wd, 4
    jg .hv_w8
    movzx               mxd, mxb
    pmovsxbw           xmm0, [base+subpel_filters+mxq*8]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmove               myd, mxd
    mov                 r5d, r7m
    pmovsxbw           xmm1, [base+subpel_filters+myq*8]
    lea                  r6, [strideq*3]
    sub                srcq, 2
    shr                 r5d, 11
    sub                srcq, r6
    psllw              xmm0, [base+prep_hv_shift+r5*8]
    psllw              xmm1, 2
    vpbroadcastd        m10, [prep_8tap_rnd]
    vpbroadcastd       ym11, [pd_128]
    mova               xm21, [prep_endA]
    mova          [tmpq+ 0], xmm0
    mova          [tmpq+16], xmm1
    vpbroadcastd         m8, [tmpq+ 4]
    vpbroadcastd         m9, [tmpq+ 8]
    vpbroadcastd       ym12, xmm1
    vpbroadcastd       ym13, [tmpq+20]
    vpbroadcastd       ym14, [tmpq+24]
    vpbroadcastd       ym15, [tmpq+28]
    movu                xm4, [srcq+strideq*0]
    vinserti32x4        ym4, [srcq+strideq*1], 1
    vinserti32x4         m4, [srcq+strideq*2], 2
    add                srcq, r6
    vinserti32x4         m4, [srcq+strideq*0], 3 ; 0 1 2 3
    movu                xm0, [srcq+strideq*1]
    vinserti32x4        ym0, [srcq+strideq*2], 1
    add                srcq, r6
    vinserti32x4         m0, [srcq+strideq*0], 2 ; 4 5 6
    vbroadcasti32x4     m19, [spel_h_shufA]
    vbroadcasti32x4     m20, [spel_h_shufB]
    mova                ym6, [spel_shuf4a]
    mova                ym7, [spel_shuf4b]
    mova                 m2, m10
    mova                 m3, m10
    pshufb               m1, m4, m19
    vpdpwssd             m2, m8, m1
    pshufb               m1, m0, m19
    vpdpwssd             m3, m8, m1
    pshufb               m4, m20
    vpdpwssd             m2, m9, m4
    pshufb               m0, m20
    vpdpwssd             m3, m9, m0
    vpermb               m1, m6, m2    ; 01 12
    vshufi32x4           m2, m3, q1032
    vpermb               m3, m6, m3    ; 45 56
    vpermb               m2, m6, m2    ; 23 34
.hv_w4_loop:
    movu               xm18, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    vinserti128        ym18, [srcq+strideq*0], 1
    mova               ym16, ym11
    mova                ym4, ym10
    pshufb             ym17, ym18, ym19
    vpdpwssd           ym16, ym12, ym1 ; a0 b0
    vpdpwssd            ym4, ym8, ym17
    pshufb             ym18, ym20
    mova                ym1, ym2
    vpdpwssd           ym16, ym13, ym2 ; a1 b1
    vpdpwssd            ym4, ym9, ym18 ; 7 8
    mova                ym2, ym3
    vpdpwssd           ym16, ym14, ym3 ; a2 b2
    vpermt2b            ym3, ym7, ym4  ; 67 78
    vpdpwssd           ym16, ym15, ym3 ; a3 b3
    vpermb             ym16, ym21, ym16
    mova             [tmpq], xm16
    add                tmpq, 16
    sub                  hd, 2
    jg .hv_w4_loop
    vzeroupper
    RET
.hv_w8:
    shr                 mxd, 16
    pmovsxbw           xmm0, [base+subpel_filters+mxq*8]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    mov                 r5d, r7m
    pmovsxbw           xmm1, [base+subpel_filters+myq*8]
    lea                  r6, [strideq*3]
    sub                srcq, 6
    shr                 r5d, 11
    sub                srcq, r6
    vpbroadcastd        m10, [prep_8tap_rnd]
    vpbroadcastd        m11, [pd_128]
    psllw              xmm0, [base+prep_hv_shift+r5*8]
    psllw              xmm1, 2
    mova          [tmpq+ 0], xmm0
    mova          [tmpq+16], xmm1
    vpbroadcastd        m12, xmm0
    vpbroadcastd        m13, [tmpq+ 4]
    vpbroadcastd        m14, [tmpq+ 8]
    vpbroadcastd        m15, [tmpq+12]
    vpbroadcastd        m16, xmm1
    vpbroadcastd        m17, [tmpq+20]
    vpbroadcastd        m18, [tmpq+24]
    vpbroadcastd        m19, [tmpq+28]
    cmp                  wd, 16
    je .hv_w16
    jg .hv_w32
    WIN64_SPILL_XMM 23
    mova                 m5, [spel_h_shufA]
    movu                ym0, [srcq+strideq*0]
    vinserti32x8         m0, [srcq+strideq*1], 1 ; 0 1
    movu                ym9, [srcq+strideq*2]
    add                srcq, r6
    vinserti32x8         m9, [srcq+strideq*0], 1 ; 2 3
    movu               ym20, [srcq+strideq*1]
    vinserti32x8        m20, [srcq+strideq*2], 1 ; 4 5
    add                srcq, r6
    movu               ym21, [srcq+strideq*0]    ; 6
    movu                 m6, [spel_h_shufB]
    movu                 m7, [spel_h_shufC]
    mova               ym22, [prep_endB]
    vpermb               m8, m5, m0
    mova                 m1, m10
    vpdpwssd             m1, m12, m8  ; a0 b0
    vpermb               m8, m5, m9
    mova                 m2, m10
    vpdpwssd             m2, m12, m8  ; c0 d0
    vpermb               m8, m5, m20
    mova                 m3, m10
    vpdpwssd             m3, m12, m8  ; e0 f0
    vpermb               m8, m5, m21
    mova                 m4, m10
    vpdpwssd             m4, m12, m8  ; g0
    vpermb               m8, m6, m0
    vpdpwssd             m1, m13, m8  ; a1 b1
    vpermb               m8, m6, m9
    vpdpwssd             m2, m13, m8  ; c1 d1
    vpermb               m8, m6, m20
    vpdpwssd             m3, m13, m8  ; e1 f1
    vpermb               m8, m6, m21
    vpdpwssd             m4, m13, m8  ; g1
    vpermb               m8, m7, m0
    vpdpwssd             m1, m14, m8  ; a2 b2
    vpermb               m8, m7, m9
    vpdpwssd             m2, m14, m8  ; c2 d2
    vpermb               m8, m7, m20
    vpdpwssd             m3, m14, m8  ; e2 f2
    vpermb               m8, m7, m21
    vpdpwssd             m4, m14, m8  ; g2
    mova                 m8, [spel_h_shufD]
    vpermb               m0, m8, m0
    vpdpwssd             m1, m15, m0  ; a3 b3
    mova                 m0, [spel_shuf8a]
    vpermb               m9, m8, m9
    vpdpwssd             m2, m15, m9  ; c3 d3
    mova                 m9, [spel_shuf8b]
    vpermb              m20, m8, m20
    vpdpwssd             m3, m15, m20 ; e3 f3
    vpermb              m21, m8, m21
    vpdpwssd             m4, m15, m21 ; g3
    vpermt2b             m1, m0, m2   ; 01 12
    vpermt2b             m2, m0, m3   ; 23 34
    vpermt2b             m3, m0, m4   ; 45 56
.hv_w8_loop:
    movu                ym0, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    vinserti32x8         m0, [srcq+strideq*0], 1
    mova                 m4, m10
    mova                m20, m11
    vpermb              m21, m5, m0
    vpdpwssd             m4, m12, m21 ; h0 i0
    vpermb              m21, m6, m0
    vpdpwssd            m20, m16, m1  ; A0 B0
    vpdpwssd             m4, m13, m21 ; h1 i1
    vpermb              m21, m7, m0
    mova                 m1, m2
    vpdpwssd            m20, m17, m2  ; A1 B1
    vpdpwssd             m4, m14, m21 ; h2 i2
    vpermb              m21, m8, m0
    mova                 m2, m3
    vpdpwssd            m20, m18, m3  ; A2 B2
    vpdpwssd             m4, m15, m21 ; h3 i3
    vpermt2b             m3, m9, m4   ; 67 78
    vpdpwssd            m20, m19, m3  ; A3 B3
    vpermb              m20, m22, m20
    mova             [tmpq], ym20
    add                tmpq, 32
    sub                  hd, 2
    jg .hv_w8_loop
    RET
.hv_w16:
    %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM 27
    vbroadcasti32x8      m5, [srcq+strideq*0+ 8]
    vinserti32x8         m4, m5, [srcq+strideq*0+ 0], 0
    vinserti32x8         m5, [srcq+strideq*0+16], 1 ; 0
    movu                ym6, [srcq+strideq*1+ 0]
    movu                ym7, [srcq+strideq*1+16]
    vinserti32x8         m6, [srcq+strideq*2+ 0], 1
    vinserti32x8         m7, [srcq+strideq*2+16], 1 ; 1 2
    add                srcq, r6
    movu               ym22, [srcq+strideq*0+ 0]
    movu               ym23, [srcq+strideq*0+16]
    vinserti32x8        m22, [srcq+strideq*1+ 0], 1
    vinserti32x8        m23, [srcq+strideq*1+16], 1 ; 3 4
    movu               ym24, [srcq+strideq*2+ 0]
    movu               ym25, [srcq+strideq*2+16]
    add                srcq, r6
    vinserti32x8        m24, [srcq+strideq*0+ 0], 1
    vinserti32x8        m25, [srcq+strideq*0+16], 1 ; 5 6
    vbroadcasti32x4     m20, [spel_h_shufA]
    vbroadcasti32x4     m21, [spel_h_shufB]
    mova                 m9, [spel_shuf16]
    mova                m26, [prep_endB]
    pshufb               m0, m4, m20
    mova                 m1, m10
    vpdpwssd             m1, m12, m0    ; a0
    pshufb               m0, m6, m20
    mova                 m2, m10
    vpdpwssd             m2, m12, m0    ; b0
    pshufb               m0, m7, m20
    mova                 m3, m10
    vpdpwssd             m3, m14, m0    ; c2
    pshufb               m0, m4, m21
    vpdpwssd             m1, m13, m0    ; a1
    pshufb               m0, m6, m21
    vpdpwssd             m2, m13, m0    ; b1
    pshufb               m0, m7, m21
    vpdpwssd             m3, m15, m0    ; c3
    pshufb               m0, m5, m20
    vpdpwssd             m1, m14, m0    ; a2
    shufpd               m6, m7, 0x55
    pshufb               m7, m6, m20
    vpdpwssd             m2, m14, m7    ; b2
    vpdpwssd             m3, m12, m7    ; c0
    pshufb               m5, m21
    vpdpwssd             m1, m15, m5    ; a3
    pshufb               m6, m21
    vpdpwssd             m2, m15, m6    ; b3
    vpdpwssd             m3, m13, m6    ; c1
    pshufb               m0, m22, m20
    mova                 m4, m10
    vpdpwssd             m4, m12, m0    ; d0
    pshufb               m0, m23, m20
    mova                 m5, m10
    vpdpwssd             m5, m14, m0    ; e2
    pshufb               m0, m24, m20
    mova                 m6, m10
    vpdpwssd             m6, m12, m0    ; f0
    pshufb               m0, m25, m20
    mova                 m7, m10
    vpdpwssd             m7, m14, m0    ; g2
    pshufb               m0, m22, m21
    vpdpwssd             m4, m13, m0    ; d1
    pshufb               m0, m23, m21
    vpdpwssd             m5, m15, m0    ; e3
    pshufb               m0, m24, m21
    vpdpwssd             m6, m13, m0    ; f1
    pshufb               m0, m25, m21
    vpdpwssd             m7, m15, m0    ; g3
    shufpd              m22, m23, 0x55
    pshufb              m23, m22, m20
    vpdpwssd             m4, m14, m23   ; d2
    vpdpwssd             m5, m12, m23   ; e0
    shufpd              m24, m25, 0x55
    pshufb              m25, m24, m20
    vpdpwssd             m6, m14, m25   ; f2
    vpdpwssd             m7, m12, m25   ; g0
    pshufb              m22, m21
    vpdpwssd             m4, m15, m22   ; d3
    vpdpwssd             m5, m13, m22   ; e1
    pshufb              m24, m21
    vpdpwssd             m6, m15, m24   ; f3
    vpdpwssd             m7, m13, m24   ; g1
    pslldq               m1, 1
    vpermt2b             m2, m9, m3     ; 12
    vpermt2b             m4, m9, m5     ; 34
    vpermt2b             m6, m9, m7     ; 56
    vpshrdd              m1, m2, 16     ; 01
    vpshrdd              m3, m2, m4, 16 ; 23
    vpshrdd              m5, m4, m6, 16 ; 45
.hv_w16_loop:
    movu               ym24, [srcq+strideq*1+ 0]
    movu               ym25, [srcq+strideq*1+16]
    lea                srcq, [srcq+strideq*2]
    vinserti32x8        m24, [srcq+strideq*0+ 0], 1
    vinserti32x8        m25, [srcq+strideq*0+16], 1
    mova                 m7, m10
    mova                 m8, m10
    pshufb               m0, m24, m20
    vpdpwssd             m7, m12, m0    ; h0
    mova                m22, m11
    pshufb               m0, m25, m20
    vpdpwssd             m8, m14, m0    ; i2
    mova                m23, m11
    vpdpwssd            m22, m16, m1    ; A0
    mova                 m1, m3
    vpdpwssd            m23, m16, m2    ; B0
    mova                 m2, m4
    pshufb               m0, m24, m21
    vpdpwssd             m7, m13, m0    ; h1
    pshufb               m0, m25, m21
    vpdpwssd             m8, m15, m0    ; i3
    vpdpwssd            m22, m17, m3    ; A1
    mova                 m3, m5
    vpdpwssd            m23, m17, m4    ; B1
    mova                 m4, m6
    shufpd              m24, m25, 0x55
    pshufb              m25, m24, m20
    vpdpwssd             m7, m14, m25   ; h2
    vpdpwssd             m8, m12, m25   ; i0
    vpdpwssd            m22, m18, m5    ; A2
    vpdpwssd            m23, m18, m6    ; B2
    pshufb              m24, m21
    vpdpwssd             m7, m15, m24   ; h3
    vpdpwssd             m8, m13, m24   ; i1
    vpermt2b             m7, m9, m8     ; 78
    vpshrdd              m5, m6, m7, 16 ; 67
    vpdpwssd            m22, m19, m5    ; A3
    vpdpwssd            m23, m19, m7    ; B3
    mova                 m6, m7
    vpermt2b            m22, m26, m23
    mova             [tmpq], m22
    add                tmpq, 64
    sub                  hd, 2
    jg .hv_w16_loop
    RET
.hv_w32:
%if WIN64
    %assign stack_offset stack_offset - stack_size_padded
    PUSH                 r8
    %assign regs_used regs_used + 1
    WIN64_SPILL_XMM 32
%endif
    vbroadcasti32x4     m20, [spel_h_shufA]
    vbroadcasti32x4     m21, [spel_h_shufB]
    mova                m22, [spel_shuf32]
    lea                 r5d, [hq+wq*8-256]
    mov                  r7, srcq
    mov                  r8, tmpq
.hv_w32_loop0:
    movu                 m6, [srcq+strideq*0+ 0]
    movu                 m7, [srcq+strideq*0+ 8]
    movu                 m8, [srcq+strideq*0+16]
    mova                 m0, m10
    mova                m23, m10
    pshufb               m9, m6, m20
    vpdpwssd             m0, m12, m9      ; a0l
    pshufb               m9, m7, m20
    vpdpwssd            m23, m12, m9      ; a0h
    vpdpwssd             m0, m14, m9      ; a2l
    pshufb               m7, m21
    vpdpwssd            m23, m13, m7      ; a1h
    vpdpwssd             m0, m15, m7      ; a3l
    pshufb               m7, m8, m20
    vpdpwssd            m23, m14, m7      ; a2h
    pshufb               m6, m21
    vpdpwssd             m0, m13, m6      ; a1l
    pshufb               m8, m21
    vpdpwssd            m23, m15, m8      ; a3h
    PUT_8TAP_HV_W32       1, 24, strideq, 1, 2 ; 12
    PUT_8TAP_HV_W32       3, 26, strideq, 0, 1 ; 34
    PUT_8TAP_HV_W32       5, 28, strideq, 2, 0 ; 56
    vpshrdd              m2, m1, m3, 16   ; 23l
    vpshrdd             m25, m24, m26, 16 ; 23h
    vpshrdd              m4, m3, m5, 16   ; 45l
    vpshrdd             m27, m26, m28, 16 ; 45h
.hv_w32_loop:
    movu                 m7, [srcq+strideq*1+ 0]
    movu                 m9, [srcq+strideq*2+ 0]
    movu                 m6, [srcq+strideq*1+ 8]
    movu                 m8, [srcq+strideq*2+ 8]
    mova                m29, m10
    mova                m31, m10
    pshufb              m30, m7, m20
    vpdpwssd            m29, m12, m30     ; h0l
    pshufb              m30, m9, m20
    vpdpwssd            m31, m12, m30     ; i0l
    pshufb               m7, m21
    vpdpwssd            m29, m13, m7      ; h1l
    pshufb               m9, m21
    vpdpwssd            m31, m13, m9      ; i1l
    pshufb               m7, m6, m20
    vpdpwssd            m29, m14, m7      ; h2l
    pshufb               m9, m8, m20
    vpdpwssd            m31, m14, m9      ; i2l
    pshufb               m6, m21
    vpdpwssd            m29, m15, m6      ; h3l
    pshufb               m8, m21
    vpdpwssd            m31, m15, m8      ; i3l
    mova                m30, m10
    vpdpwssd            m30, m12, m7      ; h0h
    movu                 m7, [srcq+strideq*1+16]
    lea                srcq, [srcq+strideq*2]
    vpermt2b            m29, m22, m31     ; 78l
    mova                m31, m10
    vpdpwssd            m31, m12, m9      ; i0h
    movu                 m9, [srcq+strideq*0+16]
    vpdpwssd            m30, m13, m6      ; h1h
    pshufb               m6, m7, m20
    vpdpwssd            m31, m13, m8      ; i1h
    pshufb               m8, m9, m20
    vpdpwssd            m30, m14, m6      ; h2h
    mova                 m6, m11
    vpdpwssd             m6, m16, m0      ; A0l
    pshufb               m7, m21
    vpdpwssd            m31, m14, m8      ; i2h
    mova                 m8, m11
    vpdpwssd             m8, m16, m23     ; A0h
    pshufb               m9, m21
    vpdpwssd            m30, m15, m7      ; h3h
    mova                 m7, m11
    vpdpwssd             m7, m16, m1      ; B0l
    vpdpwssd            m31, m15, m9      ; i3h
    mova                 m9, m11
    vpdpwssd             m9, m16, m24     ; B0h
    mova                 m0, m2
    vpdpwssd             m6, m17, m2      ; A1l
    mova                m23, m25
    vpdpwssd             m8, m17, m25     ; A1h
    mova                 m1, m3
    vpdpwssd             m7, m17, m3      ; B1l
    mova                m24, m26
    vpdpwssd             m9, m17, m26     ; B1h
    vpermt2b            m30, m22, m31     ; 78h
    mova                m31, [prep_endC]
    vpdpwssd             m6, m18, m4      ; A2l
    mova                 m2, m4
    vpdpwssd             m8, m18, m27     ; A2h
    mova                m25, m27
    vpdpwssd             m7, m18, m5      ; B2l
    mova                 m3, m5
    vpdpwssd             m9, m18, m28     ; B2h
    mova                m26, m28
    vpshrdd              m4, m5, m29, 16  ; 67l
    vpdpwssd             m6, m19, m4      ; A3l
    vpshrdd             m27, m28, m30, 16 ; 67h
    vpdpwssd             m8, m19, m27     ; A3h
    mova                 m5, m29
    vpdpwssd             m7, m19, m29     ; B3l
    mova                m28, m30
    vpdpwssd             m9, m19, m30     ; B3h
    vpermt2b             m6, m31, m8
    vpermt2b             m7, m31, m9
    mova        [tmpq+wq*0], m6
    mova        [tmpq+wq*2], m7
    lea                tmpq, [tmpq+wq*4]
    sub                  hd, 2
    jg .hv_w32_loop
    add                  r7, 64
    add                  r8, 64
    movzx                hd, r5b
    mov                srcq, r7
    mov                tmpq, r8
    sub                 r5d, 1<<8
    jg .hv_w32_loop0
    RET

%if WIN64
DECLARE_REG_TMP 5
%else
DECLARE_REG_TMP 7
%endif

cglobal warp_affine_8x8t_16bpc, 4, 7, 22, tmp, ts
%define base r6-pd_0to7
    mov                 t0d, r7m
    lea                  r6, [pd_0to7]
    shr                 t0d, 11
    vpbroadcastd         m8, [base+warp_8x8t_rnd_v]
    vpbroadcastd         m1, [base+warp_8x8_rnd_h+t0*4]
    call mangle(private_prefix %+ _warp_affine_8x8_16bpc_avx512icl).main
    psrad               m14, m16, 15
    call mangle(private_prefix %+ _warp_affine_8x8_16bpc_avx512icl).main2
    psrad               m16, 15
    packssdw            m14, m16
    call mangle(private_prefix %+ _warp_affine_8x8_16bpc_avx512icl).main2
    psrad               m15, m16, 15
    call mangle(private_prefix %+ _warp_affine_8x8_16bpc_avx512icl).main2
    add                 tsq, tsq
    psrad               m16, 15
    packssdw            m15, m16
    jmp mangle(private_prefix %+ _warp_affine_8x8_16bpc_avx512icl).end

cglobal warp_affine_8x8_16bpc, 4, 7, 22, dst, ds, src, ss, abcd
    mov                 t0d, r7m ; pixel_max
    lea                  r6, [pd_0to7]
    shr                 t0d, 11
    vpbroadcastd         m1, [base+warp_8x8_rnd_h+t0*4]
    vpbroadcastd         m8, [base+warp_8x8_rnd_v+t0*4]
    call .main
    psrad               m14, m16, 13
    call .main2
    psrad               m16, 13
    packusdw            m14, m16
    call .main2
    psrad               m15, m16, 13
    call .main2
    vpbroadcastd         m0, [base+bidir_shift+t0*4]
    vpsrlvw             m14, m0
    psrad               m16, 13
    packusdw            m15, m16
    vpsrlvw             m15, m0
.end:
    mova                 m0, [base+warp8x8_end]
    vpermb              m16, m0, m14
    lea                  r2, [dsq*3]
    mova          [dstq+dsq*0], xm16
    vextracti128  [dstq+dsq*1], ym16, 1
    vextracti32x4 [dstq+dsq*2], m16, 2
    vextracti32x4 [dstq+r2   ], m16, 3
    vpermb              m16, m0, m15
    lea                dstq, [dstq+dsq*4]
    mova          [dstq+dsq*0], xm16
    vextracti128  [dstq+dsq*1], ym16, 1
    vextracti32x4 [dstq+dsq*2], m16, 2
    vextracti32x4 [dstq+r2   ], m16, 3
    RET
.main:
    vpbroadcastd        ym3, [base+pd_512]
%if WIN64
    mov               abcdq, r5mp
    vpaddd             ym18, ym3, r6m {1to8} ; mx
%else
    add                 r5d, 512
    vpbroadcastd       ym18, r5d
%endif
    vpaddd             ym20, ym3, r7m {1to8} ; my
    mova               ym16, [base+pd_0to7]
    vpbroadcastd       ym19, [abcdq+4*0]     ; alpha
    vpbroadcastd       ym21, [abcdq+4*1]     ; gamma
    lea                  r4, [ssq*3+6]
    vpdpwssd           ym18, ym19, ym16      ; tmx
    vpdpwssd           ym20, ym21, ym16      ; tmy
    sub                srcq, r4
    mova                m10, [base+warp8x8_permA]
    lea                  r4, [mc_warp_filter+64*8]
    vbroadcasti32x4     m12, [base+warp8x8_permC]
    kxnorb               k1, k1, k1
    vbroadcasti32x4     m13, [base+warp8x8_permD]
    movu                ym5, [srcq+0]
    vinserti32x8         m5, [srcq+8], 1
    psrad              ym17, ym18, 10
    mova                m11, [base+warp8x8_permB]
    kmovb                k2, k1
    vpgatherdq       m3{k1}, [r4+ym17*8]    ; filter_x0
    psrad              ym19, 16             ; beta
    psrad              ym21, 16             ; delta
    paddd              ym18, ym19
    vpermb               m4, m10, m5
    vpbroadcastq         m9, [base+warp_shift_h+t0*8]
    pshufd               m3, m3, q3120
    paddd                m7, m1, m1
    pshufb               m2, m3, m12
    vpdpwssd             m1, m4, m2
    vpermb               m5, m11, m5
    vshufi32x4           m4, m5, q1021
    pshufb               m3, m13
    vpdpwssd             m1, m4, m3
    call .h
    psllq                m2, m1, 32
    paddd                m1, m2
    vpmultishiftqb       m1, m9, m1
    vpshrdq              m1, m0, 48          ; 01 12
    call .h
    vpshrdq              m2, m1, m0, 48      ; 23 34
    call .h
    vpshrdq              m3, m2, m0, 48      ; 45 56
.main2:
    call .h
    psrad               ym6, ym20, 10
    kmovb                k1, k2
    paddd              ym17, ym20, ym21      ; my += delta
    vpgatherdq      m20{k2}, [r4+ym6*8]      ; filter_y0
    psrad              ym16, ym17, 10
    kmovb                k2, k1
    vpgatherdq       m6{k1}, [r4+ym16*8]     ; filter_y1
    shufps               m5, m20, m6, q2020
    mova                m16, m8
    pshufb               m4, m5, m12
    vpdpwssd            m16, m1, m4          ; a0 b0
    pshufb               m5, m13
    mova                 m1, m2
    vpdpwssd            m16, m2, m5          ; a1 b1
    shufps               m6, m20, m6, q3131
    paddd              ym20, ym17, ym21
    pshufb               m4, m6, m12
    mova                 m2, m3
    vpdpwssd            m16, m3, m4          ; a2 b2
    vpshrdq              m3, m0, 48          ; 67 78
    pshufb               m6, m13
    vpdpwssd            m16, m3, m6          ; a3 b3
    ret
ALIGN function_align
.h:
    movu               ym16, [srcq+ssq*1]
    psrad               ym6, ym18, 10
    lea                srcq, [srcq+ssq*2]
    vinserti32x8         m5, m16, [srcq+ssq*0], 1
    kmovb                k1, k2
    paddd              ym17, ym18, ym19      ; mx += beta
    vpgatherdq      m18{k2}, [r4+ym6*8]      ; filter_x1
    psrad              ym16, ym17, 10
    kmovb                k2, k1
    vpgatherdq       m6{k1}, [r4+ym16*8]     ; filter_x2
    vpermb               m4, m10, m5
    shufps              m16, m18, m6, q2020
    shufps               m6, m18, m6, q3131
    mova                 m0, m7
    pshufb              m18, m16, m12
    vpdpwssd             m0, m4, m18         ; a0 b0
    vpermb               m5, m11, m5
    pshufb              m18, m6, m13
    vpdpwssd             m0, m5, m18         ; a3 b3
    paddd              ym18, ym17, ym19
    vshufi32x4          m17, m4, m5, q1021
    pshufb              m16, m13
    vpdpwssd             m0, m17, m16        ; a1 b1
    vshufi32x4           m4, m5, q2132
    pshufb               m6, m12
    vpdpwssd             m0, m4, m6          ; a2 b2
    vpmultishiftqb       m0, m9, m0          ; a a b b
    ret

%macro BIDIR_FN 0
    call .main
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    movq   [dstq          ], xm0
    movhps [dstq+strideq*1], xm0
    vextracti32x4       xm2, ym0, 1
    movq   [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm2
    cmp                  hd, 8
    jl .w4_end
    vextracti32x4       xm2, m0, 2
    lea                dstq, [dstq+strideq*4]
    movq   [dstq          ], xm2
    movhps [dstq+strideq*1], xm2
    vextracti32x4       xm0, m0, 3
    movq   [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm0
    je .w4_end
    lea                dstq, [dstq+strideq*4]
    movq   [dstq          ], xm1
    movhps [dstq+strideq*1], xm1
    vextracti32x4       xm0, ym1, 1
    movq   [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm0
    vextracti32x4       xm0, m1, 2
    lea                dstq, [dstq+strideq*4]
    movq   [dstq          ], xm0
    movhps [dstq+strideq*1], xm0
    vextracti32x4       xm1, m1, 3
    movq   [dstq+strideq*2], xm1
    movhps [dstq+stride3q ], xm1
.w4_end:
    RET
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
.w8:
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], ym0, 1
    vextracti32x4 [dstq+strideq*2], m0, 2
    vextracti32x4 [dstq+stride3q ], m0, 3
    sub                  hd, 8
    jl .w8_end
    lea                dstq, [dstq+strideq*4]
    mova          [dstq+strideq*0], xm1
    vextracti32x4 [dstq+strideq*1], ym1, 1
    vextracti32x4 [dstq+strideq*2], m1, 2
    vextracti32x4 [dstq+stride3q ], m1, 3
    jg .w8_loop
.w8_end:
    RET
.w16_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
.w16:
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    mova          [dstq+strideq*2], ym1
    vextracti32x8 [dstq+stride3q ], m1, 1
    sub                  hd, 4
    jg .w16_loop
    RET
.w32_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w32:
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    sub                  hd, 2
    jg .w32_loop
    RET
.w64_loop:
    call .main
    add                dstq, strideq
.w64:
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    call .main
    add                dstq, strideq
.w128:
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    call .main
    mova        [dstq+64*2], m0
    mova        [dstq+64*3], m1
    dec                  hd
    jg .w128_loop
    RET
%endmacro

%if WIN64
DECLARE_REG_TMP 5
%else
DECLARE_REG_TMP 7
%endif

cglobal avg_16bpc, 4, 7, 4, dst, stride, tmp1, tmp2, w, h, stride3
%define base r6-avg_avx512icl_table
    lea                  r6, [avg_avx512icl_table]
    tzcnt                wd, wm
    mov                 t0d, r6m ; pixel_max
    movsxd               wq, [r6+wq*4]
    shr                 t0d, 11
    vpbroadcastd         m2, [base+avg_round+t0*4]
    vpbroadcastd         m3, [base+avg_shift+t0*4]
    movifnidn            hd, hm
    add                  wq, r6
    BIDIR_FN
ALIGN function_align
.main:
    mova                 m0, [tmp1q+64*0]
    paddsw               m0, [tmp2q+64*0]
    mova                 m1, [tmp1q+64*1]
    paddsw               m1, [tmp2q+64*1]
    add               tmp1q, 64*2
    add               tmp2q, 64*2
    pmaxsw               m0, m2
    pmaxsw               m1, m2
    psubsw               m0, m2
    psubsw               m1, m2
    vpsrlvw              m0, m3
    vpsrlvw              m1, m3
    ret

cglobal w_avg_16bpc, 4, 7, 8, dst, stride, tmp1, tmp2, w, h, stride3
%define base r6-w_avg_avx512icl_table
    lea                  r6, [w_avg_avx512icl_table]
    tzcnt                wd, wm
    mov                 t0d, r7m ; pixel_max
    shr                 t0d, 11
    movsxd               wq, [r6+wq*4]
    vpbroadcastd         m5, [base+w_avg_round+t0*4]
    vpbroadcastd         m7, [base+bidir_shift+t0*4]
    add                  wq, r6
    mov                 r6d, r6m ; weight
    lea                 t0d, [r6-16]
    shl                 r6d, 16
    sub                 r6d, t0d ; 16-weight, weight
    movifnidn            hd, hm
    vpbroadcastd         m6, r6d
    BIDIR_FN
ALIGN function_align
.main:
    mova                 m3, [tmp1q+64*0]
    mova                 m1, [tmp2q+64*0]
    mova                 m0, [tmp1q+64*1]
    mova                 m4, [tmp2q+64*1]
    add               tmp1q, 64*2
    add               tmp2q, 64*2
    punpcklwd            m2, m1, m3
    punpckhwd            m1, m3
    punpcklwd            m3, m4, m0
    punpckhwd            m4, m0
    mova                 m0, m5
    vpdpwssd             m0, m6, m2
    mova                 m2, m5
    vpdpwssd             m2, m6, m1
    mova                 m1, m5
    vpdpwssd             m1, m6, m3
    mova                 m3, m5
    vpdpwssd             m3, m6, m4
    REPX       {psrad x, 2}, m0, m2, m1, m3
    packusdw             m0, m2
    packusdw             m1, m3
    vpsrlvw              m0, m7
    vpsrlvw              m1, m7
    ret

cglobal mask_16bpc, 4, 8, 11, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-mask_avx512icl_table
    lea                  r7, [mask_avx512icl_table]
    tzcnt                wd, wm
    mov                 r6d, r7m ; pixel_max
    movifnidn            hd, hm
    shr                 r6d, 11
    movsxd               wq, [r7+wq*4]
    vpbroadcastd         m8, [base+pw_64]
    vpbroadcastd         m9, [base+mask_round+r6*4]
    vpbroadcastd        m10, [base+bidir_shift+r6*4]
    mov               maskq, maskmp
    add                  wq, r7
    BIDIR_FN
ALIGN function_align
.main:
    pmovzxbw             m1, [maskq+32*0]
    mova                 m4, [tmp1q+64*0]
    mova                 m2, [tmp2q+64*0]
    pmovzxbw             m6, [maskq+32*1]
    mova                 m5, [tmp1q+64*1]
    mova                 m3, [tmp2q+64*1]
    add               maskq, 32*2
    add               tmp1q, 64*2
    add               tmp2q, 64*2
    punpcklwd            m7, m4, m2
    punpckhwd            m4, m2
    psubw                m0, m8, m1
    punpcklwd            m2, m1, m0 ; m, 64-m
    punpckhwd            m1, m0
    mova                 m0, m9
    vpdpwssd             m0, m7, m2
    mova                 m2, m9
    vpdpwssd             m2, m4, m1 ; tmp1 * m + tmp2 * (64-m)
    punpcklwd            m7, m5, m3
    punpckhwd            m5, m3
    psubw                m1, m8, m6
    punpcklwd            m3, m6, m1
    punpckhwd            m6, m1
    mova                 m1, m9
    vpdpwssd             m1, m7, m3
    mova                 m3, m9
    vpdpwssd             m3, m5, m6
    REPX       {psrad x, 4}, m0, m2, m1, m3
    packusdw             m0, m2
    packusdw             m1, m3
    vpsrlvw              m0, m10
    vpsrlvw              m1, m10
    ret

cglobal w_mask_420_16bpc, 4, 8, 16, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-w_mask_420_avx512icl_table
    lea                  r7, [w_mask_420_avx512icl_table]
    tzcnt                wd, wm
    mov                 r6d, r8m ; pixel_max
    movifnidn            hd, hm
    shr                 r6d, 11
    movsxd               wq, [r7+wq*4]
    vpbroadcastd        m10, [base+pw_27615] ; ((64 - 38) << 10) + 1023 - 32
    vpbroadcastd        m11, [base+pw_64]
    vpbroadcastd        m12, [base+mask_round+r6*4]
    vpbroadcastd        m13, [base+bidir_shift+r6*4]
    mov                 r6d, r7m ; sign
    vpbroadcastd        m14, [base+w_mask_round+r6*4]
    mova               ym15, [w_mask_end42x]
    mov               maskq, maskmp
    add                  wq, r7
    call .main
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    mova                 m4, [w_mask_shuf4]
    vpermt2b             m2, m4, m3
    mova                 m3, m14
    vpdpbusd             m3, m2, [pb_64] {1to16}
    vpermb               m3, m15, m3
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    vextracti32x4       xm2, ym0, 1
    movq   [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm2
    mova            [maskq], xm3
    cmp                  hd, 8
    jl .w4_end
    vextracti32x4       xm2, m0, 2
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm2
    movhps [dstq+strideq*1], xm2
    vextracti32x4       xm0, m0, 3
    movq   [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm0
    je .w4_end
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm1
    movhps [dstq+strideq*1], xm1
    vextracti32x4       xm2, ym1, 1
    movq   [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm2
    vextracti32x4       xm2, m1, 2
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm2
    movhps [dstq+strideq*1], xm2
    vextracti32x4       xm1, m1, 3
    movq   [dstq+strideq*2], xm1
    movhps [dstq+stride3q ], xm1
.w4_end:
    RET
.w8:
    mova                 m8, [w_mask_shuf8]
    vpbroadcastd         m9, [pb_64]
    jmp .w8_start
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
    add               maskq, 16
.w8_start:
    vpermt2b             m2, m8, m3
    mova                 m3, m14
    vpdpbusd             m3, m2, m9
    vpermb               m3, m15, m3
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], ym0, 1
    vextracti32x4 [dstq+strideq*2], m0, 2
    vextracti32x4 [dstq+stride3q ], m0, 3
    mova            [maskq], xm3
    sub                  hd, 8
    jl .w8_end
    lea                dstq, [dstq+strideq*4]
    mova          [dstq+strideq*0], xm1
    vextracti32x4 [dstq+strideq*1], ym1, 1
    vextracti32x4 [dstq+strideq*2], m1, 2
    vextracti32x4 [dstq+stride3q ], m1, 3
    jg .w8_loop
.w8_end:
    RET
.w16:
    mova                 m8, [w_mask_shuf16]
    vpbroadcastd         m9, [pb_64]
    jmp .w16_start
.w16_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
    add               maskq, 16
.w16_start:
    vpermt2b             m2, m8, m3
    mova                 m3, m14
    vpdpbusd             m3, m2, m9
    vpermb               m3, m15, m3
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    mova          [dstq+strideq*2], ym1
    vextracti32x8 [dstq+stride3q ], m1, 1
    mova            [maskq], xm3
    sub                  hd, 4
    jg .w16_loop
    RET
.w32_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
    add               maskq, 32
.w32:
    paddw                m2, m3
    mova                 m8, m14
    vpdpwssd             m8, m11, m2
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    call .main
    paddw                m2, m3
    mova                 m3, m14
    vpdpwssd             m3, m11, m2
    vpermt2b             m8, m15, m3
    mova   [dstq+strideq*2], m0
    mova   [dstq+stride3q ], m1
    mova            [maskq], ym8
    sub                  hd, 4
    jg .w32_loop
    RET
.w64_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
    add               maskq, 32
.w64:
    mova                 m8, m2
    mova                 m9, m3
    mova [dstq+strideq*0+64*0], m0
    mova [dstq+strideq*0+64*1], m1
    call .main
    paddw                m8, m2
    paddw                m9, m3
    mova                 m2, m14
    vpdpwssd             m2, m11, m8
    mova                 m3, m14
    vpdpwssd             m3, m11, m9
    vpermt2b             m2, m15, m3
    mova [dstq+strideq*1+64*0], m0
    mova [dstq+strideq*1+64*1], m1
    mova            [maskq], ym2
    sub                  hd, 2
    jg .w64_loop
    RET
.w128_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
    add               maskq, 64
.w128:
    mova               m16, m2
    mova                m8, m3
    mova [dstq+strideq*0+64*0], m0
    mova [dstq+strideq*0+64*1], m1
    call .main
    mova                m17, m2
    mova                 m9, m3
    mova [dstq+strideq*0+64*2], m0
    mova [dstq+strideq*0+64*3], m1
    call .main
    paddw                m2, m16
    paddw                m3, m8
    mova                m16, m14
    vpdpwssd            m16, m11, m2
    mova                 m8, m14
    vpdpwssd             m8, m11, m3
    mova [dstq+strideq*1+64*0], m0
    mova [dstq+strideq*1+64*1], m1
    call .main
    paddw                m2, m17
    paddw                m3, m9
    mova                m17, m14
    vpdpwssd            m17, m11, m2
    mova                 m9, m14
    vpdpwssd             m9, m11, m3
    vpermt2b            m16, m15, m8
    vpermt2b            m17, m15, m9
    mova [dstq+strideq*1+64*2], m0
    mova [dstq+strideq*1+64*3], m1
    mova       [maskq+32*0], ym16
    mova       [maskq+32*1], ym17
    sub                  hd, 2
    jg .w128_loop
    vzeroupper
    RET
ALIGN function_align
.main:
    mova                 m1, [tmp1q+64*0]
    mova                 m3, [tmp2q+64*0]
    mova                 m4, [tmp1q+64*1]
    mova                 m7, [tmp2q+64*1]
    add               tmp1q, 64*2
    add               tmp2q, 64*2
    psubsw               m6, m1, m3
    punpcklwd            m5, m3, m1
    pabsw                m6, m6
    punpckhwd            m3, m1
    psubusw              m6, m10, m6
    psrlw                m6, 10      ; 64-m
    psubw                m2, m11, m6 ; m
    punpcklwd            m1, m6, m2
    punpckhwd            m6, m2
    mova                 m0, m12
    vpdpwssd             m0, m5, m1
    mova                 m1, m12
    vpdpwssd             m1, m3, m6
    psubsw               m5, m4, m7
    punpcklwd            m6, m7, m4
    pabsw                m5, m5
    punpckhwd            m7, m4
    psubusw              m5, m10, m5
    psrlw                m5, 10
    psubw                m3, m11, m5
    punpcklwd            m4, m5, m3
    psrad                m0, 4
    punpckhwd            m5, m3
    psrad                m1, 4
    packusdw             m0, m1
    mova                 m1, m12
    vpdpwssd             m1, m6, m4
    mova                 m4, m12
    vpdpwssd             m4, m7, m5
    psrad                m1, 4
    psrad                m4, 4
    packusdw             m1, m4
    vpsrlvw              m0, m13
    vpsrlvw              m1, m13
    ret

cglobal w_mask_422_16bpc, 4, 8, 15, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-w_mask_422_avx512icl_table
    lea                  r7, [w_mask_422_avx512icl_table]
    tzcnt                wd, wm
    mov                 r6d, r8m ; pixel_max
    movifnidn            hd, hm
    shr                 r6d, 11
    movsxd               wq, [r7+wq*4]
    vpbroadcastd         m8, [base+pw_27615] ; ((64 - 38) << 10) + 1023 - 32
    vpbroadcastd         m9, [base+pw_64]
    vpbroadcastd        m10, [base+mask_round+r6*4]
    vpbroadcastd        m11, [base+bidir_shift+r6*4]
    mov                 r6d, r7m ; sign
    vpbroadcastd        m12, [base+w_mask_round+r6*4]
    mova               ym13, [w_mask_end42x]
    mov               maskq, maskmp
    add                  wq, r7
    paddw               m14, m9, m9 ; pw_128
    call .main
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    vextracti32x4       xm2, ym0, 1
    movq   [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm2
    cmp                  hd, 8
    jl .w4_end
    vextracti32x4       xm2, m0, 2
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm2
    movhps [dstq+strideq*1], xm2
    vextracti32x4       xm0, m0, 3
    movq   [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm0
    je .w4_end
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm1
    movhps [dstq+strideq*1], xm1
    vextracti32x4       xm2, ym1, 1
    movq   [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm2
    vextracti32x4       xm2, m1, 2
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm2
    movhps [dstq+strideq*1], xm2
    vextracti32x4       xm1, m1, 3
    movq   [dstq+strideq*2], xm1
    movhps [dstq+stride3q ], xm1
.w4_end:
    RET
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
.w8:
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], ym0, 1
    vextracti32x4 [dstq+strideq*2], m0, 2
    vextracti32x4 [dstq+stride3q ], m0, 3
    sub                  hd, 8
    jl .w8_end
    lea                dstq, [dstq+strideq*4]
    mova          [dstq+strideq*0], xm1
    vextracti32x4 [dstq+strideq*1], ym1, 1
    vextracti32x4 [dstq+strideq*2], m1, 2
    vextracti32x4 [dstq+stride3q ], m1, 3
    jg .w8_loop
.w8_end:
    RET
.w16_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
.w16:
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    mova          [dstq+strideq*2], ym1
    vextracti32x8 [dstq+stride3q ], m1, 1
    sub                  hd, 4
    jg .w16_loop
    RET
.w32_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w32:
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    sub                  hd, 2
    jg .w32_loop
    RET
.w64_loop:
    call .main
    add                dstq, strideq
.w64:
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    call .main
    add                dstq, strideq
.w128:
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    call .main
    mova        [dstq+64*2], m0
    mova        [dstq+64*3], m1
    dec                  hd
    jg .w128_loop
    RET
ALIGN function_align
.main:
    mova                 m1, [tmp1q+64*0]
    mova                 m3, [tmp2q+64*0]
    mova                 m4, [tmp1q+64*1]
    mova                 m7, [tmp2q+64*1]
    add               tmp1q, 64*2
    add               tmp2q, 64*2
    psubsw               m6, m1, m3
    punpcklwd            m5, m3, m1
    pabsw                m6, m6
    punpckhwd            m3, m1
    psubusw              m6, m8, m6
    psrlw                m6, 10
    psubw                m2, m9, m6
    punpcklwd            m1, m6, m2
    punpckhwd            m6, m2
    mova                 m0, m10
    vpdpwssd             m0, m5, m1
    mova                 m1, m10
    vpdpwssd             m1, m3, m6
    psubsw               m5, m4, m7
    punpcklwd            m6, m7, m4
    pabsw                m5, m5
    punpckhwd            m7, m4
    psubusw              m5, m8, m5
    psrlw                m5, 10
    psubw                m3, m9, m5
    punpcklwd            m4, m5, m3
    psrad                m0, 4
    punpckhwd            m5, m3
    psrad                m1, 4
    packusdw             m0, m1
    mova                 m1, m10
    vpdpwssd             m1, m6, m4
    mova                 m4, m10
    vpdpwssd             m4, m7, m5
    mova                 m5, m12
    vpdpwssd             m5, m14, m2
    mova                 m2, m12
    vpdpwssd             m2, m14, m3
    psrad                m1, 4
    psrad                m4, 4
    packusdw             m1, m4
    vpermt2b             m5, m13, m2
    vpsrlvw              m0, m11
    vpsrlvw              m1, m11
    mova            [maskq], ym5
    add               maskq, 32
    ret

cglobal w_mask_444_16bpc, 4, 8, 13, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-w_mask_444_avx512icl_table
    lea                  r7, [w_mask_444_avx512icl_table]
    tzcnt                wd, wm
    mov                 r6d, r8m ; pixel_max
    movifnidn            hd, hm
    shr                 r6d, 11
    movsxd               wq, [r7+wq*4]
    vpbroadcastd         m8, [base+pw_27615] ; ((64 - 38) << 10) + 1023 - 32
    vpbroadcastd         m9, [base+pw_64]
    vpbroadcastd        m10, [base+mask_round+r6*4]
    mova                m11, [w_mask_end444]
    vpbroadcastd        m12, [base+bidir_shift+r6*4]
    mov               maskq, maskmp
    add                  wq, r7
    call .main
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    movq   [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm0
    vextracti32x4       xm2, ym0, 1
    movq   [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm2
    cmp                  hd, 8
    jl .w4_end
    vextracti32x4       xm2, m0, 2
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm2
    movhps [dstq+strideq*1], xm2
    vextracti32x4       xm0, m0, 3
    movq   [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm0
    je .w4_end
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm1
    movhps [dstq+strideq*1], xm1
    vextracti32x4       xm2, ym1, 1
    movq   [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm2
    vextracti32x4       xm2, m1, 2
    lea                dstq, [dstq+strideq*4]
    movq   [dstq+strideq*0], xm2
    movhps [dstq+strideq*1], xm2
    vextracti32x4       xm1, m1, 3
    movq   [dstq+strideq*2], xm1
    movhps [dstq+stride3q ], xm1
.w4_end:
    RET
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
.w8:
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], ym0, 1
    vextracti32x4 [dstq+strideq*2], m0, 2
    vextracti32x4 [dstq+stride3q ], m0, 3
    sub                  hd, 8
    jl .w8_end
    lea                dstq, [dstq+strideq*4]
    mova          [dstq+strideq*0], xm1
    vextracti32x4 [dstq+strideq*1], ym1, 1
    vextracti32x4 [dstq+strideq*2], m1, 2
    vextracti32x4 [dstq+stride3q ], m1, 3
    jg .w8_loop
.w8_end:
    RET
.w16_loop:
    call .main
    lea                dstq, [dstq+strideq*4]
.w16:
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    mova          [dstq+strideq*2], ym1
    vextracti32x8 [dstq+stride3q ], m1, 1
    sub                  hd, 4
    jg .w16_loop
    RET
.w32_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w32:
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    sub                  hd, 2
    jg .w32_loop
    RET
.w64_loop:
    call .main
    add                dstq, strideq
.w64:
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    call .main
    add                dstq, strideq
.w128:
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    call .main
    mova        [dstq+64*2], m0
    mova        [dstq+64*3], m1
    dec                  hd
    jg .w128_loop
    RET
ALIGN function_align
.main:
    mova                 m1, [tmp1q+64*0]
    mova                 m3, [tmp2q+64*0]
    mova                 m4, [tmp1q+64*1]
    mova                 m7, [tmp2q+64*1]
    add               tmp1q, 64*2
    add               tmp2q, 64*2
    psubsw               m6, m1, m3
    punpcklwd            m5, m3, m1
    pabsw                m6, m6
    punpckhwd            m3, m1
    psubusw              m6, m8, m6
    psrlw                m6, 10
    psubw                m2, m9, m6
    punpcklwd            m1, m6, m2
    punpckhwd            m6, m2
    mova                 m0, m10
    vpdpwssd             m0, m5, m1
    mova                 m1, m10
    vpdpwssd             m1, m3, m6
    psubsw               m5, m4, m7
    punpcklwd            m6, m7, m4
    pabsw                m5, m5
    punpckhwd            m7, m4
    psubusw              m5, m8, m5
    psrlw                m5, 10
    psubw                m3, m9, m5
    punpcklwd            m4, m5, m3
    psrad                m0, 4
    punpckhwd            m5, m3
    psrad                m1, 4
    packusdw             m0, m1
    mova                 m1, m10
    vpdpwssd             m1, m6, m4
    mova                 m4, m10
    vpdpwssd             m4, m7, m5
    vpermt2b             m2, m11, m3
    psrad                m1, 4
    psrad                m4, 4
    packusdw             m1, m4
    vpsrlvw              m0, m12
    vpsrlvw              m1, m12
    mova            [maskq], m2
    add               maskq, 64
    ret

cglobal blend_16bpc, 3, 7, 7, dst, ds, tmp, w, h, mask
%define base r6-blend_avx512icl_table
    lea                  r6, [blend_avx512icl_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r6+wq*4]
    movifnidn         maskq, maskmp
    vpbroadcastd         m6, [base+pw_m512]
    add                  wq, r6
    lea                  r6, [dsq*3]
    jmp                  wq
.w4:
    pmovzxbw           ym19, [maskq]
    movq               xm16, [dstq+dsq*0]
    movhps             xm16, [dstq+dsq*1]
    vpbroadcastq       ym17, [dstq+dsq*2]
    vpbroadcastq       ym18, [dstq+r6   ]
    pmullw             ym19, ym6
    vpblendd           ym16, ym17, 0x30
    vpblendd           ym16, ym18, 0xc0
    psubw              ym17, ym16, [tmpq]
    add               maskq, 16
    add                tmpq, 32
    pmulhrsw           ym17, ym19
    paddw              ym16, ym17
    vextracti128       xm17, ym16, 1
    movq       [dstq+dsq*0], xm16
    movhps     [dstq+dsq*1], xm16
    movq       [dstq+dsq*2], xm17
    movhps     [dstq+r6   ], xm17
    lea                dstq, [dstq+dsq*4]
    sub                  hd, 4
    jg .w4
    vzeroupper
    RET
.w8:
    pmovzxbw             m2, [maskq]
    mova                xm0, [dstq+dsq*0]
    vinserti32x4        ym0, [dstq+dsq*1], 1
    vinserti32x4         m0, [dstq+dsq*2], 2
    vinserti32x4         m0, [dstq+r6   ], 3
    pmullw               m2, m6
    psubw                m1, m0, [tmpq]
    add               maskq, 32
    add                tmpq, 64
    pmulhrsw             m1, m2
    paddw                m0, m1
    mova          [dstq+dsq*0], xm0
    vextracti32x4 [dstq+dsq*1], ym0, 1
    vextracti32x4 [dstq+dsq*2], m0, 2
    vextracti32x4 [dstq+r6   ], m0, 3
    lea                dstq, [dstq+dsq*4]
    sub                  hd, 4
    jg .w8
    RET
.w16:
    pmovzxbw             m4, [maskq+32*0]
    pmovzxbw             m5, [maskq+32*1]
    mova                ym0, [dstq+dsq*0]
    vinserti32x8         m0, [dstq+dsq*1], 1
    mova                ym1, [dstq+dsq*2]
    vinserti32x8         m1, [dstq+r6   ], 1
    pmullw               m4, m6
    pmullw               m5, m6
    psubw                m2, m0, [tmpq+64*0]
    psubw                m3, m1, [tmpq+64*1]
    add               maskq, 32*2
    add                tmpq, 64*2
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova          [dstq+dsq*0], ym0
    vextracti32x8 [dstq+dsq*1], m0, 1
    mova          [dstq+dsq*2], ym1
    vextracti32x8 [dstq+r6   ], m1, 1
    lea                dstq, [dstq+dsq*4]
    sub                  hd, 4
    jg .w16
    RET
.w32:
    pmovzxbw             m4, [maskq+32*0]
    pmovzxbw             m5, [maskq+32*1]
    mova                 m0, [dstq+dsq*0]
    mova                 m1, [dstq+dsq*1]
    pmullw               m4, m6
    pmullw               m5, m6
    psubw                m2, m0, [tmpq+ 64*0]
    psubw                m3, m1, [tmpq+ 64*1]
    add               maskq, 32*2
    add                tmpq, 64*2
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w32
    RET

cglobal blend_v_16bpc, 3, 6, 5, dst, ds, tmp, w, h
    lea                  r5, [blend_v_avx512icl_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    add                  wq, r5
    jmp                  wq
.w2:
    vpbroadcastd       xmm2, [obmc_masks_avx2+2*2]
.w2_loop:
    movd               xmm0, [dstq+dsq*0]
    pinsrd             xmm0, [dstq+dsq*1], 1
    movq               xmm1, [tmpq]
    add                tmpq, 4*2
    psubw              xmm1, xmm0, xmm1
    pmulhrsw           xmm1, xmm2
    paddw              xmm0, xmm1
    movd       [dstq+dsq*0], xmm0
    pextrd     [dstq+dsq*1], xmm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w2_loop
    RET
.w4:
    vpbroadcastq       xmm2, [obmc_masks_avx2+4*2]
.w4_loop:
    movq               xmm0, [dstq+dsq*0]
    movhps             xmm0, [dstq+dsq*1]
    psubw              xmm1, xmm0, [tmpq]
    add                tmpq, 8*2
    pmulhrsw           xmm1, xmm2
    paddw              xmm0, xmm1
    movq       [dstq+dsq*0], xmm0
    movhps     [dstq+dsq*1], xmm0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w4_loop
    RET
.w8:
    vbroadcasti32x4     ym2, [obmc_masks_avx2+8*2]
.w8_loop:
    mova                xm0, [dstq+dsq*0]
    vinserti32x4        ym0, [dstq+dsq*1], 1
    psubw               ym1, ym0, [tmpq]
    add                tmpq, 16*2
    pmulhrsw            ym1, ym2
    paddw               ym0, ym1
    mova          [dstq+dsq*0], xm0
    vextracti32x4 [dstq+dsq*1], ym0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w8_loop
    RET
.w16:
    vbroadcasti32x8      m2, [obmc_masks_avx2+16*2]
.w16_loop:
    mova                ym0, [dstq+dsq*0]
    vinserti32x8         m0, [dstq+dsq*1], 1
    psubw                m1, m0, [tmpq]
    add                tmpq, 32*2
    pmulhrsw             m1, m2
    paddw                m0, m1
    mova          [dstq+dsq*0], ym0
    vextracti32x8 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w16_loop
    RET
.w32:
    mova                 m4, [obmc_masks_avx2+32*2]
.w32_loop:
    mova                 m0,     [dstq+dsq*0]
    psubw                m2, m0, [tmpq+ 64*0]
    mova                 m1,     [dstq+dsq*1]
    psubw                m3, m1, [tmpq+ 64*1]
    add                tmpq, 64*2
    pmulhrsw             m2, m4
    pmulhrsw             m3, m4
    paddw                m0, m2
    paddw                m1, m3
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w32_loop
    RET

cglobal blend_h_16bpc, 3, 7, 9, dst, ds, tmp, w, h, mask
%define base r6-$$
    lea                  r6, [$$]
    tzcnt                wd, wm
    mov                  hd, hm
    movsxd               wq, [base+blend_h_avx512icl_table+wq*4]
    lea               maskq, [base+obmc_masks_avx2+hq*2]
    lea                  hd, [hq*3]
    lea                  wq, [base+blend_h_avx512icl_table+wq]
    shr                  hd, 2 ; h * 3/4
    lea               maskq, [maskq+hq*2]
    neg                  hq
    jmp                  wq
.w2:
    movd               xmm0, [dstq+dsq*0]
    pinsrd             xmm0, [dstq+dsq*1], 1
    movd               xmm2, [maskq+hq*2]
    movq               xmm1, [tmpq]
    add                tmpq, 4*2
    punpcklwd          xmm2, xmm2
    psubw              xmm1, xmm0, xmm1
    pmulhrsw           xmm1, xmm2
    paddw              xmm0, xmm1
    movd       [dstq+dsq*0], xmm0
    pextrd     [dstq+dsq*1], xmm0, 1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w2
    RET
.w4:
    mova               xmm3, [blend_shuf]
.w4_loop:
    movq               xmm0, [dstq+dsq*0]
    movhps             xmm0, [dstq+dsq*1]
    movd               xmm2, [maskq+hq*2]
    psubw              xmm1, xmm0, [tmpq]
    add                tmpq, 8*2
    pshufb             xmm2, xmm3
    pmulhrsw           xmm1, xmm2
    paddw              xmm0, xmm1
    movq       [dstq+dsq*0], xmm0
    movhps     [dstq+dsq*1], xmm0
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w4_loop
    RET
.w8:
    vbroadcasti32x4     ym3, [blend_shuf]
    shufpd              ym3, ym3, 0x0c
.w8_loop:
    mova                xm0, [dstq+dsq*0]
    vinserti32x4        ym0, [dstq+dsq*1], 1
    vpbroadcastd        ym2, [maskq+hq*2]
    psubw               ym1, ym0, [tmpq]
    add                tmpq, 16*2
    pshufb              ym2, ym3
    pmulhrsw            ym1, ym2
    paddw               ym0, ym1
    mova          [dstq+dsq*0], xm0
    vextracti32x4 [dstq+dsq*1], ym0, 1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w8_loop
    RET
.w16:
    vbroadcasti32x4      m3, [blend_shuf]
    shufpd               m3, m3, 0xf0
.w16_loop:
    mova                ym0, [dstq+dsq*0]
    vinserti32x8         m0, [dstq+dsq*1], 1
    vpbroadcastd         m2, [maskq+hq*2]
    psubw                m1, m0, [tmpq]
    add                tmpq, 32*2
    pshufb               m2, m3
    pmulhrsw             m1, m2
    paddw                m0, m1
    mova          [dstq+dsq*0], ym0
    vextracti32x8 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w16_loop
    RET
.w32:
    vpbroadcastw         m4, [maskq+hq*2]
    vpbroadcastw         m5, [maskq+hq*2+2]
    mova                 m0,     [dstq+dsq*0]
    psubw                m2, m0, [tmpq+ 64*0]
    mova                 m1,     [dstq+dsq*1]
    psubw                m3, m1, [tmpq+ 64*1]
    add                tmpq, 64*2
    pmulhrsw             m2, m4
    pmulhrsw             m3, m5
    paddw                m0, m2
    paddw                m1, m3
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w32
    RET
.w64:
    vpbroadcastw         m4, [maskq+hq*2]
    mova                 m0,     [dstq+64*0]
    psubw                m2, m0, [tmpq+64*0]
    mova                 m1,     [dstq+64*1]
    psubw                m3, m1, [tmpq+64*1]
    add                tmpq, 64*2
    pmulhrsw             m2, m4
    pmulhrsw             m3, m4
    paddw                m0, m2
    paddw                m1, m3
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    add                dstq, dsq
    inc                  hq
    jl .w64
    RET
.w128:
    vpbroadcastw         m8, [maskq+hq*2]
    mova                 m0,     [dstq+64*0]
    psubw                m4, m0, [tmpq+64*0]
    mova                 m1,     [dstq+64*1]
    psubw                m5, m1, [tmpq+64*1]
    mova                 m2,     [dstq+64*2]
    psubw                m6, m2, [tmpq+64*2]
    mova                 m3,     [dstq+64*3]
    psubw                m7, m3, [tmpq+64*3]
    add                tmpq, 64*4
    REPX   {pmulhrsw x, m8}, m4, m5, m6, m7
    paddw                m0, m4
    paddw                m1, m5
    paddw                m2, m6
    paddw                m3, m7
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    mova        [dstq+64*2], m2
    mova        [dstq+64*3], m3
    add                dstq, dsq
    inc                  hq
    jl .w128
    RET

cglobal resize_16bpc, 6, 12, 32, dst, dst_stride, src, src_stride, \
                                 dst_w, h, src_w, dx, mx0, pxmax
    sub          dword mx0m, 4<<14
    sub        dword src_wm, 8
    mov                  r6, ~0
    vpbroadcastd         m5, dxm
    vpbroadcastd         m8, mx0m
    vpbroadcastd         m6, src_wm
    kmovq                k6, r6
 DEFINE_ARGS dst, dst_stride, src, src_stride, dst_w, h, x, _, _, pxmax
    LEA                  r7, $$
%define base r7-$$
    vpbroadcastd         m3, [base+pd_16384]
    vpbroadcastd         m7, [base+pd_63]
    mova                m24, [base+resize_permA]
    mova                m25, [base+resize_permB]
    mova                m26, [base+resize_permC]
    mova                m27, [base+resize_permD]
    vbroadcasti32x4     m28, [base+resize_shufA]
    vbroadcasti32x4     m29, [base+resize_shufB]
    mova                m30, [base+resize_permE]
    vpbroadcastw       ym31, pxmaxm
    vpdpwssd             m8, m5, [base+rescale_mul] ; mx+dx*[0-15]
    pslld                m5, 4                      ; dx*16
    pslld                m6, 14
    pxor                 m2, m2
.loop_y:
    xor                  xd, xd
    mova                 m4, m8     ; per-line working version of mx
.loop_x:
    pmaxsd               m0, m4, m2
    psrad                m9, m4, 8  ; filter offset (unmasked)
    pminsd               m0, m6     ; iclip(mx, 0, src_w-8)
    psubd                m1, m4, m0 ; pshufb offset
    psrad                m0, 14     ; clipped src_x offset
    psrad                m1, 14     ; pshufb edge_emu offset
    vptestmd             k5, m1, m1
    pand                 m9, m7     ; filter offset (masked)
    ktestw               k5, k5
    jz .load
    vpbroadcastq        m14, [base+pd_0_4]
    vpermq              m10, m0, q1100
    vpermq              m11, m0, q3322
    vpermq              m20, m1, q1100
    vpermq              m21, m1, q3322
    punpckldq           m10, m10
    punpckldq           m11, m11
    punpckldq           m20, m20
    punpckldq           m21, m21
    paddd               m10, m14
    paddd               m11, m14
    paddd               m20, m14
    paddd               m21, m14
    vextracti32x8      ym12, m10, 1
    vextracti32x8      ym13, m11, 1
    vextracti32x8      ym22, m20, 1
    vextracti32x8      ym23, m21, 1
    kmovq                k1, k6
    kmovq                k2, k6
    kmovq                k3, k6
    kmovq                k4, k6
    vpgatherdq      m16{k1}, [srcq+ym10*2] ; 0 1 2 3
    vpgatherdq      m17{k2}, [srcq+ym11*2] ; 4 5 6 7
    vpgatherdq      m18{k3}, [srcq+ym12*2] ; 8 9 A B
    vpgatherdq      m19{k4}, [srcq+ym13*2] ; C D E F
    kmovq                k1, k6
    kmovq                k2, k6
    kmovq                k3, k6
    kmovq                k4, k6
    vpgatherdq       m0{k1}, [base+resize_shuf+8+ym20*2]
    vpgatherdq       m1{k2}, [base+resize_shuf+8+ym21*2]
    vpgatherdq      m14{k3}, [base+resize_shuf+8+ym22*2]
    vpgatherdq      m15{k4}, [base+resize_shuf+8+ym23*2]
    pshufb              m16, m0
    pshufb              m17, m1
    pshufb              m18, m14
    pshufb              m19, m15
    mova                m20, m24
    mova                m22, m24
    mova                m21, m25
    mova                m23, m25
    vpermi2d            m20, m16, m17 ; 0-3a 0-3b 4-7a 4-7b
    vpermi2d            m21, m16, m17 ; 0-3c 0-3d 4-7c 4-7d
    vpermi2d            m22, m18, m19 ; 8-Ba 8-Bb C-Fa C-Fb
    vpermi2d            m23, m18, m19 ; 8-Bc 8-Bd C-Fc C-Fd
    mova                m15, m26
    mova                m17, m26
    mova                m16, m27
    mova                m18, m27
    vpermi2q            m15, m20, m22 ; 0-3a 4-7a 8-Ba C-Fa
    vpermi2q            m16, m20, m22 ; 0-3b 4-7b 8-Bb C-Fb
    vpermi2q            m17, m21, m23 ; 0-3c 4-7c 8-Bc C-Fc
    vpermi2q            m18, m21, m23 ; 0-3d 4-7d 8-Bd C-Fd
    kmovq                k1, k6
    kmovq                k2, k6
    vpgatherdd      m11{k1}, [base+resize_filter+m9*8+0]
    vpgatherdd      m13{k2}, [base+resize_filter+m9*8+4]
    pshufb              m10, m11, m28
    pshufb              m11, m11, m29
    pshufb              m12, m13, m28
    pshufb              m13, m13, m29
    jmp .filter
.load:
    kmovq                k1, k6
    kmovq                k2, k6
    kmovq                k3, k6
    kmovq                k4, k6
    vpgatherdd      m11{k1}, [base+resize_filter+m9*8+0]
    vpgatherdd      m13{k2}, [base+resize_filter+m9*8+4]
    pshufb              m10, m11, m28
    pshufb              m11, m11, m29
    pshufb              m12, m13, m28
    pshufb              m13, m13, m29
    vpgatherdd      m15{k3}, [srcq+m0*2+ 0]
    vpgatherdd      m16{k4}, [srcq+m0*2+ 4]
    kmovq                k1, k6
    kmovq                k2, k6
    vpgatherdd      m17{k1}, [srcq+m0*2+ 8]
    vpgatherdd      m18{k2}, [srcq+m0*2+12]
.filter:
    mova                m14, m2
    vpdpwssd            m14, m15, m10
    vpdpwssd            m14, m16, m11
    vpdpwssd            m14, m17, m12
    vpdpwssd            m14, m18, m13
    psubd               m14, m3, m14
    psrad               m14, 15
    packusdw            m14, m14
    vpermq              m14, m30, m14
    pminsw             ym14, ym31
    mova        [dstq+xq*2], ym14
    paddd                m4, m5
    add                  xd, 16
    cmp                  xd, dst_wd
    jl .loop_x
    add                dstq, dst_strideq
    add                srcq, src_strideq
    dec                  hd
    jg .loop_y
    RET

%endif ; ARCH_X86_64
