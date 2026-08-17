; Copyright © 2018, VideoLAN and dav1d authors
; Copyright © 2018, Two Orioles, LLC
; Copyright © 2018, VideoLabs
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

SECTION_RODATA 16

; dav1d_obmc_masks[] with 64-x interleaved
obmc_masks: db  0,  0,  0,  0
            ; 2 @4
            db 45, 19, 64,  0
            ; 4 @8
            db 39, 25, 50, 14, 59,  5, 64,  0
            ; 8 @16
            db 36, 28, 42, 22, 48, 16, 53, 11, 57,  7, 61,  3, 64,  0, 64,  0
            ; 16 @32
            db 34, 30, 37, 27, 40, 24, 43, 21, 46, 18, 49, 15, 52, 12, 54, 10
            db 56,  8, 58,  6, 60,  4, 61,  3, 64,  0, 64,  0, 64,  0, 64,  0
            ; 32 @64
            db 33, 31, 35, 29, 36, 28, 38, 26, 40, 24, 41, 23, 43, 21, 44, 20
            db 45, 19, 47, 17, 48, 16, 50, 14, 51, 13, 52, 12, 53, 11, 55,  9
            db 56,  8, 57,  7, 58,  6, 59,  5, 60,  4, 60,  4, 61,  3, 62,  2

warp_8x8_shufA: db 0,  2,  4,  6,  1,  3,  5,  7,  1,  3,  5,  7,  2,  4,  6,  8
warp_8x8_shufB: db 4,  6,  8, 10,  5,  7,  9, 11,  5,  7,  9, 11,  6,  8, 10, 12
warp_8x8_shufC: db 2,  4,  6,  8,  3,  5,  7,  9,  3,  5,  7,  9,  4,  6,  8, 10
warp_8x8_shufD: db 6,  8, 10, 12,  7,  9, 11, 13,  7,  9, 11, 13,  8, 10, 12, 14
blend_shuf:     db 0,  1,  0,  1,  0,  1,  0,  1,  2,  3,  2,  3,  2,  3,  2,  3
subpel_h_shuf4: db 0,  1,  2,  3,  1,  2,  3,  4,  8,  9, 10, 11,  9, 10, 11, 12
                db 2,  3,  4,  5,  3,  4,  5,  6, 10, 11, 12, 13, 11, 12, 13, 14
subpel_h_shufA: db 0,  1,  2,  3,  1,  2,  3,  4,  2,  3,  4,  5,  3,  4,  5,  6
subpel_h_shufB: db 4,  5,  6,  7,  5,  6,  7,  8,  6,  7,  8,  9,  7,  8,  9, 10
subpel_h_shufC: db 8,  9, 10, 11,  9, 10, 11, 12, 10, 11, 12, 13, 11, 12, 13, 14
subpel_s_shuf2: db 0,  1,  2,  3,  0,  1,  2,  3,  8,  9, 10, 11,  8,  9, 10, 11
subpel_s_shuf8: db 0,  1,  8,  9,  2,  3, 10, 11,  4,  5, 12, 13,  6,  7, 14, 15
bilin_h_shuf4:  db 0,  1,  1,  2,  2,  3,  3,  4,  8,  9,  9, 10, 10, 11, 11, 12
bilin_h_shuf8:  db 0,  1,  1,  2,  2,  3,  3,  4,  4,  5,  5,  6,  6,  7,  7,  8
unpckw:         db 0,  1,  4,  5,  8,  9, 12, 13,  2,  3,  6,  7, 10, 11, 14, 15
rescale_mul:    dd 0,  1,  2,  3
resize_shuf:    db 0,  0,  0,  0,  0,  1,  2,  3,  4,  5,  6,  7,  7,  7,  7,  7

wm_420_sign:    times 4 dw 258
                times 4 dw 257
wm_422_sign:    times 8 db 128
                times 8 db 127

pb_8x0_8x8: times 8 db 0
            times 8 db 8
bdct_lb_dw: times 4 db 0
            times 4 db 4
            times 4 db 8
            times 4 db 12

pb_64:    times 16 db 64
pw_m256:  times 8 dw -256
pw_1:     times 8 dw 1
pw_2:     times 8 dw 2
pw_8:     times 8 dw 8
pw_15:    times 8 dw 15
pw_26:    times 8 dw 26
pw_34:    times 8 dw 34
pw_512:   times 8 dw 512
pw_1024:  times 8 dw 1024
pw_2048:  times 8 dw 2048
pw_6903:  times 8 dw 6903
pw_8192:  times 8 dw 8192
pd_32:    times 4 dd 32
pd_63:    times 4 dd 63
pd_512:   times 4 dd 512
pd_16384: times 4 dd 16484
pd_32768: times 4 dd 32768
pd_262144:times 4 dd 262144
pd_0x3ff: times 4 dd 0x3ff
pd_0x4000:times 4 dd 0x4000
pq_0x40000000: times 2 dq 0x40000000

const mc_warp_filter2 ; dav1d_mc_warp_filter[] reordered for pmaddubsw usage
    ; [-1, 0)
    db 0, 127,   0, 0,   0,   1, 0, 0, 0, 127,   0, 0,  -1,   2, 0, 0
    db 1, 127,  -1, 0,  -3,   4, 0, 0, 1, 126,  -2, 0,  -4,   6, 1, 0
    db 1, 126,  -3, 0,  -5,   8, 1, 0, 1, 125,  -4, 0,  -6,  11, 1, 0
    db 1, 124,  -4, 0,  -7,  13, 1, 0, 2, 123,  -5, 0,  -8,  15, 1, 0
    db 2, 122,  -6, 0,  -9,  18, 1, 0, 2, 121,  -6, 0, -10,  20, 1, 0
    db 2, 120,  -7, 0, -11,  22, 2, 0, 2, 119,  -8, 0, -12,  25, 2, 0
    db 3, 117,  -8, 0, -13,  27, 2, 0, 3, 116,  -9, 0, -13,  29, 2, 0
    db 3, 114, -10, 0, -14,  32, 3, 0, 3, 113, -10, 0, -15,  35, 2, 0
    db 3, 111, -11, 0, -15,  37, 3, 0, 3, 109, -11, 0, -16,  40, 3, 0
    db 3, 108, -12, 0, -16,  42, 3, 0, 4, 106, -13, 0, -17,  45, 3, 0
    db 4, 104, -13, 0, -17,  47, 3, 0, 4, 102, -14, 0, -17,  50, 3, 0
    db 4, 100, -14, 0, -17,  52, 3, 0, 4,  98, -15, 0, -18,  55, 4, 0
    db 4,  96, -15, 0, -18,  58, 3, 0, 4,  94, -16, 0, -18,  60, 4, 0
    db 4,  91, -16, 0, -18,  63, 4, 0, 4,  89, -16, 0, -18,  65, 4, 0
    db 4,  87, -17, 0, -18,  68, 4, 0, 4,  85, -17, 0, -18,  70, 4, 0
    db 4,  82, -17, 0, -18,  73, 4, 0, 4,  80, -17, 0, -18,  75, 4, 0
    db 4,  78, -18, 0, -18,  78, 4, 0, 4,  75, -18, 0, -17,  80, 4, 0
    db 4,  73, -18, 0, -17,  82, 4, 0, 4,  70, -18, 0, -17,  85, 4, 0
    db 4,  68, -18, 0, -17,  87, 4, 0, 4,  65, -18, 0, -16,  89, 4, 0
    db 4,  63, -18, 0, -16,  91, 4, 0, 4,  60, -18, 0, -16,  94, 4, 0
    db 3,  58, -18, 0, -15,  96, 4, 0, 4,  55, -18, 0, -15,  98, 4, 0
    db 3,  52, -17, 0, -14, 100, 4, 0, 3,  50, -17, 0, -14, 102, 4, 0
    db 3,  47, -17, 0, -13, 104, 4, 0, 3,  45, -17, 0, -13, 106, 4, 0
    db 3,  42, -16, 0, -12, 108, 3, 0, 3,  40, -16, 0, -11, 109, 3, 0
    db 3,  37, -15, 0, -11, 111, 3, 0, 2,  35, -15, 0, -10, 113, 3, 0
    db 3,  32, -14, 0, -10, 114, 3, 0, 2,  29, -13, 0,  -9, 116, 3, 0
    db 2,  27, -13, 0,  -8, 117, 3, 0, 2,  25, -12, 0,  -8, 119, 2, 0
    db 2,  22, -11, 0,  -7, 120, 2, 0, 1,  20, -10, 0,  -6, 121, 2, 0
    db 1,  18,  -9, 0,  -6, 122, 2, 0, 1,  15,  -8, 0,  -5, 123, 2, 0
    db 1,  13,  -7, 0,  -4, 124, 1, 0, 1,  11,  -6, 0,  -4, 125, 1, 0
    db 1,   8,  -5, 0,  -3, 126, 1, 0, 1,   6,  -4, 0,  -2, 126, 1, 0
    db 0,   4,  -3, 0,  -1, 127, 1, 0, 0,   2,  -1, 0,   0, 127, 0, 0
    ; [0, 1)
    db  0,   0,   1, 0, 0, 127,   0,  0,  0,  -1,   2, 0, 0, 127,   0,  0
    db  0,  -3,   4, 1, 1, 127,  -2,  0,  0,  -5,   6, 1, 1, 127,  -2,  0
    db  0,  -6,   8, 1, 2, 126,  -3,  0, -1,  -7,  11, 2, 2, 126,  -4, -1
    db -1,  -8,  13, 2, 3, 125,  -5, -1, -1, -10,  16, 3, 3, 124,  -6, -1
    db -1, -11,  18, 3, 4, 123,  -7, -1, -1, -12,  20, 3, 4, 122,  -7, -1
    db -1, -13,  23, 3, 4, 121,  -8, -1, -2, -14,  25, 4, 5, 120,  -9, -1
    db -1, -15,  27, 4, 5, 119, -10, -1, -1, -16,  30, 4, 5, 118, -11, -1
    db -2, -17,  33, 5, 6, 116, -12, -1, -2, -17,  35, 5, 6, 114, -12, -1
    db -2, -18,  38, 5, 6, 113, -13, -1, -2, -19,  41, 6, 7, 111, -14, -2
    db -2, -19,  43, 6, 7, 110, -15, -2, -2, -20,  46, 6, 7, 108, -15, -2
    db -2, -20,  49, 6, 7, 106, -16, -2, -2, -21,  51, 7, 7, 104, -16, -2
    db -2, -21,  54, 7, 7, 102, -17, -2, -2, -21,  56, 7, 8, 100, -18, -2
    db -2, -22,  59, 7, 8,  98, -18, -2, -2, -22,  62, 7, 8,  96, -19, -2
    db -2, -22,  64, 7, 8,  94, -19, -2, -2, -22,  67, 8, 8,  91, -20, -2
    db -2, -22,  69, 8, 8,  89, -20, -2, -2, -22,  72, 8, 8,  87, -21, -2
    db -2, -21,  74, 8, 8,  84, -21, -2, -2, -22,  77, 8, 8,  82, -21, -2
    db -2, -21,  79, 8, 8,  79, -21, -2, -2, -21,  82, 8, 8,  77, -22, -2
    db -2, -21,  84, 8, 8,  74, -21, -2, -2, -21,  87, 8, 8,  72, -22, -2
    db -2, -20,  89, 8, 8,  69, -22, -2, -2, -20,  91, 8, 8,  67, -22, -2
    db -2, -19,  94, 8, 7,  64, -22, -2, -2, -19,  96, 8, 7,  62, -22, -2
    db -2, -18,  98, 8, 7,  59, -22, -2, -2, -18, 100, 8, 7,  56, -21, -2
    db -2, -17, 102, 7, 7,  54, -21, -2, -2, -16, 104, 7, 7,  51, -21, -2
    db -2, -16, 106, 7, 6,  49, -20, -2, -2, -15, 108, 7, 6,  46, -20, -2
    db -2, -15, 110, 7, 6,  43, -19, -2, -2, -14, 111, 7, 6,  41, -19, -2
    db -1, -13, 113, 6, 5,  38, -18, -2, -1, -12, 114, 6, 5,  35, -17, -2
    db -1, -12, 116, 6, 5,  33, -17, -2, -1, -11, 118, 5, 4,  30, -16, -1
    db -1, -10, 119, 5, 4,  27, -15, -1, -1,  -9, 120, 5, 4,  25, -14, -2
    db -1,  -8, 121, 4, 3,  23, -13, -1, -1,  -7, 122, 4, 3,  20, -12, -1
    db -1,  -7, 123, 4, 3,  18, -11, -1, -1,  -6, 124, 3, 3,  16, -10, -1
    db -1,  -5, 125, 3, 2,  13,  -8, -1, -1,  -4, 126, 2, 2,  11,  -7, -1
    db  0,  -3, 126, 2, 1,   8,  -6,  0,  0,  -2, 127, 1, 1,   6,  -5,  0
    db  0,  -2, 127, 1, 1,   4,  -3,  0,  0,   0, 127, 0, 0,   2,  -1,  0
    ; [1, 2)
    db 0, 0, 127,   0, 0,   1,   0, 0, 0, 0, 127,   0, 0,  -1,   2, 0
    db 0, 1, 127,  -1, 0,  -3,   4, 0, 0, 1, 126,  -2, 0,  -4,   6, 1
    db 0, 1, 126,  -3, 0,  -5,   8, 1, 0, 1, 125,  -4, 0,  -6,  11, 1
    db 0, 1, 124,  -4, 0,  -7,  13, 1, 0, 2, 123,  -5, 0,  -8,  15, 1
    db 0, 2, 122,  -6, 0,  -9,  18, 1, 0, 2, 121,  -6, 0, -10,  20, 1
    db 0, 2, 120,  -7, 0, -11,  22, 2, 0, 2, 119,  -8, 0, -12,  25, 2
    db 0, 3, 117,  -8, 0, -13,  27, 2, 0, 3, 116,  -9, 0, -13,  29, 2
    db 0, 3, 114, -10, 0, -14,  32, 3, 0, 3, 113, -10, 0, -15,  35, 2
    db 0, 3, 111, -11, 0, -15,  37, 3, 0, 3, 109, -11, 0, -16,  40, 3
    db 0, 3, 108, -12, 0, -16,  42, 3, 0, 4, 106, -13, 0, -17,  45, 3
    db 0, 4, 104, -13, 0, -17,  47, 3, 0, 4, 102, -14, 0, -17,  50, 3
    db 0, 4, 100, -14, 0, -17,  52, 3, 0, 4,  98, -15, 0, -18,  55, 4
    db 0, 4,  96, -15, 0, -18,  58, 3, 0, 4,  94, -16, 0, -18,  60, 4
    db 0, 4,  91, -16, 0, -18,  63, 4, 0, 4,  89, -16, 0, -18,  65, 4
    db 0, 4,  87, -17, 0, -18,  68, 4, 0, 4,  85, -17, 0, -18,  70, 4
    db 0, 4,  82, -17, 0, -18,  73, 4, 0, 4,  80, -17, 0, -18,  75, 4
    db 0, 4,  78, -18, 0, -18,  78, 4, 0, 4,  75, -18, 0, -17,  80, 4
    db 0, 4,  73, -18, 0, -17,  82, 4, 0, 4,  70, -18, 0, -17,  85, 4
    db 0, 4,  68, -18, 0, -17,  87, 4, 0, 4,  65, -18, 0, -16,  89, 4
    db 0, 4,  63, -18, 0, -16,  91, 4, 0, 4,  60, -18, 0, -16,  94, 4
    db 0, 3,  58, -18, 0, -15,  96, 4, 0, 4,  55, -18, 0, -15,  98, 4
    db 0, 3,  52, -17, 0, -14, 100, 4, 0, 3,  50, -17, 0, -14, 102, 4
    db 0, 3,  47, -17, 0, -13, 104, 4, 0, 3,  45, -17, 0, -13, 106, 4
    db 0, 3,  42, -16, 0, -12, 108, 3, 0, 3,  40, -16, 0, -11, 109, 3
    db 0, 3,  37, -15, 0, -11, 111, 3, 0, 2,  35, -15, 0, -10, 113, 3
    db 0, 3,  32, -14, 0, -10, 114, 3, 0, 2,  29, -13, 0,  -9, 116, 3
    db 0, 2,  27, -13, 0,  -8, 117, 3, 0, 2,  25, -12, 0,  -8, 119, 2
    db 0, 2,  22, -11, 0,  -7, 120, 2, 0, 1,  20, -10, 0,  -6, 121, 2
    db 0, 1,  18,  -9, 0,  -6, 122, 2, 0, 1,  15,  -8, 0,  -5, 123, 2
    db 0, 1,  13,  -7, 0,  -4, 124, 1, 0, 1,  11,  -6, 0,  -4, 125, 1
    db 0, 1,   8,  -5, 0,  -3, 126, 1, 0, 1,   6,  -4, 0,  -2, 126, 1
    db 0, 0,   4,  -3, 0,  -1, 127, 1, 0, 0,   2,  -1, 0,   0, 127, 0
    db 0, 0,   2,  -1, 0,   0, 127, 0

pw_258:  times 2 dw 258

cextern mc_subpel_filters
%define subpel_filters (mangle(private_prefix %+ _mc_subpel_filters)-8)

%macro BIDIR_JMP_TABLE 2-*
    ;evaluated at definition time (in loop below)
    %xdefine %1_%2_table (%%table - 2*%3)
    %xdefine %%base %1_%2_table
    %xdefine %%prefix mangle(private_prefix %+ _%1_8bpc_%2)
    ; dynamically generated label
    %%table:
    %rep %0 - 2 ; repeat for num args
        dd %%prefix %+ .w%3 - %%base
        %rotate 1
    %endrep
%endmacro

BIDIR_JMP_TABLE avg, ssse3,        4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_avg, ssse3,      4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE mask, ssse3,       4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_420, ssse3, 4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_422, ssse3, 4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_444, ssse3, 4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE blend, ssse3,      4, 8, 16, 32
BIDIR_JMP_TABLE blend_v, ssse3, 2, 4, 8, 16, 32
BIDIR_JMP_TABLE blend_h, ssse3, 2, 4, 8, 16, 16, 16, 16

%macro BASE_JMP_TABLE 3-*
    %xdefine %1_%2_table (%%table - %3)
    %xdefine %%base %1_%2
    %%table:
    %rep %0 - 2
        dw %%base %+ _w%3 - %%base
        %rotate 1
    %endrep
%endmacro

%xdefine prep_sse2 mangle(private_prefix %+ _prep_bilin_8bpc_sse2.prep)
%xdefine put_ssse3 mangle(private_prefix %+ _put_bilin_8bpc_ssse3.put)
%xdefine prep_ssse3 mangle(private_prefix %+ _prep_bilin_8bpc_ssse3.prep)

BASE_JMP_TABLE put,  ssse3, 2, 4, 8, 16, 32, 64, 128
BASE_JMP_TABLE prep, ssse3,    4, 8, 16, 32, 64, 128

%macro HV_JMP_TABLE 5-*
    %xdefine %%prefix mangle(private_prefix %+ _%1_%2_8bpc_%3)
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

HV_JMP_TABLE prep,  8tap,  sse2, 1,    4, 8, 16, 32, 64, 128
HV_JMP_TABLE prep, bilin,  sse2, 7,    4, 8, 16, 32, 64, 128
HV_JMP_TABLE put,   8tap, ssse3, 3, 2, 4, 8, 16, 32, 64, 128
HV_JMP_TABLE prep,  8tap, ssse3, 1,    4, 8, 16, 32, 64, 128
HV_JMP_TABLE put,  bilin, ssse3, 7, 2, 4, 8, 16, 32, 64, 128
HV_JMP_TABLE prep, bilin, ssse3, 7,    4, 8, 16, 32, 64, 128

%macro SCALED_JMP_TABLE 2-*
    %xdefine %1_%2_table (%%table - %3)
    %xdefine %%base mangle(private_prefix %+ _%1_8bpc_%2)
%%table:
    %rep %0 - 2
        dw %%base %+ .w%3 - %%base
        %rotate 1
    %endrep
    %rotate 2
%%dy_1024:
    %xdefine %1_%2_dy1_table (%%dy_1024 - %3)
    %rep %0 - 2
        dw %%base %+ .dy1_w%3 - %%base
        %rotate 1
    %endrep
    %rotate 2
%%dy_2048:
    %xdefine %1_%2_dy2_table (%%dy_2048 - %3)
    %rep %0 - 2
        dw %%base %+ .dy2_w%3 - %%base
        %rotate 1
    %endrep
%endmacro

SCALED_JMP_TABLE put_8tap_scaled, ssse3, 2, 4, 8, 16, 32, 64, 128
SCALED_JMP_TABLE prep_8tap_scaled, ssse3,   4, 8, 16, 32, 64, 128

%define table_offset(type, fn) type %+ fn %+ SUFFIX %+ _table - type %+ SUFFIX

SECTION .text

INIT_XMM ssse3

%if ARCH_X86_32
 DECLARE_REG_TMP 1
 %define base t0-put_ssse3
%else
 DECLARE_REG_TMP 7
 %define base 0
%endif

%macro RESTORE_DSQ_32 1
 %if ARCH_X86_32
   mov                  %1, dsm ; restore dsq
 %endif
%endmacro

cglobal put_bilin_8bpc, 1, 8, 0, dst, ds, src, ss, w, h, mxy
    movifnidn          mxyd, r6m ; mx
    LEA                  t0, put_ssse3
    movifnidn          srcq, srcmp
    movifnidn           ssq, ssmp
    tzcnt                wd, wm
    mov                  hd, hm
    test               mxyd, mxyd
    jnz .h
    mov                mxyd, r7m ; my
    test               mxyd, mxyd
    jnz .v
.put:
    movzx                wd, word [t0+wq*2+table_offset(put,)]
    add                  wq, t0
    RESTORE_DSQ_32       t0
    jmp                  wq
.put_w2:
    movzx               r4d, word [srcq+ssq*0]
    movzx               r6d, word [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r4w
    mov        [dstq+dsq*1], r6w
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w2
    RET
.put_w4:
    mov                 r4d, [srcq+ssq*0]
    mov                 r6d, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r4d
    mov        [dstq+dsq*1], r6d
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w4
    RET
.put_w8:
    movq                 m0, [srcq+ssq*0]
    movq                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movq       [dstq+dsq*0], m0
    movq       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w8
    RET
.put_w16:
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w16
    RET
.put_w32:
    movu                 m0, [srcq+ssq*0+16*0]
    movu                 m1, [srcq+ssq*0+16*1]
    movu                 m2, [srcq+ssq*1+16*0]
    movu                 m3, [srcq+ssq*1+16*1]
    lea                srcq, [srcq+ssq*2]
    mova  [dstq+dsq*0+16*0], m0
    mova  [dstq+dsq*0+16*1], m1
    mova  [dstq+dsq*1+16*0], m2
    mova  [dstq+dsq*1+16*1], m3
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w32
    RET
.put_w64:
    movu                 m0, [srcq+16*0]
    movu                 m1, [srcq+16*1]
    movu                 m2, [srcq+16*2]
    movu                 m3, [srcq+16*3]
    add                srcq, ssq
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    mova        [dstq+16*2], m2
    mova        [dstq+16*3], m3
    add                dstq, dsq
    dec                  hd
    jg .put_w64
    RET
.put_w128:
    movu                 m0, [srcq+16*0]
    movu                 m1, [srcq+16*1]
    movu                 m2, [srcq+16*2]
    movu                 m3, [srcq+16*3]
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    mova        [dstq+16*2], m2
    mova        [dstq+16*3], m3
    movu                 m0, [srcq+16*4]
    movu                 m1, [srcq+16*5]
    movu                 m2, [srcq+16*6]
    movu                 m3, [srcq+16*7]
    mova        [dstq+16*4], m0
    mova        [dstq+16*5], m1
    mova        [dstq+16*6], m2
    mova        [dstq+16*7], m3
    add                srcq, ssq
    add                dstq, dsq
    dec                  hd
    jg .put_w128
    RET
.h:
    ; (16 * src[x] + (mx * (src[x + 1] - src[x])) + 8) >> 4
    ; = ((16 - mx) * src[x] + mx * src[x + 1] + 8) >> 4
    imul               mxyd, 0x00ff00ff
    mova                 m4, [base+bilin_h_shuf8]
    mova                 m0, [base+bilin_h_shuf4]
    add                mxyd, 0x00100010
    movd                 m5, mxyd
    mov                mxyd, r7m ; my
    pshufd               m5, m5, q0000
    test               mxyd, mxyd
    jnz .hv
    movzx                wd, word [t0+wq*2+table_offset(put, _bilin_h)]
    mova                 m3, [base+pw_2048]
    add                  wq, t0
    movifnidn           dsq, dsmp
    jmp                  wq
.h_w2:
    pshufd               m4, m4, q3120 ; m4 = {1, 0, 2, 1, 5, 4, 6, 5}
.h_w2_loop:
    movd                 m0, [srcq+ssq*0]
    movd                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpckldq            m0, m1
    pshufb               m0, m4
    pmaddubsw            m0, m5
    pmulhrsw             m0, m3
    packuswb             m0, m0
    movd                r6d, m0
    mov        [dstq+dsq*0], r6w
    shr                 r6d, 16
    mov        [dstq+dsq*1], r6w
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w2_loop
    RET
.h_w4:
    movq                 m4, [srcq+ssq*0]
    movhps               m4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb               m4, m0
    pmaddubsw            m4, m5
    pmulhrsw             m4, m3
    packuswb             m4, m4
    movd       [dstq+dsq*0], m4
    psrlq                m4, 32
    movd       [dstq+dsq*1], m4
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w4
    RET
.h_w8:
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    movq       [dstq+dsq*0], m0
    movhps     [dstq+dsq*1], m0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    movu                 m0, [srcq+8*0]
    movu                 m1, [srcq+8*1]
    add                srcq, ssq
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    mova             [dstq], m0
    add                dstq, dsq
    dec                  hd
    jg .h_w16
    RET
.h_w32:
    movu                 m0, [srcq+mmsize*0+8*0]
    movu                 m1, [srcq+mmsize*0+8*1]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    movu                 m1, [srcq+mmsize*1+8*0]
    movu                 m2, [srcq+mmsize*1+8*1]
    add                srcq, ssq
    pshufb               m1, m4
    pshufb               m2, m4
    pmaddubsw            m1, m5
    pmaddubsw            m2, m5
    pmulhrsw             m1, m3
    pmulhrsw             m2, m3
    packuswb             m1, m2
    mova        [dstq+16*0], m0
    mova        [dstq+16*1], m1
    add                dstq, dsq
    dec                  hd
    jg .h_w32
    RET
.h_w64:
    mov                  r6, -16*3
.h_w64_loop:
    movu                 m0, [srcq+r6+16*3+8*0]
    movu                 m1, [srcq+r6+16*3+8*1]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    mova     [dstq+r6+16*3], m0
    add                  r6, 16
    jle .h_w64_loop
    add                srcq, ssq
    add                dstq, dsq
    dec                  hd
    jg .h_w64
    RET
.h_w128:
    mov                  r6, -16*7
.h_w128_loop:
    movu                 m0, [srcq+r6+16*7+8*0]
    movu                 m1, [srcq+r6+16*7+8*1]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    mova     [dstq+r6+16*7], m0
    add                  r6, 16
    jle .h_w128_loop
    add                srcq, ssq
    add                dstq, dsq
    dec                  hd
    jg .h_w128
    RET
.v:
    movzx                wd, word [t0+wq*2+table_offset(put, _bilin_v)]
    imul               mxyd, 0x00ff00ff
    mova                 m5, [base+pw_2048]
    add                mxyd, 0x00100010
    add                  wq, t0
    movd                 m4, mxyd
    pshufd               m4, m4, q0000
    movifnidn           dsq, dsmp
    jmp                  wq
.v_w2:
    movd                 m0, [srcq+ssq*0]
.v_w2_loop:
    pinsrw               m0, [srcq+ssq*1], 1 ; 0 1
    lea                srcq, [srcq+ssq*2]
    pshuflw              m1, m0, q2301
    pinsrw               m0, [srcq+ssq*0], 0 ; 2 1
    punpcklbw            m1, m0
    pmaddubsw            m1, m4
    pmulhrsw             m1, m5
    packuswb             m1, m1
    movd                r6d, m1
    mov        [dstq+dsq*1], r6w
    shr                 r6d, 16
    mov        [dstq+dsq*0], r6w
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
    movd                 m0, [srcq+ssq*0]
.v_w4_loop:
    movd                 m2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova                 m1, m0
    movd                 m0, [srcq+ssq*0]
    punpckldq            m1, m2 ; 0 1
    punpckldq            m2, m0 ; 1 2
    punpcklbw            m1, m2
    pmaddubsw            m1, m4
    pmulhrsw             m1, m5
    packuswb             m1, m1
    movd       [dstq+dsq*0], m1
    psrlq                m1, 32
    movd       [dstq+dsq*1], m1
    ;
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
    RET
.v_w8:
    movq                 m0, [srcq+ssq*0]
.v_w8_loop:
    movq                 m2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova                 m1, m0
    movq                 m0, [srcq+ssq*0]
    punpcklbw            m1, m2
    punpcklbw            m2, m0
    pmaddubsw            m1, m4
    pmaddubsw            m2, m4
    pmulhrsw             m1, m5
    pmulhrsw             m2, m5
    packuswb             m1, m2
    movq       [dstq+dsq*0], m1
    movhps     [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w8_loop
    RET
%macro PUT_BILIN_V_W16 0
    movu                 m0, [srcq+ssq*0]
%%loop:
    movu                 m3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova                 m1, m0
    mova                 m2, m0
    movu                 m0, [srcq+ssq*0]
    punpcklbw            m1, m3
    punpckhbw            m2, m3
    pmaddubsw            m1, m4
    pmaddubsw            m2, m4
    pmulhrsw             m1, m5
    pmulhrsw             m2, m5
    packuswb             m1, m2
    punpcklbw            m2, m3, m0
    punpckhbw            m3, m0
    pmaddubsw            m2, m4
    pmaddubsw            m3, m4
    pmulhrsw             m2, m5
    pmulhrsw             m3, m5
    packuswb             m2, m3
    mova       [dstq+dsq*0], m1
    mova       [dstq+dsq*1], m2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg %%loop
%endmacro
.v_w16:
    PUT_BILIN_V_W16
    RET
.v_w128:
    lea                 r6d, [hq+(7<<16)]
    jmp .v_w16gt
.v_w64:
    lea                 r6d, [hq+(3<<16)]
    jmp .v_w16gt
.v_w32:
    lea                 r6d, [hq+(1<<16)]
.v_w16gt:
    mov                  r4, srcq
%if ARCH_X86_64
    mov                  r7, dstq
%endif
.v_w16gt_loop:
    PUT_BILIN_V_W16
%if ARCH_X86_64
    add                  r4, 16
    add                  r7, 16
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
%else
    mov                dstq, dstmp
    add                  r4, 16
    movzx                hd, r6w
    add                dstq, 16
    mov                srcq, r4
    mov               dstmp, dstq
%endif
    sub                 r6d, 1<<16
    jg .v_w16gt
    RET
.hv:
    ; (16 * src[x] + (my * (src[x + src_stride] - src[x])) + 128) >> 8
    ; = (src[x] + ((my * (src[x + src_stride] - src[x])) >> 4) + 8) >> 4
    movzx                wd, word [t0+wq*2+table_offset(put, _bilin_hv)]
    WIN64_SPILL_XMM       8
    shl                mxyd, 11 ; can't shift by 12 due to signed overflow
    mova                 m7, [base+pw_15]
    movd                 m6, mxyd
    add                  wq, t0
    pshuflw              m6, m6, q0000
    paddb                m5, m5
    punpcklqdq           m6, m6
    jmp                  wq
.hv_w2:
    RESTORE_DSQ_32       t0
    movd                 m0, [srcq+ssq*0]
    punpckldq            m0, m0
    pshufb               m0, m4
    pmaddubsw            m0, m5
.hv_w2_loop:
    movd                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movd                 m2, [srcq+ssq*0]
    punpckldq            m1, m2
    pshufb               m1, m4
    pmaddubsw            m1, m5             ; 1 _ 2 _
    shufps               m2, m0, m1, q1032  ; 0 _ 1 _
    mova                 m0, m1
    psubw                m1, m2   ; 2 * (src[x + src_stride] - src[x])
    pmulhw               m1, m6   ; (my * (src[x + src_stride] - src[x]) >> 4
    pavgw                m2, m7   ; src[x] + 8
    paddw                m1, m2   ; src[x] + ((my * (src[x + src_stride] - src[x])) >> 4) + 8
    psrlw                m1, 4
    packuswb             m1, m1
%if ARCH_X86_64
    movq                 r6, m1
%else
    pshuflw              m1, m1, q2020
    movd                r6d, m1
%endif
    mov        [dstq+dsq*0], r6w
    shr                  r6, gprsize*4
    mov        [dstq+dsq*1], r6w
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    RET
.hv_w4:
    mova                 m4, [base+bilin_h_shuf4]
    movddup              m0, [srcq+ssq*0]
    movifnidn           dsq, dsmp
    pshufb               m0, m4
    pmaddubsw            m0, m5
.hv_w4_loop:
    movq                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movhps               m1, [srcq+ssq*0]
    pshufb               m1, m4
    pmaddubsw            m1, m5            ; 1 2
    shufps               m2, m0, m1, q1032 ; 0 1
    mova                 m0, m1
    psubw                m1, m2
    pmulhw               m1, m6
    pavgw                m2, m7
    paddw                m1, m2
    psrlw                m1, 4
    packuswb             m1, m1
    movd       [dstq+dsq*0], m1
    psrlq                m1, 32
    movd       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
    RET
.hv_w8:
    movu                 m0, [srcq+ssq*0]
    movifnidn           dsq, dsmp
    pshufb               m0, m4
    pmaddubsw            m0, m5
.hv_w8_loop:
    movu                 m2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb               m2, m4
    pmaddubsw            m2, m5
    psubw                m1, m2, m0
    pmulhw               m1, m6
    pavgw                m0, m7
    paddw                m1, m0
    movu                 m0, [srcq+ssq*0]
    pshufb               m0, m4
    pmaddubsw            m0, m5
    psubw                m3, m0, m2
    pmulhw               m3, m6
    pavgw                m2, m7
    paddw                m3, m2
    psrlw                m1, 4
    psrlw                m3, 4
    packuswb             m1, m3
    movq       [dstq+dsq*0], m1
    movhps     [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w8_loop
    RET
.hv_w128:
    lea                 r6d, [hq+(7<<16)]
    jmp .hv_w16_start
.hv_w64:
    lea                 r6d, [hq+(3<<16)]
    jmp .hv_w16_start
.hv_w32:
    lea                 r6d, [hq+(1<<16)]
.hv_w16_start:
    mov                  r4, srcq
%if ARCH_X86_32
    %define m8 [dstq]
%else
    mov                  r7, dstq
%endif
.hv_w16:
    movifnidn           dsq, dsmp
%if WIN64
    movaps              r4m, m8
%endif
.hv_w16_loop0:
    movu                 m0, [srcq+8*0]
    movu                 m1, [srcq+8*1]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
.hv_w16_loop:
    add                srcq, ssq
    movu                 m2, [srcq+8*0]
    movu                 m3, [srcq+8*1]
    pshufb               m2, m4
    pshufb               m3, m4
    pmaddubsw            m2, m5
    pmaddubsw            m3, m5
    mova                 m8, m2
    psubw                m2, m0
    pmulhw               m2, m6
    pavgw                m0, m7
    paddw                m2, m0
    mova                 m0, m3
    psubw                m3, m1
    pmulhw               m3, m6
    pavgw                m1, m7
    paddw                m3, m1
    mova                 m1, m0
    mova                 m0, m8
    psrlw                m2, 4
    psrlw                m3, 4
    packuswb             m2, m3
    mova             [dstq], m2
    add                dstq, dsmp
    dec                  hd
    jg .hv_w16_loop
%if ARCH_X86_32
    mov                dstq, dstm
    add                  r4, 16
    movzx                hd, r6w
    add                dstq, 16
    mov                srcq, r4
    mov                dstm, dstq
%else
    add                  r4, 16
    add                  r7, 16
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
%endif
    sub                 r6d, 1<<16
    jg .hv_w16_loop0
%if WIN64
    movaps               m8, r4m
%endif
    RET

%macro PSHUFB_BILIN_H8 2 ; dst, src
 %if cpuflag(ssse3)
    pshufb               %1, %2
 %else
    psrldq               %2, %1, 1
    punpcklbw            %1, %2
 %endif
%endmacro

%macro PSHUFB_BILIN_H4 3 ; dst, src, tmp
 %if cpuflag(ssse3)
    pshufb               %1, %2
 %else
    psrldq               %2, %1, 1
    punpckhbw            %3, %1, %2
    punpcklbw            %1, %2
    punpcklqdq           %1, %3
 %endif
%endmacro

%macro PMADDUBSW 5 ; dst/src1, src2, zero, tmp, reset_zero
 %if cpuflag(ssse3)
    pmaddubsw            %1, %2
 %else
  %if %5 == 1
    pxor                 %3, %3
  %endif
    punpckhbw            %4, %1, %3
    punpcklbw            %1, %1, %3
    pmaddwd              %4, %2
    pmaddwd              %1, %2
    packssdw             %1, %4
 %endif
%endmacro

%macro PMULHRSW 5 ; dst, src, tmp, rndval, shift
 %if cpuflag(ssse3)
    pmulhrsw             %1, %2
 %else
    punpckhwd            %3, %1, %4
    punpcklwd            %1, %4
    pmaddwd              %3, %2
    pmaddwd              %1, %2
    psrad                %3, %5
    psrad                %1, %5
    packssdw             %1, %3
 %endif
%endmacro

%macro PREP_BILIN 0
%if ARCH_X86_32
    %define base r6-prep%+SUFFIX
%else
    %define base 0
%endif

cglobal prep_bilin_8bpc, 3, 7, 0, tmp, src, stride, w, h, mxy, stride3
    movifnidn          mxyd, r5m ; mx
    LEA                  r6, prep%+SUFFIX
    tzcnt                wd, wm
    movifnidn            hd, hm
    test               mxyd, mxyd
    jnz .h
    mov                mxyd, r6m ; my
    test               mxyd, mxyd
    jnz .v
.prep:
%if notcpuflag(ssse3)
    add                  r6, prep_ssse3 - prep_sse2
    jmp prep_ssse3
%else
    movzx                wd, word [r6+wq*2+table_offset(prep,)]
    pxor                 m4, m4
    add                  wq, r6
    lea            stride3q, [strideq*3]
    jmp                  wq
.prep_w4:
    movd                 m0, [srcq+strideq*0]
    movd                 m1, [srcq+strideq*1]
    movd                 m2, [srcq+strideq*2]
    movd                 m3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    punpckldq            m0, m1
    punpckldq            m2, m3
    punpcklbw            m0, m4
    punpcklbw            m2, m4
    psllw                m0, 4
    psllw                m2, 4
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m2
    add                tmpq, 16*2
    sub                  hd, 4
    jg .prep_w4
    RET
.prep_w8:
    movq                 m0, [srcq+strideq*0]
    movq                 m1, [srcq+strideq*1]
    movq                 m2, [srcq+strideq*2]
    movq                 m3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    punpcklbw            m0, m4
    punpcklbw            m1, m4
    punpcklbw            m2, m4
    punpcklbw            m3, m4
    psllw                m0, 4
    psllw                m1, 4
    psllw                m2, 4
    psllw                m3, 4
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    mova        [tmpq+16*2], m2
    mova        [tmpq+16*3], m3
    add                tmpq, 16*4
    sub                  hd, 4
    jg .prep_w8
    RET
.prep_w16:
    movu                 m1, [srcq+strideq*0]
    movu                 m3, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    punpcklbw            m0, m1, m4
    punpckhbw            m1, m4
    punpcklbw            m2, m3, m4
    punpckhbw            m3, m4
    psllw                m0, 4
    psllw                m1, 4
    psllw                m2, 4
    psllw                m3, 4
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    mova        [tmpq+16*2], m2
    mova        [tmpq+16*3], m3
    add                tmpq, 16*4
    sub                  hd, 2
    jg .prep_w16
    RET
.prep_w128:
    mov                  r3, -128
    jmp .prep_w32_start
.prep_w64:
    mov                  r3, -64
    jmp .prep_w32_start
.prep_w32:
    mov                  r3, -32
.prep_w32_start:
    sub                srcq, r3
.prep_w32_vloop:
    mov                  r6, r3
.prep_w32_hloop:
    movu                 m1, [srcq+r6+16*0]
    movu                 m3, [srcq+r6+16*1]
    punpcklbw            m0, m1, m4
    punpckhbw            m1, m4
    punpcklbw            m2, m3, m4
    punpckhbw            m3, m4
    psllw                m0, 4
    psllw                m1, 4
    psllw                m2, 4
    psllw                m3, 4
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    mova        [tmpq+16*2], m2
    mova        [tmpq+16*3], m3
    add                tmpq, 16*4
    add                  r6, 32
    jl .prep_w32_hloop
    add                srcq, strideq
    dec                  hd
    jg .prep_w32_vloop
    RET
%endif
.h:
    ; 16 * src[x] + (mx * (src[x + 1] - src[x]))
    ; = (16 - mx) * src[x] + mx * src[x + 1]
%if cpuflag(ssse3)
    imul               mxyd, 0x00ff00ff
    mova                 m4, [base+bilin_h_shuf8]
    add                mxyd, 0x00100010
%else
    imul               mxyd, 0xffff
    add                mxyd, 16
%endif
    movd                 m5, mxyd
    mov                mxyd, r6m ; my
    pshufd               m5, m5, q0000
    test               mxyd, mxyd
    jnz .hv
    movzx                wd, word [r6+wq*2+table_offset(prep, _bilin_h)]
%if notcpuflag(ssse3)
    WIN64_SPILL_XMM 8
    pxor                 m6, m6
%endif
    add                  wq, r6
    jmp                  wq
.h_w4:
%if cpuflag(ssse3)
    mova                 m4, [base+bilin_h_shuf4]
%endif
    lea            stride3q, [strideq*3]
.h_w4_loop:
    movq                 m0, [srcq+strideq*0]
    movhps               m0, [srcq+strideq*1]
    movq                 m1, [srcq+strideq*2]
    movhps               m1, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    PSHUFB_BILIN_H4      m0, m4, m2
    PMADDUBSW            m0, m5, m6, m2, 0
    PSHUFB_BILIN_H4      m1, m4, m2
    PMADDUBSW            m1, m5, m6, m2, 0
    mova          [tmpq+0 ], m0
    mova          [tmpq+16], m1
    add                tmpq, 32
    sub                  hd, 4
    jg .h_w4_loop
    RET
.h_w8:
    lea            stride3q, [strideq*3]
.h_w8_loop:
    movu                 m0, [srcq+strideq*0]
    movu                 m1, [srcq+strideq*1]
    movu                 m2, [srcq+strideq*2]
    movu                 m3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    PSHUFB_BILIN_H8      m0, m4
    PSHUFB_BILIN_H8      m1, m4
    PSHUFB_BILIN_H8      m2, m4
    PSHUFB_BILIN_H8      m3, m4
    PMADDUBSW            m0, m5, m6, m7, 0
    PMADDUBSW            m1, m5, m6, m7, 0
    PMADDUBSW            m2, m5, m6, m7, 0
    PMADDUBSW            m3, m5, m6, m7, 0
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    mova        [tmpq+16*2], m2
    mova        [tmpq+16*3], m3
    add                tmpq, 16*4
    sub                  hd, 4
    jg .h_w8_loop
    RET
.h_w16:
    movu                 m0, [srcq+strideq*0+8*0]
    movu                 m1, [srcq+strideq*0+8*1]
    movu                 m2, [srcq+strideq*1+8*0]
    movu                 m3, [srcq+strideq*1+8*1]
    lea                srcq, [srcq+strideq*2]
    PSHUFB_BILIN_H8      m0, m4
    PSHUFB_BILIN_H8      m1, m4
    PSHUFB_BILIN_H8      m2, m4
    PSHUFB_BILIN_H8      m3, m4
    PMADDUBSW            m0, m5, m6, m7, 0
    PMADDUBSW            m1, m5, m6, m7, 0
    PMADDUBSW            m2, m5, m6, m7, 0
    PMADDUBSW            m3, m5, m6, m7, 0
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    mova        [tmpq+16*2], m2
    mova        [tmpq+16*3], m3
    add                tmpq, 16*4
    sub                  hd, 2
    jg .h_w16
    RET
.h_w128:
    mov                  r3, -128
    jmp .h_w32_start
.h_w64:
    mov                  r3, -64
    jmp .h_w32_start
.h_w32:
    mov                  r3, -32
.h_w32_start:
    sub                srcq, r3
.h_w32_vloop:
    mov                  r6, r3
.h_w32_hloop:
    movu                 m0, [srcq+r6+8*0]
    movu                 m1, [srcq+r6+8*1]
    movu                 m2, [srcq+r6+8*2]
    movu                 m3, [srcq+r6+8*3]
    PSHUFB_BILIN_H8      m0, m4
    PSHUFB_BILIN_H8      m1, m4
    PSHUFB_BILIN_H8      m2, m4
    PSHUFB_BILIN_H8      m3, m4
    PMADDUBSW            m0, m5, m6, m7, 0
    PMADDUBSW            m1, m5, m6, m7, 0
    PMADDUBSW            m2, m5, m6, m7, 0
    PMADDUBSW            m3, m5, m6, m7, 0
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    mova        [tmpq+16*2], m2
    mova        [tmpq+16*3], m3
    add                tmpq, 16*4
    add                  r6, 32
    jl .h_w32_hloop
    add                srcq, strideq
    dec                  hd
    jg .h_w32_vloop
    RET
.v:
%if notcpuflag(ssse3)
 %assign stack_offset stack_offset - stack_size_padded
    WIN64_SPILL_XMM 8
%endif
    movzx                wd, word [r6+wq*2+table_offset(prep, _bilin_v)]
%if cpuflag(ssse3)
    imul               mxyd, 0x00ff00ff
    add                mxyd, 0x00100010
%else
    imul               mxyd, 0xffff
    pxor                 m6, m6
    add                mxyd, 16
%endif
    add                  wq, r6
    lea            stride3q, [strideq*3]
    movd                 m5, mxyd
    pshufd               m5, m5, q0000
    jmp                  wq
.v_w4:
    movd                 m0, [srcq+strideq*0]
.v_w4_loop:
    movd                 m1, [srcq+strideq*1]
    movd                 m2, [srcq+strideq*2]
    movd                 m3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    punpckldq            m0, m1
    punpckldq            m1, m2
    punpcklbw            m0, m1 ; 01 12
    PMADDUBSW            m0, m5, m6, m7, 0
    mova        [tmpq+16*0], m0
    movd                 m0, [srcq+strideq*0]
    punpckldq            m2, m3
    punpckldq            m3, m0
    punpcklbw            m2, m3 ; 23 34
    PMADDUBSW            m2, m5, m6, m7, 0
    mova        [tmpq+16*1], m2
    add                tmpq, 16*2
    sub                  hd, 4
    jg .v_w4_loop
    RET
.v_w8:
    movq                 m0, [srcq+strideq*0]
.v_w8_loop:
    movq                 m1, [srcq+strideq*1]
    movq                 m2, [srcq+strideq*2]
    movq                 m3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    punpcklbw            m0, m1 ; 01
    punpcklbw            m1, m2 ; 12
    PMADDUBSW            m0, m5, m6, m7, 0
    PMADDUBSW            m1, m5, m6, m7, 0
    mova        [tmpq+16*0], m0
    movq                 m0, [srcq+strideq*0]
    punpcklbw            m2, m3 ; 23
    punpcklbw            m3, m0 ; 34
    PMADDUBSW            m2, m5, m6, m7, 0
    mova        [tmpq+16*1], m1
    PMADDUBSW            m3, m5, m6, m7, 0
    mova        [tmpq+16*2], m2
    mova        [tmpq+16*3], m3
    add                tmpq, 16*4
    sub                  hd, 4
    jg .v_w8_loop
    RET
.v_w16:
    movu                 m0, [srcq+strideq*0]
.v_w16_loop:
    movu                 m1, [srcq+strideq*1]
    movu                 m2, [srcq+strideq*2]
    movu                 m3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    punpcklbw            m4, m0, m1
    punpckhbw            m0, m1
    PMADDUBSW            m4, m5, m6, m7, 0
    PMADDUBSW            m0, m5, m6, m7, 0
    mova        [tmpq+16*0], m4
    punpcklbw            m4, m1, m2
    punpckhbw            m1, m2
    PMADDUBSW            m4, m5, m6, m7, 0
    mova        [tmpq+16*1], m0
    movu                 m0, [srcq+strideq*0]
    PMADDUBSW            m1, m5, m6, m7, 0
    mova        [tmpq+16*2], m4
    punpcklbw            m4, m2, m3
    punpckhbw            m2, m3
    PMADDUBSW            m4, m5, m6, m7, 0
    mova        [tmpq+16*3], m1
    PMADDUBSW            m2, m5, m6, m7, 0
    mova        [tmpq+16*4], m4
    punpcklbw            m4, m3, m0
    punpckhbw            m3, m0
    PMADDUBSW            m4, m5, m6, m7, 0
    mova        [tmpq+16*5], m2
    PMADDUBSW            m3, m5, m6, m7, 0
    mova        [tmpq+16*6], m4
    mova        [tmpq+16*7], m3
    add                tmpq, 16*8
    sub                  hd, 4
    jg .v_w16_loop
    RET
.v_w128:
    lea                 r3d, [hq+(3<<8)]
    mov                 r6d, 256
    jmp .v_w32_start
.v_w64:
    lea                 r3d, [hq+(1<<8)]
    mov                 r6d, 128
    jmp .v_w32_start
.v_w32:
    xor                 r3d, r3d
    mov                 r6d, 64
.v_w32_start:
%if ARCH_X86_64
 %if WIN64
    PUSH                 r7
 %endif
    mov                  r7, tmpq
%endif
    mov                  r5, srcq
.v_w32_hloop:
    movu                 m0, [srcq+strideq*0+16*0]
    movu                 m1, [srcq+strideq*0+16*1]
.v_w32_vloop:
    movu                 m2, [srcq+strideq*1+16*0]
    movu                 m3, [srcq+strideq*1+16*1]
    lea                srcq, [srcq+strideq*2]
    punpcklbw            m4, m0, m2
    punpckhbw            m0, m2
    PMADDUBSW            m4, m5, m6, m7, 0
    PMADDUBSW            m0, m5, m6, m7, 0
    mova        [tmpq+16*0], m4
    mova        [tmpq+16*1], m0
    movu                 m0, [srcq+strideq*0+16*0]
    punpcklbw            m4, m1, m3
    punpckhbw            m1, m3
    PMADDUBSW            m4, m5, m6, m7, 0
    PMADDUBSW            m1, m5, m6, m7, 0
    mova        [tmpq+16*2], m4
    mova        [tmpq+16*3], m1
    movu                 m1, [srcq+strideq*0+16*1]
    add                tmpq, r6
    punpcklbw            m4, m2, m0
    punpckhbw            m2, m0
    PMADDUBSW            m4, m5, m6, m7, 0
    PMADDUBSW            m2, m5, m6, m7, 0
    mova        [tmpq+16*0], m4
    mova        [tmpq+16*1], m2
    punpcklbw            m4, m3, m1
    punpckhbw            m3, m1
    PMADDUBSW            m4, m5, m6, m7, 0
    PMADDUBSW            m3, m5, m6, m7, 0
    mova        [tmpq+16*2], m4
    mova        [tmpq+16*3], m3
    add                tmpq, r6
    sub                  hd, 2
    jg .v_w32_vloop
    add                  r5, 32
    movzx                hd, r3b
    mov                srcq, r5
%if ARCH_X86_64
    add                  r7, 16*4
    mov                tmpq, r7
%else
    mov                tmpq, tmpmp
    add                tmpq, 16*4
    mov               tmpmp, tmpq
%endif
    sub                 r3d, 1<<8
    jg .v_w32_hloop
%if WIN64
    POP                  r7
%endif
    RET
.hv:
    ; (16 * src[x] + (my * (src[x + src_stride] - src[x])) + 8) >> 4
    ; = src[x] + (((my * (src[x + src_stride] - src[x])) + 8) >> 4)
    movzx                wd, word [r6+wq*2+table_offset(prep, _bilin_hv)]
%assign stack_offset stack_offset - stack_size_padded
%if cpuflag(ssse3)
    imul               mxyd, 0x08000800
    WIN64_SPILL_XMM 8
%else
    or                 mxyd, 1<<16
    WIN64_SPILL_XMM 9
 %if ARCH_X86_64
    mova                 m8, [base+pw_8]
 %else
  %define                m8  [base+pw_8]
 %endif
    pxor                 m7, m7
%endif
    movd                 m6, mxyd
    add                  wq, r6
    pshufd               m6, m6, q0000
    jmp                  wq
.hv_w4:
%if cpuflag(ssse3)
    mova                 m4, [base+bilin_h_shuf4]
    movddup              m0, [srcq+strideq*0]
%else
    movhps               m0, [srcq+strideq*0]
%endif
    lea                  r3, [strideq*3]
    PSHUFB_BILIN_H4      m0, m4, m3
    PMADDUBSW            m0, m5, m7, m4, 0 ; _ 0
.hv_w4_loop:
    movq                 m1, [srcq+strideq*1]
    movhps               m1, [srcq+strideq*2]
    movq                 m2, [srcq+r3       ]
    lea                srcq, [srcq+strideq*4]
    movhps               m2, [srcq+strideq*0]
    PSHUFB_BILIN_H4      m1, m4, m3
    PSHUFB_BILIN_H4      m2, m4, m3
    PMADDUBSW            m1, m5, m7, m4, 0 ; 1 2
    PMADDUBSW            m2, m5, m7, m4, 0 ; 3 4
    shufpd               m0, m1, 0x01      ; 0 1
    shufpd               m3, m1, m2, 0x01  ; 2 3
    psubw                m1, m0
    PMULHRSW             m1, m6, m4, m8, 4
    paddw                m1, m0
    mova                 m0, m2
    psubw                m2, m3
    PMULHRSW             m2, m6, m4, m8, 4
    paddw                m2, m3
    mova        [tmpq+16*0], m1
    mova        [tmpq+16*1], m2
    add                tmpq, 32
    sub                  hd, 4
    jg .hv_w4_loop
    RET
.hv_w8:
    movu                 m0, [srcq+strideq*0]
    PSHUFB_BILIN_H8      m0, m4
    PMADDUBSW            m0, m5, m7, m4, 0 ; 0
.hv_w8_loop:
    movu                 m1, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    movu                 m2, [srcq+strideq*0]
    PSHUFB_BILIN_H8      m1, m4
    PSHUFB_BILIN_H8      m2, m4
    PMADDUBSW            m1, m5, m7, m4, 0 ; 1
    PMADDUBSW            m2, m5, m7, m4, 0 ; 2
    psubw                m3, m1, m0
    PMULHRSW             m3, m6, m4, m8, 4
    paddw                m3, m0
    mova                 m0, m2
    psubw                m2, m1
    PMULHRSW             m2, m6, m4, m8, 4
    paddw                m2, m1
    mova        [tmpq+16*0], m3
    mova        [tmpq+16*1], m2
    add                tmpq, 16*2
    sub                  hd, 2
    jg .hv_w8_loop
    RET
.hv_w128:
    lea                 r3d, [hq+(7<<8)]
    mov                 r5d, 256
    jmp .hv_w16_start
.hv_w64:
    lea                 r3d, [hq+(3<<8)]
    mov                 r5d, 128
    jmp .hv_w16_start
.hv_w32:
    lea                 r3d, [hq+(1<<8)]
    mov                 r5d, 64
    jmp .hv_w16_start
.hv_w16:
    xor                 r3d, r3d
    mov                 r5d, 32
.hv_w16_start:
%if ARCH_X86_64 || cpuflag(ssse3)
    mov                  r6, srcq
%endif
%if ARCH_X86_64
 %if WIN64
    PUSH                 r7
 %endif
    mov                  r7, tmpq
%endif
.hv_w16_hloop:
    movu                 m0, [srcq+strideq*0+8*0]
    movu                 m1, [srcq+strideq*0+8*1]
    PSHUFB_BILIN_H8      m0, m4
    PSHUFB_BILIN_H8      m1, m4
    PMADDUBSW            m0, m5, m7, m4, 0 ; 0a
    PMADDUBSW            m1, m5, m7, m4, 0 ; 0b
.hv_w16_vloop:
    movu                 m2, [srcq+strideq*1+8*0]
    PSHUFB_BILIN_H8      m2, m4
    PMADDUBSW            m2, m5, m7, m4, 0 ; 1a
    psubw                m3, m2, m0
    PMULHRSW             m3, m6, m4, m8, 4
    paddw                m3, m0
    mova        [tmpq+16*0], m3
    movu                 m3, [srcq+strideq*1+8*1]
    lea                srcq, [srcq+strideq*2]
    PSHUFB_BILIN_H8      m3, m4
    PMADDUBSW            m3, m5, m7, m4, 0 ; 1b
    psubw                m0, m3, m1
    PMULHRSW             m0, m6, m4, m8, 4
    paddw                m0, m1
    mova        [tmpq+16*1], m0
    add                tmpq, r5
    movu                 m0, [srcq+strideq*0+8*0]
    PSHUFB_BILIN_H8      m0, m4
    PMADDUBSW            m0, m5, m7, m4, 0 ; 2a
    psubw                m1, m0, m2
    PMULHRSW             m1, m6, m4, m8, 4
    paddw                m1, m2
    mova        [tmpq+16*0], m1
    movu                 m1, [srcq+strideq*0+8*1]
    PSHUFB_BILIN_H8      m1, m4
    PMADDUBSW            m1, m5, m7, m4, 0 ; 2b
    psubw                m2, m1, m3
    PMULHRSW             m2, m6, m4, m8, 4
    paddw                m2, m3
    mova        [tmpq+16*1], m2
    add                tmpq, r5
    sub                  hd, 2
    jg .hv_w16_vloop
    movzx                hd, r3b
%if ARCH_X86_64
    add                  r6, 16
    add                  r7, 2*16
    mov                srcq, r6
    mov                tmpq, r7
%elif cpuflag(ssse3)
    mov                tmpq, tmpm
    add                  r6, 16
    add                tmpq, 2*16
    mov                srcq, r6
    mov                tmpm, tmpq
%else
    mov                srcq, srcm
    mov                tmpq, tmpm
    add                srcq, 16
    add                tmpq, 2*16
    mov                srcm, srcq
    mov                tmpm, tmpq
%endif
    sub                 r3d, 1<<8
    jg .hv_w16_hloop
%if WIN64
    POP                  r7
%endif
    RET
%endmacro

; int8_t subpel_filters[5][15][8]
%assign FILTER_REGULAR (0*15 << 16) | 3*15
%assign FILTER_SMOOTH  (1*15 << 16) | 4*15
%assign FILTER_SHARP   (2*15 << 16) | 3*15

%macro FN 4 ; prefix, type, type_h, type_v
cglobal %1_%2_8bpc
    mov                 t0d, FILTER_%3
%ifidn %3, %4
    mov                 t1d, t0d
%else
    mov                 t1d, FILTER_%4
%endif
%ifnidn %2, regular ; skip the jump in the last filter
    jmp mangle(private_prefix %+ _%1_8bpc %+ SUFFIX)
%endif
%endmacro

%if ARCH_X86_32
DECLARE_REG_TMP 1, 2
%elif WIN64
DECLARE_REG_TMP 4, 5
%else
DECLARE_REG_TMP 7, 8
%endif

FN put_8tap, sharp,          SHARP,   SHARP
FN put_8tap, sharp_smooth,   SHARP,   SMOOTH
FN put_8tap, smooth_sharp,   SMOOTH,  SHARP
FN put_8tap, smooth,         SMOOTH,  SMOOTH
FN put_8tap, sharp_regular,  SHARP,   REGULAR
FN put_8tap, regular_sharp,  REGULAR, SHARP
FN put_8tap, smooth_regular, SMOOTH,  REGULAR
FN put_8tap, regular_smooth, REGULAR, SMOOTH
FN put_8tap, regular,        REGULAR, REGULAR

%if ARCH_X86_32
 %define base_reg r1
 %define base base_reg-put_ssse3
%else
 %define base_reg r8
 %define base 0
%endif

cglobal put_8tap_8bpc, 1, 9, 0, dst, ds, src, ss, w, h, mx, my, ss3
%assign org_stack_offset stack_offset
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 8tap_h, mx, 4tap_h
%if ARCH_X86_64
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 8tap_v, my, 4tap_v
%else
    imul                ssd, mym, 0x010101
    add                 ssd, t1d ; 8tap_v, my, 4tap_v
    mov                srcq, srcm
%endif
    mov                  wd, wm
    movifnidn            hd, hm
    LEA            base_reg, put_ssse3
    test                mxd, 0xf00
    jnz .h
%if ARCH_X86_32
    test                ssd, 0xf00
%else
    test                myd, 0xf00
%endif
    jnz .v
    tzcnt                wd, wd
    movzx                wd, word [base_reg+wq*2+table_offset(put,)]
    add                  wq, base_reg
; put_bilin mangling jump
%assign stack_offset org_stack_offset
    movifnidn           dsq, dsmp
    movifnidn           ssq, ssmp
%if WIN64
    pop                  r8
%endif
    lea                  r6, [ssq*3]
    jmp                  wq
.h:
%if ARCH_X86_32
    test                ssd, 0xf00
%else
    test                myd, 0xf00
%endif
    jnz .hv
    movifnidn           ssq, ssmp
    WIN64_SPILL_XMM      12
    cmp                  wd, 4
    jl .h_w2
    je .h_w4
    tzcnt                wd, wd
%if ARCH_X86_64
    mova                m10, [base+subpel_h_shufA]
    mova                m11, [base+subpel_h_shufB]
    mova                 m9, [base+subpel_h_shufC]
%endif
    shr                 mxd, 16
    sub                srcq, 3
    movzx                wd, word [base_reg+wq*2+table_offset(put, _8tap_h)]
    movq                 m6, [base_reg+mxq*8+subpel_filters-put_ssse3]
    mova                 m7, [base+pw_34] ; 2 + (8 << 2)
    pshufd               m5, m6, q0000
    pshufd               m6, m6, q1111
    add                  wq, base_reg
    jmp                  wq
.h_w2:
%if ARCH_X86_32
    and                 mxd, 0x7f
%else
    movzx               mxd, mxb
%endif
    dec                srcq
    mova                 m4, [base+subpel_h_shuf4]
    movd                 m3, [base_reg+mxq*8+subpel_filters-put_ssse3+2]
    mova                 m5, [base+pw_34] ; 2 + (8 << 2)
    pshufd               m3, m3, q0000
    movifnidn           dsq, dsmp
.h_w2_loop:
    movq                 m0, [srcq+ssq*0]
    movhps               m0, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb               m0, m4
    pmaddubsw            m0, m3
    phaddw               m0, m0
    paddw                m0, m5 ; pw34
    psraw                m0, 6
    packuswb             m0, m0
    movd                r6d, m0
    mov        [dstq+dsq*0], r6w
    shr                 r6d, 16
    mov        [dstq+dsq*1], r6w
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w2_loop
    RET
.h_w4:
%if ARCH_X86_32
    and                 mxd, 0x7f
%else
    movzx               mxd, mxb
%endif
    dec                srcq
    movd                 m3, [base_reg+mxq*8+subpel_filters-put_ssse3+2]
    mova                 m6, [base+subpel_h_shufA]
    mova                 m5, [base+pw_34] ; 2 + (8 << 2)
    pshufd               m3, m3, q0000
    movifnidn           dsq, dsmp
.h_w4_loop:
    movq                 m0, [srcq+ssq*0] ; 1
    movq                 m1, [srcq+ssq*1] ; 2
    lea                srcq, [srcq+ssq*2]
    pshufb               m0, m6 ; subpel_h_shufA
    pshufb               m1, m6 ; subpel_h_shufA
    pmaddubsw            m0, m3 ; subpel_filters
    pmaddubsw            m1, m3 ; subpel_filters
    phaddw               m0, m1
    paddw                m0, m5 ; pw34
    psraw                m0, 6
    packuswb             m0, m0
    movd       [dstq+dsq*0], m0
    psrlq                m0, 32
    movd       [dstq+dsq*1], m0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w4_loop
    RET
%macro PUT_8TAP_H 4 ; dst/src, tmp[1-3]
 %if ARCH_X86_32
    pshufb              %2, %1, [base+subpel_h_shufB]
    pshufb              %3, %1, [base+subpel_h_shufC]
    pshufb              %1,     [base+subpel_h_shufA]
 %else
    pshufb              %2, %1, m11; subpel_h_shufB
    pshufb              %3, %1, m9 ; subpel_h_shufC
    pshufb              %1, m10    ; subpel_h_shufA
 %endif
    pmaddubsw           %4, %2, m5 ; subpel +0 B0
    pmaddubsw           %2, m6     ; subpel +4 B4
    pmaddubsw           %3, m6     ; C4
    pmaddubsw           %1, m5     ; A0
    paddw               %3, %4     ; C4+B0
    paddw               %1, %2     ; A0+B4
    phaddw              %1, %3
    paddw               %1, m7     ; pw34
    psraw               %1, 6
%endmacro
.h_w8:
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    PUT_8TAP_H           m0, m2, m3, m4
    PUT_8TAP_H           m1, m2, m3, m4
    packuswb             m0, m1
%if ARCH_X86_32
    movq             [dstq], m0
    add                dstq, dsm
    movhps           [dstq], m0
    add                dstq, dsm
%else
    movq       [dstq+dsq*0], m0
    movhps     [dstq+dsq*1], m0
    lea                dstq, [dstq+dsq*2]
%endif
    sub                  hd, 2
    jg .h_w8
    RET
.h_w128:
    mov                  r4, -16*7
    jmp .h_w16_start
.h_w64:
    mov                  r4, -16*3
    jmp .h_w16_start
.h_w32:
    mov                  r4, -16*1
    jmp .h_w16_start
.h_w16:
    xor                 r4d, r4d
.h_w16_start:
    sub                srcq, r4
    sub                dstq, r4
.h_w16_loop_v:
    mov                  r6, r4
.h_w16_loop_h:
    movu                 m0, [srcq+r6+8*0]
    movu                 m1, [srcq+r6+8*1]
    PUT_8TAP_H           m0, m2, m3, m4
    PUT_8TAP_H           m1, m2, m3, m4
    packuswb             m0, m1
    mova          [dstq+r6], m0
    add                  r6, 16
    jle .h_w16_loop_h
    add                srcq, ssq
    add                dstq, dsmp
    dec                  hd
    jg .h_w16_loop_v
    RET
.v:
%if ARCH_X86_32
    movzx               mxd, ssb
    shr                 ssd, 16
    cmp                  hd, 6
    cmovs               ssd, mxd
    movq                 m0, [base_reg+ssq*8+subpel_filters-put_ssse3]
%else
 %assign stack_offset org_stack_offset
    WIN64_SPILL_XMM      16
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    movq                 m0, [base_reg+myq*8+subpel_filters-put_ssse3]
%endif
    tzcnt               r6d, wd
    movzx               r6d, word [base_reg+r6*2+table_offset(put, _8tap_v)]
    punpcklwd            m0, m0
    mova                 m7, [base+pw_512]
    add                  r6, base_reg
%if ARCH_X86_32
 %define            subpel0  [rsp+mmsize*0]
 %define            subpel1  [rsp+mmsize*1]
 %define            subpel2  [rsp+mmsize*2]
 %define            subpel3  [rsp+mmsize*3]
%assign regs_used 2 ; use r1 (ds) as tmp for stack alignment if needed
    ALLOC_STACK       -16*4
%assign regs_used 7
    pshufd               m1, m0, q0000
    mova            subpel0, m1
    pshufd               m1, m0, q1111
    mova            subpel1, m1
    pshufd               m1, m0, q2222
    mova            subpel2, m1
    pshufd               m1, m0, q3333
    mova            subpel3, m1
    mov                 ssq, [rstk+stack_offset+gprsize*4]
    lea                 ssq, [ssq*3]
    sub                srcq, ssq
    mov                 ssq, [rstk+stack_offset+gprsize*4]
    mov                 dsq, [rstk+stack_offset+gprsize*2]
%else
 %define            subpel0  m8
 %define            subpel1  m9
 %define            subpel2  m10
 %define            subpel3  m11
    lea                ss3q, [ssq*3]
    pshufd               m8, m0, q0000
    sub                srcq, ss3q
    pshufd               m9, m0, q1111
    pshufd              m10, m0, q2222
    pshufd              m11, m0, q3333
%endif
    jmp                  r6
.v_w2:
    movd                 m1, [srcq+ssq*0]
    movd                 m0, [srcq+ssq*1]
%if ARCH_X86_32
    lea                srcq, [srcq+ssq*2]
    movd                 m2, [srcq+ssq*0]
    movd                 m5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movd                 m3, [srcq+ssq*0]
    movd                 m4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
%else
    movd                 m2, [srcq+ssq*2]
    add                srcq, ss3q
    movd                 m5, [srcq+ssq*0]
    movd                 m3, [srcq+ssq*1]
    movd                 m4, [srcq+ssq*2]
    add                srcq, ss3q
%endif
    punpcklwd            m1, m0           ; 0 1
    punpcklwd            m0, m2           ; 1 2
    punpcklbw            m1, m0           ; 01 12
    movd                 m0, [srcq+ssq*0]
    punpcklwd            m2, m5           ; 2 3
    punpcklwd            m5, m3           ; 3 4
    punpcklwd            m3, m4           ; 4 5
    punpcklwd            m4, m0           ; 5 6
    punpcklbw            m2, m5           ; 23 34
    punpcklbw            m3, m4           ; 45 56
.v_w2_loop:
    movd                 m4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddubsw            m5, m1, subpel0     ; a0 b0
    mova                 m1, m2
    pmaddubsw            m2, subpel1         ; a1 b1
    paddw                m5, m2
    mova                 m2, m3
    pmaddubsw            m3, subpel2         ; a2 b2
    paddw                m5, m3
    punpcklwd            m3, m0, m4          ; 6 7
    movd                 m0, [srcq+ssq*0]
    punpcklwd            m4, m0              ; 7 8
    punpcklbw            m3, m4              ; 67 78
    pmaddubsw            m4, m3, subpel3     ; a3 b3
    paddw                m5, m4
    pmulhrsw             m5, m7
    packuswb             m5, m5
    movd                r6d, m5
    mov        [dstq+dsq*0], r6w
    shr                 r6d, 16
    mov        [dstq+dsq*1], r6w
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
%if ARCH_X86_32
.v_w8:
.v_w16:
.v_w32:
.v_w64:
.v_w128:
    shl                  wd, 14
%if STACK_ALIGNMENT < 16
 %define               dstm [rsp+mmsize*4+gprsize]
    mov                dstm, dstq
%endif
    lea                 r6d, [hq+wq-(1<<16)]
    mov                  r4, srcq
.v_w4_loop0:
%endif
    movd                 m1, [srcq+ssq*0]
    movd                 m0, [srcq+ssq*1]
%if ARCH_X86_32
    lea                srcq, [srcq+ssq*2]
    movd                 m2, [srcq+ssq*0]
    movd                 m5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movd                 m3, [srcq+ssq*0]
    movd                 m4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
%else
    movd                 m2, [srcq+ssq*2]
    add                srcq, ss3q
    movd                 m5, [srcq+ssq*0]
    movd                 m3, [srcq+ssq*1]
    movd                 m4, [srcq+ssq*2]
    add                srcq, ss3q
%endif
    punpckldq            m1, m0           ; 0 1
    punpckldq            m0, m2           ; 1 2
    punpcklbw            m1, m0           ; 01 12
    movd                 m0, [srcq+ssq*0]
    punpckldq            m2, m5           ; 2 3
    punpckldq            m5, m3           ; 3 4
    punpckldq            m3, m4           ; 4 5
    punpckldq            m4, m0           ; 5 6
    punpcklbw            m2, m5           ; 23 34
    punpcklbw            m3, m4           ; 45 56
.v_w4_loop:
    movd                 m4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddubsw            m5, m1, subpel0  ; a0 b0
    mova                 m1, m2
    pmaddubsw            m2, subpel1      ; a1 b1
    paddw                m5, m2
    mova                 m2, m3
    pmaddubsw            m3, subpel2      ; a2 b2
    paddw                m5, m3
    punpckldq            m3, m0, m4       ; 6 7 _ _
    movd                 m0, [srcq+ssq*0]
    punpckldq            m4, m0           ; 7 8 _ _
    punpcklbw            m3, m4           ; 67 78
    pmaddubsw            m4, m3, subpel3  ; a3 b3
    paddw                m5, m4
    pmulhrsw             m5, m7
    packuswb             m5, m5
    movd       [dstq+dsq*0], m5
    psrlq                m5, 32
    movd       [dstq+dsq*1], m5
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
%if ARCH_X86_32
    mov                dstq, dstm
    add                  r4, 4
    movzx                hd, r6w
    add                dstq, 4
    mov                srcq, r4
    mov                dstm, dstq
    sub                 r6d, 1<<16
    jg .v_w4_loop0
%endif
    RET
%if ARCH_X86_64
.v_w8:
.v_w16:
.v_w32:
.v_w64:
.v_w128:
    lea                 r6d, [wq*8-64]
    mov                  r4, srcq
    mov                  r7, dstq
    lea                 r6d, [hq+r6*4]
.v_w8_loop0:
    movq                 m1, [srcq+ssq*0]
    movq                 m2, [srcq+ssq*1]
    movq                 m3, [srcq+ssq*2]
    add                srcq, ss3q
    movq                 m4, [srcq+ssq*0]
    movq                 m5, [srcq+ssq*1]
    movq                 m6, [srcq+ssq*2]
    add                srcq, ss3q
    movq                 m0, [srcq+ssq*0]
    punpcklbw            m1, m2 ; 01
    punpcklbw            m2, m3 ; 12
    punpcklbw            m3, m4 ; 23
    punpcklbw            m4, m5 ; 34
    punpcklbw            m5, m6 ; 45
    punpcklbw            m6, m0 ; 56
.v_w8_loop:
    movq                m13, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddubsw           m14, m1, subpel0 ; a0
    mova                 m1, m3
    pmaddubsw           m15, m2, subpel0 ; b0
    mova                 m2, m4
    pmaddubsw            m3, subpel1 ; a1
    mova                m12, m0
    pmaddubsw            m4, subpel1 ; b1
    movq                 m0, [srcq+ssq*0]
    paddw               m14, m3
    paddw               m15, m4
    mova                 m3, m5
    pmaddubsw            m5, subpel2 ; a2
    mova                 m4, m6
    pmaddubsw            m6, subpel2 ; b2
    punpcklbw           m12, m13     ; 67
    punpcklbw           m13, m0      ; 78
    paddw               m14, m5
    mova                 m5, m12
    pmaddubsw           m12, subpel3 ; a3
    paddw               m15, m6
    mova                 m6, m13
    pmaddubsw           m13, subpel3 ; b3
    paddw               m14, m12
    paddw               m15, m13
    pmulhrsw            m14, m7
    pmulhrsw            m15, m7
    packuswb            m14, m15
    movq       [dstq+dsq*0], m14
    movhps     [dstq+dsq*1], m14
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w8_loop
    add                  r4, 8
    add                  r7, 8
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
    sub                 r6d, 1<<8
    jg .v_w8_loop0
    RET
%endif ;ARCH_X86_64
%undef subpel0
%undef subpel1
%undef subpel2
%undef subpel3
.hv:
    %assign stack_offset org_stack_offset
    cmp                  wd, 4
    jg .hv_w8
%if ARCH_X86_32
    and                 mxd, 0x7f
%else
    movzx               mxd, mxb
%endif
    dec                srcq
    movd                 m1, [base_reg+mxq*8+subpel_filters-put_ssse3+2]
%if ARCH_X86_32
    movzx               mxd, ssb
    shr                 ssd, 16
    cmp                  hd, 6
    cmovs               ssd, mxd
    movq                 m0, [base_reg+ssq*8+subpel_filters-put_ssse3]
    mov                 ssq, ssmp
    lea                  r6, [ssq*3]
    sub                srcq, r6
 %define           base_reg  r6
    mov                  r6, r1; use as new base
 %assign regs_used 2
    ALLOC_STACK  -mmsize*14
 %assign regs_used 7
    mov                 dsq, [rstk+stack_offset+gprsize*2]
 %define           subpelv0  [rsp+mmsize*0]
 %define           subpelv1  [rsp+mmsize*1]
 %define           subpelv2  [rsp+mmsize*2]
 %define           subpelv3  [rsp+mmsize*3]
    punpcklbw            m0, m0
    psraw                m0, 8 ; sign-extend
    pshufd               m6, m0, q0000
    mova           subpelv0, m6
    pshufd               m6, m0, q1111
    mova           subpelv1, m6
    pshufd               m6, m0, q2222
    mova           subpelv2, m6
    pshufd               m6, m0, q3333
    mova           subpelv3, m6
%else
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    movq                 m0, [base_reg+myq*8+subpel_filters-put_ssse3]
    ALLOC_STACK   mmsize*14, 14
    lea                ss3q, [ssq*3]
    sub                srcq, ss3q
 %define           subpelv0  m10
 %define           subpelv1  m11
 %define           subpelv2  m12
 %define           subpelv3  m13
    punpcklbw            m0, m0
    psraw                m0, 8 ; sign-extend
    mova                 m8, [base+pw_8192]
    mova                 m9, [base+pd_512]
    pshufd              m10, m0, q0000
    pshufd              m11, m0, q1111
    pshufd              m12, m0, q2222
    pshufd              m13, m0, q3333
%endif
    pshufd               m7, m1, q0000
    cmp                  wd, 4
    je .hv_w4
.hv_w2:
    mova                 m6, [base+subpel_h_shuf4]
    movq                 m2, [srcq+ssq*0]     ; 0
    movhps               m2, [srcq+ssq*1]     ; 0 _ 1
%if ARCH_X86_32
 %define           w8192reg  [base+pw_8192]
 %define            d512reg  [base+pd_512]
    lea                srcq, [srcq+ssq*2]
    movq                 m0, [srcq+ssq*0]     ; 2
    movhps               m0, [srcq+ssq*1]     ; 2 _ 3
    lea                srcq, [srcq+ssq*2]
%else
 %define           w8192reg  m8
 %define            d512reg  m9
    movq                 m0, [srcq+ssq*2]     ; 2
    add                srcq, ss3q
    movhps               m0, [srcq+ssq*0]     ; 2 _ 3
%endif
    pshufb               m2, m6 ; 0 ~ 1 ~
    pshufb               m0, m6 ; 2 ~ 3 ~
    pmaddubsw            m2, m7 ; subpel_filters
    pmaddubsw            m0, m7 ; subpel_filters
    phaddw               m2, m0 ; 0 1 2 3
    pmulhrsw             m2, w8192reg
%if ARCH_X86_32
    movq                 m3, [srcq+ssq*0]     ; 4
    movhps               m3, [srcq+ssq*1]     ; 4 _ 5
    lea                srcq, [srcq+ssq*2]
%else
    movq                 m3, [srcq+ssq*1]     ; 4
    movhps               m3, [srcq+ssq*2]     ; 4 _ 5
    add                srcq, ss3q
%endif
    movq                 m0, [srcq+ssq*0]     ; 6
    pshufb               m3, m6 ; 4 ~ 5 ~
    pshufb               m0, m6 ; 6 ~
    pmaddubsw            m3, m7 ; subpel_filters
    pmaddubsw            m0, m7 ; subpel_filters
    phaddw               m3, m0 ; 4 5 6 _
    pmulhrsw             m3, w8192reg
    palignr              m4, m3, m2, 4; V        1 2 3 4
    punpcklwd            m1, m2, m4   ; V 01 12    0 1 1 2
    punpckhwd            m2, m4       ; V 23 34    2 3 3 4
    pshufd               m0, m3, q2121; V          5 6 5 6
    punpcklwd            m3, m0       ; V 45 56    4 5 5 6
.hv_w2_loop:
    movq                 m4, [srcq+ssq*1] ; V 7
    lea                srcq, [srcq+ssq*2] ; V
    movhps               m4, [srcq+ssq*0] ; V 7 8
    pshufb               m4, m6
    pmaddubsw            m4, m7
    pmaddwd              m5, m1, subpelv0; V a0 b0
    mova                 m1, m2       ; V
    pmaddwd              m2, subpelv1 ; V a1 b1
    paddd                m5, m2       ; V
    mova                 m2, m3       ; V
    pmaddwd              m3, subpelv2 ; a2 b2
    phaddw               m4, m4
    pmulhrsw             m4, w8192reg
    paddd                m5, m3       ; V
    palignr              m3, m4, m0, 12
    mova                 m0, m4
    punpcklwd            m3, m0           ; V 67 78
    pmaddwd              m4, m3, subpelv3 ; V a3 b3
    paddd                m5, d512reg
    paddd                m5, m4
    psrad                m5, 10
    packssdw             m5, m5
    packuswb             m5, m5
    movd                r4d, m5
    mov        [dstq+dsq*0], r4w
    shr                 r4d, 16
    mov        [dstq+dsq*1], r4w
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    RET
%undef w8192reg
%undef d512reg
.hv_w4:
%define hv4_line_0_0 4
%define hv4_line_0_1 5
%define hv4_line_0_2 6
%define hv4_line_0_3 7
%define hv4_line_0_4 8
%define hv4_line_0_5 9
%define hv4_line_1_0 10
%define hv4_line_1_1 11
%define hv4_line_1_2 12
%define hv4_line_1_3 13
%macro SAVELINE_W4 3
    mova     [rsp+mmsize*hv4_line_%3_%2], %1
%endmacro
%macro RESTORELINE_W4 3
    mova     %1, [rsp+mmsize*hv4_line_%3_%2]
%endmacro
%if ARCH_X86_32
 %define           w8192reg  [base+pw_8192]
 %define            d512reg  [base+pd_512]
%else
 %define           w8192reg  m8
 %define            d512reg  m9
%endif
    ; lower shuffle 0 1 2 3 4
    mova                 m6, [base+subpel_h_shuf4]
    movq                 m5, [srcq+ssq*0]   ; 0 _ _ _
    movhps               m5, [srcq+ssq*1]   ; 0 _ 1 _
%if ARCH_X86_32
    lea                srcq, [srcq+ssq*2]
    movq                 m4, [srcq+ssq*0]   ; 2 _ _ _
    movhps               m4, [srcq+ssq*1]   ; 2 _ 3 _
    lea                srcq, [srcq+ssq*2]
%else
    movq                 m4, [srcq+ssq*2]   ; 2 _ _ _
    movhps               m4, [srcq+ss3q ]   ; 2 _ 3 _
    lea                srcq, [srcq+ssq*4]
%endif
    pshufb               m2, m5, m6 ;H subpel_h_shuf4 0 ~ 1 ~
    pshufb               m0, m4, m6 ;H subpel_h_shuf4 2 ~ 3 ~
    pmaddubsw            m2, m7 ;H subpel_filters
    pmaddubsw            m0, m7 ;H subpel_filters
    phaddw               m2, m0 ;H 0 1 2 3
    pmulhrsw             m2, w8192reg ;H pw_8192
    SAVELINE_W4          m2, 2, 0
    ; upper shuffle 2 3 4 5 6
    mova                 m6, [base+subpel_h_shuf4+16]
    pshufb               m2, m5, m6 ;H subpel_h_shuf4 0 ~ 1 ~
    pshufb               m0, m4, m6 ;H subpel_h_shuf4 2 ~ 3 ~
    pmaddubsw            m2, m7 ;H subpel_filters
    pmaddubsw            m0, m7 ;H subpel_filters
    phaddw               m2, m0 ;H 0 1 2 3
    pmulhrsw             m2, w8192reg ;H pw_8192
    ;
    ; lower shuffle
    mova                 m6, [base+subpel_h_shuf4]
    movq                 m5, [srcq+ssq*0]   ; 4 _ _ _
    movhps               m5, [srcq+ssq*1]   ; 4 _ 5 _
%if ARCH_X86_32
    lea                srcq, [srcq+ssq*2]
    movq                 m4, [srcq+ssq*0]   ; 6 _ _ _
    add                srcq, ssq
%else
    movq                 m4, [srcq+ssq*2]   ; 6 _ _ _
    add                srcq, ss3q
%endif
    pshufb               m3, m5, m6 ;H subpel_h_shuf4 4 ~ 5 ~
    pshufb               m0, m4, m6 ;H subpel_h_shuf4 6 ~ 6 ~
    pmaddubsw            m3, m7 ;H subpel_filters
    pmaddubsw            m0, m7 ;H subpel_filters
    phaddw               m3, m0 ;H 4 5 6 7
    pmulhrsw             m3, w8192reg ;H pw_8192
    SAVELINE_W4          m3, 3, 0
    ; upper shuffle
    mova                 m6, [base+subpel_h_shuf4+16]
    pshufb               m3, m5, m6 ;H subpel_h_shuf4 4 ~ 5 ~
    pshufb               m0, m4, m6 ;H subpel_h_shuf4 6 ~ 6 ~
    pmaddubsw            m3, m7 ;H subpel_filters
    pmaddubsw            m0, m7 ;H subpel_filters
    phaddw               m3, m0 ;H 4 5 6 7
    pmulhrsw             m3, w8192reg ;H pw_8192
    ;process high
    palignr              m4, m3, m2, 4;V 1 2 3 4
    punpcklwd            m1, m2, m4  ; V 01 12
    punpckhwd            m2, m4      ; V 23 34
    pshufd               m0, m3, q2121;V 5 6 5 6
    punpcklwd            m3, m0      ; V 45 56
    SAVELINE_W4          m0, 0, 1
    SAVELINE_W4          m1, 1, 1
    SAVELINE_W4          m2, 2, 1
    SAVELINE_W4          m3, 3, 1
    ;process low
    RESTORELINE_W4       m2, 2, 0
    RESTORELINE_W4       m3, 3, 0
    palignr              m4, m3, m2, 4;V 1 2 3 4
    punpcklwd            m1, m2, m4  ; V 01 12
    punpckhwd            m2, m4      ; V 23 34
    pshufd               m0, m3, q2121;V 5 6 5 6
    punpcklwd            m3, m0      ; V 45 56
.hv_w4_loop:
    ;process low
    pmaddwd              m5, m1, subpelv0 ; V a0 b0
    mova                 m1, m2
    pmaddwd              m2, subpelv1; V a1 b1
    paddd                m5, m2
    mova                 m2, m3
    pmaddwd              m3, subpelv2; V a2 b2
    paddd                m5, m3
    mova                 m6, [base+subpel_h_shuf4]
    movq                 m4, [srcq+ssq*0] ; 7
    movhps               m4, [srcq+ssq*1] ; 7 _ 8 _
    pshufb               m4, m6 ;H subpel_h_shuf4 7 ~ 8 ~
    pmaddubsw            m4, m7 ;H subpel_filters
    phaddw               m4, m4 ;H                7 8 7 8
    pmulhrsw             m4, w8192reg ;H pw_8192
    palignr              m3, m4, m0, 12         ; 6 7 8 7
    mova                 m0, m4
    punpcklwd            m3, m4      ; 67 78
    pmaddwd              m4, m3, subpelv3; a3 b3
    paddd                m5, d512reg ; pd_512
    paddd                m5, m4
    psrad                m5, 10
    SAVELINE_W4          m0, 0, 0
    SAVELINE_W4          m1, 1, 0
    SAVELINE_W4          m2, 2, 0
    SAVELINE_W4          m3, 3, 0
    SAVELINE_W4          m5, 5, 0
    ;process high
    RESTORELINE_W4       m0, 0, 1
    RESTORELINE_W4       m1, 1, 1
    RESTORELINE_W4       m2, 2, 1
    RESTORELINE_W4       m3, 3, 1
    pmaddwd              m5, m1, subpelv0; V a0 b0
    mova                 m1, m2
    pmaddwd              m2, subpelv1; V a1 b1
    paddd                m5, m2
    mova                 m2, m3
    pmaddwd              m3, subpelv2; V a2 b2
    paddd                m5, m3
    mova                 m6, [base+subpel_h_shuf4+16]
    movq                 m4, [srcq+ssq*0] ; 7
    movhps               m4, [srcq+ssq*1] ; 7 _ 8 _
    lea                srcq, [srcq+ssq*2]
    pshufb               m4, m6 ;H subpel_h_shuf4 7 ~ 8 ~
    pmaddubsw            m4, m7 ;H subpel_filters
    phaddw               m4, m4 ;H                7 8 7 8
    pmulhrsw             m4, w8192reg ;H pw_8192
    palignr              m3, m4, m0, 12         ; 6 7 8 7
    mova                 m0, m4
    punpcklwd            m3, m4      ; 67 78
    pmaddwd              m4, m3, subpelv3; a3 b3
    paddd                m5, d512reg ; pd_512
    paddd                m5, m4
    psrad                m4, m5, 10
    RESTORELINE_W4       m5, 5, 0
    packssdw             m5, m4 ; d -> w
    packuswb             m5, m5 ; w -> b
    pshuflw              m5, m5, q3120
    movd       [dstq+dsq*0], m5
    psrlq                m5, 32
    movd       [dstq+dsq*1], m5
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    SAVELINE_W4          m0, 0, 1
    SAVELINE_W4          m1, 1, 1
    SAVELINE_W4          m2, 2, 1
    SAVELINE_W4          m3, 3, 1
    RESTORELINE_W4       m0, 0, 0
    RESTORELINE_W4       m1, 1, 0
    RESTORELINE_W4       m2, 2, 0
    RESTORELINE_W4       m3, 3, 0
    jg .hv_w4_loop
    RET
%undef subpelv0
%undef subpelv1
%undef subpelv2
%undef subpelv3
.hv_w8:
    %assign stack_offset org_stack_offset
%define hv8_line_1 0
%define hv8_line_2 1
%define hv8_line_3 2
%define hv8_line_4 3
%define hv8_line_6 4
%macro SAVELINE_W8 2
    mova     [rsp+hv8_line_%1*mmsize], %2
%endmacro
%macro RESTORELINE_W8 2
    mova     %2, [rsp+hv8_line_%1*mmsize]
%endmacro
    shr                 mxd, 16
    sub                srcq, 3
%if ARCH_X86_32
 %define           base_reg  r1
 %define           subpelh0  [rsp+mmsize*5]
 %define           subpelh1  [rsp+mmsize*6]
 %define           subpelv0  [rsp+mmsize*7]
 %define           subpelv1  [rsp+mmsize*8]
 %define           subpelv2  [rsp+mmsize*9]
 %define           subpelv3  [rsp+mmsize*10]
 %define             accuv0  [rsp+mmsize*11]
 %define             accuv1  [rsp+mmsize*12]
    movq                 m1, [base_reg+mxq*8+subpel_filters-put_ssse3]
    movzx               mxd, ssb
    shr                 ssd, 16
    cmp                  hd, 6
    cmovs               ssd, mxd
    movq                 m5, [base_reg+ssq*8+subpel_filters-put_ssse3]
    mov                 ssq, ssmp
    ALLOC_STACK  -mmsize*13
%if STACK_ALIGNMENT < 16
 %define               dstm  [rsp+mmsize*13+gprsize*1]
 %define                dsm  [rsp+mmsize*13+gprsize*2]
    mov                  r6, [rstk+stack_offset+gprsize*2]
    mov                 dsm, r6
%endif
    pshufd               m0, m1, q0000
    pshufd               m1, m1, q1111
    punpcklbw            m5, m5
    psraw                m5, 8 ; sign-extend
    pshufd               m2, m5, q0000
    pshufd               m3, m5, q1111
    pshufd               m4, m5, q2222
    pshufd               m5, m5, q3333
    mova           subpelh0, m0
    mova           subpelh1, m1
    mova           subpelv0, m2
    mova           subpelv1, m3
    mova           subpelv2, m4
    mova           subpelv3, m5
    lea                  r6, [ssq*3]
    mov                dstm, dstq
    sub                srcq, r6
%else
    ALLOC_STACK        16*5, 16
 %define           subpelh0  m10
 %define           subpelh1  m11
 %define           subpelv0  m12
 %define           subpelv1  m13
 %define           subpelv2  m14
 %define           subpelv3  m15
 %define             accuv0  m8
 %define             accuv1  m9
    movq                 m0, [base_reg+mxq*8+subpel_filters-put_ssse3]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    movq                 m1, [base_reg+myq*8+subpel_filters-put_ssse3]
    pshufd         subpelh0, m0, q0000
    pshufd         subpelh1, m0, q1111
    punpcklbw            m1, m1
    psraw                m1, 8 ; sign-extend
    pshufd         subpelv0, m1, q0000
    pshufd         subpelv1, m1, q1111
    pshufd         subpelv2, m1, q2222
    pshufd         subpelv3, m1, q3333
    lea                ss3q, [ssq*3]
    mov                  r7, dstq
    sub                srcq, ss3q
%endif
    shl                  wd, 14
    lea                 r6d, [hq+wq-(1<<16)]
    mov                  r4, srcq
.hv_w8_loop0:
    movu                 m4, [srcq+ssq*0] ; 0 = _ _
    movu                 m5, [srcq+ssq*1] ; 1 = _ _
%if ARCH_X86_32
    lea                srcq, [srcq+ssq*2]
%endif
%macro HV_H_W8 4-7 ; src/dst, tmp[1-3], shuf[1-3]
 %if ARCH_X86_32
    pshufb               %3, %1, [base+subpel_h_shufB]
    pshufb               %4, %1, [base+subpel_h_shufC]
    pshufb               %1,     [base+subpel_h_shufA]
 %else
    pshufb               %3, %1, %6  ; subpel_h_shufB
    pshufb               %4, %1, %7  ; subpel_h_shufC
    pshufb               %1, %5      ; subpel_h_shufA
 %endif
    pmaddubsw            %2, %3, subpelh0 ; subpel +0 C0
    pmaddubsw            %4, subpelh1; subpel +4 B4
    pmaddubsw            %3, subpelh1; C4
    pmaddubsw            %1, subpelh0; A0
    paddw                %2, %4      ; C0+B4
    paddw                %1, %3      ; A0+C4
    phaddw               %1, %2
%endmacro
%if ARCH_X86_64
    mova                 m7, [base+subpel_h_shufA]
    mova                 m8, [base+subpel_h_shufB]
    mova                 m9, [base+subpel_h_shufC]
%endif
    HV_H_W8              m4, m1, m2, m3, m7, m8, m9 ; 0 ~ ~ ~
    HV_H_W8              m5, m1, m2, m3, m7, m8, m9 ; 1 ~ ~ ~
%if ARCH_X86_32
    movu                 m6, [srcq+ssq*0] ; 2 = _ _
    movu                 m0, [srcq+ssq*1] ; 3 = _ _
    lea                srcq, [srcq+ssq*2]
%else
    movu                 m6, [srcq+ssq*2] ; 2 = _ _
    add                srcq, ss3q
    movu                 m0, [srcq+ssq*0] ; 3 = _ _
%endif
    HV_H_W8              m6, m1, m2, m3, m7, m8, m9 ; 2 ~ ~ ~
    HV_H_W8              m0, m1, m2, m3, m7, m8, m9 ; 3 ~ ~ ~
    mova                 m7, [base+pw_8192]
    pmulhrsw             m4, m7 ; H pw_8192
    pmulhrsw             m5, m7 ; H pw_8192
    pmulhrsw             m6, m7 ; H pw_8192
    pmulhrsw             m0, m7 ; H pw_8192
    punpcklwd            m1, m4, m5  ; 0 1 ~
    punpcklwd            m2, m5, m6  ; 1 2 ~
    punpcklwd            m3, m6, m0  ; 2 3 ~
    SAVELINE_W8           1, m1
    SAVELINE_W8           2, m2
    SAVELINE_W8           3, m3
    mova                 m7, [base+subpel_h_shufA]
%if ARCH_X86_32
    movu                 m4, [srcq+ssq*0]       ; 4 = _ _
    movu                 m5, [srcq+ssq*1]       ; 5 = _ _
    lea                srcq, [srcq+ssq*2]
%else
    movu                 m4, [srcq+ssq*1]       ; 4 = _ _
    movu                 m5, [srcq+ssq*2]       ; 5 = _ _
    add                srcq, ss3q
%endif
    movu                 m6, [srcq+ssq*0]       ; 6 = _ _
    HV_H_W8              m4, m1, m2, m3, m7, m8, m9 ; 4 ~ ~ ~
    HV_H_W8              m5, m1, m2, m3, m7, m8, m9 ; 5 ~ ~ ~
    HV_H_W8              m6, m1, m2, m3, m7, m8, m9 ; 6 ~ ~ ~
    mova                 m7, [base+pw_8192]
    pmulhrsw             m1, m4, m7 ; H pw_8192 4 ~
    pmulhrsw             m2, m5, m7 ; H pw_8192 5 ~
    pmulhrsw             m3, m6, m7 ; H pw_8192 6 ~
    punpcklwd            m4, m0, m1  ; 3 4 ~
    punpcklwd            m5, m1, m2  ; 4 5 ~
    punpcklwd            m6, m2, m3  ; 5 6 ~
    SAVELINE_W8           6, m3
    RESTORELINE_W8        1, m1
    RESTORELINE_W8        2, m2
    RESTORELINE_W8        3, m3
.hv_w8_loop:
    ; m8 accu for V a
    ; m9 accu for V b
    SAVELINE_W8           1, m3
    SAVELINE_W8           2, m4
    SAVELINE_W8           3, m5
    SAVELINE_W8           4, m6
%if ARCH_X86_32
    pmaddwd              m0, m1, subpelv0 ; a0
    pmaddwd              m7, m2, subpelv0 ; b0
    pmaddwd              m3, subpelv1     ; a1
    pmaddwd              m4, subpelv1     ; b1
    paddd                m0, m3
    paddd                m7, m4
    pmaddwd              m5, subpelv2     ; a2
    pmaddwd              m6, subpelv2     ; b2
    paddd                m0, m5
    paddd                m7, m6
    mova                 m5, [base+pd_512]
    paddd                m0, m5 ;   pd_512
    paddd                m7, m5 ;   pd_512
    mova             accuv0, m0
    mova             accuv1, m7
%else
    pmaddwd              m8, m1, subpelv0 ; a0
    pmaddwd              m9, m2, subpelv0 ; b0
    pmaddwd              m3, subpelv1     ; a1
    pmaddwd              m4, subpelv1     ; b1
    paddd                m8, m3
    paddd                m9, m4
    pmaddwd              m5, subpelv2     ; a2
    pmaddwd              m6, subpelv2     ; b2
    paddd                m8, m5
    paddd                m9, m6
    mova                 m7, [base+pd_512]
    paddd                m8, m7 ;   pd_512
    paddd                m9, m7 ;   pd_512
    mova                 m7, [base+subpel_h_shufB]
    mova                 m6, [base+subpel_h_shufC]
    mova                 m5, [base+subpel_h_shufA]
%endif
    movu                 m0, [srcq+ssq*1] ; 7
    movu                 m4, [srcq+ssq*2] ; 8
    lea                srcq, [srcq+ssq*2]
    HV_H_W8              m0, m1, m2, m3, m5, m7, m6
    HV_H_W8              m4, m1, m2, m3, m5, m7, m6
    mova                 m5, [base+pw_8192]
    pmulhrsw             m0, m5 ; H pw_8192
    pmulhrsw             m4, m5 ; H pw_8192
    RESTORELINE_W8        6, m6
    punpcklwd            m5, m6, m0  ; 6 7  ~
    punpcklwd            m6, m0, m4  ; 7 8 ~
    pmaddwd              m1, m5, subpelv3 ; a3
    paddd                m2, m1, accuv0
    pmaddwd              m1, m6, subpelv3 ; b3
    paddd                m1, m1, accuv1 ; H + V
    psrad                m2, 10
    psrad                m1, 10
    packssdw             m2, m1  ; d -> w
    packuswb             m2, m1 ; w -> b
    movd       [dstq+dsq*0], m2
    psrlq                m2, 32
%if ARCH_X86_32
    add                dstq, dsm
    movd       [dstq+dsq*0], m2
    add                dstq, dsm
%else
    movd       [dstq+dsq*1], m2
    lea                dstq, [dstq+dsq*2]
%endif
    sub                  hd, 2
    jle .hv_w8_outer
    SAVELINE_W8           6, m4
    RESTORELINE_W8        1, m1
    RESTORELINE_W8        2, m2
    RESTORELINE_W8        3, m3
    RESTORELINE_W8        4, m4
    jmp .hv_w8_loop
.hv_w8_outer:
%if ARCH_X86_32
    mov                dstq, dstm
    add                  r4, 4
    movzx                hd, r6w
    add                dstq, 4
    mov                srcq, r4
    mov                dstm, dstq
%else
    add                  r4, 4
    add                  r7, 4
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
%endif
    sub                 r6d, 1<<16
    jg .hv_w8_loop0
    RET

%macro PSHUFB_SUBPEL_H_4 5 ; dst/src1, src2/mask, tmp1, tmp2, reset_mask
 %if cpuflag(ssse3)
    pshufb               %1, %2
 %else
  %if %5 == 1
    pcmpeqd              %2, %2
    psrlq                %2, 32
  %endif
    psrldq               %3, %1, 1
    pshufd               %3, %3, q2301
    pand                 %1, %2
    pandn                %4, %2, %3
    por                  %1, %4
 %endif
%endmacro

%macro PSHUFB_SUBPEL_H_4a 6 ; dst, src1, src2/mask, tmp1, tmp2, reset_mask
 %ifnidn %1, %2
    mova                 %1, %2
 %endif
    PSHUFB_SUBPEL_H_4    %1, %3, %4, %5, %6
%endmacro

%macro PSHUFB_SUBPEL_H_4b 6 ; dst, src1, src2/mask, tmp1, tmp2, reset_mask
 %if notcpuflag(ssse3)
    psrlq                %1, %2, 16
 %elifnidn %1, %2
    mova                 %1, %2
 %endif
    PSHUFB_SUBPEL_H_4    %1, %3, %4, %5, %6
%endmacro

%macro PALIGNR 4-5 ; dst, src1, src2, shift[, tmp]
 %if cpuflag(ssse3)
    palignr              %1, %2, %3, %4
 %else
  %if %0 == 4
   %assign %%i regnumof%+%1 + 1
   %define %%tmp m %+ %%i
  %else
   %define %%tmp %5
  %endif
    psrldq               %1, %3, %4
    pslldq            %%tmp, %2, 16-%4
    por                  %1, %%tmp
 %endif
%endmacro

%macro PHADDW 4 ; dst, src, pw_1/tmp, load_pw_1
 %if cpuflag(ssse3)
    phaddw               %1, %2
 %elifnidn %1, %2
   %if %4 == 1
    mova                 %3, [base+pw_1]
   %endif
    pmaddwd              %1, %3
    pmaddwd              %2, %3
    packssdw             %1, %2
 %else
   %if %4 == 1
    pmaddwd              %1, [base+pw_1]
   %else
    pmaddwd              %1, %3
   %endif
    packssdw             %1, %1
 %endif
%endmacro

%macro PMULHRSW_POW2 4 ; dst, src1, src2, shift
 %if cpuflag(ssse3)
    pmulhrsw             %1, %2, %3
 %else
    paddw                %1, %2, %3
    psraw                %1, %4
 %endif
%endmacro

%macro PMULHRSW_8192 3 ; dst, src1, src2
    PMULHRSW_POW2        %1, %2, %3, 2
%endmacro

%macro PREP_8TAP_H_LOAD4 5 ; dst, src_memloc, tmp[1-2]
   movd                  %1, [%2+0]
   movd                  %3, [%2+1]
   movd                  %4, [%2+2]
   movd                  %5, [%2+3]
   punpckldq             %1, %3
   punpckldq             %4, %5
   punpcklqdq            %1, %4
%endmacro

%macro PREP_8TAP_H_LOAD 2 ; dst0, src_memloc
 %if cpuflag(ssse3)
    movu                m%1, [%2]
    pshufb               m2, m%1, m11 ; subpel_h_shufB
    pshufb               m3, m%1, m9  ; subpel_h_shufC
    pshufb              m%1, m10      ; subpel_h_shufA
 %else
  %if ARCH_X86_64
    SWAP                m12, m5
    SWAP                m13, m6
    SWAP                m14, m7
   %define %%mx0 m%+%%i
   %define %%mx1 m%+%%j
   %assign %%i 0
   %rep 12
    movd              %%mx0, [%2+%%i]
    %assign %%i %%i+1
   %endrep
   %assign %%i 0
   %rep 6
    %assign %%j %%i+1
    punpckldq         %%mx0, %%mx1
    %assign %%i %%i+2
   %endrep
   %assign %%i 0
   %rep 3
    %assign %%j %%i+2
    punpcklqdq        %%mx0, %%mx1
    %assign %%i %%i+4
   %endrep
    SWAP                m%1, m0
    SWAP                 m2, m4
    SWAP                 m3, m8
    SWAP                 m5, m12
    SWAP                 m6, m13
    SWAP                 m7, m14
  %else
    PREP_8TAP_H_LOAD4    m0, %2+0, m1, m4, m7
    PREP_8TAP_H_LOAD4    m2, %2+4, m1, m4, m7
    PREP_8TAP_H_LOAD4    m3, %2+8, m1, m4, m7
    SWAP                m%1, m0
  %endif
 %endif
%endmacro

%macro PREP_8TAP_H 2 ; dst, src_memloc
    PREP_8TAP_H_LOAD     %1, %2
 %if ARCH_X86_64 && notcpuflag(ssse3)
    SWAP                 m8, m1
    SWAP                 m9, m7
 %endif
 %xdefine mX m%+%1
 %assign %%i regnumof%+mX
 %define mX m%+%%i
    mova                 m4, m2
    PMADDUBSW            m4, m5, m1, m7, 1  ; subpel +0 B0
    PMADDUBSW            m2, m6, m1, m7, 0  ; subpel +4 B4
    PMADDUBSW            m3, m6, m1, m7, 0  ; subpel +4 C4
    PMADDUBSW            mX, m5, m1, m7, 0  ; subpel +0 A0
 %undef mX
 %if ARCH_X86_64 && notcpuflag(ssse3)
    SWAP                 m1, m8
    SWAP                 m7, m9
 %endif
    paddw                m3, m4
    paddw               m%1, m2
    PHADDW              m%1, m3, m15, ARCH_X86_32
 %if ARCH_X86_64 || cpuflag(ssse3)
    PMULHRSW_8192       m%1, m%1, m7
 %else
    PMULHRSW_8192       m%1, m%1, [base+pw_2]
 %endif
%endmacro

%macro PREP_8TAP_HV 4 ; dst, src_memloc, tmp[1-2]
 %if cpuflag(ssse3)
    movu                 %1, [%2]
    pshufb               m2, %1, shufB
    pshufb               m3, %1, shufC
    pshufb               %1, shufA
 %else
    PREP_8TAP_H_LOAD4    %1, %2+0, m1, %3, %4
    PREP_8TAP_H_LOAD4    m2, %2+4, m1, %3, %4
    PREP_8TAP_H_LOAD4    m3, %2+8, m1, %3, %4
 %endif
    mova                 m1, m2
    PMADDUBSW            m1, subpelh0, %3, %4, 1 ; subpel +0 C0
    PMADDUBSW            m3, subpelh1, %3, %4, 0 ; subpel +4 B4
    PMADDUBSW            m2, subpelh1, %3, %4, 0 ; C4
    PMADDUBSW            %1, subpelh0, %3, %4, 0 ; A0
    paddw                m1, m3           ; C0+B4
    paddw                %1, m2           ; A0+C4
    PHADDW               %1, m1, %3, 1
%endmacro

%macro PREP_8TAP 0
%if ARCH_X86_32
 DECLARE_REG_TMP 1, 2
%elif WIN64
 DECLARE_REG_TMP 6, 4
%else
 DECLARE_REG_TMP 6, 7
%endif

FN prep_8tap, sharp,          SHARP,   SHARP
FN prep_8tap, sharp_smooth,   SHARP,   SMOOTH
FN prep_8tap, smooth_sharp,   SMOOTH,  SHARP
FN prep_8tap, smooth,         SMOOTH,  SMOOTH
FN prep_8tap, sharp_regular,  SHARP,   REGULAR
FN prep_8tap, regular_sharp,  REGULAR, SHARP
FN prep_8tap, smooth_regular, SMOOTH,  REGULAR
FN prep_8tap, regular_smooth, REGULAR, SMOOTH
FN prep_8tap, regular,        REGULAR, REGULAR

%if ARCH_X86_32
 %define base_reg r2
 %define base base_reg-prep%+SUFFIX
%else
 %define base_reg r7
 %define base 0
%endif
cglobal prep_8tap_8bpc, 1, 9, 0, tmp, src, stride, w, h, mx, my, stride3
%assign org_stack_offset stack_offset
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 8tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 8tap_v, my, 4tap_v
    mov                  wd, wm
    movifnidn          srcd, srcm
    movifnidn            hd, hm
    test                mxd, 0xf00
    jnz .h
    test                myd, 0xf00
    jnz .v
    LEA            base_reg, prep_ssse3
    tzcnt                wd, wd
    movzx                wd, word [base_reg-prep_ssse3+prep_ssse3_table+wq*2]
    pxor                 m4, m4
    add                  wq, base_reg
    movifnidn       strided, stridem
    lea                  r6, [strideq*3]
    %assign stack_offset org_stack_offset
%if WIN64
    pop                  r8
    pop                  r7
%endif
    jmp                  wq
.h:
    LEA            base_reg, prep%+SUFFIX
    test                myd, 0xf00
    jnz .hv
%if cpuflag(ssse3)
    WIN64_SPILL_XMM      12
%else
    WIN64_SPILL_XMM      16
%endif
%if ARCH_X86_32
 %define strideq r6
    mov             strideq, stridem
%endif
    cmp                  wd, 4
    je .h_w4
    tzcnt                wd, wd
%if cpuflag(ssse3)
 %if ARCH_X86_64
    mova                m10, [base+subpel_h_shufA]
    mova                m11, [base+subpel_h_shufB]
    mova                 m9, [base+subpel_h_shufC]
 %else
  %define m10 [base+subpel_h_shufA]
  %define m11 [base+subpel_h_shufB]
  %define m9  [base+subpel_h_shufC]
 %endif
%endif
    shr                 mxd, 16
    sub                srcq, 3
    movzx                wd, word [base_reg+wq*2+table_offset(prep, _8tap_h)]
    movq                 m6, [base_reg+mxq*8+subpel_filters-prep%+SUFFIX]
%if cpuflag(ssse3)
    mova                 m7, [base+pw_8192]
    pshufd               m5, m6, q0000
    pshufd               m6, m6, q1111
%else
    punpcklbw            m6, m6
    psraw                m6, 8
 %if ARCH_X86_64
    mova                 m7, [pw_2]
    mova                m15, [pw_1]
 %else
  %define m15 m4
 %endif
    pshufd               m5, m6, q1010
    punpckhqdq           m6, m6
%endif
    add                  wq, base_reg
    jmp                  wq
.h_w4:
%if ARCH_X86_32
    and                 mxd, 0x7f
%else
    movzx               mxd, mxb
%endif
    dec                srcq
    movd                 m4, [base_reg+mxq*8+subpel_filters-prep%+SUFFIX+2]
%if cpuflag(ssse3)
    mova                 m6, [base+pw_8192]
    mova                 m5, [base+subpel_h_shufA]
    pshufd               m4, m4, q0000
%else
    mova                 m6, [base+pw_2]
 %if ARCH_X86_64
    mova                m14, [pw_1]
 %else
  %define m14 m7
 %endif
    punpcklbw            m4, m4
    psraw                m4, 8
    punpcklqdq           m4, m4
%endif
%if ARCH_X86_64
    lea            stride3q, [strideq*3]
%endif
.h_w4_loop:
%if cpuflag(ssse3)
    movq                 m0, [srcq+strideq*0] ; 0
    movq                 m1, [srcq+strideq*1] ; 1
 %if ARCH_X86_32
    lea                srcq, [srcq+strideq*2]
    movq                 m2, [srcq+strideq*0] ; 2
    movq                 m3, [srcq+strideq*1] ; 3
    lea                srcq, [srcq+strideq*2]
 %else
    movq                 m2, [srcq+strideq*2] ; 2
    movq                 m3, [srcq+stride3q ] ; 3
    lea                srcq, [srcq+strideq*4]
 %endif
    pshufb               m0, m5
    pshufb               m1, m5
    pshufb               m2, m5
    pshufb               m3, m5
%elif ARCH_X86_64
    movd                 m0, [srcq+strideq*0+0]
    movd                m12, [srcq+strideq*0+1]
    movd                 m1, [srcq+strideq*1+0]
    movd                 m5, [srcq+strideq*1+1]
    movd                 m2, [srcq+strideq*2+0]
    movd                m13, [srcq+strideq*2+1]
    movd                 m3, [srcq+stride3q +0]
    movd                 m7, [srcq+stride3q +1]
    punpckldq            m0, m12
    punpckldq            m1, m5
    punpckldq            m2, m13
    punpckldq            m3, m7
    movd                m12, [srcq+strideq*0+2]
    movd                 m8, [srcq+strideq*0+3]
    movd                 m5, [srcq+strideq*1+2]
    movd                 m9, [srcq+strideq*1+3]
    movd                m13, [srcq+strideq*2+2]
    movd                m10, [srcq+strideq*2+3]
    movd                 m7, [srcq+stride3q +2]
    movd                m11, [srcq+stride3q +3]
    lea                srcq, [srcq+strideq*4]
    punpckldq           m12, m8
    punpckldq            m5, m9
    punpckldq           m13, m10
    punpckldq            m7, m11
    punpcklqdq           m0, m12 ; 0
    punpcklqdq           m1, m5  ; 1
    punpcklqdq           m2, m13 ; 2
    punpcklqdq           m3, m7  ; 3
%else
    movd                 m0, [srcq+strideq*0+0]
    movd                 m1, [srcq+strideq*0+1]
    movd                 m2, [srcq+strideq*0+2]
    movd                 m3, [srcq+strideq*0+3]
    punpckldq            m0, m1
    punpckldq            m2, m3
    punpcklqdq           m0, m2 ; 0
    movd                 m1, [srcq+strideq*1+0]
    movd                 m2, [srcq+strideq*1+1]
    movd                 m3, [srcq+strideq*1+2]
    movd                 m7, [srcq+strideq*1+3]
    lea                srcq, [srcq+strideq*2]
    punpckldq            m1, m2
    punpckldq            m3, m7
    punpcklqdq           m1, m3 ; 1
    movd                 m2, [srcq+strideq*0+0]
    movd                 m3, [srcq+strideq*0+1]
    movd                 m7, [srcq+strideq*0+2]
    movd                 m5, [srcq+strideq*0+3]
    punpckldq            m2, m3
    punpckldq            m7, m5
    punpcklqdq           m2, m7 ; 2
    movd                 m3, [srcq+strideq*1+0]
    movd                 m7, [srcq+strideq*1+1]
    punpckldq            m3, m7
    movd                 m7, [srcq+strideq*1+2]
    movd                 m5, [srcq+strideq*1+3]
    lea                srcq, [srcq+strideq*2]
    punpckldq            m7, m5
    punpcklqdq           m3, m7 ; 3
%endif
    PMADDUBSW            m0, m4, m5, m7, 1 ; subpel_filters + 2
    PMADDUBSW            m1, m4, m5, m7, 0
    PMADDUBSW            m2, m4, m5, m7, 0
    PMADDUBSW            m3, m4, m5, m7, 0
    PHADDW               m0, m1, m14, ARCH_X86_32
    PHADDW               m2, m3, m14, 0
    PMULHRSW_8192        m0, m0, m6
    PMULHRSW_8192        m2, m2, m6
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m2
    add                tmpq, 32
    sub                  hd, 4
    jg .h_w4_loop
    RET
.h_w8:
%if cpuflag(ssse3)
    PREP_8TAP_H           0, srcq+strideq*0
    PREP_8TAP_H           1, srcq+strideq*1
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    lea                srcq, [srcq+strideq*2]
    add                tmpq, 32
    sub                  hd, 2
%else
    PREP_8TAP_H           0, srcq
    mova             [tmpq], m0
    add                srcq, strideq
    add                tmpq, 16
    dec                  hd
%endif
    jg .h_w8
    RET
.h_w16:
    mov                  r3, -16*1
    jmp .h_start
.h_w32:
    mov                  r3, -16*2
    jmp .h_start
.h_w64:
    mov                  r3, -16*4
    jmp .h_start
.h_w128:
    mov                  r3, -16*8
.h_start:
    sub                srcq, r3
    mov                  r5, r3
.h_loop:
%if cpuflag(ssse3)
    PREP_8TAP_H           0, srcq+r3+8*0
    PREP_8TAP_H           1, srcq+r3+8*1
    mova        [tmpq+16*0], m0
    mova        [tmpq+16*1], m1
    add                tmpq, 32
    add                  r3, 16
%else
    PREP_8TAP_H           0, srcq+r3
    mova             [tmpq], m0
    add                tmpq, 16
    add                  r3, 8
%endif
    jl .h_loop
    add                srcq, strideq
    mov                  r3, r5
    dec                  hd
    jg .h_loop
    RET
.v:
    LEA            base_reg, prep%+SUFFIX
%if ARCH_X86_32
    mov                 mxd, myd
    and                 mxd, 0x7f
%else
 %assign stack_offset org_stack_offset
    WIN64_SPILL_XMM      16
    movzx               mxd, myb
%endif
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    movq                 m0, [base_reg+myq*8+subpel_filters-prep%+SUFFIX]
%if cpuflag(ssse3)
    mova                 m2, [base+pw_512]
    mova                 m7, [base+pw_8192]
    punpcklwd            m0, m0
%else
    punpcklbw            m0, m0
    psraw                m0, 8
%endif
%if ARCH_X86_32
 %define            subpel0  [rsp+mmsize*0]
 %define            subpel1  [rsp+mmsize*1]
 %define            subpel2  [rsp+mmsize*2]
 %define            subpel3  [rsp+mmsize*3]
%assign regs_used 6 ; use r5 (mx) as tmp for stack alignment if needed
 %if cpuflag(ssse3)
    ALLOC_STACK   -mmsize*4
 %else
    ALLOC_STACK   -mmsize*5
 %endif
%assign regs_used 7
    mov             strideq, [rstk+stack_offset+gprsize*3]
    pshufd               m1, m0, q0000
    mova            subpel0, m1
    pshufd               m1, m0, q1111
    mova            subpel1, m1
    lea                  r5, [strideq*3]
    pshufd               m1, m0, q2222
    mova            subpel2, m1
    pshufd               m1, m0, q3333
    mova            subpel3, m1
    sub                srcq, r5
%else
 %define            subpel0  m8
 %define            subpel1  m9
 %define            subpel2  m10
 %define            subpel3  m11
    pshufd               m8, m0, q0000
    pshufd               m9, m0, q1111
    lea            stride3q, [strideq*3]
    pshufd              m10, m0, q2222
    pshufd              m11, m0, q3333
    sub                srcq, stride3q
    cmp                  wd, 8
    jns .v_w8
%endif
.v_w4:
%if notcpuflag(ssse3)
    pxor                 m6, m6
 %if ARCH_X86_64
    mova                 m7, [base+pw_2]
 %endif
%endif
%if ARCH_X86_32
 %if STACK_ALIGNMENT < mmsize
  %define srcm [esp+stack_size+gprsize*1]
  %define tmpm [esp+stack_size+gprsize*2]
 %endif
    mov                tmpm, tmpq
    mov                srcm, srcq
    lea                 r5d, [wq - 4] ; horizontal loop
    shl                 r5d, (16 - 2)  ; (wq / 4) << 16
    mov                 r5w, hw
.v_w4_loop0:
%endif
    movd                 m1, [srcq+strideq*0]
    movd                 m0, [srcq+strideq*1]
%if ARCH_X86_32
    lea                srcq, [srcq+strideq*2]
    movd                 m2, [srcq+strideq*0]
    movd                 m4, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    movd                 m3, [srcq+strideq*0]
    movd                 m5, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
%else
    movd                 m2, [srcq+strideq*2]
    add                srcq, stride3q
    movd                 m4, [srcq+strideq*0]
    movd                 m3, [srcq+strideq*1]
    movd                 m5, [srcq+strideq*2]
    add                srcq, stride3q
%endif
    punpckldq            m1, m0 ; 0 1
    punpckldq            m0, m2 ; 1 2
    punpcklbw            m1, m0 ; 01 12
    movd                 m0, [srcq+strideq*0]
    punpckldq            m2, m4 ; 2 3
    punpckldq            m4, m3 ; 3 4
    punpckldq            m3, m5 ; 4 5
    punpckldq            m5, m0 ; 5 6
    punpcklbw            m2, m4 ; 23 34
    punpcklbw            m3, m5 ; 45 56
.v_w4_loop:
%if ARCH_X86_32 && notcpuflag(ssse3)
    mova                 m7, subpel0
 %define subpel0 m7
%endif
    mova                 m5, m1
    PMADDUBSW            m5, subpel0, m6, m4, 0  ; a0 b0
%if ARCH_X86_32 && notcpuflag(ssse3)
    mova                 m7, subpel1
 %define subpel1 m7
%endif
    mova                 m1, m2
    PMADDUBSW            m2, subpel1, m6, m4, 0  ; a1 b1
    paddw                m5, m2
%if ARCH_X86_32 && notcpuflag(ssse3)
    mova                 m7, subpel2
 %define subpel2 m7
%endif
    mova                 m2, m3
    PMADDUBSW            m3, subpel2, m6, m4, 0  ; a2 b2
    movd                 m4, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    paddw                m5, m3
    punpckldq            m3, m0, m4       ; 6 7 _ _
    movd                 m0, [srcq+strideq*0]
    punpckldq            m4, m0           ; 7 8 _ _
    punpcklbw            m3, m4           ; 67 78
%if notcpuflag(ssse3)
 %if ARCH_X86_64
    SWAP                m12, m0
 %else
    mova     [esp+mmsize*4], m0
    mova                 m7, subpel3
  %define subpel3 m7
 %endif
%endif
    mova                 m4, m3
    PMADDUBSW            m4, subpel3, m6, m0, 0  ; a3 b3
    paddw                m5, m4
%if ARCH_X86_64 || cpuflag(ssse3)
 %if notcpuflag(ssse3)
    SWAP                 m0, m12
 %endif
    PMULHRSW_8192        m5, m5, m7
%else
    mova                 m0, [esp+mmsize*4]
    PMULHRSW_8192        m5, m5, [base+pw_2]
%endif
    movq        [tmpq+wq*0], m5
    movhps      [tmpq+wq*2], m5
    lea                tmpq, [tmpq+wq*4]
    sub                  hd, 2
    jg .v_w4_loop
%if ARCH_X86_32
    mov                srcq, srcm
    mov                tmpq, tmpm
    movzx                hd, r5w
    add                srcq, 4
    add                tmpq, 8
    mov                srcm, srcq
    mov                tmpm, tmpq
    sub                 r5d, 1<<16 ; horizontal--
    jg .v_w4_loop0
%endif
    RET
%if ARCH_X86_64
.v_w8:
    lea                 r6d, [wq*8-64]
    mov                  r5, srcq
    mov                  r8, tmpq
    lea                 r6d, [hq+r6*4]
.v_w8_loop0:
    movq                 m1, [srcq+strideq*0]
    movq                 m2, [srcq+strideq*1]
    movq                 m3, [srcq+strideq*2]
    add                srcq, stride3q
    movq                 m4, [srcq+strideq*0]
    movq                 m5, [srcq+strideq*1]
    movq                 m6, [srcq+strideq*2]
    add                srcq, stride3q
    movq                 m0, [srcq+strideq*0]
    punpcklbw            m1, m2 ; 01
    punpcklbw            m2, m3 ; 12
    punpcklbw            m3, m4 ; 23
    punpcklbw            m4, m5 ; 34
    punpcklbw            m5, m6 ; 45
    punpcklbw            m6, m0 ; 56
.v_w8_loop:
    movq                m13, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
%if cpuflag(ssse3)
    pmaddubsw           m14, m1, subpel0 ; a0
    pmaddubsw           m15, m2, subpel0 ; b0
    mova                 m1, m3
    mova                 m2, m4
    pmaddubsw            m3, subpel1 ; a1
    pmaddubsw            m4, subpel1 ; b1
    paddw               m14, m3
    paddw               m15, m4
    mova                 m3, m5
    mova                 m4, m6
    pmaddubsw            m5, subpel2 ; a2
    pmaddubsw            m6, subpel2 ; b2
    punpcklbw           m12, m0, m13 ; 67
    movq                 m0, [srcq+strideq*0]
    punpcklbw           m13, m0      ; 78
    paddw               m14, m5
    mova                 m5, m12
    pmaddubsw           m12, subpel3 ; a3
    paddw               m15, m6
    mova                 m6, m13
    pmaddubsw           m13, subpel3 ; b3
    paddw               m14, m12
    paddw               m15, m13
    pmulhrsw            m14, m7
    pmulhrsw            m15, m7
%else
    mova                m14, m1
    PMADDUBSW           m14, subpel0, m7, m12, 1 ; a0
    mova                m15, m2
    PMADDUBSW           m15, subpel0, m7, m12, 0 ; b0
    mova                 m1, m3
    PMADDUBSW            m3, subpel1, m7, m12, 0 ; a1
    mova                 m2, m4
    PMADDUBSW            m4, subpel1, m7, m12, 0 ; b1
    paddw               m14, m3
    mova                 m3, m5
    PMADDUBSW            m5, subpel2, m7, m12, 0 ; a2
    paddw               m15, m4
    mova                 m4, m6
    PMADDUBSW            m6, subpel2, m7, m12, 0 ; b2
    paddw               m15, m6
    punpcklbw           m12, m0, m13 ; 67
    movq                 m0, [srcq+strideq*0]
    punpcklbw           m13, m0      ; 78
    paddw               m14, m5
    mova                 m5, m12
    PMADDUBSW           m12, subpel3, m7, m6, 0  ; a3
    paddw               m14, m12
    mova                 m6, m13
    PMADDUBSW           m13, subpel3, m7, m12, 0 ; b3
    paddw               m15, m13
    PMULHRSW_8192       m14, m14, [base+pw_2]
    PMULHRSW_8192       m15, m15, [base+pw_2]
%endif
    movu        [tmpq+wq*0], m14
    movu        [tmpq+wq*2], m15
    lea                tmpq, [tmpq+wq*4]
    sub                  hd, 2
    jg .v_w8_loop
    add                  r5, 8
    add                  r8, 16
    movzx                hd, r6b
    mov                srcq, r5
    mov                tmpq, r8
    sub                 r6d, 1<<8
    jg .v_w8_loop0
    RET
%endif ;ARCH_X86_64
%undef subpel0
%undef subpel1
%undef subpel2
%undef subpel3
.hv:
    %assign stack_offset org_stack_offset
    cmp                  wd, 4
    jg .hv_w8
    and                 mxd, 0x7f
    movd                 m1, [base_reg+mxq*8+subpel_filters-prep%+SUFFIX+2]
%if ARCH_X86_32
    mov                 mxd, myd
    shr                 myd, 16
    and                 mxd, 0x7f
    cmp                  hd, 6
    cmovs               myd, mxd
    movq                 m0, [base_reg+myq*8+subpel_filters-prep%+SUFFIX]
    mov             strideq, stridem
 %assign regs_used 6
    ALLOC_STACK  -mmsize*14
 %assign regs_used 7
    lea                  r5, [strideq*3+1]
    sub                srcq, r5
 %define           subpelv0  [rsp+mmsize*0]
 %define           subpelv1  [rsp+mmsize*1]
 %define           subpelv2  [rsp+mmsize*2]
 %define           subpelv3  [rsp+mmsize*3]
    punpcklbw            m0, m0
    psraw                m0, 8
    pshufd               m6, m0, q0000
    mova           subpelv0, m6
    pshufd               m6, m0, q1111
    mova           subpelv1, m6
    pshufd               m6, m0, q2222
    mova           subpelv2, m6
    pshufd               m6, m0, q3333
    mova           subpelv3, m6
%else
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    movq                 m0, [base_reg+myq*8+subpel_filters-prep%+SUFFIX]
 %if cpuflag(ssse3)
    ALLOC_STACK   mmsize*14, 14
 %else
    ALLOC_STACK   mmsize*14, 16
 %endif
    lea            stride3q, [strideq*3]
    sub                srcq, stride3q
    dec                srcq
 %define           subpelv0  m10
 %define           subpelv1  m11
 %define           subpelv2  m12
 %define           subpelv3  m13
    punpcklbw            m0, m0
    psraw                m0, 8
 %if cpuflag(ssse3)
    mova                 m8, [base+pw_8192]
 %else
    mova                 m8, [base+pw_2]
 %endif
    mova                 m9, [base+pd_32]
    pshufd              m10, m0, q0000
    pshufd              m11, m0, q1111
    pshufd              m12, m0, q2222
    pshufd              m13, m0, q3333
%endif
    pshufd               m7, m1, q0000
%if notcpuflag(ssse3)
    punpcklbw            m7, m7
    psraw                m7, 8
%endif
%define hv4_line_0_0 4
%define hv4_line_0_1 5
%define hv4_line_0_2 6
%define hv4_line_0_3 7
%define hv4_line_0_4 8
%define hv4_line_0_5 9
%define hv4_line_1_0 10
%define hv4_line_1_1 11
%define hv4_line_1_2 12
%define hv4_line_1_3 13
%if ARCH_X86_32
 %if cpuflag(ssse3)
  %define          w8192reg  [base+pw_8192]
 %else
  %define          w8192reg  [base+pw_2]
 %endif
 %define             d32reg  [base+pd_32]
%else
 %define           w8192reg  m8
 %define             d32reg  m9
%endif
    ; lower shuffle 0 1 2 3 4
%if cpuflag(ssse3)
    mova                 m6, [base+subpel_h_shuf4]
%else
 %if ARCH_X86_64
    mova                m15, [pw_1]
 %else
  %define               m15 m1
 %endif
%endif
    movq                 m5, [srcq+strideq*0]   ; 0 _ _ _
    movhps               m5, [srcq+strideq*1]   ; 0 _ 1 _
%if ARCH_X86_32
    lea                srcq, [srcq+strideq*2]
    movq                 m4, [srcq+strideq*0]   ; 2 _ _ _
    movhps               m4, [srcq+strideq*1]   ; 2 _ 3 _
    lea                srcq, [srcq+strideq*2]
%else
    movq                 m4, [srcq+strideq*2]   ; 2 _ _ _
    movhps               m4, [srcq+stride3q ]   ; 2 _ 3 _
    lea                srcq, [srcq+strideq*4]
%endif
    PSHUFB_SUBPEL_H_4a   m2, m5, m6, m1, m3, 1    ;H subpel_h_shuf4 0~1~
    PSHUFB_SUBPEL_H_4a   m0, m4, m6, m1, m3, 0    ;H subpel_h_shuf4 2~3~
    PMADDUBSW            m2, m7, m1, m3, 1        ;H subpel_filters
    PMADDUBSW            m0, m7, m1, m3, 0        ;H subpel_filters
    PHADDW               m2, m0, m15, ARCH_X86_32 ;H 0 1 2 3
    PMULHRSW_8192        m2, m2, w8192reg
    SAVELINE_W4          m2, 2, 0
    ; upper shuffle 2 3 4 5 6
%if cpuflag(ssse3)
    mova                 m6, [base+subpel_h_shuf4+16]
%endif
    PSHUFB_SUBPEL_H_4b   m2, m5, m6, m1, m3, 0    ;H subpel_h_shuf4 0~1~
    PSHUFB_SUBPEL_H_4b   m0, m4, m6, m1, m3, 0    ;H subpel_h_shuf4 2~3~
    PMADDUBSW            m2, m7, m1, m3, 1        ;H subpel_filters
    PMADDUBSW            m0, m7, m1, m3, 0        ;H subpel_filters
    PHADDW               m2, m0, m15, ARCH_X86_32 ;H 0 1 2 3
    PMULHRSW_8192        m2, m2, w8192reg
%if notcpuflag(ssse3)
 %if ARCH_X86_64
    SWAP                m14, m2
 %else
    mova     [esp+mmsize*4], m2
 %endif
%endif
    ; lower shuffle
%if cpuflag(ssse3)
    mova                 m6, [base+subpel_h_shuf4]
%endif
    movq                 m5, [srcq+strideq*0]   ; 4 _ _ _
    movhps               m5, [srcq+strideq*1]   ; 4 _ 5 _
%if ARCH_X86_32
    lea                srcq, [srcq+strideq*2]
    movq                 m4, [srcq+strideq*0]   ; 6 _ _ _
    add                srcq, strideq
%else
    movq                 m4, [srcq+strideq*2]   ; 6 _ _ _
    add                srcq, stride3q
%endif
    PSHUFB_SUBPEL_H_4a   m3, m5, m6, m1, m2, 0    ;H subpel_h_shuf4 4~5~
    PSHUFB_SUBPEL_H_4a   m0, m4, m6, m1, m2, 0    ;H subpel_h_shuf4 6~6~
    PMADDUBSW            m3, m7, m1, m2, 1        ;H subpel_filters
    PMADDUBSW            m0, m7, m1, m2, 0        ;H subpel_filters
    PHADDW               m3, m0, m15, ARCH_X86_32 ;H 4 5 6 7
    PMULHRSW_8192        m3, m3, w8192reg
    SAVELINE_W4          m3, 3, 0
    ; upper shuffle
%if cpuflag(ssse3)
    mova                 m6, [base+subpel_h_shuf4+16]
%endif
    PSHUFB_SUBPEL_H_4b   m3, m5, m6, m1, m2, 0    ;H subpel_h_shuf4 4~5~
    PSHUFB_SUBPEL_H_4b   m0, m4, m6, m1, m2, 0    ;H subpel_h_shuf4 6~6~
    PMADDUBSW            m3, m7, m1, m2, 1        ;H subpel_filters
    PMADDUBSW            m0, m7, m1, m2, 0        ;H subpel_filters
    PHADDW               m3, m0, m15, ARCH_X86_32 ;H 4 5 6 7
    PMULHRSW_8192        m3, m3, w8192reg
%if notcpuflag(ssse3)
 %if ARCH_X86_64
    SWAP                 m2, m14
 %else
    mova                 m2, [esp+mmsize*4]
 %endif
%endif
    ;process high
    PALIGNR              m4, m3, m2, 4;V 1 2 3 4
    punpcklwd            m1, m2, m4  ; V 01 12
    punpckhwd            m2, m4      ; V 23 34
    pshufd               m0, m3, q2121;V 5 6 5 6
    punpcklwd            m3, m0      ; V 45 56
    SAVELINE_W4          m0, 0, 1
    SAVELINE_W4          m1, 1, 1
    SAVELINE_W4          m2, 2, 1
    SAVELINE_W4          m3, 3, 1
    ;process low
    RESTORELINE_W4       m2, 2, 0
    RESTORELINE_W4       m3, 3, 0
    PALIGNR              m4, m3, m2, 4;V 1 2 3 4
    punpcklwd            m1, m2, m4  ; V 01 12
    punpckhwd            m2, m4      ; V 23 34
    pshufd               m0, m3, q2121;V 5 6 5 6
    punpcklwd            m3, m0      ; V 45 56
.hv_w4_loop:
    ;process low
    pmaddwd              m5, m1, subpelv0 ; V a0 b0
    mova                 m1, m2
    pmaddwd              m2, subpelv1; V a1 b1
    paddd                m5, m2
    mova                 m2, m3
    pmaddwd              m3, subpelv2; V a2 b2
    paddd                m5, m3
%if notcpuflag(ssse3)
 %if ARCH_X86_64
    SWAP                m14, m5
 %else
    mova     [esp+mmsize*4], m5
  %define m15 m3
 %endif
%endif
%if cpuflag(ssse3)
    mova                 m6, [base+subpel_h_shuf4]
%endif
    movq                 m4, [srcq+strideq*0] ; 7
    movhps               m4, [srcq+strideq*1] ; 7 _ 8 _
    PSHUFB_SUBPEL_H_4a   m4, m4, m6, m3, m5, 0    ; H subpel_h_shuf4 7~8~
    PMADDUBSW            m4, m7, m3, m5, 1        ; H subpel_filters
    PHADDW               m4, m4, m15, ARCH_X86_32 ; H                7878
    PMULHRSW_8192        m4, m4, w8192reg
    PALIGNR              m3, m4, m0, 12, m5       ;                  6787
    mova                 m0, m4
    punpcklwd            m3, m4      ; 67 78
    pmaddwd              m4, m3, subpelv3; a3 b3
%if notcpuflag(ssse3)
 %if ARCH_X86_64
    SWAP                 m5, m14
 %else
    mova                 m5, [esp+mmsize*4]
 %endif
%endif
    paddd                m5, d32reg ; pd_32
    paddd                m5, m4
    psrad                m5, 6
    SAVELINE_W4          m0, 0, 0
    SAVELINE_W4          m1, 1, 0
    SAVELINE_W4          m2, 2, 0
    SAVELINE_W4          m3, 3, 0
    SAVELINE_W4          m5, 5, 0
    ;process high
    RESTORELINE_W4       m0, 0, 1
    RESTORELINE_W4       m1, 1, 1
    RESTORELINE_W4       m2, 2, 1
    RESTORELINE_W4       m3, 3, 1
    pmaddwd              m5, m1, subpelv0; V a0 b0
    mova                 m1, m2
    pmaddwd              m2, subpelv1; V a1 b1
    paddd                m5, m2
    mova                 m2, m3
    pmaddwd              m3, subpelv2; V a2 b2
    paddd                m5, m3
%if notcpuflag(ssse3)
 %if ARCH_X86_64
    SWAP                m14, m5
 %else
    mova         [esp+0xA0], m5
 %endif
%endif
%if cpuflag(ssse3)
    mova                 m6, [base+subpel_h_shuf4+16]
%endif
    movq                 m4, [srcq+strideq*0] ; 7
    movhps               m4, [srcq+strideq*1] ; 7 _ 8 _
    PSHUFB_SUBPEL_H_4b   m4, m4, m6, m3, m5, 0    ; H subpel_h_shuf4 7~8~
    PMADDUBSW            m4, m7, m3, m5, 1        ; H subpel_filters
    PHADDW               m4, m4, m15, ARCH_X86_32 ; H                7878
    PMULHRSW_8192        m4, m4, w8192reg
    PALIGNR              m3, m4, m0, 12, m5       ;                  6787
    mova                 m0, m4
    punpcklwd            m3, m4      ; 67 78
    pmaddwd              m4, m3, subpelv3; a3 b3
%if notcpuflag(ssse3)
 %if ARCH_X86_64
    SWAP                 m5, m14
 %else
    mova                 m5, [esp+0xA0]
 %endif
%endif
    paddd                m5, d32reg ; pd_32
    paddd                m5, m4
    psrad                m4, m5, 6
    RESTORELINE_W4       m5, 5, 0
    packssdw             m5, m4
    pshufd               m5, m5, q3120
    movu             [tmpq], m5
    lea                srcq, [srcq+strideq*2]
    add                tmpq, 16
    sub                  hd, 2
    SAVELINE_W4          m0, 0, 1
    SAVELINE_W4          m1, 1, 1
    SAVELINE_W4          m2, 2, 1
    SAVELINE_W4          m3, 3, 1
    RESTORELINE_W4       m0, 0, 0
    RESTORELINE_W4       m1, 1, 0
    RESTORELINE_W4       m2, 2, 0
    RESTORELINE_W4       m3, 3, 0
    jg .hv_w4_loop
    RET
%undef subpelv0
%undef subpelv1
%undef subpelv2
%undef subpelv3
.hv_w8:
    %assign stack_offset org_stack_offset
%define hv8_line_1 0
%define hv8_line_2 1
%define hv8_line_3 2
%define hv8_line_4 3
%define hv8_line_6 4
    shr                 mxd, 16
%if ARCH_X86_32
 %define           subpelh0  [rsp+mmsize*5]
 %define           subpelh1  [rsp+mmsize*6]
 %define           subpelv0  [rsp+mmsize*7]
 %define           subpelv1  [rsp+mmsize*8]
 %define           subpelv2  [rsp+mmsize*9]
 %define           subpelv3  [rsp+mmsize*10]
 %define             accuv0  [rsp+mmsize*11]
 %define             accuv1  [rsp+mmsize*12]
    movq                 m1, [base_reg+mxq*8+subpel_filters-prep%+SUFFIX]
    mov                 mxd, myd
    shr                 myd, 16
    and                 mxd, 0x7f
    cmp                  hd, 6
    cmovs               myd, mxd
    movq                 m5, [base_reg+myq*8+subpel_filters-prep%+SUFFIX]
    mov             strideq, stridem
 %assign regs_used 6
    ALLOC_STACK  -mmsize*14
 %assign regs_used 7
 %if STACK_ALIGNMENT < mmsize
  %define              tmpm  [rsp+mmsize*13+gprsize*1]
  %define              srcm  [rsp+mmsize*13+gprsize*2]
  %define           stridem  [rsp+mmsize*13+gprsize*3]
    mov                tmpm, tmpq
    mov             stridem, strideq
 %endif
 %if cpuflag(ssse3)
    pshufd               m0, m1, q0000
    pshufd               m1, m1, q1111
 %else
    punpcklbw            m1, m1
    psraw                m1, 8
    pshufd               m0, m1, q1010
    punpckhqdq           m1, m1
 %endif
    punpcklbw            m5, m5
    psraw                m5, 8
    pshufd               m2, m5, q0000
    pshufd               m3, m5, q1111
    pshufd               m4, m5, q2222
    pshufd               m5, m5, q3333
    mova           subpelh0, m0
    mova           subpelh1, m1
    mova           subpelv0, m2
    mova           subpelv1, m3
    mova           subpelv2, m4
    mova           subpelv3, m5
    lea                  r5, [strideq*3+3]
    sub                srcq, r5
    mov                srcm, srcq
%else
    ALLOC_STACK    mmsize*5, 16
 %define           subpelh0  m10
 %define           subpelh1  m11
 %define           subpelv0  m12
 %define           subpelv1  m13
 %define           subpelv2  m14
 %define           subpelv3  m15
 %define             accuv0  m8
 %define             accuv1  m9
    movq                 m0, [base_reg+mxq*8+subpel_filters-prep%+SUFFIX]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    movq                 m1, [base_reg+myq*8+subpel_filters-prep%+SUFFIX]
 %if cpuflag(ssse3)
    pshufd         subpelh0, m0, q0000
    pshufd         subpelh1, m0, q1111
 %else
    punpcklbw            m0, m0
    psraw                m0, 8
    pshufd         subpelh0, m0, q1010
    pshufd         subpelh1, m0, q3232
    mova                 m7, [base+pw_2]
 %endif
    punpcklbw            m1, m1
    psraw                m1, 8
    pshufd         subpelv0, m1, q0000
    pshufd         subpelv1, m1, q1111
    pshufd         subpelv2, m1, q2222
    pshufd         subpelv3, m1, q3333
    lea            stride3q, [strideq*3]
    sub                srcq, 3
    sub                srcq, stride3q
    mov                  r6, srcq
    mov                  r8, tmpq
%endif
    lea                 r5d, [wq-4]
    shl                 r5d, 14
    add                 r5d, hd
.hv_w8_loop0:
%if cpuflag(ssse3)
 %if ARCH_X86_64
    mova                 m7, [base+subpel_h_shufA]
    mova                 m8, [base+subpel_h_shufB]
    mova                 m9, [base+subpel_h_shufC]
  %define shufA m7
  %define shufB m8
  %define shufC m9
 %else
  %define shufA [base+subpel_h_shufA]
  %define shufB [base+subpel_h_shufB]
  %define shufC [base+subpel_h_shufC]
 %endif
%endif
    PREP_8TAP_HV         m4, srcq+strideq*0, m7, m0
    PREP_8TAP_HV         m5, srcq+strideq*1, m7, m0
%if ARCH_X86_64
    PREP_8TAP_HV         m6, srcq+strideq*2, m7, m0
    add                srcq, stride3q
    PREP_8TAP_HV         m0, srcq+strideq*0, m7, m9
%else
    lea                srcq, [srcq+strideq*2]
 %if notcpuflag(ssse3)
    mova              [esp], m4
 %endif
    PREP_8TAP_HV         m6, srcq+strideq*0, m7, m4
    PREP_8TAP_HV         m0, srcq+strideq*1, m7, m4
    lea                srcq, [srcq+strideq*2]
%endif
%if cpuflag(ssse3)
    mova                 m7, [base+pw_8192]
%else
    mova                 m7, [base+pw_2]
 %if ARCH_X86_32
    mova                 m4, [esp]
 %endif
%endif
    PMULHRSW_8192        m4, m4, m7
    PMULHRSW_8192        m5, m5, m7
    PMULHRSW_8192        m6, m6, m7
    PMULHRSW_8192        m0, m0, m7
    punpcklwd            m1, m4, m5 ; 01
    punpcklwd            m2, m5, m6 ; 12
    punpcklwd            m3, m6, m0 ; 23
    SAVELINE_W8           1, m1
    SAVELINE_W8           2, m2
    SAVELINE_W8           3, m3
%if cpuflag(ssse3)
    mova                 m7, [base+subpel_h_shufA]
%endif
%if ARCH_X86_64
    PREP_8TAP_HV         m4, srcq+strideq*1, m8, m9
    PREP_8TAP_HV         m5, srcq+strideq*2, m8, m9
    add                srcq, stride3q
    PREP_8TAP_HV         m6, srcq+strideq*0, m8, m9
%else
 %if notcpuflag(ssse3)
    mova         [esp+0x30], m0
 %endif
    PREP_8TAP_HV         m4, srcq+strideq*0, m7, m0
    PREP_8TAP_HV         m5, srcq+strideq*1, m7, m0
    lea                srcq, [srcq+strideq*2]
    PREP_8TAP_HV         m6, srcq+strideq*0, m7, m0
%endif
%if cpuflag(ssse3)
    mova                 m7, [base+pw_8192]
%elif ARCH_X86_32
    mova                 m0, [esp+0x30]
    mova                 m7, [base+pw_2]
%endif
    PMULHRSW_8192        m1, m4, m7
    PMULHRSW_8192        m2, m5, m7
    PMULHRSW_8192        m3, m6, m7
    punpcklwd            m4, m0, m1 ; 34
    punpcklwd            m5, m1, m2 ; 45
    punpcklwd            m6, m2, m3 ; 56
    SAVELINE_W8           6, m3
    RESTORELINE_W8        1, m1
    RESTORELINE_W8        2, m2
    RESTORELINE_W8        3, m3
.hv_w8_loop:
    SAVELINE_W8           1, m3
    SAVELINE_W8           2, m4
    SAVELINE_W8           3, m5
    SAVELINE_W8           4, m6
%if ARCH_X86_32
    pmaddwd              m0, m1, subpelv0 ; a0
    pmaddwd              m7, m2, subpelv0 ; b0
    pmaddwd              m3, subpelv1     ; a1
    pmaddwd              m4, subpelv1     ; b1
    paddd                m0, m3
    paddd                m7, m4
    pmaddwd              m5, subpelv2     ; a2
    pmaddwd              m6, subpelv2     ; b2
    paddd                m0, m5
    paddd                m7, m6
    mova                 m5, [base+pd_32]
    paddd                m0, m5
    paddd                m7, m5
    mova             accuv0, m0
    mova             accuv1, m7
%else
    pmaddwd          accuv0, m1, subpelv0 ; a0
    pmaddwd          accuv1, m2, subpelv0 ; b0
    pmaddwd              m3, subpelv1     ; a1
    pmaddwd              m4, subpelv1     ; b1
    paddd            accuv0, m3
    paddd            accuv1, m4
    pmaddwd              m5, subpelv2     ; a2
    pmaddwd              m6, subpelv2     ; b2
    paddd            accuv0, m5
    paddd            accuv1, m6
    mova                 m7, [base+pd_32]
    paddd            accuv0, m7
    paddd            accuv1, m7
 %if cpuflag(ssse3)
    mova                 m7, [base+subpel_h_shufB]
    mova                 m6, [base+subpel_h_shufC]
    mova                 m5, [base+subpel_h_shufA]
  %define shufA m5
  %define shufB m7
  %define shufC m6
 %endif
%endif
    PREP_8TAP_HV         m0, srcq+strideq*1, m5, m6
    lea                srcq, [srcq+strideq*2]
    PREP_8TAP_HV         m4, srcq+strideq*0, m5, m6
%if cpuflag(ssse3)
    mova                 m5, [base+pw_8192]
%else
    mova                 m5, [base+pw_2]
%endif
    PMULHRSW_8192        m0, m0, m5
    PMULHRSW_8192        m4, m4, m5
    RESTORELINE_W8        6, m6
    punpcklwd            m5, m6, m0 ; 67
    punpcklwd            m6, m0, m4 ; 78
    pmaddwd              m1, m5, subpelv3 ; a3
    paddd                m2, m1, accuv0
    pmaddwd              m1, m6, subpelv3 ; b3
    paddd                m1, m1, accuv1
    psrad                m2, 6
    psrad                m1, 6
    packssdw             m2, m1
    movq        [tmpq+wq*0], m2
    movhps      [tmpq+wq*2], m2
    lea                tmpq, [tmpq+wq*4]
    sub                  hd, 2
    jle .hv_w8_outer
    SAVELINE_W8           6, m4
    RESTORELINE_W8        1, m1
    RESTORELINE_W8        2, m2
    RESTORELINE_W8        3, m3
    RESTORELINE_W8        4, m4
    jmp .hv_w8_loop
.hv_w8_outer:
%if ARCH_X86_32
    mov                srcq, srcm
    mov                tmpq, tmpm
    movzx                hd, r5w
    add                srcq, 4
    add                tmpq, 8
    mov                srcm, srcq
    mov                tmpm, tmpq
%else
    add                  r6, 4
    add                  r8, 8
    movzx                hd, r5b
    mov                srcq, r6
    mov                tmpq, r8
%endif
    sub                 r5d, 1<<16
    jg .hv_w8_loop0
    RET
%endmacro

%macro movifprep 2
 %if isprep
    mov %1, %2
 %endif
%endmacro

%macro SAVE_REG 1
 %xdefine r%1_save  r%1
 %xdefine r%1q_save r%1q
 %xdefine r%1d_save r%1d
 %if ARCH_X86_32
  %define r%1m_save [rstk+stack_offset+(%1+1)*4]
 %endif
%endmacro

%macro LOAD_REG 1
 %xdefine r%1  r%1_save
 %xdefine r%1q r%1q_save
 %xdefine r%1d r%1d_save
 %if ARCH_X86_32
  %define r%1m r%1m_save
 %endif
 %undef r%1d_save
 %undef r%1q_save
 %undef r%1_save
%endmacro

%macro REMAP_REG 2-3
 %xdefine r%1  r%2
 %xdefine r%1q r%2q
 %xdefine r%1d r%2d
 %if ARCH_X86_32
  %if %3 == 0
   %xdefine r%1m r%2m
  %else
   %define r%1m [rstk+stack_offset+(%1+1)*4]
  %endif
 %endif
%endmacro

%macro MCT_8TAP_SCALED_REMAP_REGS_TO_PREV 0
 %if isprep
  %if ARCH_X86_64
   SAVE_REG 14
   %assign %%i 14
   %rep 14
    %assign %%j %%i-1
    REMAP_REG %%i, %%j
    %assign %%i %%i-1
   %endrep
  %else
   SAVE_REG 5
   %assign %%i 5
   %rep 5
    %assign %%j %%i-1
    REMAP_REG %%i, %%j, 0
    %assign %%i %%i-1
   %endrep
  %endif
 %endif
%endmacro

%macro MCT_8TAP_SCALED_REMAP_REGS_TO_DEFAULT 0
 %if isprep
  %assign %%i 1
  %if ARCH_X86_64
   %rep 13
    %assign %%j %%i+1
    REMAP_REG %%i, %%j
    %assign %%i %%i+1
   %endrep
   LOAD_REG 14
  %else
   %rep 4
    %assign %%j %%i+1
    REMAP_REG %%i, %%j, 1
    %assign %%i %%i+1
   %endrep
   LOAD_REG 5
  %endif
 %endif
%endmacro

%macro MC_8TAP_SCALED_RET 0-1 1 ; leave_mapping_unchanged
    MCT_8TAP_SCALED_REMAP_REGS_TO_DEFAULT
    RET
 %if %1
    MCT_8TAP_SCALED_REMAP_REGS_TO_PREV
 %endif
%endmacro

%if ARCH_X86_64
 %macro MC_8TAP_SCALED_H 12 ; dst[0-1], tmp[0-5], weights[0-3]
    SWAP                m%2, m%5
    movq                m%1, [srcq+ r4]
    movq                m%2, [srcq+ r6]
    movhps              m%1, [srcq+ r7]
    movhps              m%2, [srcq+ r9]
    movq                m%3, [srcq+r10]
    movq                m%4, [srcq+r11]
    movhps              m%3, [srcq+r13]
    movhps              m%4, [srcq+ rX]
    add                srcq, ssq
    movq                m%5, [srcq+ r4]
    movq                m%6, [srcq+ r6]
    movhps              m%5, [srcq+ r7]
    movhps              m%6, [srcq+ r9]
    movq                m%7, [srcq+r10]
    movq                m%8, [srcq+r11]
    movhps              m%7, [srcq+r13]
    movhps              m%8, [srcq+ rX]
    add                srcq, ssq
    pmaddubsw           m%1, m%9
    pmaddubsw           m%5, m%9
    pmaddubsw           m%2, m%10
    pmaddubsw           m%6, m%10
    pmaddubsw           m%3, m%11
    pmaddubsw           m%7, m%11
    pmaddubsw           m%4, m%12
    pmaddubsw           m%8, m%12
    phaddw              m%1, m%2
    phaddw              m%5, m%6
    phaddw              m%3, m%4
    phaddw              m%7, m%8
    phaddw              m%1, m%3
    phaddw              m%5, m%7
    pmulhrsw            m%1, m12
    pmulhrsw            m%5, m12
    SWAP                m%2, m%5
 %endmacro
%else
 %macro MC_8TAP_SCALED_H 2-3 1 ; weights_mem_start, h_mem_start, load_fh_offsets
  %if %3 == 1
    mov                  r0, [esp+ 0]
    mov                  rX, [esp+ 8]
    mov                  r4, [esp+ 4]
    mov                  r5, [esp+12]
  %endif
    movq                 m0, [srcq+r0]
    movq                 m1, [srcq+rX]
    movhps               m0, [srcq+r4]
    movhps               m1, [srcq+r5]
    add                srcq, ssq
    movq                 m4, [srcq+r0]
    movq                 m5, [srcq+rX]
    movhps               m4, [srcq+r4]
    movhps               m5, [srcq+r5]
    mov                  r0, [esp+16]
    mov                  rX, [esp+24]
    mov                  r4, [esp+20]
    mov                  r5, [esp+28]
    sub                srcq, ssq
    movq                 m2, [srcq+r0]
    movq                 m3, [srcq+rX]
    movhps               m2, [srcq+r4]
    movhps               m3, [srcq+r5]
    add                srcq, ssq
    movq                 m6, [srcq+r0]
    movq                 m7, [srcq+rX]
    movhps               m6, [srcq+r4]
    movhps               m7, [srcq+r5]
    add                srcq, ssq
    pmaddubsw            m0, [esp+%1+ 0]
    pmaddubsw            m4, [esp+%1+ 0]
    pmaddubsw            m1, [esp+%1+16]
    pmaddubsw            m5, [esp+%1+16]
    pmaddubsw            m2, [esp+%1+32]
    pmaddubsw            m6, [esp+%1+32]
    pmaddubsw            m3, [esp+%1+48]
    pmaddubsw            m7, [esp+%1+48]
    phaddw               m0, m1
    phaddw               m4, m5
    phaddw               m2, m3
    phaddw               m6, m7
    phaddw               m0, m2
    phaddw               m4, m6
    pmulhrsw             m0, m12
    pmulhrsw             m4, m12
  %if %2 != 0
    mova        [esp+%2+ 0], m0
    mova        [esp+%2+16], m4
  %endif
 %endmacro
%endif

%macro MC_8TAP_SCALED 1
%ifidn %1, put
 %assign isprep 0
 %if ARCH_X86_64
  %if required_stack_alignment <= STACK_ALIGNMENT
cglobal put_8tap_scaled_8bpc, 2, 15, 16, 0x180, dst, ds, src, ss, w, h, mx, my, dx, dy
  %else
cglobal put_8tap_scaled_8bpc, 2, 14, 16, 0x180, dst, ds, src, ss, w, h, mx, my, dx, dy
  %endif
 %else ; ARCH_X86_32
  %if required_stack_alignment <= STACK_ALIGNMENT
cglobal put_8tap_scaled_8bpc, 0, 7, 8, 0x200, dst, ds, src, ss, w, h, mx, my, dx, dy
  %else
cglobal put_8tap_scaled_8bpc, 0, 7, 8, -0x200-0x20, dst, ds, src, ss, w, h, mx, my, dx, dy
  %endif
 %endif
 %xdefine base_reg r12
 %define rndshift 10
%else ; prep
 %assign isprep 1
 %if ARCH_X86_64
  %if required_stack_alignment <= STACK_ALIGNMENT
cglobal prep_8tap_scaled_8bpc, 2, 15, 16, 0x180, tmp, src, ss, w, h, mx, my, dx, dy
   %xdefine tmp_stridem r14q
  %else
cglobal prep_8tap_scaled_8bpc, 2, 14, 16, 0x180, tmp, src, ss, w, h, mx, my, dx, dy
   %define tmp_stridem qword [rsp+0x138]
  %endif
  %xdefine base_reg r11
 %else ; ARCH_X86_32
  %if required_stack_alignment <= STACK_ALIGNMENT
cglobal prep_8tap_scaled_8bpc, 0, 7, 8, 0x200, tmp, src, ss, w, h, mx, my, dx, dy
  %else
cglobal prep_8tap_scaled_8bpc, 0, 6, 8, 0x200, tmp, src, ss, w, h, mx, my, dx, dy
  %endif
  %define tmp_stridem dword [esp+0x138]
 %endif
 %define rndshift 6
%endif
%if ARCH_X86_32
    mov         [esp+0x1f0], t0d
    mov         [esp+0x1f4], t1d
 %if !isprep && required_stack_alignment > STACK_ALIGNMENT
    mov                dstd, dstm
    mov                 dsd, dsm
    mov                srcd, srcm
    mov                 ssd, ssm
    mov                  hd, hm
    mov                  r4, mxm
  %define r0m  [esp+0x200]
  %define dsm  [esp+0x204]
  %define dsmp dsm
  %define r1m  dsm
  %define r2m  [esp+0x208]
  %define ssm  [esp+0x20c]
  %define r3m  ssm
  %define hm   [esp+0x210]
  %define mxm  [esp+0x214]
    mov                 r0m, dstd
    mov                 dsm, dsd
    mov                 r2m, srcd
    mov                 ssm, ssd
    mov                  hm, hd
    mov                  r0, mym
    mov                  r1, dxm
    mov                  r2, dym
  %define mym [esp+0x218]
  %define dxm [esp+0x09c]
  %define dym [esp+0x21c]
    mov                 mxm, r4
    mov                 mym, r0
    mov                 dxm, r1
    mov                 dym, r2
    tzcnt                wd, wm
 %endif
 %if isprep && required_stack_alignment > STACK_ALIGNMENT
  %xdefine base_reg r5
 %else
  %xdefine base_reg r6
 %endif
    mov                 ssd, ssm
%endif
    LEA            base_reg, %1_8tap_scaled_8bpc_ssse3
%xdefine base base_reg-%1_8tap_scaled_8bpc_ssse3
%if ARCH_X86_64 || isprep || required_stack_alignment <= STACK_ALIGNMENT
    tzcnt                wd, wm
%endif
%if ARCH_X86_32
 %define m8  m0
 %define m9  m1
 %define m14 m4
 %define m15 m3
%endif
    movd                 m8, dxm
    movd                m14, mxm
    pshufd               m8, m8, q0000
    pshufd              m14, m14, q0000
%if isprep && UNIX64
    mov                 r5d, t0d
 DECLARE_REG_TMP 5, 7
%endif
%if ARCH_X86_64
    mov                 dyd, dym
%endif
%ifidn %1, put
 %if WIN64
    mov                 r8d, hm
  DEFINE_ARGS dst, ds, src, ss, w, _, _, my, h, dy, ss3
  %define hm r5m
  %define dxm r8m
 %elif ARCH_X86_64
  DEFINE_ARGS dst, ds, src, ss, w, h, _, my, dx, dy, ss3
  %define hm r6m
 %endif
 %if ARCH_X86_64
  %if required_stack_alignment > STACK_ALIGNMENT
   %define dsm [rsp+0x138]
   %define rX r1
   %define rXd r1d
  %else
   %define dsm dsq
   %define rX r14
   %define rXd r14d
  %endif
 %else
  %define rX r1
 %endif
%else ; prep
 %if WIN64
    mov                 r7d, hm
  DEFINE_ARGS tmp, src, ss, w, _, _, my, h, dy, ss3
  %define hm r4m
  %define dxm r7m
 %elif ARCH_X86_64
  DEFINE_ARGS tmp, src, ss, w, h, _, my, dx, dy, ss3
  %define hm [rsp+0x94]
 %endif
 MCT_8TAP_SCALED_REMAP_REGS_TO_PREV
 %if ARCH_X86_64
  %define rX r14
  %define rXd r14d
 %else
  %define rX r3
 %endif
%endif
%if ARCH_X86_64
    mova                m10, [base+pd_0x3ff]
    mova                m12, [base+pw_8192]
 %ifidn %1, put
    mova                m13, [base+pd_512]
 %else
    mova                m13, [base+pd_32]
 %endif
%else
 %define m10 [base+pd_0x3ff]
 %define m12 [base+pw_8192]
 %ifidn %1, put
  %define m13 [base+pd_512]
 %else
  %define m13 [base+pd_32]
 %endif
%endif
    pxor                 m9, m9
%if ARCH_X86_64
    lea                ss3q, [ssq*3]
    movzx               r7d, t1b
    shr                 t1d, 16
    cmp                  hd, 6
    cmovs               t1d, r7d
    sub                srcq, ss3q
%else
 MCT_8TAP_SCALED_REMAP_REGS_TO_DEFAULT
    mov                  r1, [esp+0x1f4]
    lea                  r0, [ssq*3]
    movzx                r2, r1b
    shr                  r1, 16
    cmp            dword hm, 6
    cmovs                r1, r2
    mov         [esp+0x1f4], r1
    mov                  r1, r1m
    mov                  r2, r2m
    sub                srcq, r0
 MCT_8TAP_SCALED_REMAP_REGS_TO_PREV
 %define ss3q r0
 %define myd r4
 %define dyd dword dym
 %define hd  dword hm
%endif
    cmp                 dyd, 1024
    je .dy1
    cmp                 dyd, 2048
    je .dy2
    movzx                wd, word [base+%1_8tap_scaled_ssse3_table+wq*2]
    add                  wq, base_reg
    jmp                  wq
%ifidn %1, put
.w2:
 %if ARCH_X86_64
    mov                 myd, mym
    movzx               t0d, t0b
    dec                srcq
    movd                m15, t0d
 %else
    movzx                r4, byte [esp+0x1f0]
    dec                srcq
    movd                m15, r4
 %endif
    punpckldq            m9, m8
    SWAP                 m8, m9
    paddd               m14, m8 ; mx+dx*[0-1]
 %if ARCH_X86_64
    mova                m11, [base+pd_0x4000]
 %else
  %define m11 [base+pd_0x4000]
 %endif
    pshufd              m15, m15, q0000
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd               m15, m8
    movd                r4d, m15
    psrldq              m15, 4
 %if ARCH_X86_64
    movd                r6d, m15
 %else
    movd                r3d, m15
 %endif
    mova                 m5, [base+bdct_lb_dw]
    mova                 m6, [base+subpel_s_shuf2]
    movd                m15, [base+subpel_filters+r4*8+2]
 %if ARCH_X86_64
    movd                 m7, [base+subpel_filters+r6*8+2]
 %else
    movd                 m7, [base+subpel_filters+r3*8+2]
 %endif
    pxor                 m9, m9
    pcmpeqd              m8, m9
    psrld               m14, 10
 %if ARCH_X86_32
    mov                  r3, r3m
    pshufb              m14, m5
    paddb               m14, m6
    mova        [rsp+0x180], m14
    SWAP                 m5, m0
    SWAP                 m6, m3
  %define m8  m5
  %define m15 m6
 %endif
    movq                 m0, [srcq+ssq*0]
    movq                 m2, [srcq+ssq*2]
    movhps               m0, [srcq+ssq*1]
    movhps               m2, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
 %if ARCH_X86_64
    pshufb              m14, m5
    paddb               m14, m6
 %endif
    movq                 m1, [srcq+ssq*0]
    movq                 m3, [srcq+ssq*2]
    movhps               m1, [srcq+ssq*1]
    movhps               m3, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    punpckldq           m15, m7
    punpcklqdq          m15, m15
 %if ARCH_X86_64
    pand                m11, m8
    pandn                m8, m15
    SWAP                m15, m8
    por                 m15, m11
 %else
    pand                 m7, m8, m11
    pandn                m8, m15
  %define m8  m6
  %define m15 m5
    por                 m15, m7
    mova        [rsp+0x190], m15
 %endif
    pshufb               m0, m14
    pshufb               m2, m14
    pshufb               m1, m14
    pshufb               m3, m14
    pmaddubsw            m0, m15
    pmaddubsw            m2, m15
    pmaddubsw            m1, m15
    pmaddubsw            m3, m15
    phaddw               m0, m2
    phaddw               m1, m3
    pmulhrsw             m0, m12       ; 0 1 2 3
    pmulhrsw             m1, m12       ; 4 5 6 7
    palignr              m2, m1, m0, 4 ; 1 2 3 4
    punpcklwd            m3, m0, m2    ; 01 12
    punpckhwd            m0, m2        ; 23 34
    pshufd               m5, m1, q0321 ; 5 6 7 _
    punpcklwd            m2, m1, m5    ; 45 56
    punpckhwd            m4, m1, m5    ; 67 __
 %if ARCH_X86_32
    mov                 myd, mym
    mov                  r0, r0m
    mova        [rsp+0x1a0], m3
    mova        [rsp+0x1b0], m0
    mova        [rsp+0x1c0], m2
    mova        [rsp+0x1d0], m4
 %endif
.w2_loop:
    and                 myd, 0x3ff
 %if ARCH_X86_64
    mov                 r6d, 64 << 24
    mov                 r4d, myd
    shr                 r4d, 6
    lea                 r4d, [t1+r4]
    cmovnz              r6q, [base+subpel_filters+r4*8]
    movq                m11, r6q
    punpcklbw           m11, m11
    psraw               m11, 8
    pshufd               m8, m11, q0000
    pshufd               m9, m11, q1111
    pshufd              m10, m11, q2222
    pshufd              m11, m11, q3333
    pmaddwd              m5, m3, m8
    pmaddwd              m6, m0, m9
    pmaddwd              m7, m2, m10
    pmaddwd              m8, m4, m11
    paddd                m5, m6
    paddd                m7, m8
 %else
    mov                 mym, myd
    mov                  r1, [esp+0x1f4]
    xor                  r3, r3
    shr                  r4, 6
    lea                  r1, [r1+r4]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r1*8+0]
    cmovnz               r3, [base+subpel_filters+r1*8+4]
    movd                 m7, r4
    movd                 m6, r3
    punpckldq            m7, m6
    punpcklbw            m7, m7
    psraw                m7, 8
    pshufd               m5, m7, q0000
    pshufd               m6, m7, q1111
    pmaddwd              m3, m5
    pmaddwd              m0, m6
    pshufd               m5, m7, q2222
    pshufd               m7, m7, q3333
    pmaddwd              m2, m5
    pmaddwd              m4, m7
    paddd                m3, m0
    paddd                m2, m4
    SWAP                 m5, m3
    SWAP                 m7, m2
 %endif
    paddd                m5, m13
    paddd                m5, m7
    psrad                m5, 10
    packssdw             m5, m5
    packuswb             m5, m5
 %if ARCH_X86_64
    pextrw              r6d, m5, 0
    mov              [dstq], r6w
    add                dstq, dsq
    dec                  hd
    jz .ret
    add                 myd, dyd
 %else
    pextrw              r3d, m5, 0
    mov              [dstq], r3w
    add                dstq, dsm
    dec                  hd
    jz .ret
    mov                 myd, mym
    add                 myd, dym
 %endif
    test                myd, ~0x3ff
 %if ARCH_X86_32
    SWAP                 m3, m5
    SWAP                 m2, m7
    mova                 m3, [rsp+0x1a0]
    mova                 m0, [rsp+0x1b0]
    mova                 m2, [rsp+0x1c0]
    mova                 m4, [rsp+0x1d0]
  %define m14 [esp+0x180]
  %define m15 [esp+0x190]
 %endif
    jz .w2_loop
 %if ARCH_X86_32
    mov                  r3, r3m
 %endif
    movq                 m5, [srcq]
    test                myd, 0x400
    jz .w2_skip_line
    add                srcq, ssq
    shufps               m3, m0, q1032      ; 01 12
    shufps               m0, m2, q1032      ; 23 34
    shufps               m2, m4, q1032      ; 45 56
    pshufb               m5, m14
    pmaddubsw            m5, m15
    phaddw               m5, m5
    pmulhrsw             m5, m12
    palignr              m4, m5, m1, 12
    punpcklqdq           m1, m4, m4         ; 6 7 6 7
    punpcklwd            m4, m1, m5         ; 67 __
 %if ARCH_X86_32
    mova        [rsp+0x1a0], m3
    mova        [rsp+0x1b0], m0
    mova        [rsp+0x1c0], m2
    mova        [rsp+0x1d0], m4
 %endif
    jmp .w2_loop
.w2_skip_line:
    movhps               m5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova                 m3, m0             ; 01 12
    mova                 m0, m2             ; 23 34
    pshufb               m5, m14
    pmaddubsw            m5, m15
    phaddw               m5, m5
    pmulhrsw             m5, m12            ; 6 7 6 7
    palignr              m4, m5, m1, 8      ; 4 5 6 7
    pshufd               m5, m4, q0321      ; 5 6 7 _
    mova                 m1, m4
    punpcklwd            m2, m4, m5         ; 45 56
    punpckhwd            m4, m5             ; 67 __
 %if ARCH_X86_32
    mova        [rsp+0x1a0], m3
    mova        [rsp+0x1b0], m0
    mova        [rsp+0x1c0], m2
    mova        [rsp+0x1d0], m4
 %endif
    jmp .w2_loop
%endif
INIT_XMM ssse3
.w4:
%if ARCH_X86_64
    mov                 myd, mym
    movzx               t0d, t0b
    dec                srcq
    movd                m15, t0d
%else
 %define m8  m0
 %xdefine m14 m4
 %define m15 m3
    movzx                r4, byte [esp+0x1f0]
    dec                srcq
    movd                m15, r4
%endif
    pmaddwd              m8, [base+rescale_mul]
%if ARCH_X86_64
    mova                m11, [base+pd_0x4000]
%else
  %define m11 [base+pd_0x4000]
%endif
    pshufd              m15, m15, q0000
    paddd               m14, m8 ; mx+dx*[0-3]
    pand                 m0, m14, m10
    psrld                m0, 6
    paddd               m15, m0
    psrldq               m7, m15, 8
%if ARCH_X86_64
    movd                r4d, m15
    movd               r11d, m7
    psrldq              m15, 4
    psrldq               m7, 4
    movd                r6d, m15
    movd               r13d, m7
    movd                m15, [base+subpel_filters+ r4*8+2]
    movd                 m2, [base+subpel_filters+r11*8+2]
    movd                 m3, [base+subpel_filters+ r6*8+2]
    movd                 m4, [base+subpel_filters+r13*8+2]
%else
    movd                 r0, m15
    movd                 rX, m7
    psrldq              m15, 4
    psrldq               m7, 4
    movd                 r4, m15
    movd                 r5, m7
    movd                 m1, [base+subpel_filters+r0*8+2]
    movd                 m2, [base+subpel_filters+rX*8+2]
    movd                 m3, [base+subpel_filters+r4*8+2]
    movd                 m7, [base+subpel_filters+r5*8+2]
    movifprep            r3, r3m
    SWAP                 m4, m7
 %define m15 m1
%endif
    mova                 m5, [base+bdct_lb_dw]
    movq                 m6, [base+subpel_s_shuf2]
    psrld               m14, 10
    punpckldq           m15, m3
    punpckldq            m2, m4
    punpcklqdq          m15, m2
    punpcklqdq           m6, m6
    pshufb              m14, m5
    paddb               m14, m6
%if ARCH_X86_64
    pcmpeqd              m0, m9
    pand                m11, m0
%else
    mova        [esp+0x180], m14
    SWAP                 m7, m4
    pxor                 m3, m3
    pcmpeqd              m0, m3
    pand                 m2, m11, m0
 %define m11 m2
%endif
    pandn                m0, m15
%if ARCH_X86_64
    SWAP                m15, m0
%else
 %define m15 m0
%endif
    por                 m15, m11
%if ARCH_X86_64
    movu                 m7, [srcq+ssq*0]
    movu                 m9, [srcq+ssq*1]
    movu                 m8, [srcq+ssq*2]
    movu                m10, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    movu                 m2, [srcq+ssq*0]
    movu                 m4, [srcq+ssq*1]
    movu                 m3, [srcq+ssq*2]
    movu                 m5, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    pshufb               m7, m14
    pshufb               m9, m14
    pshufb               m8, m14
    pshufb              m10, m14
    pshufb               m2, m14
    pshufb               m4, m14
    pshufb               m3, m14
    pshufb               m5, m14
    pmaddubsw            m7, m15
    pmaddubsw            m9, m15
    pmaddubsw            m8, m15
    pmaddubsw           m10, m15
    pmaddubsw            m2, m15
    pmaddubsw            m4, m15
    pmaddubsw            m3, m15
    pmaddubsw            m5, m15
    phaddw               m7, m9
    phaddw               m8, m10
    phaddw               m9, m2, m4
    phaddw               m3, m5
    pmulhrsw             m7, m12            ; 0 1
    pmulhrsw             m8, m12            ; 2 3
    pmulhrsw             m9, m12            ; 4 5
    pmulhrsw             m3, m12            ; 6 7
    shufps               m4, m7, m8, q1032  ; 1 2
    shufps               m5, m8, m9, q1032  ; 3 4
    shufps               m6, m9, m3, q1032  ; 5 6
    psrldq              m11, m3, 8          ; 7 _
    punpcklwd            m0, m7, m4 ; 01
    punpckhwd            m7, m4     ; 12
    punpcklwd            m1, m8, m5 ; 23
    punpckhwd            m8, m5     ; 34
    punpcklwd            m2, m9, m6 ; 45
    punpckhwd            m9, m6     ; 56
    punpcklwd            m3, m11    ; 67
    mova         [rsp+0x00], m7
    mova         [rsp+0x10], m8
    mova         [rsp+0x20], m9
%else
    mova        [esp+0x190], m15
    lea                ss3q, [ssq*3]
    movu                 m2, [srcq+ssq*0]
    movu                 m3, [srcq+ssq*1]
    movu                 m7, [srcq+ssq*2]
    movu                 m6, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    pshufb               m2, m14
    pshufb               m3, m14
    pshufb               m7, m14
    pshufb               m6, m14
    pmaddubsw            m2, m15
    pmaddubsw            m3, m15
    pmaddubsw            m7, m15
    pmaddubsw            m6, m15
    phaddw               m2, m3
    phaddw               m7, m6
    movu                 m1, [srcq+ssq*0]
    movu                 m5, [srcq+ssq*1]
    movu                 m3, [srcq+ssq*2]
    movu                 m6, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    pshufb               m1, m14
    pshufb               m5, m14
    pshufb               m3, m14
    pshufb               m6, m14
    pmaddubsw            m1, m15
    pmaddubsw            m5, m15
    pmaddubsw            m3, m15
    pmaddubsw            m6, m15
    phaddw               m1, m5
    phaddw               m3, m6
    pmulhrsw             m2, m12
    pmulhrsw             m7, m12
    pmulhrsw             m1, m12
    pmulhrsw             m3, m12
    shufps               m4, m2, m7, q1032  ; 1 2
    shufps               m5, m7, m1, q1032  ; 3 4
    shufps               m6, m1, m3, q1032  ; 5 6
    psrldq               m0, m3, 8          ; 7 _
    mova        [esp+0x1a0], m0
 %define m11 [esp+0x1a0]
    punpcklwd            m0, m2, m4      ; 01
    punpckhwd            m2, m4          ; 12
    punpcklwd            m4, m7, m5      ; 23
    punpckhwd            m7, m5          ; 34
    punpcklwd            m5, m1, m6      ; 45
    punpckhwd            m1, m6          ; 56
    punpcklwd            m3, [esp+0x1a0] ; 67
    mov                 myd, mym
    mov                  r0, r0m
    mova        [esp+0x1b0], m0 ; 01
    mova        [esp+0x1c0], m4 ; 23
    mova        [esp+0x1d0], m5 ; 45
    mova        [esp+0x1e0], m3 ; 67
    mova         [rsp+0x00], m2 ; 12
    mova         [rsp+0x10], m7 ; 34
    mova         [rsp+0x20], m1 ; 56
    SWAP                 m1, m4
    SWAP                 m2, m5
%endif
.w4_loop:
    and                 myd, 0x3ff
%if ARCH_X86_64
    mov                 r6d, 64 << 24
    mov                 r4d, myd
    shr                 r4d, 6
    lea                 r4d, [t1+r4]
    cmovnz              r6q, [base+subpel_filters+r4*8]
    movq                m10, r6q
    punpcklbw           m10, m10
    psraw               m10, 8
    pshufd               m7, m10, q0000
    pshufd               m8, m10, q1111
    pshufd               m9, m10, q2222
    pshufd              m10, m10, q3333
    pmaddwd              m4, m0, m7
    pmaddwd              m5, m1, m8
    pmaddwd              m6, m2, m9
    pmaddwd              m7, m3, m10
    paddd                m4, m5
    paddd                m6, m7
    paddd                m4, m13
    paddd                m4, m6
%else
    mov                 mym, myd
    mov                  r5, [esp+0x1f4]
    xor                  r3, r3
    shr                  r4, 6
    lea                  r5, [r5+r4]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r5*8+0]
    cmovnz               r3, [base+subpel_filters+r5*8+4]
    movd                 m7, r4
    movd                 m6, r3
    punpckldq            m7, m6
    punpcklbw            m7, m7
    psraw                m7, 8
    pshufd               m4, m7, q0000
    pshufd               m5, m7, q1111
    pshufd               m6, m7, q2222
    pshufd               m7, m7, q3333
    pmaddwd              m0, m4
    pmaddwd              m1, m5
    pmaddwd              m2, m6
    pmaddwd              m3, m7
    paddd                m0, m1
    paddd                m2, m3
    paddd                m0, m13
    paddd                m0, m2
    SWAP                 m4, m0
%endif
    psrad                m4, rndshift
    packssdw             m4, m4
%ifidn %1, put
    packuswb             m4, m4
    movd             [dstq], m4
    add                dstq, dsmp
%else
    movq             [tmpq], m4
    add                tmpq, 8
%endif
    dec                  hd
    jz .ret
%if ARCH_X86_64
    add                 myd, dyd
    test                myd, ~0x3ff
    jz .w4_loop
%else
    SWAP                 m0, m4
    mov                 myd, mym
    mov                  r3, r3m
    add                 myd, dym
    test                myd, ~0x3ff
    jnz .w4_next_line
    mova                 m0, [esp+0x1b0]
    mova                 m1, [esp+0x1c0]
    mova                 m2, [esp+0x1d0]
    mova                 m3, [esp+0x1e0]
    jmp .w4_loop
.w4_next_line:
  %define m14 [esp+0x180]
  %define m15 [esp+0x190]
%endif
    movu                 m4, [srcq]
    test                myd, 0x400
    jz .w4_skip_line
%if ARCH_X86_64
    mova                 m0, [rsp+0x00]
    mova         [rsp+0x00], m1
    mova                 m1, [rsp+0x10]
    mova         [rsp+0x10], m2
    mova                 m2, [rsp+0x20]
    mova         [rsp+0x20], m3
%else
    mova                 m5, [esp+0x1c0]
    mova                 m0, [rsp+0x000]
    mova         [rsp+0x00], m5
    mova        [esp+0x1b0], m0
    mova                 m6, [esp+0x1d0]
    mova                 m1, [rsp+0x010]
    mova         [rsp+0x10], m6
    mova        [esp+0x1c0], m1
    mova                 m7, [esp+0x1e0]
    mova                 m2, [rsp+0x020]
    mova         [rsp+0x20], m7
    mova        [esp+0x1d0], m2
%endif
    pshufb               m4, m14
    pmaddubsw            m4, m15
    phaddw               m4, m4
    pmulhrsw             m4, m12
    punpcklwd            m3, m11, m4
%if ARCH_X86_32
    mova        [esp+0x1e0], m3
%endif
    mova                m11, m4
    add                srcq, ssq
    jmp .w4_loop
.w4_skip_line:
%if ARCH_X86_32
    mova                 m0, [esp+0x1c0]
    mova                 m1, [esp+0x1d0]
    mova                 m2, [esp+0x1e0]
%endif
    movu                 m5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova                 m6, [rsp+0x10]
    mova                 m7, [rsp+0x20]
    pshufb               m4, m14
    pshufb               m5, m14
    pmaddubsw            m4, m15
    pmaddubsw            m5, m15
    phaddw               m4, m5
    pmulhrsw             m4, m12
    punpcklwd            m5, m11, m4
    mova         [rsp+0x00], m6
    mova         [rsp+0x10], m7
    mova         [rsp+0x20], m5
%if ARCH_X86_64
    psrldq              m11, m4, 8
    mova                 m0, m1
    mova                 m1, m2
    mova                 m2, m3
    punpcklwd            m3, m4, m11
%else
    psrldq               m6, m4, 8
    punpcklwd            m3, m4, m6
    mova        [esp+0x1a0], m6
    mova        [esp+0x1b0], m0
    mova        [esp+0x1c0], m1
    mova        [esp+0x1d0], m2
    mova        [esp+0x1e0], m3
%endif
    jmp .w4_loop
INIT_XMM ssse3
.w8:
    mov    dword [rsp+0x90], 1
    movifprep   tmp_stridem, 16
    jmp .w_start
.w16:
    mov    dword [rsp+0x90], 2
    movifprep   tmp_stridem, 32
    jmp .w_start
.w32:
    mov    dword [rsp+0x90], 4
    movifprep   tmp_stridem, 64
    jmp .w_start
.w64:
    mov    dword [rsp+0x90], 8
    movifprep   tmp_stridem, 128
    jmp .w_start
.w128:
    mov    dword [rsp+0x90], 16
    movifprep   tmp_stridem, 256
.w_start:
%ifidn %1, put
    movifnidn           dsm, dsq
%endif
%if ARCH_X86_64
    shr                 t0d, 16
    movd                m15, t0d
%else
 %define m8  m0
 %xdefine m14 m4
 %define m15 m3
 %if isprep
  %define ssq ssm
 %endif
    mov                  r4, [esp+0x1f0]
    shr                  r4, 16
    movd                m15, r4
    mov                  r0, r0m
    mov                 myd, mym
%endif
    sub                srcq, 3
    pslld                m7, m8, 2 ; dx*4
    pmaddwd              m8, [base+rescale_mul] ; dx*[0-3]
    pshufd              m15, m15, q0000
    paddd               m14, m8 ; mx+dx*[0-3]
    mova        [rsp+0x100], m7
    mova        [rsp+0x120], m15
    mov         [rsp+0x098], srcq
    mov         [rsp+0x130], r0q ; dstq / tmpq
%if ARCH_X86_64 && UNIX64
    mov                  hm, hd
%elif ARCH_X86_32
    mov                  r5, hm
    mov         [esp+0x094], myd
    mov         [esp+0x134], r5
%endif
    jmp .hloop
.hloop_prep:
    dec   dword [rsp+0x090]
    jz .ret
%if ARCH_X86_64
    add   qword [rsp+0x130], 8*(isprep+1)
    mov                  hd, hm
%else
    add   dword [esp+0x130], 8*(isprep+1)
    mov                 myd, [esp+0x094]
    mov                  r5, [esp+0x134]
    mov                  r0, [esp+0x130]
%endif
    mova                 m7, [rsp+0x100]
    mova                m14, [rsp+0x110]
%if ARCH_X86_64
    mova                m10, [base+pd_0x3ff]
%endif
    mova                m15, [rsp+0x120]
    pxor                 m9, m9
    mov                srcq, [rsp+0x098]
%if ARCH_X86_64
    mov                 r0q, [rsp+0x130] ; dstq / tmpq
%else
    mov                 mym, myd
    mov                  hm, r5
    mov                 r0m, r0
    mov                  r3, r3m
%endif
    paddd               m14, m7
.hloop:
%if ARCH_X86_64
    mova                m11, [base+pq_0x40000000]
%else
 %define m11 [base+pq_0x40000000]
%endif
    psrld                m2, m14, 10
    mova              [rsp], m2
    pand                 m6, m14, m10
    psrld                m6, 6
    paddd                m5, m15, m6
    pcmpeqd              m6, m9
    psrldq               m2, m5, 8
%if ARCH_X86_64
    movd                r4d, m5
    movd                r6d, m2
    psrldq               m5, 4
    psrldq               m2, 4
    movd                r7d, m5
    movd                r9d, m2
    movq                 m0, [base+subpel_filters+r4*8]
    movq                 m1, [base+subpel_filters+r6*8]
    movhps               m0, [base+subpel_filters+r7*8]
    movhps               m1, [base+subpel_filters+r9*8]
%else
    movd                 r0, m5
    movd                 rX, m2
    psrldq               m5, 4
    psrldq               m2, 4
    movd                 r4, m5
    movd                 r5, m2
    movq                 m0, [base+subpel_filters+r0*8]
    movq                 m1, [base+subpel_filters+rX*8]
    movhps               m0, [base+subpel_filters+r4*8]
    movhps               m1, [base+subpel_filters+r5*8]
    pxor                 m2, m2
 %define m9 m2
%endif
    paddd               m14, m7 ; mx+dx*[4-7]
    pand                 m5, m14, m10
    psrld                m5, 6
    paddd               m15, m5
    pcmpeqd              m5, m9
    mova        [rsp+0x110], m14
    psrldq               m4, m15, 8
%if ARCH_X86_64
    movd               r10d, m15
    movd               r11d, m4
    psrldq              m15, 4
    psrldq               m4, 4
    movd               r13d, m15
    movd                rXd, m4
    movq                 m2, [base+subpel_filters+r10*8]
    movq                 m3, [base+subpel_filters+r11*8]
    movhps               m2, [base+subpel_filters+r13*8]
    movhps               m3, [base+subpel_filters+ rX*8]
    psrld               m14, 10
    psrldq               m4, m14, 8
    movd               r10d, m14
    movd               r11d, m4
    psrldq              m14, 4
    psrldq               m4, 4
    movd               r13d, m14
    movd                rXd, m4
    mov                 r4d, [rsp+ 0]
    mov                 r6d, [rsp+ 8]
    mov                 r7d, [rsp+ 4]
    mov                 r9d, [rsp+12]
    pshufd               m4, m6, q1100
    pshufd               m6, m6, q3322
    pshufd              m14, m5, q1100
    pshufd               m5, m5, q3322
    pand                 m7, m11, m4
    pand                 m8, m11, m6
    pand                m15, m11, m14
    pand                m11, m11, m5
    pandn                m4, m0
    pandn                m6, m1
    pandn               m14, m2
    pandn                m5, m3
    por                  m7, m4
    por                  m8, m6
    por                 m15, m14
    por                 m11, m5
    mova         [rsp+0x10], m7
    mova         [rsp+0x20], m8
    mova         [rsp+0x30], m15
    mova         [rsp+0x40], m11
    MC_8TAP_SCALED_H 1, 2, 3, 4, 5, 6, 9, 10, 7, 8, 15, 11 ; 0-1
    mova         [rsp+0x50], m1
    mova         [rsp+0x60], m2
    MC_8TAP_SCALED_H 3, 4, 5, 6, 1, 2, 9, 10, 7, 8, 15, 11 ; 2-3
    mova         [rsp+0x70], m3
    mova         [rsp+0x80], m4
    MC_8TAP_SCALED_H 5, 6, 1, 2, 3, 4, 9, 10, 7, 8, 15, 11 ; 4-5
    MC_8TAP_SCALED_H 0,14, 1, 2, 3, 4, 9, 10, 7, 8, 15, 11 ; 6-7
    SWAP                 m7, m0
    SWAP                 m8, m14
    mova                 m1, [rsp+0x50]
    mova                 m2, [rsp+0x60]
    mova                 m3, [rsp+0x70]
    mova                 m9, [rsp+0x80]
    mov                 myd, mym
    mov                 dyd, dym
    punpcklwd            m4, m5, m6 ; 45a
    punpckhwd            m5, m6     ; 45b
    punpcklwd            m6, m7, m8 ; 67a
    punpckhwd            m7, m8     ; 67b
    punpcklwd            m0, m1, m2 ; 01a
    punpckhwd            m1, m2     ; 01b
    punpcklwd            m2, m3, m9 ; 23a
    punpckhwd            m3, m9     ; 23b
    mova         [rsp+0x50], m4
    mova         [rsp+0x60], m5
    mova         [rsp+0x70], m6
    mova         [rsp+0x80], m7
    SWAP                m14, m8
.vloop:
    and                 myd, 0x3ff
    mov                 r6d, 64 << 24
    mov                 r4d, myd
    shr                 r4d, 6
    lea                 r4d, [t1+r4]
    cmovnz              r6q, [base+subpel_filters+r4*8]
    movq                m11, r6q
    punpcklbw           m11, m11
    psraw               m11, 8
    pshufd               m5, m11, q0000
    pshufd               m7, m11, q1111
    pshufd              m10, m11, q2222
    pshufd              m11, m11, q3333
    pmaddwd              m4, m5, m0
    pmaddwd              m5, m5, m1
    pmaddwd              m6, m7, m2
    pmaddwd              m7, m7, m3
    paddd                m4, m13
    paddd                m5, m13
    paddd                m4, m6
    paddd                m5, m7
    pmaddwd              m6, [rsp+0x50], m10
    pmaddwd              m7, [rsp+0x60], m10
    pmaddwd              m8, [rsp+0x70], m11
    pmaddwd              m9, [rsp+0x80], m11
    paddd                m4, m6
    paddd                m5, m7
    paddd                m4, m8
    paddd                m5, m9
%else
    movd                 r0, m15
    movd                 rX, m4
    psrldq              m15, 4
    psrldq               m4, 4
    movd                 r4, m15
    movd                 r5, m4
    mova                m14, [esp+0x110]
    movq                 m2, [base+subpel_filters+r0*8]
    movq                 m3, [base+subpel_filters+rX*8]
    movhps               m2, [base+subpel_filters+r4*8]
    movhps               m3, [base+subpel_filters+r5*8]
    psrld               m14, 10
    mova           [esp+16], m14
    mov                  r0, [esp+ 0]
    mov                  rX, [esp+ 8]
    mov                  r4, [esp+ 4]
    mov                  r5, [esp+12]
    mova         [esp+0x20], m0
    mova         [esp+0x30], m1
    mova         [esp+0x40], m2
    mova         [esp+0x50], m3
    pshufd               m4, m6, q1100
    pshufd               m6, m6, q3322
    pshufd               m7, m5, q1100
    pshufd               m5, m5, q3322
    pand                 m0, m11, m4
    pand                 m1, m11, m6
    pand                 m2, m11, m7
    pand                 m3, m11, m5
    pandn                m4, [esp+0x20]
    pandn                m6, [esp+0x30]
    pandn                m7, [esp+0x40]
    pandn                m5, [esp+0x50]
    por                  m0, m4
    por                  m1, m6
    por                  m2, m7
    por                  m3, m5
    mova         [esp+0x20], m0
    mova         [esp+0x30], m1
    mova         [esp+0x40], m2
    mova         [esp+0x50], m3
    MC_8TAP_SCALED_H   0x20, 0x140, 0 ; 0-1
    MC_8TAP_SCALED_H   0x20, 0x160    ; 2-3
    MC_8TAP_SCALED_H   0x20, 0x180    ; 4-5
    MC_8TAP_SCALED_H   0x20, 0x1a0    ; 6-7
    mova                 m5, [esp+0x180]
    mova                 m6, [esp+0x190]
    mova                 m7, [esp+0x1a0]
    mova                 m0, [esp+0x1b0]
    mov                 myd, mym
    punpcklwd            m4, m5, m6      ; 45a
    punpckhwd            m5, m6          ; 45b
    punpcklwd            m6, m7, m0      ; 67a
    punpckhwd            m7, m0          ; 67b
    mova        [esp+0x180], m4
    mova        [esp+0x190], m5
    mova        [esp+0x1a0], m6
    mova        [esp+0x1b0], m7
    mova                 m1, [esp+0x140]
    mova                 m2, [esp+0x150]
    mova                 m3, [esp+0x160]
    mova                 m4, [esp+0x170]
    punpcklwd            m0, m1, m2      ; 01a
    punpckhwd            m1, m2          ; 01b
    punpcklwd            m2, m3, m4      ; 23a
    punpckhwd            m3, m4          ; 23b
    mova        [esp+0x140], m0
    mova        [esp+0x150], m1
    mova        [esp+0x160], m2
    mova        [esp+0x170], m3
.vloop:
    mov                  r0, r0m
    mov                  r5, [esp+0x1f4]
    and                 myd, 0x3ff
    mov                 mym, myd
    xor                  r3, r3
    shr                  r4, 6
    lea                  r5, [r5+r4]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r5*8+0]
    cmovnz               r3, [base+subpel_filters+r5*8+4]
    movd                 m7, r4
    movd                 m6, r3
    punpckldq            m7, m6
    punpcklbw            m7, m7
    psraw                m7, 8
    pshufd               m4, m7, q0000
    pshufd               m5, m7, q1111
    pmaddwd              m0, m4
    pmaddwd              m1, m4
    pmaddwd              m2, m5
    pmaddwd              m3, m5
    pshufd               m6, m7, q2222
    pshufd               m7, m7, q3333
    paddd                m0, m2
    paddd                m1, m3
    pmaddwd              m2, [esp+0x180], m6
    pmaddwd              m3, [esp+0x190], m6
    pmaddwd              m4, [esp+0x1a0], m7
    pmaddwd              m5, [esp+0x1b0], m7
    paddd                m0, m2
    paddd                m1, m3
    paddd                m0, m13
    paddd                m1, m13
    paddd                m4, m0
    paddd                m5, m1
%endif
    psrad                m4, rndshift
    psrad                m5, rndshift
    packssdw             m4, m5
%ifidn %1, put
    packuswb             m4, m4
    movq             [dstq], m4
    add                dstq, dsm
%else
    mova             [tmpq], m4
    add                tmpq, tmp_stridem
%endif
    dec                  hd
    jz .hloop_prep
%if ARCH_X86_64
    add                 myd, dyd
    test                myd, ~0x3ff
    jz .vloop
    test                myd, 0x400
    mov         [rsp+0x140], myd
    mov                 r4d, [rsp+ 0]
    mov                 r6d, [rsp+ 8]
    mov                 r7d, [rsp+ 4]
    mov                 r9d, [rsp+12]
    jz .skip_line
    mova                m14, [base+unpckw]
    movq                 m6, [srcq+r10]
    movq                 m7, [srcq+r11]
    movhps               m6, [srcq+r13]
    movhps               m7, [srcq+ rX]
    movq                 m4, [srcq+ r4]
    movq                 m5, [srcq+ r6]
    movhps               m4, [srcq+ r7]
    movhps               m5, [srcq+ r9]
    add                srcq, ssq
    mov                 myd, [rsp+0x140]
    mov                 dyd, dym
    pshufd               m9, m14, q1032
    pshufb               m0, m14                ; 0a 1a
    pshufb               m1, m14                ; 0b 1b
    pshufb               m2, m9                 ; 3a 2a
    pshufb               m3, m9                 ; 3b 2b
    pmaddubsw            m6, [rsp+0x30]
    pmaddubsw            m7, [rsp+0x40]
    pmaddubsw            m4, [rsp+0x10]
    pmaddubsw            m5, [rsp+0x20]
    phaddw               m6, m7
    phaddw               m4, m5
    phaddw               m4, m6
    pmulhrsw             m4, m12
    pshufb               m5, [rsp+0x50], m14    ; 4a 5a
    pshufb               m6, [rsp+0x60], m14    ; 4b 5b
    pshufb               m7, [rsp+0x70], m9     ; 7a 6a
    pshufb               m8, [rsp+0x80], m9     ; 7b 6b
    punpckhwd            m0, m2 ; 12a
    punpckhwd            m1, m3 ; 12b
    punpcklwd            m2, m5 ; 34a
    punpcklwd            m3, m6 ; 34b
    punpckhwd            m5, m7 ; 56a
    punpckhwd            m6, m8 ; 56b
    punpcklwd            m7, m4 ; 78a
    punpckhqdq           m4, m4
    punpcklwd            m8, m4 ; 78b
    mova         [rsp+0x50], m5
    mova         [rsp+0x60], m6
    mova         [rsp+0x70], m7
    mova         [rsp+0x80], m8
    jmp .vloop
.skip_line:
    mova                 m0, [rsp+0x10]
    mova                 m1, [rsp+0x20]
    mova                m14, [rsp+0x30]
    mova                m15, [rsp+0x40]
    MC_8TAP_SCALED_H 4, 8, 5, 6, 7, 9, 10, 11, 0, 1, 14, 15
    mov                 myd, [rsp+0x140]
    mov                 dyd, dym
    mova                 m0, m2         ; 01a
    mova                 m1, m3         ; 01b
    mova                 m2, [rsp+0x50] ; 23a
    mova                 m3, [rsp+0x60] ; 23b
    mova                 m5, [rsp+0x70] ; 45a
    mova                 m6, [rsp+0x80] ; 45b
    punpcklwd            m7, m4, m8     ; 67a
    punpckhwd            m4, m8         ; 67b
    mova         [rsp+0x50], m5
    mova         [rsp+0x60], m6
    mova         [rsp+0x70], m7
    mova         [rsp+0x80], m4
%else
    mov                 r0m, r0
    mov                 myd, mym
    mov                  r3, r3m
    add                 myd, dym
    test                myd, ~0x3ff
    mov                 mym, myd
    jnz .next_line
    mova                 m0, [esp+0x140]
    mova                 m1, [esp+0x150]
    mova                 m2, [esp+0x160]
    mova                 m3, [esp+0x170]
    jmp .vloop
.next_line:
    test                myd, 0x400
    mov                  r0, [esp+ 0]
    mov                  rX, [esp+ 8]
    mov                  r4, [esp+ 4]
    mov                  r5, [esp+12]
    jz .skip_line
    mova                 m6, [base+unpckw]
    mova                 m0, [esp+0x140]
    mova                 m1, [esp+0x150]
    mova                 m7, [esp+0x180]
    movq                 m4, [srcq+r0]
    movq                 m5, [srcq+rX]
    movhps               m4, [srcq+r4]
    movhps               m5, [srcq+r5]
    pshufb               m0, m6         ; 0a 1a
    pshufb               m1, m6         ; 0b 1b
    pshufb               m7, m6         ; 4a 5a
    mov                  r0, [esp+16]
    mov                  rX, [esp+24]
    mov                  r4, [esp+20]
    mov                  r5, [esp+28]
    movq                 m3, [srcq+r0]
    movq                 m2, [srcq+rX]
    movhps               m3, [srcq+r4]
    movhps               m2, [srcq+r5]
    add                srcq, ssq
    pmaddubsw            m4, [esp+0x20]
    pmaddubsw            m5, [esp+0x30]
    pmaddubsw            m3, [esp+0x40]
    pmaddubsw            m2, [esp+0x50]
    phaddw               m4, m5
    phaddw               m3, m2
    mova                 m5, [esp+0x190]
    mova                 m2, [esp+0x160]
    phaddw               m4, m3
    mova                 m3, [esp+0x170]
    pmulhrsw             m4, m12        ; 8a 8b
    mov                 myd, mym
    pshufb               m5, m6         ; 4b 5b
    pshufd               m6, m6, q1032
    pshufb               m2, m6         ; 3a 2a
    pshufb               m3, m6         ; 3b 2b
    punpckhwd            m0, m2         ; 12a
    punpckhwd            m1, m3         ; 12b
    mova        [esp+0x140], m0
    mova        [esp+0x150], m1
    mova                 m0, [esp+0x1a0]
    mova                 m1, [esp+0x1b0]
    punpcklwd            m2, m7         ; 34a
    punpcklwd            m3, m5         ; 34b
    mova        [esp+0x160], m2
    mova        [esp+0x170], m3
    pshufb               m0, m6         ; 7a 6a
    pshufb               m1, m6         ; 7b 6b
    punpckhwd            m7, m0         ; 56a
    punpckhwd            m5, m1         ; 56b
    punpcklwd            m0, m4
    punpckhqdq           m4, m4
    punpcklwd            m1, m4
    mova        [esp+0x180], m7
    mova        [esp+0x190], m5
    mova        [esp+0x1a0], m0
    mova        [esp+0x1b0], m1
    mova                 m0, [esp+0x140]
    mova                 m1, [esp+0x150]
    jmp .vloop
.skip_line:
    MC_8TAP_SCALED_H   0x20, 0x1c0, 0
    mov                 myd, mym
    mova                 m0, [esp+0x160]
    mova                 m1, [esp+0x170]
    mova                 m2, [esp+0x180]
    mova                 m3, [esp+0x190]
    mova         [esp+0x140], m0
    mova         [esp+0x150], m1
    mova                 m4, [esp+0x1a0]
    mova                 m5, [esp+0x1b0]
    mova        [esp+0x160], m2
    mova        [esp+0x170], m3
    mova                 m6, [esp+0x1c0]
    mova                 m7, [esp+0x1d0]
    mova        [esp+0x180], m4
    mova        [esp+0x190], m5
    punpcklwd            m4, m6, m7
    punpckhwd            m6, m7
    mova        [esp+0x1a0], m4
    mova        [esp+0x1b0], m6
%endif
    jmp .vloop
INIT_XMM ssse3
.dy1:
    movzx                wd, word [base+%1_8tap_scaled_ssse3_dy1_table+wq*2]
    add                  wq, base_reg
    jmp                  wq
%ifidn %1, put
.dy1_w2:
 %if ARCH_X86_64
    mov                 myd, mym
    movzx               t0d, t0b
    dec                srcq
    movd                m15, t0d
 %else
  %define m8  m0
  %define m9  m1
  %define m14 m4
  %define m15 m3
    movzx                r5, byte [esp+0x1f0]
    dec                srcd
    movd                m15, r5
 %endif
    punpckldq            m9, m8
    SWAP                 m8, m9
    paddd               m14, m8 ; mx+dx*[0-1]
 %if ARCH_X86_64
    mova                m11, [base+pd_0x4000]
 %else
  %define m11 [base+pd_0x4000]
 %endif
    pshufd              m15, m15, q0000
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd               m15, m8
    movd                r4d, m15
    psrldq              m15, 4
 %if ARCH_X86_64
    movd                r6d, m15
 %else
    movd                r3d, m15
 %endif
    mova                 m5, [base+bdct_lb_dw]
    mova                 m6, [base+subpel_s_shuf2]
    movd                m15, [base+subpel_filters+r4*8+2]
 %if ARCH_X86_64
    movd                 m7, [base+subpel_filters+r6*8+2]
 %else
    movd                 m7, [base+subpel_filters+r3*8+2]
 %endif
    pxor                 m9, m9
    pcmpeqd              m8, m9
    psrld               m14, 10
 %if ARCH_X86_32
    mov                  r3, r3m
    pshufb              m14, m5
    paddb               m14, m6
    mova         [esp+0x00], m14
  %define m14 [esp+0x00]
    SWAP                 m5, m0
    SWAP                 m6, m3
  %define m8  m5
  %define m15 m6
 %endif
    movq                 m0, [srcq+ssq*0]
    movq                 m2, [srcq+ssq*2]
    movhps               m0, [srcq+ssq*1]
    movhps               m2, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
 %if ARCH_X86_64
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    pshufb              m14, m5
    paddb               m14, m6
    movq                m10, r4
 %else
    mov                 myd, mym
    mov                  r5, [esp+0x1f4]
    xor                  r3, r3
    shr                 myd, 6
    lea                  r5, [r5+myd]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r5*8+0]
    cmovnz               r3, [base+subpel_filters+r5*8+4]
  %define m10 m4
    movd                m10, r4
    movd                 m3, r3
    mov                  r3, r3m
    punpckldq           m10, m3
 %endif
    movq                 m1, [srcq+ssq*0]
    movq                 m3, [srcq+ssq*2]
    movhps               m1, [srcq+ssq*1]
    add                srcq, ss3q
    punpcklbw           m10, m10
    psraw               m10, 8
    punpckldq           m15, m7
    punpcklqdq          m15, m15
 %if ARCH_X86_64
    pand                m11, m8
 %else
    pand                 m7, m11, m8
  %define m11 m7
 %endif
    pandn                m8, m15
    SWAP                m15, m8
    por                 m15, m11
 %if ARCH_X86_64
    pshufd               m8, m10, q0000
    pshufd               m9, m10, q1111
    pshufd              m11, m10, q3333
    pshufd              m10, m10, q2222
 %else
    mova         [esp+0x10], m15
  %define m15 [esp+0x10]
    mov                  r0, r0m
    pshufd               m5, m4, q0000
    pshufd               m6, m4, q1111
    pshufd               m7, m4, q2222
    pshufd               m4, m4, q3333
  %define m8  [esp+0x20]
  %define m9  [esp+0x30]
  %define m10 [esp+0x40]
  %define m11 [esp+0x50]
    mova                 m8, m5
    mova                 m9, m6
    mova                m10, m7
    mova                m11, m4
 %endif
    pshufb               m0, m14
    pshufb               m2, m14
    pshufb               m1, m14
    pshufb               m3, m14
    pmaddubsw            m0, m15
    pmaddubsw            m2, m15
    pmaddubsw            m1, m15
    pmaddubsw            m3, m15
    phaddw               m0, m2
    phaddw               m1, m3
    pmulhrsw             m0, m12
    pmulhrsw             m1, m12
    palignr              m2, m1, m0, 4
    pshufd               m4, m1, q2121
    punpcklwd            m3, m0, m2     ; 01 12
    punpckhwd            m0, m2         ; 23 34
    punpcklwd            m2, m1, m4     ; 45 56
.dy1_w2_loop:
    movq                 m1, [srcq+ssq*0]
    movhps               m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddwd              m5, m3, m8
    pmaddwd              m6, m0, m9
    pmaddwd              m7, m2, m10
    mova                 m3, m0
    mova                 m0, m2
    paddd                m5, m13
    paddd                m6, m7
    pshufb               m1, m14
    pmaddubsw            m1, m15
    phaddw               m1, m1
    pmulhrsw             m1, m12
    palignr              m7, m1, m4, 12
    punpcklwd            m2, m7, m1     ; 67 78
    pmaddwd              m7, m2, m11
    mova                 m4, m1
    paddd                m5, m6
    paddd                m5, m7
    psrad                m5, rndshift
    packssdw             m5, m5
    packuswb             m5, m5
    movd                r4d, m5
    mov        [dstq+dsq*0], r4w
    shr                 r4d, 16
    mov        [dstq+dsq*1], r4w
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .dy1_w2_loop
    RET
%endif
INIT_XMM ssse3
.dy1_w4:
%if ARCH_X86_64
    mov                 myd, mym
    movzx               t0d, t0b
    dec                srcq
    movd                m15, t0d
%else
 %define m10 [base+pd_0x3ff]
 %define m11 [base+pd_0x4000]
 %define m8  m0
 %xdefine m14 m4
 %define m15 m3
 %if isprep
  %define ssq r3
 %endif
    movzx                r4, byte [esp+0x1f0]
    dec                srcq
    movd                m15, r4
%endif
    pmaddwd              m8, [base+rescale_mul]
%if ARCH_X86_64
    mova                m11, [base+pd_0x4000]
%endif
    pshufd              m15, m15, q0000
    paddd               m14, m8 ; mx+dx*[0-3]
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd               m15, m8
    psrldq               m7, m15, 8
%if ARCH_X86_64
    movd                r4d, m15
    movd               r11d, m7
    psrldq              m15, 4
    psrldq               m7, 4
    movd                r6d, m15
    movd               r13d, m7
    movd                m15, [base+subpel_filters+ r4*8+2]
    movd                 m2, [base+subpel_filters+r11*8+2]
    movd                 m3, [base+subpel_filters+ r6*8+2]
    movd                 m4, [base+subpel_filters+r13*8+2]
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
%else
    movd                 r1, m15
    movd                 r3, m7
    psrldq              m15, 4
    psrldq               m7, 4
    movd                 r4, m15
    movd                 r5, m7
 %define m15 m5
    SWAP                 m4, m7
    movd                m15, [base+subpel_filters+r1*8+2]
    movd                 m2, [base+subpel_filters+r3*8+2]
    movd                 m3, [base+subpel_filters+r4*8+2]
    movd                 m4, [base+subpel_filters+r5*8+2]
    mov                 myd, mym
    mov                  rX, [esp+0x1f4]
    xor                  r5, r5
    shr                 myd, 6
    lea                  rX, [rX+myd]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+rX*8+0]
    cmovnz               r5, [base+subpel_filters+rX*8+4]
    mov                  r3, r3m
 %if isprep
    lea                ss3q, [ssq*3]
 %endif
%endif
    punpckldq           m15, m3
    punpckldq            m2, m4
    punpcklqdq          m15, m2
    movq                 m6, [base+subpel_s_shuf2]
%if ARCH_X86_64
    pcmpeqd              m8, m9
    psrld               m14, 10
    pshufb              m14, [base+bdct_lb_dw]
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*1]
    movu                 m2, [srcq+ssq*2]
    movu                 m3, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    punpcklqdq           m6, m6
    movu                 m4, [srcq+ssq*0]
    movu                 m5, [srcq+ssq*1]
    movu                 m7, [srcq+ssq*2]
    add                srcq, ss3q
    pand                m11, m8
    pandn                m8, m15
    SWAP                m15, m8
    por                 m15, m11
    paddb               m14, m6
    movq                m10, r4q
    punpcklbw           m10, m10
    psraw               m10, 8
    pshufb               m0, m14
    pshufb               m1, m14
    pshufb               m2, m14
    pshufb               m3, m14
    pshufb               m4, m14
    pshufb               m5, m14
    pshufb               m7, m14
    pmaddubsw            m0, m15
    pmaddubsw            m1, m15
    pmaddubsw            m2, m15
    pmaddubsw            m3, m15
    pmaddubsw            m4, m15
    pmaddubsw            m5, m15
    pmaddubsw            m7, m15
    phaddw               m0, m1
    phaddw               m2, m3
    phaddw               m4, m5
    phaddw               m6, m7, m7
    pmulhrsw             m0, m12    ; 0 1
    pmulhrsw             m2, m12    ; 2 3
    pmulhrsw             m4, m12    ; 4 5
    pmulhrsw             m6, m12    ; 6 _
    shufps               m1, m0, m2, q1032  ; 1 2
    shufps               m3, m2, m4, q1032  ; 3 4
    shufps               m5, m4, m6, q1032  ; 5 6
    punpcklwd            m7, m0, m1 ; 01
    punpckhwd            m0, m1     ; 12
    punpcklwd            m8, m2, m3 ; 23
    punpckhwd            m2, m3     ; 34
    punpcklwd            m9, m4, m5 ; 45
    punpckhwd            m4, m5     ; 56
%else
    pxor                 m3, m3
    pcmpeqd              m8, m3
    psrld               m14, 10
    pshufb              m14, [base+bdct_lb_dw]
    movu                 m1, [srcq+ssq*0]
    movu                 m2, [srcq+ssq*1]
    movu                 m3, [srcq+ssq*2]
    add                srcq, ss3q
    punpcklqdq           m6, m6
    SWAP                 m4, m7
    pand                 m7, m11, m8
    pandn                m8, m15
    SWAP                 m5, m0
    por                 m15, m7
    paddb               m14, m6
    movu                 m0, [srcq+ssq*0]
    movu                 m7, [srcq+ssq*1]
    movu                 m6, [srcq+ssq*2]
    pshufb               m1, m14
    pshufb               m2, m14
    pshufb               m3, m14
    pshufb               m0, m14
    pshufb               m7, m14
    pshufb               m6, m14
    pmaddubsw            m1, m15
    pmaddubsw            m2, m15
    pmaddubsw            m3, m15
    mova         [esp+0x00], m14
    mova         [esp+0x10], m15
    pmaddubsw            m0, m15
    pmaddubsw            m7, m15
    pmaddubsw            m6, m15
    phaddw               m1, m2
    movu                 m2, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    mov                  r0, r0m
    phaddw               m3, m0
    pshufb               m2, m14
    pmaddubsw            m2, m15
 %define m14 [esp+0x00]
 %define m15 [esp+0x10]
    phaddw               m7, m6
    phaddw               m2, m2
    movd                 m6, r4
    movd                 m0, r5
    punpckldq            m6, m0
    punpcklbw            m6, m6
    psraw                m6, 8
    mova         [esp+0x20], m6
    pmulhrsw             m1, m12 ; 0 1
    pmulhrsw             m3, m12 ; 2 3
    pmulhrsw             m7, m12 ; 4 5
    pmulhrsw             m2, m12 ; 6 _
    shufps               m0, m1, m3, q1032  ; 1 2
    shufps               m4, m3, m7, q1032  ; 3 4
    shufps               m5, m7, m2, q1032  ; 5 6
    punpcklwd            m6, m1, m0 ; 01
    punpckhwd            m1, m0     ; 12
    mova         [esp+0x30], m1
    punpcklwd            m1, m3, m4 ; 23
    punpckhwd            m3, m4     ; 34
    mova         [esp+0x40], m3
    punpcklwd            m3, m7, m5 ; 45
    punpckhwd            m7, m5     ; 56
    mova         [esp+0x50], m7
    mova         [esp+0x60], m2
    mova                 m0, [esp+0x20]
 %xdefine m8 m1
 %xdefine m9 m3
 %xdefine m10 m0
    SWAP                 m7, m6
    SWAP                 m1, m4
    SWAP                 m3, m2
%endif
    pshufd               m1, m10, q0000
    pshufd               m3, m10, q1111
    pshufd               m5, m10, q2222
    pshufd              m10, m10, q3333
%if ARCH_X86_64
    mova         [rsp+0x00], m8
    mova         [rsp+0x10], m2
    mova         [rsp+0x20], m9
    mova         [rsp+0x30], m4
%else
    mova         [esp+0x70], m8
    mova         [esp+0x80], m9
    mova         [esp+0x90], m1
    mova         [esp+0xa0], m3
    mova         [esp+0xb0], m5
    mova         [esp+0xc0], m10
 %ifidn %1, put
    mov                 dsd, dsm
 %endif
 %define m11 m6
%endif
.dy1_w4_loop:
%if ARCH_X86_64
    movu                m11, [srcq+ssq*0]
    pmaddwd              m7, m1
    pmaddwd              m8, m3
    pmaddwd              m0, m1
    pmaddwd              m2, m3
    pmaddwd              m9, m5
    pmaddwd              m4, m5
    paddd                m7, m8
    paddd                m0, m2
    movu                 m8, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb              m11, m14
    pmaddubsw           m11, m15
    paddd                m7, m13
    paddd                m0, m13
    paddd                m7, m9
    paddd                m0, m4
    pshufb               m8, m14
    pmaddubsw            m8, m15
    phaddw              m11, m8
    mova                 m8, [rsp+0x20]
    pmulhrsw            m11, m12
    punpcklwd            m9, m6, m11    ; 67
    psrldq               m6, m11, 8
    punpcklwd            m4, m11, m6    ; 78
    pmaddwd              m2, m9, m10
    pmaddwd             m11, m4, m10
    paddd                m7, m2
    mova                 m2, [rsp+0x30]
    paddd                m0, m11
%else
    SWAP                 m7, m6
    SWAP                 m1, m4
    SWAP                 m3, m2
    movu                 m5, [srcq+ssq*0]
    mova                 m0, [esp+0x30]
    mova                 m2, [esp+0x40]
    mova                 m4, [esp+0x50]
    pmaddwd              m6, [esp+0x90]
    pmaddwd              m1, [esp+0xa0]
    pmaddwd              m0, [esp+0x90]
    pmaddwd              m2, [esp+0xa0]
    pmaddwd              m3, [esp+0xb0]
    pmaddwd              m4, [esp+0xb0]
    paddd                m6, m1
    paddd                m0, m2
    movu                 m7, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb               m5, m14
    pmaddubsw            m5, m15
    paddd                m6, m13
    paddd                m0, m13
    paddd                m6, m3
    paddd                m0, m4
    pshufb               m7, m14
    pmaddubsw            m7, m15
    phaddw               m5, m7
    mova                 m7, [rsp+0x80]
    pmulhrsw             m5, m12
    punpcklwd            m3, [esp+0x60], m5 ; 67
    psrldq               m1, m5, 8
    punpcklwd            m4, m5, m1         ; 78
    pmaddwd              m2, m3, [esp+0xc0]
    pmaddwd              m5, m4, [esp+0xc0]
    mova         [esp+0x60], m1
    paddd                m6, m2
    mova                 m2, [esp+0x50]
    paddd                m0, m5
    SWAP                 m7, m6
%endif
    psrad                m7, rndshift
    psrad                m0, rndshift
    packssdw             m7, m0
%if ARCH_X86_64
    mova                 m0, [rsp+0x10]
%else
    mova                 m0, [esp+0x40]
%define m11 m5
%endif
%ifidn %1, put
    packuswb             m7, m7
    psrldq              m11, m7, 4
    movd       [dstq+dsq*0], m7
    movd       [dstq+dsq*1], m11
    lea                dstq, [dstq+dsq*2]
%else
    mova             [tmpq], m7
    add                tmpq, 16
%endif
    sub                  hd, 2
    jz .ret
%if ARCH_X86_64
    mova                 m7, [rsp+0x00]
    mova         [rsp+0x00], m8
    mova         [rsp+0x10], m2
    mova         [rsp+0x20], m9
    mova         [rsp+0x30], m4
%else
    mova                 m7, [esp+0x70] ; 01
    mova                 m1, [esp+0x80] ; 23
    mova                 m2, [esp+0x50] ; 34
    mova         [esp+0x30], m0
    mova         [esp+0x70], m1
    mova         [esp+0x40], m2
    mova         [esp+0x80], m3
    mova         [esp+0x50], m4
%endif
    jmp .dy1_w4_loop
INIT_XMM ssse3
.dy1_w8:
    mov    dword [rsp+0x90], 1
    movifprep   tmp_stridem, 16
    jmp .dy1_w_start
.dy1_w16:
    mov    dword [rsp+0x90], 2
    movifprep   tmp_stridem, 32
    jmp .dy1_w_start
.dy1_w32:
    mov    dword [rsp+0x90], 4
    movifprep   tmp_stridem, 64
    jmp .dy1_w_start
.dy1_w64:
    mov    dword [rsp+0x90], 8
    movifprep   tmp_stridem, 128
    jmp .dy1_w_start
.dy1_w128:
    mov    dword [rsp+0x90], 16
    movifprep   tmp_stridem, 256
.dy1_w_start:
    mov                 myd, mym
%ifidn %1, put
    movifnidn           dsm, dsq
%endif
%if ARCH_X86_64
    shr                 t0d, 16
    sub                srcq, 3
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    movd                m15, t0d
%else
 %define m8   m0
 %define m9   m1
 %xdefine m14 m4
 %xdefine m15 m3
 %if isprep
  %define ssq ssm
 %endif
    mov                  r5, [esp+0x1f0]
    mov                  r3, [esp+0x1f4]
    shr                  r5, 16
    sub                srcq, 3
    movd                m15, r5
    xor                  r5, r5
    shr                 myd, 6
    lea                  r3, [r3+myd]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r3*8+0]
    cmovnz               r5, [base+subpel_filters+r3*8+4]
    mov                  r0, r0m
    mov                  r3, r3m
%endif
    pslld                m7, m8, 2 ; dx*4
    pmaddwd              m8, [base+rescale_mul] ; dx*[0-3]
    pshufd              m15, m15, q0000
    paddd               m14, m8 ; mx+dx*[0-3]
%if ARCH_X86_64
    movq                 m3, r4q
    punpcklbw            m3, m3
    psraw                m3, 8
%else
    movd                 m5, r4
    movd                 m6, r5
    punpckldq            m5, m6
    punpcklbw            m5, m5
    psraw                m5, 8
    SWAP                 m3, m5
%endif
    mova        [rsp+0x100], m7
    mova        [rsp+0x120], m15
    mov         [rsp+0x098], srcq
    mov         [rsp+0x130], r0q ; dstq / tmpq
    pshufd               m0, m3, q0000
    pshufd               m1, m3, q1111
    pshufd               m2, m3, q2222
    pshufd               m3, m3, q3333
    mova        [rsp+0x140], m0
    mova        [rsp+0x150], m1
    mova        [rsp+0x160], m2
    mova        [rsp+0x170], m3
%if ARCH_X86_64 && UNIX64
    mov                  hm, hd
%elif ARCH_X86_32
    SWAP                  m5, m3
    mov                   r5, hm
    mov          [esp+0x134], r5
%endif
    jmp .dy1_hloop
.dy1_hloop_prep:
    dec   dword [rsp+0x090]
    jz .ret
%if ARCH_X86_64
    add   qword [rsp+0x130], 8*(isprep+1)
    mov                  hd, hm
%else
    add   dword [rsp+0x130], 8*(isprep+1)
    mov                  r5, [esp+0x134]
    mov                  r0, [esp+0x130]
%endif
    mova                 m7, [rsp+0x100]
    mova                m14, [rsp+0x110]
%if ARCH_X86_64
    mova                m10, [base+pd_0x3ff]
%else
 %define m10 [base+pd_0x3ff]
%endif
    mova                m15, [rsp+0x120]
    mov                srcq, [rsp+0x098]
%if ARCH_X86_64
    mov                 r0q, [rsp+0x130] ; dstq / tmpq
%else
    mov                  hm, r5
    mov                 r0m, r0
    mov                  r3, r3m
%endif
    paddd               m14, m7
.dy1_hloop:
    pxor                 m9, m9
%if ARCH_X86_64
    mova                m11, [base+pq_0x40000000]
%else
 %define m11 [base+pq_0x40000000]
%endif
    psrld                m2, m14, 10
    mova              [rsp], m2
    pand                 m6, m14, m10
    psrld                m6, 6
    paddd                m5, m15, m6
    pcmpeqd              m6, m9
    psrldq               m2, m5, 8
%if ARCH_X86_64
    movd                r4d, m5
    movd                r6d, m2
    psrldq               m5, 4
    psrldq               m2, 4
    movd                r7d, m5
    movd                r9d, m2
    movq                 m0, [base+subpel_filters+r4*8]
    movq                 m1, [base+subpel_filters+r6*8]
    movhps               m0, [base+subpel_filters+r7*8]
    movhps               m1, [base+subpel_filters+r9*8]
%else
    movd                 r0, m5
    movd                 rX, m2
    psrldq               m5, 4
    psrldq               m2, 4
    movd                 r4, m5
    movd                 r5, m2
    movq                 m0, [base+subpel_filters+r0*8]
    movq                 m1, [base+subpel_filters+rX*8]
    movhps               m0, [base+subpel_filters+r4*8]
    movhps               m1, [base+subpel_filters+r5*8]
    pxor                 m2, m2
 %define m9 m2
%endif
    paddd               m14, m7 ; mx+dx*[4-7]
    pand                 m5, m14, m10
    psrld                m5, 6
    paddd               m15, m5
    pcmpeqd              m5, m9
    mova        [rsp+0x110], m14
    psrldq               m4, m15, 8
%if ARCH_X86_64
    movd               r10d, m15
    movd               r11d, m4
    psrldq              m15, 4
    psrldq               m4, 4
    movd               r13d, m15
    movd                rXd, m4
    movq                 m2, [base+subpel_filters+r10*8]
    movq                 m3, [base+subpel_filters+r11*8]
    movhps               m2, [base+subpel_filters+r13*8]
    movhps               m3, [base+subpel_filters+ rX*8]
    psrld               m14, 10
    psrldq               m4, m14, 8
    movd               r10d, m14
    movd               r11d, m4
    psrldq              m14, 4
    psrldq               m4, 4
    movd               r13d, m14
    movd                rXd, m4
    mov                 r4d, [rsp+ 0]
    mov                 r6d, [rsp+ 8]
    mov                 r7d, [rsp+ 4]
    mov                 r9d, [rsp+12]
    pshufd               m4, m6, q1100
    pshufd               m6, m6, q3322
    pshufd               m7, m5, q1100
    pshufd               m5, m5, q3322
    pand                 m8, m11, m4
    pand                 m9, m11, m6
    pand                m15, m11, m7
    pand                m11, m11, m5
    pandn                m4, m0
    pandn                m6, m1
    pandn                m7, m2
    pandn                m5, m3
    por                  m8, m4
    por                  m9, m6
    por                 m15, m7
    por                 m11, m5
    mova         [rsp+0x10], m8
    mova         [rsp+0x20], m9
    mova         [rsp+0x30], m15
    mova         [rsp+0x40], m11
    MC_8TAP_SCALED_H 1, 2, 3, 4, 5, 6, 7, 10, 8, 9, 15, 11 ; 0-1
    mova         [rsp+0x50], m1
    mova         [rsp+0x60], m2
    MC_8TAP_SCALED_H 3, 4, 5, 6, 1, 2, 7, 10, 8, 9, 15, 11 ; 2-3
    mova         [rsp+0x70], m3
    mova         [rsp+0x80], m4
    MC_8TAP_SCALED_H 5, 6, 1, 2, 3, 4, 7, 10, 8, 9, 15, 11 ; 4-5
    MC_8TAP_SCALED_H 0,14, 1, 2, 3, 4, 7, 10, 8, 9, 15, 11 ; 6-7
    SWAP                 m7, m0
    SWAP                 m8, m14
    mova                 m1, [rsp+0x50]
    mova                 m2, [rsp+0x60]
    mova                 m3, [rsp+0x70]
    mova                m15, [rsp+0x80]
    punpcklwd            m4, m5, m6 ; 45a
    punpckhwd            m5, m6     ; 45b
    punpcklwd            m6, m7, m8 ; 67a
    punpckhwd            m7, m8     ; 67b
    SWAP                m14, m8
    mova                 m8, [rsp+0x140]
    mova                 m9, [rsp+0x150]
    mova                m10, [rsp+0x160]
    mova                m11, [rsp+0x170]
    punpcklwd            m0, m1, m2 ; 01a
    punpckhwd            m1, m2     ; 01b
    punpcklwd            m2, m3, m15; 23a
    punpckhwd            m3, m15    ; 23b
    mova         [rsp+0x50], m4
    mova         [rsp+0x60], m5
    mova         [rsp+0x70], m6
    mova         [rsp+0x80], m7
    mova                m14, [base+unpckw]
%else
    movd                 r0, m15
    movd                 rX, m4
    psrldq              m15, 4
    psrldq               m4, 4
    movd                 r4, m15
    movd                 r5, m4
    mova                m14, [esp+0x110]
    movq                 m2, [base+subpel_filters+r0*8]
    movq                 m3, [base+subpel_filters+rX*8]
    movhps               m2, [base+subpel_filters+r4*8]
    movhps               m3, [base+subpel_filters+r5*8]
    psrld               m14, 10
    mova           [esp+16], m14
    mov                  r0, [esp+ 0]
    mov                  rX, [esp+ 8]
    mov                  r4, [esp+ 4]
    mov                  r5, [esp+12]
    mova         [esp+0x20], m0
    mova         [esp+0x30], m1
    mova         [esp+0x40], m2
    mova         [esp+0x50], m3
    pshufd               m4, m6, q1100
    pshufd               m6, m6, q3322
    pshufd               m7, m5, q1100
    pshufd               m5, m5, q3322
    pand                 m0, m11, m4
    pand                 m1, m11, m6
    pand                 m2, m11, m7
    pand                 m3, m11, m5
    pandn                m4, [esp+0x20]
    pandn                m6, [esp+0x30]
    pandn                m7, [esp+0x40]
    pandn                m5, [esp+0x50]
    por                  m0, m4
    por                  m1, m6
    por                  m2, m7
    por                  m3, m5
    mova        [esp+0x20], m0
    mova        [esp+0x30], m1
    mova        [esp+0x40], m2
    mova        [esp+0x50], m3
    MC_8TAP_SCALED_H   0x20, 0x60, 0 ; 0-1
    MC_8TAP_SCALED_H   0x20, 0x180   ; 2-3
    MC_8TAP_SCALED_H   0x20, 0x1a0   ; 4-5
    MC_8TAP_SCALED_H   0x20, 0x1c0   ; 6-7
    mova                 m5, [esp+0x1a0]
    mova                 m6, [esp+0x1b0]
    mova                 m7, [esp+0x1c0]
    mova                 m0, [esp+0x1d0]
    punpcklwd            m4, m5, m6      ; 45a
    punpckhwd            m5, m6          ; 45b
    punpcklwd            m6, m7, m0      ; 67a
    punpckhwd            m7, m0          ; 67b
    mova        [esp+0x1a0], m4
    mova        [esp+0x1b0], m5
    mova        [esp+0x1c0], m6
    mova        [esp+0x1d0], m7
    mova                 m1, [esp+0x060]
    mova                 m2, [esp+0x070]
    mova                 m3, [esp+0x180]
    mova                 m4, [esp+0x190]
    punpcklwd            m0, m1, m2      ; 01a
    punpckhwd            m1, m2          ; 01b
    punpcklwd            m2, m3, m4      ; 23a
    punpckhwd            m3, m4          ; 23b
    mova        [esp+0x060], m0
    mova        [esp+0x070], m1
    mova        [esp+0x180], m2
    mova        [esp+0x190], m3
 %define m8  [esp+0x140]
 %define m9  [esp+0x150]
 %define m10 [esp+0x160]
 %define m11 [esp+0x170]
%endif
.dy1_vloop:
%if ARCH_X86_32
    mov                  r0, r0m
%endif
    pmaddwd              m4, m0, m8
    pmaddwd              m5, m1, m8
    pmaddwd              m6, m2, m9
    pmaddwd              m7, m3, m9
    paddd                m4, m13
    paddd                m5, m13
    paddd                m4, m6
    paddd                m5, m7
%if ARCH_X86_64
    pmaddwd              m6, [rsp+0x50], m10
    pmaddwd              m7, [rsp+0x60], m10
%else
    pmaddwd              m6, [rsp+0x1a0], m10
    pmaddwd              m7, [rsp+0x1b0], m10
%endif
    paddd                m4, m6
    paddd                m5, m7
%if ARCH_X86_64
    pmaddwd              m6, [rsp+0x70], m11
    pmaddwd              m7, [rsp+0x80], m11
%else
    pmaddwd              m6, [rsp+0x1c0], m11
    pmaddwd              m7, [rsp+0x1d0], m11
%endif
    paddd                m4, m6
    paddd                m5, m7
    psrad                m4, rndshift
    psrad                m5, rndshift
    packssdw             m4, m5
%ifidn %1, put
    packuswb             m4, m4
    movq             [dstq], m4
    add                dstq, dsm
%else
    mova             [tmpq], m4
    add                tmpq, tmp_stridem
%endif
%if ARCH_X86_32
    mov                 r0m, r0
%endif
    dec                  hd
    jz .dy1_hloop_prep
%if ARCH_X86_64
    movq                 m4, [srcq+ r4]
    movq                 m5, [srcq+ r6]
    movhps               m4, [srcq+ r7]
    movhps               m5, [srcq+ r9]
    movq                 m6, [srcq+r10]
    movq                 m7, [srcq+r11]
    movhps               m6, [srcq+r13]
    movhps               m7, [srcq+ rX]
    add                srcq, ssq
    pshufd              m15, m14, q1032
    pshufb               m0, m14                ; 0a 1a
    pshufb               m1, m14                ; 0b 1b
    pshufb               m2, m15                ; 3a 2a
    pshufb               m3, m15                ; 3b 2b
    pmaddubsw            m4, [rsp+0x10]
    pmaddubsw            m5, [rsp+0x20]
    pmaddubsw            m6, [rsp+0x30]
    pmaddubsw            m7, [rsp+0x40]
    phaddw               m4, m5
    phaddw               m6, m7
    phaddw               m4, m6
    pmulhrsw             m4, m12
    pshufb               m5, [rsp+0x70], m15    ; 7a 6a
    pshufb               m7, [rsp+0x80], m15    ; 7b 6b
    pshufb               m6, [rsp+0x50], m14    ; 4a 5a
    pshufb              m15, [rsp+0x60], m14    ; 4b 5b
    punpckhwd            m0, m2  ; 12a
    punpckhwd            m1, m3  ; 12b
    punpcklwd            m2, m6  ; 34a
    punpcklwd            m3, m15 ; 34b
    punpckhwd            m6, m5  ; 56a
    punpckhwd           m15, m7  ; 56b
    punpcklwd            m5, m4  ; 78a
    psrldq               m4, 8
    punpcklwd            m7, m4  ; 78b
    mova         [rsp+0x50], m6
    mova         [rsp+0x60], m15
    mova         [rsp+0x70], m5
    mova         [rsp+0x80], m7
%else
    mov                  r0, [esp+ 0]
    mov                  rX, [esp+ 8]
    mov                  r4, [esp+ 4]
    mov                  r5, [esp+12]
    mova                 m6, [base+unpckw]
    mova                 m0, [esp+0x060]
    mova                 m1, [esp+0x070]
    mova                 m7, [esp+0x1a0]
    movq                 m4, [srcq+r0]
    movq                 m5, [srcq+rX]
    movhps               m4, [srcq+r4]
    movhps               m5, [srcq+r5]
    pshufb               m0, m6         ; 0a 1a
    pshufb               m1, m6         ; 0b 1b
    pshufb               m7, m6         ; 4a 5a
    mov                  r0, [esp+16]
    mov                  rX, [esp+24]
    mov                  r4, [esp+20]
    mov                  r5, [esp+28]
    movq                 m3, [srcq+r0]
    movq                 m2, [srcq+rX]
    movhps               m3, [srcq+r4]
    movhps               m2, [srcq+r5]
    add                srcq, ssq
    pmaddubsw            m4, [esp+0x20]
    pmaddubsw            m5, [esp+0x30]
    pmaddubsw            m3, [esp+0x40]
    pmaddubsw            m2, [esp+0x50]
    phaddw               m4, m5
    phaddw               m3, m2
    mova                 m5, [esp+0x1b0]
    mova                 m2, [esp+0x180]
    phaddw               m4, m3
    mova                 m3, [esp+0x190]
    pmulhrsw             m4, m12        ; 8a 8b
    pshufb               m5, m6         ; 4b 5b
    pshufd               m6, m6, q1032
    pshufb               m2, m6         ; 3a 2a
    pshufb               m3, m6         ; 3b 2b
    punpckhwd            m0, m2         ; 12a
    punpckhwd            m1, m3         ; 12b
    mova         [esp+0x60], m0
    mova         [esp+0x70], m1
    mova                 m0, [esp+0x1c0]
    mova                 m1, [esp+0x1d0]
    punpcklwd            m2, m7         ; 34a
    punpcklwd            m3, m5         ; 34b
    mova        [esp+0x180], m2
    mova        [esp+0x190], m3
    pshufb               m0, m6         ; 7a 6a
    pshufb               m1, m6         ; 7b 6b
    punpckhwd            m7, m0         ; 56a
    punpckhwd            m5, m1         ; 56b
    punpcklwd            m0, m4
    punpckhqdq           m4, m4
    punpcklwd            m1, m4
    mova        [esp+0x1a0], m7
    mova        [esp+0x1b0], m5
    mova        [esp+0x1c0], m0
    mova        [esp+0x1d0], m1
    mova                 m0, [esp+0x60]
    mova                 m1, [esp+0x70]
%endif
    jmp .dy1_vloop
INIT_XMM ssse3
.dy2:
    movzx                wd, word [base+%1_8tap_scaled_ssse3_dy2_table+wq*2]
    add                  wq, base_reg
    jmp                  wq
%ifidn %1, put
.dy2_w2:
 %if ARCH_X86_64
    mov                 myd, mym
    movzx               t0d, t0b
    dec                srcq
    movd                m15, t0d
 %else
  %define m10 [base+pd_0x3ff]
  %define m11 [base+pd_0x4000]
  %define m8  m0
  %define m9  m1
  %define m14 m4
  %define m15 m3
    movzx                r5, byte [esp+0x1f0]
    dec                srcd
    movd                m15, r5
 %endif
    punpckldq            m9, m8
    SWAP                 m8, m9
    paddd               m14, m8 ; mx+dx*[0-1]
 %if ARCH_X86_64
    mova                m11, [base+pd_0x4000]
 %endif
    pshufd              m15, m15, q0000
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd               m15, m8
    movd                r4d, m15
    psrldq              m15, 4
 %if ARCH_X86_64
    movd                r6d, m15
 %else
    movd                r3d, m15
 %endif
    mova                 m5, [base+bdct_lb_dw]
    mova                 m6, [base+subpel_s_shuf2]
    movd                m15, [base+subpel_filters+r4*8+2]
 %if ARCH_X86_64
    movd                 m7, [base+subpel_filters+r6*8+2]
 %else
    movd                 m7, [base+subpel_filters+r3*8+2]
 %endif
    pxor                 m9, m9
    pcmpeqd              m8, m9
    psrld               m14, 10
 %if ARCH_X86_32
    mov                  r3, r3m
    pshufb              m14, m5
    paddb               m14, m6
    mova         [esp+0x00], m14
  %define m14 [esp+0x00]
    SWAP                 m5, m0
    SWAP                 m6, m3
  %define m8  m5
  %define m15 m6
 %endif
    movq                 m0, [srcq+ssq*0]
    movq                 m1, [srcq+ssq*1]
    movhps               m0, [srcq+ssq*2]
    movhps               m1, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
 %if ARCH_X86_64
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    pshufb              m14, m5
    paddb               m14, m6
    movq                m10, r4q
 %else
    mov                 myd, mym
    mov                  r3, [esp+0x1f4]
    xor                  r5, r5
    shr                 myd, 6
    lea                  r3, [r3+myd]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r3*8+0]
    cmovnz               r5, [base+subpel_filters+r3*8+4]
    mov                  r3, r3m
  %define m10 m4
    movd                m10, r4
    movd                 m3, r5
    punpckldq           m10, m3
 %endif
    movq                 m3, [srcq+ssq*0]
    movhps               m3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklbw           m10, m10
    psraw               m10, 8
    punpckldq           m15, m7
    punpcklqdq          m15, m15
 %if ARCH_X86_64
    pand                m11, m8
 %else
    pand                 m7, m11, m8
  %define m11 m7
 %endif
    pandn                m8, m15
    SWAP                m15, m8
    por                 m15, m11
 %if ARCH_X86_64
    pshufd               m8, m10, q0000
    pshufd               m9, m10, q1111
    pshufd              m11, m10, q3333
    pshufd              m10, m10, q2222
 %else
    mova         [esp+0x10], m15
  %define m15 [esp+0x10]
    mov                  r5, r0m
  %define dstq r5
    mov                 dsd, dsm
    pshufd               m5, m4, q0000
    pshufd               m6, m4, q1111
    pshufd               m7, m4, q2222
    pshufd               m4, m4, q3333
  %define m8  [esp+0x20]
  %define m9  [esp+0x30]
  %define m10 [esp+0x40]
  %define m11 [esp+0x50]
    mova                 m8, m5
    mova                 m9, m6
    mova                m10, m7
    mova                m11, m4
 %endif
    pshufb               m0, m14
    pshufb               m1, m14
    pshufb               m3, m14
    pmaddubsw            m0, m15
    pmaddubsw            m1, m15
    pmaddubsw            m3, m15
    pslldq               m2, m3, 8
    phaddw               m0, m2
    phaddw               m1, m3
    pmulhrsw             m0, m12            ; 0 2 _ 4
    pmulhrsw             m1, m12            ; 1 3 _ 5
    pshufd               m2, m0, q3110      ; 0 2 2 4
    pshufd               m1, m1, q3110      ; 1 3 3 5
    punpcklwd            m3, m2, m1         ; 01 23
    punpckhwd            m2, m1             ; 23 45
.dy2_w2_loop:
    movq                 m6, [srcq+ssq*0]
    movq                 m7, [srcq+ssq*1]
    movhps               m6, [srcq+ssq*2]
    movhps               m7, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    pmaddwd              m4, m3, m8
    pmaddwd              m5, m2, m9
    pshufb               m6, m14
    pshufb               m7, m14
    pmaddubsw            m6, m15
    pmaddubsw            m7, m15
    phaddw               m6, m7
    pmulhrsw             m6, m12
    psrldq               m7, m6, 8
    palignr              m6, m0, 8
    palignr              m7, m1, 8
    mova                 m0, m6
    mova                 m1, m7
    pshufd               m6, m6, q3221
    pshufd               m7, m7, q3221
    punpcklwd            m3, m6, m7       ; 45 67
    punpckhwd            m2, m6, m7       ; 67 89
    pmaddwd              m6, m3, m10
    pmaddwd              m7, m2, m11
    paddd                m4, m5
    paddd                m4, m13
    paddd                m6, m7
    paddd                m4, m6
    psrad                m4, rndshift
    packssdw             m4, m4
    packuswb             m4, m4
    movd                r4d, m4
    mov        [dstq+dsq*0], r4w
    shr                 r4d, 16
    mov        [dstq+dsq*1], r4w
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .dy2_w2_loop
    RET
%endif
INIT_XMM ssse3
.dy2_w4:
%if ARCH_X86_64
    mov                 myd, mym
    movzx               t0d, t0b
    dec                srcq
    movd                m15, t0d
%else
 %define m10 [base+pd_0x3ff]
 %define m11 [base+pd_0x4000]
 %define m8  m0
 %xdefine m14 m4
 %define m15 m3
 %define dstq r0
 %if isprep
  %define ssq r3
 %endif
    movzx                r4, byte [esp+0x1f0]
    dec                srcq
    movd                m15, r4
%endif
    pmaddwd              m8, [base+rescale_mul]
%if ARCH_X86_64
    mova                m11, [base+pd_0x4000]
%endif
    pshufd              m15, m15, q0000
    paddd               m14, m8 ; mx+dx*[0-3]
    pand                 m8, m14, m10
    psrld                m8, 6
    paddd               m15, m8
    psrldq               m7, m15, 8
%if ARCH_X86_64
    movd                r4d, m15
    movd               r11d, m7
    psrldq              m15, 4
    psrldq               m7, 4
    movd                r6d, m15
    movd               r13d, m7
    movd                m15, [base+subpel_filters+ r4*8+2]
    movd                 m2, [base+subpel_filters+r11*8+2]
    movd                 m3, [base+subpel_filters+ r6*8+2]
    movd                 m4, [base+subpel_filters+r13*8+2]
    movq                 m6, [base+subpel_s_shuf2]
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
%else
    movd                 r1, m15
    movd                 r3, m7
    psrldq              m15, 4
    psrldq               m7, 4
    movd                 r4, m15
    movd                 r5, m7
 %define m15 m5
    SWAP                 m4, m7
    movd                m15, [base+subpel_filters+r1*8+2]
    movd                 m2, [base+subpel_filters+r3*8+2]
    movd                 m3, [base+subpel_filters+r4*8+2]
    movd                 m4, [base+subpel_filters+r5*8+2]
    movq                 m6, [base+subpel_s_shuf2]
    mov                 myd, mym
    mov                  r3, [esp+0x1f4]
    xor                  r5, r5
    shr                 myd, 6
    lea                  r3, [r3+myd]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r3*8+0]
    cmovnz               r5, [base+subpel_filters+r3*8+4]
    mov                  r3, r3m
 %if isprep
    lea                ss3q, [ssq*3]
 %endif
%endif
    punpckldq           m15, m3
    punpckldq            m2, m4
    punpcklqdq          m15, m2
%if ARCH_X86_64
    pcmpeqd              m8, m9
    psrld               m14, 10
    movu                 m0, [srcq+ssq*0]
    movu                 m2, [srcq+ssq*2]
    movu                 m1, [srcq+ssq*1]
    movu                 m3, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    punpcklqdq           m6, m6
    pshufb              m14, [base+bdct_lb_dw]
    movu                 m4, [srcq+ssq*0]
    movu                 m5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pand                m11, m8
    pandn                m8, m15
    SWAP                m15, m8
    por                 m15, m11
    paddb               m14, m6
    movq                m11, r4q
    punpcklbw           m11, m11
    psraw               m11, 8
    pshufb               m0, m14
    pshufb               m2, m14
    pshufb               m1, m14
    pshufb               m3, m14
    pshufb               m4, m14
    pshufb               m5, m14
    pmaddubsw            m0, m15
    pmaddubsw            m2, m15
    pmaddubsw            m1, m15
    pmaddubsw            m3, m15
    pmaddubsw            m4, m15
    pmaddubsw            m5, m15
    phaddw               m0, m2
    phaddw               m1, m3
    phaddw               m4, m5
    pmulhrsw             m0, m12    ; 0 2
    pmulhrsw             m1, m12    ; 1 3
    pmulhrsw             m4, m12    ; 4 5
    pshufd               m8, m11, q0000
    pshufd               m9, m11, q1111
    pshufd              m10, m11, q2222
    pshufd              m11, m11, q3333
%else
    pxor                 m3, m3
    pcmpeqd              m8, m3
    psrld               m14, 10
    pshufb              m14, [base+bdct_lb_dw]
    movu                 m1, [srcq+ssq*0]
    movu                 m2, [srcq+ssq*2]
    movu                 m3, [srcq+ssq*1]
    add                srcq, ss3q
    punpcklqdq           m6, m6
    SWAP                 m4, m7
    pand                 m7, m11, m8
    pandn                m8, m15
    SWAP                m15, m8
    por                 m15, m7
    paddb               m14, m6
    movu                 m0, [srcq+ssq*0]
    movu                 m7, [srcq+ssq*1]
    movu                 m6, [srcq+ssq*2]
    add                srcq, ss3q
    pshufb               m1, m14
    pshufb               m2, m14
    pshufb               m3, m14
    pshufb               m0, m14
    pshufb               m7, m14
    pshufb               m6, m14
    pmaddubsw            m1, m15
    pmaddubsw            m2, m15
    pmaddubsw            m3, m15
    mova         [esp+0x00], m14
    mova         [esp+0x10], m15
    pmaddubsw            m0, m15
    pmaddubsw            m7, m15
    pmaddubsw            m6, m15
 %define m14 [esp+0x00]
 %define m15 [esp+0x10]
    phaddw               m1, m2
    phaddw               m3, m0
    phaddw               m7, m6
 %ifidn %1, put
    mov                 dsd, dsm
  %define dstq r5
 %else
  %define tmpq r5
 %endif
    movd                 m6, r4
    movd                 m0, r5
    punpckldq            m6, m0
    punpcklbw            m6, m6
    psraw                m6, 8
    mov                  r5, r0m
    pmulhrsw             m1, m12 ; 0 2
    pmulhrsw             m3, m12 ; 1 3
    pmulhrsw             m7, m12 ; 4 5
    SWAP                 m0, m1, m3
    SWAP                 m4, m7
    pshufd               m2, m6, q0000
    pshufd               m3, m6, q1111
    pshufd               m7, m6, q2222
    pshufd               m6, m6, q3333
    mova         [esp+0x30], m2
    mova         [esp+0x40], m3
    mova         [esp+0x50], m7
    mova         [esp+0x60], m6
 %define m8  [esp+0x30]
 %define m9  [esp+0x40]
 %define m10 [esp+0x50]
 %define m11 [esp+0x60]
%endif
    psrldq               m5, m4, 8  ; 5 _
    punpckhwd            m2, m0, m1 ; 23
    punpcklwd            m0, m1     ; 01
    punpcklwd            m4, m5     ; 45
.dy2_w4_loop:
    pmaddwd              m0, m8         ; a0
    pmaddwd              m5, m2, m8     ; b0
    pmaddwd              m2, m9         ; a1
    pmaddwd              m7, m4, m9     ; b1
    pmaddwd              m3, m4, m10    ; a2
    paddd                m0, m13
    paddd                m5, m13
    paddd                m0, m2
    paddd                m5, m7
    paddd                m0, m3
    movu                 m6, [srcq+ssq*0]
    movu                 m7, [srcq+ssq*1]
    movu                 m3, [srcq+ssq*2]
    movu                 m1, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    pshufb               m6, m14
    pshufb               m7, m14
    pshufb               m3, m14
    pshufb               m1, m14
    pmaddubsw            m6, m15
    pmaddubsw            m7, m15
    pmaddubsw            m3, m15
    pmaddubsw            m1, m15
    phaddw               m6, m7
    phaddw               m3, m1
    pmulhrsw             m6, m12    ; 6 7
    pmulhrsw             m3, m12    ; 8 9
    psrldq               m7, m6, 8
    psrldq               m1, m3, 8
    punpcklwd            m6, m7     ; 67
    punpcklwd            m3, m1     ; 89
    mova                 m2, m6
    pmaddwd              m1, m6, m10    ; b2
    pmaddwd              m6, m11        ; a3
    pmaddwd              m7, m3, m11    ; b3
    paddd                m5, m1
    paddd                m0, m6
    paddd                m5, m7
    psrad                m0, rndshift
    psrad                m5, rndshift
    packssdw             m0, m5
%ifidn %1, put
    packuswb             m0, m0
    psrldq               m1, m0, 4
    movd       [dstq+dsq*0], m0
    movd       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
%else
    mova             [tmpq], m0
    add                tmpq, 16
%endif
    mova                 m0, m4
    mova                 m4, m3
    sub                  hd, 2
    jg .dy2_w4_loop
    MC_8TAP_SCALED_RET
INIT_XMM ssse3
.dy2_w8:
    mov    dword [rsp+0x90], 1
    movifprep   tmp_stridem, 16
    jmp .dy2_w_start
.dy2_w16:
    mov    dword [rsp+0x90], 2
    movifprep   tmp_stridem, 32
    jmp .dy2_w_start
.dy2_w32:
    mov    dword [rsp+0x90], 4
    movifprep   tmp_stridem, 64
    jmp .dy2_w_start
.dy2_w64:
    mov    dword [rsp+0x90], 8
    movifprep   tmp_stridem, 128
    jmp .dy2_w_start
.dy2_w128:
    mov    dword [rsp+0x90], 16
    movifprep   tmp_stridem, 256
.dy2_w_start:
    mov                 myd, mym
%ifidn %1, put
    movifnidn           dsm, dsq
%endif
%if ARCH_X86_64
    shr                 t0d, 16
    sub                srcq, 3
    shr                 myd, 6
    mov                 r4d, 64 << 24
    lea                 myd, [t1+myq]
    cmovnz              r4q, [base+subpel_filters+myq*8]
    movd                m15, t0d
%else
 %define m10 [base+pd_0x3ff]
 %define m11 [base+pd_0x4000]
 %define m8   m0
 %define m9   m1
 %xdefine m14 m4
 %xdefine m15 m3
 %if isprep
  %define tmpq r0
  %define ssq ssm
 %else
  %define dstq r0
 %endif
    mov                  r5, [esp+0x1f0]
    mov                  r3, [esp+0x1f4]
    shr                  r5, 16
    sub                srcq, 3
    movd                m15, r5
    xor                  r5, r5
    shr                 myd, 6
    lea                  r3, [r3+myd]
    mov                  r4, 64 << 24
    cmovnz               r4, [base+subpel_filters+r3*8+0]
    cmovnz               r5, [base+subpel_filters+r3*8+4]
    mov                  r0, r0m
    mov                  r3, r3m
%endif
    pslld                m7, m8, 2 ; dx*4
    pmaddwd              m8, [base+rescale_mul] ; dx*[0-3]
    pshufd              m15, m15, q0000
    paddd               m14, m8 ; mx+dx*[0-3]
%if ARCH_X86_64
    movq                 m3, r4q
    punpcklbw            m3, m3
    psraw                m3, 8
%else
    movd                 m5, r4
    movd                 m6, r5
    punpckldq            m5, m6
    punpcklbw            m5, m5
    psraw                m5, 8
    SWAP                 m3, m5
%endif
    mova        [rsp+0x100], m7
    mova        [rsp+0x120], m15
    mov         [rsp+0x098], srcq
    mov         [rsp+0x130], r0q ; dstq / tmpq
    pshufd               m0, m3, q0000
    pshufd               m1, m3, q1111
    pshufd               m2, m3, q2222
    pshufd               m3, m3, q3333
    mova        [rsp+0x140], m0
    mova        [rsp+0x150], m1
    mova        [rsp+0x160], m2
    mova        [rsp+0x170], m3
%if ARCH_X86_64 && UNIX64
    mov                  hm, hd
%elif ARCH_X86_32
    SWAP                  m5, m3
    mov                   r5, hm
    mov          [esp+0x134], r5
%endif
    jmp .dy2_hloop
.dy2_hloop_prep:
    dec   dword [rsp+0x090]
    jz .ret
%if ARCH_X86_64
    add   qword [rsp+0x130], 8*(isprep+1)
    mov                  hd, hm
%else
    add   dword [rsp+0x130], 8*(isprep+1)
    mov                  r5, [esp+0x134]
    mov                  r0, [esp+0x130]
%endif
    mova                 m7, [rsp+0x100]
    mova                m14, [rsp+0x110]
%if ARCH_X86_64
    mova                m10, [base+pd_0x3ff]
%else
 %define m10 [base+pd_0x3ff]
%endif
    mova                m15, [rsp+0x120]
    mov                srcq, [rsp+0x098]
%if ARCH_X86_64
    mov                 r0q, [rsp+0x130] ; dstq / tmpq
%else
    mov                  hm, r5
    mov                 r0m, r0
    mov                  r3, r3m
%endif
    paddd               m14, m7
.dy2_hloop:
    pxor                 m9, m9
%if ARCH_X86_64
    mova                m11, [base+pq_0x40000000]
%else
 %define m11 [base+pq_0x40000000]
%endif
    psrld                m2, m14, 10
    mova              [rsp], m2
    pand                 m6, m14, m10
    psrld                m6, 6
    paddd                m5, m15, m6
    pcmpeqd              m6, m9
    psrldq               m2, m5, 8
%if ARCH_X86_64
    movd                r4d, m5
    movd                r6d, m2
    psrldq               m5, 4
    psrldq               m2, 4
    movd                r7d, m5
    movd                r9d, m2
    movq                 m0, [base+subpel_filters+r4*8]
    movq                 m1, [base+subpel_filters+r6*8]
    movhps               m0, [base+subpel_filters+r7*8]
    movhps               m1, [base+subpel_filters+r9*8]
%else
    movd                 r0, m5
    movd                 rX, m2
    psrldq               m5, 4
    psrldq               m2, 4
    movd                 r4, m5
    movd                 r5, m2
    movq                 m0, [base+subpel_filters+r0*8]
    movq                 m1, [base+subpel_filters+rX*8]
    movhps               m0, [base+subpel_filters+r4*8]
    movhps               m1, [base+subpel_filters+r5*8]
    pxor                 m2, m2
 %define m9 m2
%endif
    paddd               m14, m7 ; mx+dx*[4-7]
    pand                 m5, m14, m10
    psrld                m5, 6
    paddd               m15, m5
    pcmpeqd              m5, m9
    mova        [rsp+0x110], m14
    psrldq               m4, m15, 8
%if ARCH_X86_64
    movd               r10d, m15
    movd               r11d, m4
    psrldq              m15, 4
    psrldq               m4, 4
    movd               r13d, m15
    movd                rXd, m4
    movq                 m2, [base+subpel_filters+r10*8]
    movq                 m3, [base+subpel_filters+r11*8]
    movhps               m2, [base+subpel_filters+r13*8]
    movhps               m3, [base+subpel_filters+ rX*8]
    psrld               m14, 10
    psrldq               m4, m14, 8
    movd               r10d, m14
    movd               r11d, m4
    psrldq              m14, 4
    psrldq               m4, 4
    movd               r13d, m14
    movd                rXd, m4
    mov                 r4d, [rsp+ 0]
    mov                 r6d, [rsp+ 8]
    mov                 r7d, [rsp+ 4]
    mov                 r9d, [rsp+12]
    pshufd               m4, m6, q1100
    pshufd               m6, m6, q3322
    pshufd               m7, m5, q1100
    pshufd               m5, m5, q3322
    pand                 m8, m11, m4
    pand                 m9, m11, m6
    pand                m15, m11, m7
    pand                m11, m11, m5
    pandn                m4, m0
    pandn                m6, m1
    pandn                m7, m2
    pandn                m5, m3
    por                  m8, m4
    por                  m9, m6
    por                 m15, m7
    por                 m11, m5
    mova         [rsp+0x10], m8
    mova         [rsp+0x20], m9
    mova         [rsp+0x30], m15
    mova         [rsp+0x40], m11
    MC_8TAP_SCALED_H 1, 2, 3, 4, 5, 6, 7, 10, 8, 9, 15, 11 ; 0-1
    mova         [rsp+0x50], m1
    mova         [rsp+0x60], m2
    MC_8TAP_SCALED_H 3, 4, 5, 6, 1, 2, 7, 10, 8, 9, 15, 11 ; 2-3
    mova         [rsp+0x70], m3
    mova         [rsp+0x80], m4
    MC_8TAP_SCALED_H 5, 6, 1, 2, 3, 4, 7, 10, 8, 9, 15, 11 ; 4-5
    MC_8TAP_SCALED_H 0,14, 1, 2, 3, 4, 7, 10, 8, 9, 15, 11 ; 6-7
    SWAP                 m7, m0
    SWAP                 m8, m14
    mova                 m1, [rsp+0x50]
    mova                 m2, [rsp+0x60]
    mova                 m3, [rsp+0x70]
    mova                m15, [rsp+0x80]
    punpcklwd            m4, m5, m6 ; 45a
    punpckhwd            m5, m6     ; 45b
    punpcklwd            m6, m7, m8 ; 67a
    punpckhwd            m7, m8     ; 67b
    SWAP                m14, m8
    mova                 m8, [rsp+0x140]
    mova                 m9, [rsp+0x150]
    mova                m10, [rsp+0x160]
    mova                m11, [rsp+0x170]
    punpcklwd            m0, m1, m2 ; 01a
    punpckhwd            m1, m2     ; 01b
    punpcklwd            m2, m3, m15; 23a
    punpckhwd            m3, m15    ; 23b
    mova         [rsp+0x50], m4
    mova         [rsp+0x60], m5
    mova         [rsp+0x70], m6
    mova         [rsp+0x80], m7
%else
    movd                 r0, m15
    movd                 rX, m4
    psrldq              m15, 4
    psrldq               m4, 4
    movd                 r4, m15
    movd                 r5, m4
    mova                m14, [esp+0x110]
    movq                 m2, [base+subpel_filters+r0*8]
    movq                 m3, [base+subpel_filters+rX*8]
    movhps               m2, [base+subpel_filters+r4*8]
    movhps               m3, [base+subpel_filters+r5*8]
    psrld               m14, 10
    mova           [esp+16], m14
    mov                  r0, [esp+ 0]
    mov                  rX, [esp+ 8]
    mov                  r4, [esp+ 4]
    mov                  r5, [esp+12]
    mova         [esp+0x20], m0
    mova         [esp+0x30], m1
    mova         [esp+0x40], m2
    mova         [esp+0x50], m3
    pshufd               m4, m6, q1100
    pshufd               m6, m6, q3322
    pshufd               m7, m5, q1100
    pshufd               m5, m5, q3322
    pand                 m0, m11, m4
    pand                 m1, m11, m6
    pand                 m2, m11, m7
    pand                 m3, m11, m5
    pandn                m4, [esp+0x20]
    pandn                m6, [esp+0x30]
    pandn                m7, [esp+0x40]
    pandn                m5, [esp+0x50]
    por                  m0, m4
    por                  m1, m6
    por                  m2, m7
    por                  m3, m5
    mova        [esp+0x20], m0
    mova        [esp+0x30], m1
    mova        [esp+0x40], m2
    mova        [esp+0x50], m3
    MC_8TAP_SCALED_H   0x20, 0x60, 0 ; 0-1
    MC_8TAP_SCALED_H   0x20, 0x180   ; 2-3
    MC_8TAP_SCALED_H   0x20, 0x1a0   ; 4-5
    MC_8TAP_SCALED_H   0x20, 0x1c0   ; 6-7
    mova                 m5, [esp+0x1a0]
    mova                 m6, [esp+0x1b0]
    mova                 m7, [esp+0x1c0]
    mova                 m0, [esp+0x1d0]
    punpcklwd            m4, m5, m6      ; 45a
    punpckhwd            m5, m6          ; 45b
    punpcklwd            m6, m7, m0      ; 67a
    punpckhwd            m7, m0          ; 67b
    mova        [esp+0x1a0], m4
    mova        [esp+0x1b0], m5
    mova        [esp+0x1c0], m6
    mova        [esp+0x1d0], m7
    mova                 m1, [esp+0x060]
    mova                 m2, [esp+0x070]
    mova                 m3, [esp+0x180]
    mova                 m4, [esp+0x190]
    punpcklwd            m0, m1, m2      ; 01a
    punpckhwd            m1, m2          ; 01b
    punpcklwd            m2, m3, m4      ; 23a
    punpckhwd            m3, m4          ; 23b
    mova        [esp+0x180], m2
    mova        [esp+0x190], m3
 %define m8  [esp+0x140]
 %define m9  [esp+0x150]
 %define m10 [esp+0x160]
 %define m11 [esp+0x170]
%endif
.dy2_vloop:
%if ARCH_X86_32
    mov                  r0, r0m
%endif
    pmaddwd              m4, m0, m8
    pmaddwd              m5, m1, m8
    pmaddwd              m6, m2, m9
    pmaddwd              m7, m3, m9
    paddd                m4, m13
    paddd                m5, m13
    paddd                m4, m6
    paddd                m5, m7
%if ARCH_X86_64
    pmaddwd              m6, [rsp+0x50], m10
    pmaddwd              m7, [rsp+0x60], m10
%else
    pmaddwd              m6, [esp+0x1a0], m10
    pmaddwd              m7, [esp+0x1b0], m10
%endif
    paddd                m4, m6
    paddd                m5, m7
%if ARCH_X86_64
    pmaddwd              m6, [rsp+0x70], m11
    pmaddwd              m7, [rsp+0x80], m11
%else
    pmaddwd              m6, [esp+0x1c0], m11
    pmaddwd              m7, [esp+0x1d0], m11
%endif
    paddd                m4, m6
    paddd                m5, m7
    psrad                m4, rndshift
    psrad                m5, rndshift
    packssdw             m4, m5
%ifidn %1, put
    packuswb             m4, m4
    movq             [dstq], m4
    add                dstq, dsm
%else
    mova             [tmpq], m4
    add                tmpq, tmp_stridem
%endif
%if ARCH_X86_32
    mov                 r0m, r0
%endif
    dec                  hd
    jz .dy2_hloop_prep
%if ARCH_X86_64
    mova                 m8, [rsp+0x10]
    mova                 m9, [rsp+0x20]
    mova                m10, [rsp+0x30]
    mova                m11, [rsp+0x40]
    mova                 m0, m2             ; 01a
    mova                 m1, m3             ; 01b
    MC_8TAP_SCALED_H 2, 6, 3, 4, 5, 7, 14, 15, 8, 9, 10, 11
    mova                 m3, [rsp+0x50] ; 23a
    mova                 m4, [rsp+0x60] ; 23b
    mova                 m5, [rsp+0x70] ; 45a
    mova                 m7, [rsp+0x80] ; 45b
    mova                 m8, [rsp+0x140]
    mova                 m9, [rsp+0x150]
    mova                m10, [rsp+0x160]
    mova                m11, [rsp+0x170]
    punpcklwd           m14, m2, m6     ; 67a
    punpckhwd            m2, m6         ; 67b
    mova         [rsp+0x50], m5
    mova         [rsp+0x60], m7
    mova         [rsp+0x70], m14
    mova         [rsp+0x80], m2
    mova                 m2, m3
    mova                 m3, m4
%else
    MC_8TAP_SCALED_H   0x20, 0
    punpcklwd            m6, m0, m4
    punpckhwd            m7, m0, m4
    mova                 m0, [esp+0x180] ; 01a
    mova                 m1, [esp+0x190] ; 01b
    mova                 m2, [rsp+0x1a0]  ; 23a
    mova                 m3, [esp+0x1b0]  ; 23b
    mova                 m4, [esp+0x1c0]  ; 45a
    mova                 m5, [esp+0x1d0]  ; 45b
    mova        [esp+0x180], m2
    mova        [esp+0x190], m3
    mova        [esp+0x1a0], m4
    mova        [esp+0x1b0], m5
    mova        [esp+0x1c0], m6          ; 67a
    mova        [esp+0x1d0], m7          ; 67b
%endif
    jmp .dy2_vloop
.ret:
    MC_8TAP_SCALED_RET 0
%if ARCH_X86_32 && !isprep && required_stack_alignment > STACK_ALIGNMENT
 %define r0m [rstk+stack_offset+ 4]
 %define r1m [rstk+stack_offset+ 8]
 %define r2m [rstk+stack_offset+12]
 %define r3m [rstk+stack_offset+16]
%endif
%undef isprep
%endmacro

%macro BILIN_SCALED_FN 1
cglobal %1_bilin_scaled_8bpc
    mov                 t0d, (5*15 << 16) | 5*15
    mov                 t1d, (5*15 << 16) | 5*15
    jmp mangle(private_prefix %+ _%1_8tap_scaled_8bpc %+ SUFFIX)
%endmacro

%if WIN64
DECLARE_REG_TMP 6, 5
%elif ARCH_X86_64
DECLARE_REG_TMP 6, 8
%else
DECLARE_REG_TMP 1, 2
%endif
BILIN_SCALED_FN put
FN put_8tap_scaled, sharp,          SHARP,   SHARP
FN put_8tap_scaled, sharp_smooth,   SHARP,   SMOOTH
FN put_8tap_scaled, smooth_sharp,   SMOOTH,  SHARP
FN put_8tap_scaled, smooth,         SMOOTH,  SMOOTH
FN put_8tap_scaled, sharp_regular,  SHARP,   REGULAR
FN put_8tap_scaled, regular_sharp,  REGULAR, SHARP
FN put_8tap_scaled, smooth_regular, SMOOTH,  REGULAR
FN put_8tap_scaled, regular_smooth, REGULAR, SMOOTH
FN put_8tap_scaled, regular,        REGULAR, REGULAR
MC_8TAP_SCALED put

%if WIN64
DECLARE_REG_TMP 5, 4
%elif ARCH_X86_64
DECLARE_REG_TMP 6, 7
%else
DECLARE_REG_TMP 1, 2
%endif
BILIN_SCALED_FN prep
FN prep_8tap_scaled, sharp,          SHARP,   SHARP
FN prep_8tap_scaled, sharp_smooth,   SHARP,   SMOOTH
FN prep_8tap_scaled, smooth_sharp,   SMOOTH,  SHARP
FN prep_8tap_scaled, smooth,         SMOOTH,  SMOOTH
FN prep_8tap_scaled, sharp_regular,  SHARP,   REGULAR
FN prep_8tap_scaled, regular_sharp,  REGULAR, SHARP
FN prep_8tap_scaled, smooth_regular, SMOOTH,  REGULAR
FN prep_8tap_scaled, regular_smooth, REGULAR, SMOOTH
FN prep_8tap_scaled, regular,        REGULAR, REGULAR
MC_8TAP_SCALED prep

%if ARCH_X86_32
 %macro SAVE_ALPHA_BETA 0
    mov              alpham, alphad
    mov               betam, betad
 %endmacro

 %macro SAVE_DELTA_GAMMA 0
    mov              deltam, deltad
    mov              gammam, gammad
 %endmacro

 %macro LOAD_ALPHA_BETA_MX 0
    mov                 mym, myd
    mov              alphad, alpham
    mov               betad, betam
    mov                 mxd, mxm
 %endmacro

 %macro LOAD_DELTA_GAMMA_MY 0
    mov                 mxm, mxd
    mov              deltad, deltam
    mov              gammad, gammam
    mov                 myd, mym
 %endmacro

 %define PIC_reg r2
 %define PIC_base_offset $$
 %define PIC_sym(sym) (PIC_reg+(sym)-PIC_base_offset)
%else
 %define SAVE_ALPHA_BETA
 %define SAVE_DELTA_GAMMA
 %define PIC_sym(sym) sym
%endif

%if ARCH_X86_32
 %if STACK_ALIGNMENT < required_stack_alignment
  %assign copy_args 8*4
 %else
  %assign copy_args 0
 %endif
%endif

%macro RELOC_ARGS 0
 %if copy_args
    mov                  r0, r0m
    mov                  r1, r1m
    mov                  r2, r2m
    mov                  r3, r3m
    mov                  r5, r5m
    mov                dstm, r0
    mov                 dsm, r1
    mov                srcm, r2
    mov                 ssm, r3
    mov                 mxm, r5
    mov                  r0, r6m
    mov                 mym, r0
 %endif
%endmacro

%macro BLENDHWDW 2 ; blend high words from dwords, src1, src2
 %if cpuflag(sse4)
    pblendw              %1, %2, 0xAA
 %else
    pand                 %2, m10
    por                  %1, %2
 %endif
%endmacro

%macro WARP_V 10 ; dst0, dst1, 0, 2, 4, 6, 1, 3, 5, 7
 %if ARCH_X86_32
  %define m8  m4
  %define m9  m5
  %define m14 m6
  %define m15 m7
  %define m11 m7
 %endif
 %if notcpuflag(ssse3) || ARCH_X86_32
    pxor                m11, m11
 %endif
    lea               tmp1d, [myq+deltaq*4]
    lea               tmp2d, [myq+deltaq*1]
    shr                 myd, 10
    shr               tmp1d, 10
    movq                 m2, [filterq+myq  *8] ; a
    movq                 m8, [filterq+tmp1q*8] ; e
    lea               tmp1d, [tmp2q+deltaq*4]
    lea                 myd, [tmp2q+deltaq*1]
    shr               tmp2d, 10
    shr               tmp1d, 10
    movq                 m3, [filterq+tmp2q*8] ; b
    movq                 m0, [filterq+tmp1q*8] ; f
    punpcklwd            m2, m3
    punpcklwd            m8, m0
    lea               tmp1d, [myq+deltaq*4]
    lea               tmp2d, [myq+deltaq*1]
    shr                 myd, 10
    shr               tmp1d, 10
    movq                 m0, [filterq+myq  *8] ; c
    movq                 m9, [filterq+tmp1q*8] ; g
    lea               tmp1d, [tmp2q+deltaq*4]
    lea                 myd, [tmp2q+gammaq]       ; my += gamma
    shr               tmp2d, 10
    shr               tmp1d, 10
    movq                 m3, [filterq+tmp2q*8] ; d
    movq                 m1, [filterq+tmp1q*8] ; h
    punpcklwd            m0, m3
    punpcklwd            m9, m1
    punpckldq            m1, m2, m0
    punpckhdq            m2, m0
    punpcklbw            m0, m11, m1 ; a0 a2 b0 b2 c0 c2 d0 d2 << 8
    punpckhbw            m3, m11, m1 ; a4 a6 b4 b6 c4 c6 d4 d6 << 8
    punpcklbw            m1, m11, m2 ; a1 a3 b1 b3 c1 c3 d1 d3 << 8
    punpckhbw           m14, m11, m2 ; a5 a7 b5 b7 c5 c7 d5 d7 << 8
    pmaddwd              m0, %3
    pmaddwd              m3, %5
    pmaddwd              m1, %7
    pmaddwd             m14, %9
    paddd                m0, m3
    paddd                m1, m14
    paddd                m0, m1
    mova                 %1, m0
 %if ARCH_X86_64
    SWAP                 m3, m14
 %endif
    punpckldq            m0, m8, m9
    punpckhdq            m8, m9
    punpcklbw            m1, m11, m0 ; e0 e2 f0 f2 g0 g2 h0 h2 << 8
    punpckhbw           m14, m11, m0 ; e4 e6 f4 f6 g4 g6 h4 h6 << 8
    punpcklbw            m2, m11, m8 ; e1 e3 f1 f3 g1 g3 h1 h3 << 8
    punpckhbw           m15, m11, m8 ; e5 e7 f5 f7 g5 g7 h5 h7 << 8
    pmaddwd              m1, %4
    pmaddwd             m14, %6
    pmaddwd              m2, %8
    pmaddwd             m15, %10
    paddd                m1, m14
    paddd                m2, m15
    paddd                m1, m2
    mova                 %2, m1
 %if ARCH_X86_64
    SWAP                m14, m3
 %endif
%endmacro

%if ARCH_X86_64
 %define counterd r4d
%else
 %if copy_args == 0
  %define counterd dword r4m
 %else
  %define counterd dword [esp+stack_size-4*7]
 %endif
%endif

%macro WARP_AFFINE_8X8T 0
%if ARCH_X86_64
cglobal warp_affine_8x8t_8bpc, 6, 14, 16, 0x90, tmp, ts
%else
cglobal warp_affine_8x8t_8bpc, 0, 7, 16, -0x130-copy_args, tmp, ts
 %if copy_args
  %define tmpm [esp+stack_size-4*1]
  %define tsm  [esp+stack_size-4*2]
 %endif
%endif
    call mangle(private_prefix %+ _warp_affine_8x8_8bpc_%+cpuname).main
.loop:
%if ARCH_X86_32
 %define m12 m4
 %define m13 m5
 %define m14 m6
 %define m15 m7
    mova                m12, [esp+0xC0]
    mova                m13, [esp+0xD0]
    mova                m14, [esp+0xE0]
    mova                m15, [esp+0xF0]
%endif
%if cpuflag(ssse3)
    psrad               m12, 13
    psrad               m13, 13
    psrad               m14, 13
    psrad               m15, 13
    packssdw            m12, m13
    packssdw            m14, m15
    mova                m13, [PIC_sym(pw_8192)]
    pmulhrsw            m12, m13 ; (x + (1 << 6)) >> 7
    pmulhrsw            m14, m13
%else
 %if ARCH_X86_32
  %define m10 m0
 %endif
    mova                m10, [PIC_sym(pd_16384)]
    paddd               m12, m10
    paddd               m13, m10
    paddd               m14, m10
    paddd               m15, m10
    psrad               m12, 15
    psrad               m13, 15
    psrad               m14, 15
    psrad               m15, 15
    packssdw            m12, m13
    packssdw            m14, m15
%endif
    mova       [tmpq+tsq*0], m12
    mova       [tmpq+tsq*2], m14
    dec            counterd
    jz   mangle(private_prefix %+ _warp_affine_8x8_8bpc_%+cpuname).end
%if ARCH_X86_32
    mov                tmpm, tmpd
    mov                  r0, [esp+0x100]
    mov                  r1, [esp+0x104]
%endif
    call mangle(private_prefix %+ _warp_affine_8x8_8bpc_%+cpuname).main2
    lea                tmpq, [tmpq+tsq*4]
    jmp .loop
%endmacro

%macro WARP_AFFINE_8X8 0
%if ARCH_X86_64
cglobal warp_affine_8x8_8bpc, 6, 14, 16, 0x90, \
                              dst, ds, src, ss, abcd, mx, tmp2, alpha, beta, \
                              filter, tmp1, delta, my, gamma
%else
cglobal warp_affine_8x8_8bpc, 0, 7, 16, -0x130-copy_args, \
                              dst, ds, src, ss, abcd, mx, tmp2, alpha, beta, \
                              filter, tmp1, delta, my, gamma
 %define alphaq     r0
 %define alphad     r0
 %define alpham     [esp+gprsize+0x100]
 %define betaq      r1
 %define betad      r1
 %define betam      [esp+gprsize+0x104]
 %define deltaq     r0
 %define deltad     r0
 %define deltam     [esp+gprsize+0x108]
 %define gammaq     r1
 %define gammad     r1
 %define gammam     [esp+gprsize+0x10C]
 %define filterq    r3
 %define tmp1q      r4
 %define tmp1d      r4
 %define tmp1m      [esp+gprsize+0x110]
 %define myq        r5
 %define myd        r5
 %define mym        r6m
 %if copy_args
  %define dstm [esp+stack_size-4*1]
  %define dsm  [esp+stack_size-4*2]
  %define srcm [esp+stack_size-4*3]
  %define ssm  [esp+stack_size-4*4]
  %define mxm  [esp+stack_size-4*5]
  %define mym  [esp+stack_size-4*6]
 %endif
%endif
    call .main
    jmp .start
.loop:
%if ARCH_X86_32
    mov                dstm, dstd
    mov              alphad, [esp+0x100]
    mov               betad, [esp+0x104]
%endif
    call .main2
    lea                dstq, [dstq+dsq*2]
.start:
%if notcpuflag(sse4)
 %if cpuflag(ssse3)
  %define roundval pw_8192
 %else
  %define roundval pd_262144
 %endif
 %if ARCH_X86_64
    mova                m10, [PIC_sym(roundval)]
 %else
  %define m10 [PIC_sym(roundval)]
 %endif
%endif
%if ARCH_X86_32
 %define m12 m5
 %define m13 m6
    mova                m12, [esp+0xC0]
    mova                m13, [esp+0xD0]
%endif
%if cpuflag(sse4)
 %if ARCH_X86_32
  %define m11 m4
    pxor                m11, m11
 %endif
    psrad               m12, 18
    psrad               m13, 18
    packusdw            m12, m13
    pavgw               m12, m11 ; (x + (1 << 10)) >> 11
%else
 %if cpuflag(ssse3)
    psrad               m12, 17
    psrad               m13, 17
    packssdw            m12, m13
    pmulhrsw            m12, m10
 %else
    paddd               m12, m10
    paddd               m13, m10
    psrad               m12, 19
    psrad               m13, 19
    packssdw            m12, m13
 %endif
%endif
%if ARCH_X86_32
 %define m14 m6
 %define m15 m7
    mova                m14, [esp+0xE0]
    mova                m15, [esp+0xF0]
%endif
%if cpuflag(sse4)
    psrad               m14, 18
    psrad               m15, 18
    packusdw            m14, m15
    pavgw               m14, m11 ; (x + (1 << 10)) >> 11
%else
 %if cpuflag(ssse3)
    psrad               m14, 17
    psrad               m15, 17
    packssdw            m14, m15
    pmulhrsw            m14, m10
 %else
    paddd               m14, m10
    paddd               m15, m10
    psrad               m14, 19
    psrad               m15, 19
    packssdw            m14, m15
 %endif
%endif
    packuswb            m12, m14
    movq       [dstq+dsq*0], m12
    movhps     [dstq+dsq*1], m12
    dec            counterd
    jg .loop
.end:
    RET
ALIGN function_align
.main:
%assign stack_offset stack_offset+gprsize
%if ARCH_X86_32
 %assign stack_size stack_size+4
 %if copy_args
  %assign stack_offset stack_offset-4
 %endif
    RELOC_ARGS
    LEA             PIC_reg, $$
 %define PIC_mem [esp+gprsize+0x114]
    mov               abcdd, abcdm
 %if copy_args == 0
    mov                 ssd, ssm
    mov                 mxd, mxm
 %endif
    mov             PIC_mem, PIC_reg
    mov                srcd, srcm
%endif
    movsx            deltad, word [abcdq+2*2]
    movsx            gammad, word [abcdq+2*3]
    lea               tmp1d, [deltaq*3]
    sub              gammad, tmp1d    ; gamma -= delta*3
    SAVE_DELTA_GAMMA
%if ARCH_X86_32
    mov               abcdd, abcdm
%endif
    movsx            alphad, word [abcdq+2*0]
    movsx             betad, word [abcdq+2*1]
    lea               tmp1q, [ssq*3+3]
    add                 mxd, 512+(64<<10)
    lea               tmp2d, [alphaq*3]
    sub                srcq, tmp1q    ; src -= src_stride*3 + 3
%if ARCH_X86_32
    mov                srcm, srcd
    mov             PIC_reg, PIC_mem
%endif
    sub               betad, tmp2d    ; beta -= alpha*3
    lea             filterq, [PIC_sym(mc_warp_filter2)]
%if ARCH_X86_64
    mov                 myd, r6m
 %if cpuflag(ssse3)
    pxor                m11, m11
 %endif
%endif
    call .h
    psrld                m2, m0, 16
    psrld                m3, m1, 16
%if ARCH_X86_32
 %if notcpuflag(ssse3)
    mova [esp+gprsize+0x00], m2
 %endif
    mova [esp+gprsize+0x10], m3
%endif
    call .h
    psrld                m4, m0, 16
    psrld                m5, m1, 16
%if ARCH_X86_32
    mova [esp+gprsize+0x20], m4
    mova [esp+gprsize+0x30], m5
%endif
    call .h
%if ARCH_X86_64
 %define blendmask [rsp+gprsize+0x80]
%else
 %if notcpuflag(ssse3)
    mova                 m2, [esp+gprsize+0x00]
 %endif
    mova                 m3, [esp+gprsize+0x10]
 %define blendmask [esp+gprsize+0x120]
 %define m10 m7
%endif
    pcmpeqd             m10, m10
    pslld               m10, 16
    mova          blendmask, m10
    BLENDHWDW            m2, m0 ; 0
    BLENDHWDW            m3, m1 ; 2
    mova [rsp+gprsize+0x00], m2
    mova [rsp+gprsize+0x10], m3
    call .h
%if ARCH_X86_32
    mova                 m4, [esp+gprsize+0x20]
    mova                 m5, [esp+gprsize+0x30]
%endif
    mova                m10, blendmask
    BLENDHWDW            m4, m0 ; 1
    BLENDHWDW            m5, m1 ; 3
    mova [rsp+gprsize+0x20], m4
    mova [rsp+gprsize+0x30], m5
    call .h
%if ARCH_X86_32
 %if notcpuflag(ssse3)
    mova                 m2, [esp+gprsize+0x00]
 %endif
    mova                 m3, [esp+gprsize+0x10]
 %define m10 m5
%endif
    psrld                m6, m2, 16
    psrld                m7, m3, 16
    mova                m10, blendmask
    BLENDHWDW            m6, m0 ; 2
    BLENDHWDW            m7, m1 ; 4
    mova [rsp+gprsize+0x40], m6
    mova [rsp+gprsize+0x50], m7
    call .h
%if ARCH_X86_32
    mova                m4, [esp+gprsize+0x20]
    mova                m5, [esp+gprsize+0x30]
%endif
    psrld               m2, m4, 16
    psrld               m3, m5, 16
    mova                m10, blendmask
    BLENDHWDW           m2, m0 ; 3
    BLENDHWDW           m3, m1 ; 5
    mova [rsp+gprsize+0x60], m2
    mova [rsp+gprsize+0x70], m3
    call .h
%if ARCH_X86_32
    mova                 m6, [esp+gprsize+0x40]
    mova                 m7, [esp+gprsize+0x50]
 %define m10 m7
%endif
    psrld                m4, m6, 16
    psrld                m5, m7, 16
    mova                m10, blendmask
    BLENDHWDW            m4, m0 ; 4
    BLENDHWDW            m5, m1 ; 6
%if ARCH_X86_64
    add                 myd, 512+(64<<10)
    mova                 m6, m2
    mova                 m7, m3
%else
    mova [esp+gprsize+0x80], m4
    mova [esp+gprsize+0x90], m5
    add           dword mym, 512+(64<<10)
%endif
    mov            counterd, 4
    SAVE_ALPHA_BETA
.main2:
    call .h
%if ARCH_X86_32
    mova                 m6, [esp+gprsize+0x60]
    mova                 m7, [esp+gprsize+0x70]
 %define m10 m5
%endif
    psrld                m6, 16
    psrld                m7, 16
    mova                m10, blendmask
    BLENDHWDW            m6, m0 ; 5
    BLENDHWDW            m7, m1 ; 7
%if ARCH_X86_64
    WARP_V              m12, m13, [rsp+gprsize+0x00], [rsp+gprsize+0x10], \
                                  m4, m5, \
                                  [rsp+gprsize+0x20], [rsp+gprsize+0x30], \
                                  m6, m7
%else
    mova [esp+gprsize+0xA0], m6
    mova [esp+gprsize+0xB0], m7
    LOAD_DELTA_GAMMA_MY
    WARP_V [esp+gprsize+0xC0], [esp+gprsize+0xD0], \
           [esp+gprsize+0x00], [esp+gprsize+0x10], \
           [esp+gprsize+0x80], [esp+gprsize+0x90], \
           [esp+gprsize+0x20], [esp+gprsize+0x30], \
           [esp+gprsize+0xA0], [esp+gprsize+0xB0]
    LOAD_ALPHA_BETA_MX
%endif
    call .h
    mova                 m2, [rsp+gprsize+0x40]
    mova                 m3, [rsp+gprsize+0x50]
%if ARCH_X86_32
    mova                 m4, [rsp+gprsize+0x80]
    mova                 m5, [rsp+gprsize+0x90]
 %define m10 m7
%endif
    mova [rsp+gprsize+0x00], m2
    mova [rsp+gprsize+0x10], m3
    mova [rsp+gprsize+0x40], m4
    mova [rsp+gprsize+0x50], m5
    psrld                m4, 16
    psrld                m5, 16
    mova                m10, blendmask
    BLENDHWDW            m4, m0 ; 6
    BLENDHWDW            m5, m1 ; 8
%if ARCH_X86_64
    WARP_V              m14, m15, [rsp+gprsize+0x20], [rsp+gprsize+0x30], \
                                  m6, m7, \
                                  [rsp+gprsize+0x00], [rsp+gprsize+0x10], \
                                  m4, m5
%else
    mova [esp+gprsize+0x80], m4
    mova [esp+gprsize+0x90], m5
    LOAD_DELTA_GAMMA_MY
    WARP_V [esp+gprsize+0xE0], [esp+gprsize+0xF0], \
           [esp+gprsize+0x20], [esp+gprsize+0x30], \
           [esp+gprsize+0xA0], [esp+gprsize+0xB0], \
           [esp+gprsize+0x00], [esp+gprsize+0x10], \
           [esp+gprsize+0x80], [esp+gprsize+0x90]
    mov                 mym, myd
    mov                dstd, dstm
    mov                 dsd, dsm
    mov                 mxd, mxm
%endif
    mova                 m2, [rsp+gprsize+0x60]
    mova                 m3, [rsp+gprsize+0x70]
%if ARCH_X86_32
    mova                 m6, [esp+gprsize+0xA0]
    mova                 m7, [esp+gprsize+0xB0]
%endif
    mova [rsp+gprsize+0x20], m2
    mova [rsp+gprsize+0x30], m3
    mova [rsp+gprsize+0x60], m6
    mova [rsp+gprsize+0x70], m7
    ret
ALIGN function_align
.h:
%if ARCH_X86_32
 %define m8  m3
 %define m9  m4
 %define m10 m5
 %define m14 m6
 %define m15 m7
%endif
    lea               tmp1d, [mxq+alphaq*4]
    lea               tmp2d, [mxq+alphaq*1]
%if ARCH_X86_32
 %assign stack_offset stack_offset+4
 %assign stack_size stack_size+4
 %define PIC_mem [esp+gprsize*2+0x114]
    mov             PIC_mem, PIC_reg
    mov                srcd, srcm
%endif
    movu                m10, [srcq]
%if ARCH_X86_32
    add                srcd, ssm
    mov                srcm, srcd
    mov             PIC_reg, PIC_mem
%else
    add                srcq, ssq
%endif
    shr                 mxd, 10
    shr               tmp1d, 10
    movq                 m1, [filterq+mxq  *8]  ; 0 X
    movq                 m8, [filterq+tmp1q*8]  ; 4 X
    lea               tmp1d, [tmp2q+alphaq*4]
    lea                 mxd, [tmp2q+alphaq*1]
    shr               tmp2d, 10
    shr               tmp1d, 10
    movhps               m1, [filterq+tmp2q*8]  ; 0 1
    movhps               m8, [filterq+tmp1q*8]  ; 4 5
    lea               tmp1d, [mxq+alphaq*4]
    lea               tmp2d, [mxq+alphaq*1]
    shr                 mxd, 10
    shr               tmp1d, 10
%if cpuflag(ssse3)
    movq                m14, [filterq+mxq  *8]  ; 2 X
    movq                 m9, [filterq+tmp1q*8]  ; 6 X
    lea               tmp1d, [tmp2q+alphaq*4]
    lea                 mxd, [tmp2q+betaq]  ; mx += beta
    shr               tmp2d, 10
    shr               tmp1d, 10
    movhps              m14, [filterq+tmp2q*8]  ; 2 3
    movhps               m9, [filterq+tmp1q*8]  ; 6 7
    pshufb               m0, m10, [PIC_sym(warp_8x8_shufA)]
    pmaddubsw            m0, m1
    pshufb               m1, m10, [PIC_sym(warp_8x8_shufB)]
    pmaddubsw            m1, m8
    pshufb              m15, m10, [PIC_sym(warp_8x8_shufC)]
    pmaddubsw           m15, m14
    pshufb              m10, m10, [PIC_sym(warp_8x8_shufD)]
    pmaddubsw           m10, m9
    phaddw               m0, m15
    phaddw               m1, m10
%else
 %if ARCH_X86_32
  %define m11 m2
 %endif
    pcmpeqw              m0, m0
    psrlw               m14, m0, 8
    psrlw               m15, m10, 8     ; 01 03 05 07  09 11 13 15
    pand                m14, m10        ; 00 02 04 06  08 10 12 14
    packuswb            m14, m15        ; 00 02 04 06  08 10 12 14  01 03 05 07  09 11 13 15
    psrldq               m9, m0, 4
    pshufd               m0, m14, q0220
    pand                 m0, m9
    psrldq              m14, 1          ; 02 04 06 08  10 12 14 01  03 05 07 09  11 13 15 __
    pslldq              m15, m14, 12
    por                  m0, m15    ; shufA
    psrlw               m15, m0, 8
    psraw               m11, m1, 8
    psllw                m0, 8
    psllw                m1, 8
    psrlw                m0, 8
    psraw                m1, 8
    pmullw              m15, m11
    pmullw               m0, m1
    paddw                m0, m15    ; pmaddubsw m0, m1
    pshufd              m15, m14, q0220
    pand                m15, m9
    psrldq              m14, 1          ; 04 06 08 10  12 14 01 03  05 07 09 11  13 15 __ __
    pslldq               m1, m14, 12
    por                 m15, m1     ; shufC
    pshufd               m1, m14, q0220
    pand                 m1, m9
    psrldq              m14, 1          ; 06 08 10 12  14 01 03 05  07 09 11 13  15 __ __ __
    pslldq              m11, m14, 12
    por                  m1, m11    ; shufB
    pshufd              m10, m14, q0220
    pand                m10, m9
    psrldq              m14, 1          ; 08 10 12 14  01 03 05 07  09 11 13 15  __ __ __ __
    pslldq              m14, m14, 12
    por                 m10, m14    ; shufD
    psrlw                m9, m1, 8
    psraw               m11, m8, 8
    psllw                m1, 8
    psllw                m8, 8
    psrlw                m1, 8
    psraw                m8, 8
    pmullw               m9, m11
    pmullw               m1, m8
    paddw                m1, m9     ; pmaddubsw m1, m8
    movq                m14, [filterq+mxq  *8]  ; 2 X
    movq                 m9, [filterq+tmp1q*8]  ; 6 X
    lea               tmp1d, [tmp2q+alphaq*4]
    lea                 mxd, [tmp2q+betaq]  ; mx += beta
    shr               tmp2d, 10
    shr               tmp1d, 10
    movhps              m14, [filterq+tmp2q*8]  ; 2 3
    movhps               m9, [filterq+tmp1q*8]  ; 6 7
    psrlw                m8, m15, 8
    psraw               m11, m14, 8
    psllw               m15, 8
    psllw               m14, 8
    psrlw               m15, 8
    psraw               m14, 8
    pmullw               m8, m11
    pmullw              m15, m14
    paddw               m15, m8     ; pmaddubsw m15, m14
    psrlw                m8, m10, 8
    psraw               m11, m9, 8
    psllw               m10, 8
    psllw                m9, 8
    psrlw               m10, 8
    psraw                m9, 8
    pmullw               m8, m11
    pmullw              m10, m9
    paddw               m10, m8     ; pmaddubsw m10, m9
    pslld                m8, m0, 16
    pslld                m9, m1, 16
    pslld               m14, m15, 16
    pslld               m11, m10, 16
    paddw                m0, m8
    paddw                m1, m9
    paddw               m15, m14
    paddw               m10, m11
    psrad                m0, 16
    psrad                m1, 16
    psrad               m15, 16
    psrad               m10, 16
    packssdw             m0, m15    ; phaddw m0, m15
    packssdw             m1, m10    ; phaddw m1, m10
%endif
    mova                m14, [PIC_sym(pw_8192)]
    mova                 m9, [PIC_sym(pd_32768)]
    pmaddwd              m0, m14 ; 17-bit intermediate, upshifted by 13
    pmaddwd              m1, m14
    paddd                m0, m9  ; rounded 14-bit result in upper 16 bits of dword
    paddd                m1, m9
    ret
%endmacro

%if WIN64
DECLARE_REG_TMP 6, 4
%else
DECLARE_REG_TMP 6, 7
%endif

%macro BIDIR_FN 1 ; op
    %1                    0
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4_loop:
    %1_INC_PTR            2
    %1                    0
    lea                dstq, [dstq+strideq*4]
.w4: ; tile 4x
    movd   [dstq          ], m0      ; copy dw[0]
    pshuflw              m1, m0, q1032 ; swap dw[1] and dw[0]
    movd   [dstq+strideq*1], m1      ; copy dw[1]
    punpckhqdq           m0, m0      ; swap dw[3,2] with dw[1,0]
    movd   [dstq+strideq*2], m0      ; dw[2]
    psrlq                m0, 32      ; shift right in dw[3]
    movd   [dstq+stride3q ], m0      ; copy
    sub                  hd, 4
    jg .w4_loop
    RET
.w8_loop:
    %1_INC_PTR            2
    %1                    0
    lea                dstq, [dstq+strideq*2]
.w8:
    movq   [dstq          ], m0
    movhps [dstq+strideq*1], m0
    sub                  hd, 2
    jg .w8_loop
    RET
.w16_loop:
    %1_INC_PTR            2
    %1                    0
    lea                dstq, [dstq+strideq]
.w16:
    mova   [dstq          ], m0
    dec                  hd
    jg .w16_loop
    RET
.w32_loop:
    %1_INC_PTR            4
    %1                    0
    lea                dstq, [dstq+strideq]
.w32:
    mova   [dstq          ], m0
    %1                    2
    mova   [dstq + 16     ], m0
    dec                  hd
    jg .w32_loop
    RET
.w64_loop:
    %1_INC_PTR            8
    %1                    0
    add                dstq, strideq
.w64:
    %assign i 0
    %rep 4
    mova   [dstq + i*16   ], m0
    %assign i i+1
    %if i < 4
    %1                    2*i
    %endif
    %endrep
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    %1_INC_PTR            16
    %1                    0
    add                dstq, strideq
.w128:
    %assign i 0
    %rep 8
    mova   [dstq + i*16   ], m0
    %assign i i+1
    %if i < 8
    %1                    2*i
    %endif
    %endrep
    dec                  hd
    jg .w128_loop
    RET
%endmacro

%macro AVG 1 ; src_offset
    ; writes AVG of tmp1 tmp2 uint16 coeffs into uint8 pixel
    mova                 m0, [tmp1q+(%1+0)*mmsize] ; load 8 coef(2bytes) from tmp1
    paddw                m0, [tmp2q+(%1+0)*mmsize] ; load/add 8 coef(2bytes) tmp2
    mova                 m1, [tmp1q+(%1+1)*mmsize]
    paddw                m1, [tmp2q+(%1+1)*mmsize]
    pmulhrsw             m0, m2
    pmulhrsw             m1, m2
    packuswb             m0, m1 ; pack/trunc 16 bits from m0 & m1 to 8 bit
%endmacro

%macro AVG_INC_PTR 1
    add               tmp1q, %1*mmsize
    add               tmp2q, %1*mmsize
%endmacro

cglobal avg_8bpc, 4, 7, 3, dst, stride, tmp1, tmp2, w, h, stride3
    LEA                  r6, avg_ssse3_table
    tzcnt                wd, wm ; leading zeros
    movifnidn            hd, hm ; move h(stack) to h(register) if not already that register
    movsxd               wq, dword [r6+wq*4] ; push table entry matching the tile width (tzcnt) in widen reg
    mova                 m2, [pw_1024+r6-avg_ssse3_table] ; fill m2 with shift/align
    add                  wq, r6
    BIDIR_FN            AVG

%macro W_AVG 1 ; src_offset
    ; (a * weight + b * (16 - weight) + 128) >> 8
    ; = ((a - b) * weight + (b << 4) + 128) >> 8
    ; = ((((a - b) * ((weight-16) << 12)) >> 16) + a + 8) >> 4
    ; = ((((b - a) * (-weight     << 12)) >> 16) + b + 8) >> 4
    mova                 m2, [tmp1q+(%1+0)*mmsize]
    mova                 m0, m2
    psubw                m2, [tmp2q+(%1+0)*mmsize]
    mova                 m3, [tmp1q+(%1+1)*mmsize]
    mova                 m1, m3
    psubw                m3, [tmp2q+(%1+1)*mmsize]
    pmulhw               m2, m4
    pmulhw               m3, m4
    paddw                m0, m2
    paddw                m1, m3
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
%endmacro

%define W_AVG_INC_PTR AVG_INC_PTR

cglobal w_avg_8bpc, 4, 7, 6, dst, stride, tmp1, tmp2, w, h, stride3
    LEA                  r6, w_avg_ssse3_table
    tzcnt                wd, wm
    movd                 m4, r6m
    movifnidn            hd, hm
    pxor                 m0, m0
    movsxd               wq, dword [r6+wq*4]
    mova                 m5, [pw_2048+r6-w_avg_ssse3_table]
    pshufb               m4, m0
    psllw                m4, 12 ; (weight-16) << 12 when interpreted as signed
    add                  wq, r6
    cmp           dword r6m, 7
    jg .weight_gt7
    mov                  r6, tmp1q
    psubw                m0, m4
    mov               tmp1q, tmp2q
    mova                 m4, m0 ; -weight
    mov               tmp2q, r6
.weight_gt7:
    BIDIR_FN          W_AVG

%macro MASK 1 ; src_offset
    ; (a * m + b * (64 - m) + 512) >> 10
    ; = ((a - b) * m + (b << 6) + 512) >> 10
    ; = ((((b - a) * (-m << 10)) >> 16) + b + 8) >> 4
    mova                 m3,     [maskq+(%1+0)*(mmsize/2)]
    mova                 m0,     [tmp2q+(%1+0)*mmsize] ; b
    psubw                m1, m0, [tmp1q+(%1+0)*mmsize] ; b - a
    mova                 m6, m3      ; m
    psubb                m3, m4, m6  ; -m
    paddw                m1, m1     ; (b - a) << 1
    paddb                m3, m3     ; -m << 1
    punpcklbw            m2, m4, m3 ; -m << 9 (<< 8 when ext as uint16)
    pmulhw               m1, m2     ; (-m * (b - a)) << 10
    paddw                m0, m1     ; + b
    mova                 m1,     [tmp2q+(%1+1)*mmsize] ; b
    psubw                m2, m1, [tmp1q+(%1+1)*mmsize] ; b - a
    paddw                m2, m2  ; (b - a) << 1
    mova                 m6, m3  ; (-m << 1)
    punpckhbw            m3, m4, m6 ; (-m << 9)
    pmulhw               m2, m3 ; (-m << 9)
    paddw                m1, m2 ; (-m * (b - a)) << 10
    pmulhrsw             m0, m5 ; round
    pmulhrsw             m1, m5 ; round
    packuswb             m0, m1 ; interleave 16 -> 8
%endmacro

%macro MASK_INC_PTR 1
    add               maskq, %1*mmsize/2
    add               tmp1q, %1*mmsize
    add               tmp2q, %1*mmsize
%endmacro

%if ARCH_X86_64
cglobal mask_8bpc, 4, 8, 7, dst, stride, tmp1, tmp2, w, h, mask, stride3
    movifnidn            hd, hm
%else
cglobal mask_8bpc, 4, 7, 7, dst, stride, tmp1, tmp2, w, mask, stride3
%define hd dword r5m
%endif
%define base r6-mask_ssse3_table
    LEA                  r6, mask_ssse3_table
    tzcnt                wd, wm
    movsxd               wq, dword [r6+wq*4]
    pxor                 m4, m4
    mova                 m5, [base+pw_2048]
    add                  wq, r6
    mov               maskq, r6m
    BIDIR_FN           MASK
%undef hd

%macro W_MASK_420_END 1-*
%rep %0
    call .main
    paddw                m2, [maskq+16*%1]
    mova      [maskq+16*%1], m2
    mova [dstq+strideq*1+16*(2*%1+0)], m0
    call .main
    psubw                m3, m7, m2
    psubw                m1, m7, [maskq+16*%1]
    psubw                m3, [dstq+strideq*1+16*(2*%1+1)]
    psrlw                m1, 2
    psrlw                m3, 2
    packuswb             m1, m3
    mova      [maskq+16*%1], m1
    mova [dstq+strideq*1+16*(2*%1+1)], m0
    %rotate 1
%endrep
%endmacro

%if UNIX64
DECLARE_REG_TMP 7
%else
DECLARE_REG_TMP 5
%endif

cglobal w_mask_420_8bpc, 4, 7, 9, dst, stride, tmp1, tmp2, w, h, mask
%define base t0-w_mask_420_ssse3_table
    LEA                  t0, w_mask_420_ssse3_table
    tzcnt                wd, wm
    mov                 r6d, r7m ; sign
    sub               tmp2q, tmp1q
    movsxd               wq, [t0+wq*4]
    mova                 m6, [base+pw_2048]
    movddup              m7, [base+wm_420_sign+r6*8] ; 258 - sign
    add                  wq, t0
%if ARCH_X86_64
    mova                 m8, [base+pw_6903] ; ((64 - 38) << 8) + 255 - 8
    movifnidn            hd, hm
%else
    %define              m8  [base+pw_6903]
    %define              hd  dword hm
%endif
    mov               maskq, maskmp
    call .main
    jmp                  wq
.w4_loop:
    call .main
    add               maskq, 4
    lea                dstq, [dstq+strideq*2]
.w4:
    pshufd               m3, m2, q2020
    pshufd               m2, m2, q3131
    psubw                m1, m7, m3
    psubw                m1, m2
    psrlw                m1, 2
    packuswb             m1, m1
    movd            [maskq], m1
    movd   [dstq+strideq*0], m0
    pshuflw              m1, m0, q1032
    movd   [dstq+strideq*1], m1
    punpckhqdq           m0, m0
    lea                dstq, [dstq+strideq*2]
    movd   [dstq+strideq*0], m0
    pshuflw              m1, m0, q1032
    movd   [dstq+strideq*1], m1
    sub                  hd, 4
    jg .w4_loop
    RET
.w8_loop:
    call .main
    add               maskq, 4
    lea                dstq, [dstq+strideq*2]
.w8:
    movhlps              m3, m2
    psubw                m1, m7, m2
    psubw                m1, m3
    psrlw                m1, 2
    packuswb             m1, m1
    movd            [maskq], m1
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    sub                  hd, 2
    jg .w8_loop
    RET
.w16_loop:
    call .main
    add               maskq, 8
    lea                dstq, [dstq+strideq*2]
.w16:
    mova   [dstq+strideq*1], m2
    mova   [dstq+strideq*0], m0
    call .main
    psubw                m1, m7, [dstq+strideq*1]
    psubw                m1, m2
    psrlw                m1, 2
    packuswb             m1, m1
    movq            [maskq], m1
    mova   [dstq+strideq*1], m0
    sub                  hd, 2
    jg .w16_loop
    RET
.w32_loop:
    call .main
    add               maskq, 16
    lea                dstq, [dstq+strideq*2]
.w32:
    mova            [maskq], m2
    mova [dstq+strideq*0+16*0], m0
    call .main
    mova [dstq+strideq*1+16*1], m2
    mova [dstq+strideq*0+16*1], m0
    W_MASK_420_END        0
    sub                  hd, 2
    jg .w32_loop
    RET
.w64_loop:
    call .main
    add               maskq, 16*2
    lea                dstq, [dstq+strideq*2]
.w64:
    mova       [maskq+16*0], m2
    mova [dstq+strideq*0+16*0], m0
    call .main
    mova [dstq+strideq*1+16*1], m2
    mova [dstq+strideq*0+16*1], m0
    call .main
    mova       [maskq+16*1], m2
    mova [dstq+strideq*0+16*2], m0
    call .main
    mova [dstq+strideq*1+16*3], m2
    mova [dstq+strideq*0+16*3], m0
    W_MASK_420_END        0, 1
    sub                  hd, 2
    jg .w64_loop
    RET
.w128_loop:
    call .main
    add               maskq, 16*4
    lea                dstq, [dstq+strideq*2]
.w128:
    mova       [maskq+16*0], m2
    mova [dstq+strideq*0+16*0], m0
    call .main
    mova [dstq+strideq*1+16*1], m2
    mova [dstq+strideq*0+16*1], m0
    call .main
    mova       [maskq+16*1], m2
    mova [dstq+strideq*0+16*2], m0
    call .main
    mova [dstq+strideq*1+16*3], m2
    mova [dstq+strideq*0+16*3], m0
    call .main
    mova       [maskq+16*2], m2
    mova [dstq+strideq*0+16*4], m0
    call .main
    mova [dstq+strideq*1+16*5], m2
    mova [dstq+strideq*0+16*5], m0
    call .main
    mova       [maskq+16*3], m2
    mova [dstq+strideq*0+16*6], m0
    call .main
    mova [dstq+strideq*1+16*7], m2
    mova [dstq+strideq*0+16*7], m0
    W_MASK_420_END        0, 1, 2, 3
    sub                  hd, 2
    jg .w128_loop
    RET
ALIGN function_align
.main:
    mova                 m0, [tmp1q      +16*0]
    mova                 m3, [tmp1q+tmp2q+16*0]
    mova                 m1, [tmp1q      +16*1]
    mova                 m4, [tmp1q+tmp2q+16*1]
    add               tmp1q, 16*2
    psubw                m3, m0
    psubw                m4, m1
    pabsw                m5, m3
    psubusw              m2, m8, m5
    psrlw                m2, 8 ; 64 - m
    psllw                m5, m2, 10
    pmulhw               m3, m5
    pabsw                m5, m4
    paddw                m0, m3
    psubusw              m3, m8, m5
    psrlw                m3, 8
    phaddw               m2, m3
    psllw                m3, 10
    pmulhw               m4, m3
    paddw                m1, m4
    pmulhrsw             m0, m6
    pmulhrsw             m1, m6
    packuswb             m0, m1
    ret

%macro W_MASK_422_BACKUP 1 ; mask_offset
%if ARCH_X86_64
    mova                m10, m2
%else
    mova      [maskq+16*%1], m2
%endif
%endmacro

%macro W_MASK_422_END 1 ; mask_offset
%if ARCH_X86_64
    packuswb            m10, m2
    psubb                m1, m7, m10
    pavgb                m1, m9
%else
    mova                 m3, [maskq+16*%1]
    packuswb             m3, m2
    pxor                 m2, m2
    psubb                m1, m7, m3
    pavgb                m1, m2
%endif
    mova      [maskq+16*%1], m1
%endmacro

cglobal w_mask_422_8bpc, 4, 7, 11, dst, stride, tmp1, tmp2, w, h, mask
%define base t0-w_mask_422_ssse3_table
    LEA                  t0, w_mask_422_ssse3_table
    tzcnt                wd, wm
    mov                 r6d, r7m ; sign
    sub               tmp2q, tmp1q
    movsxd               wq, [t0+wq*4]
    mova                 m6, [base+pw_2048]
    movddup              m7, [base+wm_422_sign+r6*8] ; 128 - sign
    add                  wq, t0
%if ARCH_X86_64
    mova                 m8, [base+pw_6903]
    pxor                 m9, m9
    movifnidn            hd, hm
%else
    add                  t0, w_mask_420_ssse3_table-w_mask_422_ssse3_table
    %define              hd  dword hm
%endif
    mov               maskq, maskmp
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    jmp                  wq
.w4_loop:
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    add               maskq, 8
    lea                dstq, [dstq+strideq*2]
.w4:
    packuswb             m2, m2
    psubb                m1, m7, m2
%if ARCH_X86_64
    pavgb                m1, m9
%else
    pxor                 m2, m2
    pavgb                m1, m2
%endif
    movq            [maskq], m1
    movd   [dstq+strideq*0], m0
    pshuflw              m1, m0, q1032
    movd   [dstq+strideq*1], m1
    punpckhqdq           m0, m0
    lea                dstq, [dstq+strideq*2]
    movd   [dstq+strideq*0], m0
    pshuflw              m1, m0, q1032
    movd   [dstq+strideq*1], m1
    sub                  hd, 4
    jg .w4_loop
    RET
.w8_loop:
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    add               maskq, 16
    lea                dstq, [dstq+strideq*2]
.w8:
    W_MASK_422_BACKUP     0
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    lea                dstq, [dstq+strideq*2]
    W_MASK_422_END        0
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    sub                  hd, 4
    jg .w8_loop
    RET
.w16_loop:
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    add               maskq, 16
    lea                dstq, [dstq+strideq*2]
.w16:
    W_MASK_422_BACKUP     0
    mova   [dstq+strideq*0], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    W_MASK_422_END        0
    mova   [dstq+strideq*1], m0
    sub                  hd, 2
    jg .w16_loop
    RET
.w32_loop:
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    add               maskq, 16
    add                dstq, strideq
.w32:
    W_MASK_422_BACKUP     0
    mova        [dstq+16*0], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    W_MASK_422_END        0
    mova        [dstq+16*1], m0
    dec                  hd
    jg .w32_loop
    RET
.w64_loop:
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    add               maskq, 16*2
    add                dstq, strideq
.w64:
    W_MASK_422_BACKUP     0
    mova        [dstq+16*0], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    W_MASK_422_END        0
    mova        [dstq+16*1], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    W_MASK_422_BACKUP     1
    mova        [dstq+16*2], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    W_MASK_422_END        1
    mova        [dstq+16*3], m0
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    add               maskq, 16*4
    add                dstq, strideq
.w128:
    W_MASK_422_BACKUP     0
    mova        [dstq+16*0], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    W_MASK_422_END        0
    mova        [dstq+16*1], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    W_MASK_422_BACKUP     1
    mova        [dstq+16*2], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    W_MASK_422_END        1
    mova        [dstq+16*3], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    W_MASK_422_BACKUP     2
    mova        [dstq+16*4], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    W_MASK_422_END        2
    mova        [dstq+16*5], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    W_MASK_422_BACKUP     3
    mova        [dstq+16*6], m0
    call mangle(private_prefix %+ _w_mask_420_8bpc_ssse3).main
    W_MASK_422_END        3
    mova        [dstq+16*7], m0
    dec                  hd
    jg .w128_loop
    RET

cglobal w_mask_444_8bpc, 4, 7, 9, dst, stride, tmp1, tmp2, w, h, mask
%define base t0-w_mask_444_ssse3_table
    LEA                  t0, w_mask_444_ssse3_table
    tzcnt                wd, wm
    mov               maskq, maskmp
    sub               tmp2q, tmp1q
    movsxd               wq, [t0+wq*4]
    mova                 m6, [base+pw_6903]
    mova                 m7, [base+pw_2048]
    add                  wq, t0
%if ARCH_X86_64
    mova                 m8, [base+pb_64]
    movifnidn            hd, hm
%else
    %define              m8  [base+pb_64]
    %define              hd  dword hm
%endif
    call .main
    jmp                  wq
.w4_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w4:
    movd   [dstq+strideq*0], m0
    pshuflw              m1, m0, q1032
    movd   [dstq+strideq*1], m1
    punpckhqdq           m0, m0
    lea                dstq, [dstq+strideq*2]
    movd   [dstq+strideq*0], m0
    pshuflw              m1, m0, q1032
    movd   [dstq+strideq*1], m1
    sub                  hd, 4
    jg .w4_loop
    RET
.w8_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w8:
    movq   [dstq+strideq*0], m0
    movhps [dstq+strideq*1], m0
    sub                  hd, 2
    jg .w8_loop
    RET
.w16_loop:
    call .main
    lea                dstq, [dstq+strideq*2]
.w16:
    mova   [dstq+strideq*0], m0
    call .main
    mova   [dstq+strideq*1], m0
    sub                  hd, 2
    jg .w16_loop
    RET
.w32_loop:
    call .main
    add                dstq, strideq
.w32:
    mova        [dstq+16*0], m0
    call .main
    mova        [dstq+16*1], m0
    dec                  hd
    jg .w32_loop
    RET
.w64_loop:
    call .main
    add                dstq, strideq
.w64:
    mova        [dstq+16*0], m0
    call .main
    mova        [dstq+16*1], m0
    call .main
    mova        [dstq+16*2], m0
    call .main
    mova        [dstq+16*3], m0
    dec                  hd
    jg .w64_loop
    RET
.w128_loop:
    call .main
    add                dstq, strideq
.w128:
    mova        [dstq+16*0], m0
    call .main
    mova        [dstq+16*1], m0
    call .main
    mova        [dstq+16*2], m0
    call .main
    mova        [dstq+16*3], m0
    call .main
    mova        [dstq+16*4], m0
    call .main
    mova        [dstq+16*5], m0
    call .main
    mova        [dstq+16*6], m0
    call .main
    mova        [dstq+16*7], m0
    dec                  hd
    jg .w128_loop
    RET
ALIGN function_align
.main:
    mova                 m0, [tmp1q      +16*0]
    mova                 m3, [tmp1q+tmp2q+16*0]
    mova                 m1, [tmp1q      +16*1]
    mova                 m4, [tmp1q+tmp2q+16*1]
    add               tmp1q, 16*2
    psubw                m3, m0
    psubw                m4, m1
    pabsw                m5, m3
    psubusw              m2, m6, m5
    psrlw                m2, 8 ; 64 - m
    psllw                m5, m2, 10
    pmulhw               m3, m5
    pabsw                m5, m4
    paddw                m0, m3
    psubusw              m3, m6, m5
    psrlw                m3, 8
    packuswb             m2, m3
    psllw                m3, 10
    pmulhw               m4, m3
    psubb                m3, m8, m2
    paddw                m1, m4
    pmulhrsw             m0, m7
    pmulhrsw             m1, m7
    mova            [maskq], m3
    add               maskq, 16
    packuswb             m0, m1
    ret

%macro BLEND_64M 4; a, b, mask1, mask2
    punpcklbw            m0, %1, %2; {b;a}[7..0]
    punpckhbw            %1, %2    ; {b;a}[15..8]
    pmaddubsw            m0, %3    ; {b*m[0] + (64-m[0])*a}[7..0] u16
    pmaddubsw            %1, %4    ; {b*m[1] + (64-m[1])*a}[15..8] u16
    pmulhrsw             m0, m5    ; {((b*m[0] + (64-m[0])*a) + 1) / 32}[7..0] u16
    pmulhrsw             %1, m5    ; {((b*m[1] + (64-m[0])*a) + 1) / 32}[15..8] u16
    packuswb             m0, %1    ; {blendpx}[15..0] u8
%endmacro

%macro BLEND 2; a, b
    psubb                m3, m4, m0 ; m3 = (64 - m)
    punpcklbw            m2, m3, m0 ; {m;(64-m)}[7..0]
    punpckhbw            m3, m0     ; {m;(64-m)}[15..8]
    BLEND_64M            %1, %2, m2, m3
%endmacro

cglobal blend_8bpc, 3, 7, 7, dst, ds, tmp, w, h, mask
%define base r6-blend_ssse3_table
    LEA                  r6, blend_ssse3_table
    tzcnt                wd, wm
    movifnidn            hd, hm
    movifnidn         maskq, maskmp
    movsxd               wq, dword [r6+wq*4]
    mova                 m4, [base+pb_64]
    mova                 m5, [base+pw_512]
    add                  wq, r6
    lea                  r6, [dsq*3]
    jmp                  wq
.w4:
    movq                 m0, [maskq]; m
    movd                 m1, [dstq+dsq*0] ; a
    movd                 m6, [dstq+dsq*1]
    punpckldq            m1, m6
    movq                 m6, [tmpq] ; b
    psubb                m3, m4, m0 ; m3 = (64 - m)
    punpcklbw            m2, m3, m0 ; {m;(64-m)}[7..0]
    punpcklbw            m1, m6    ; {b;a}[7..0]
    pmaddubsw            m1, m2    ; {b*m[0] + (64-m[0])*a}[7..0] u16
    pmulhrsw             m1, m5    ; {((b*m[0] + (64-m[0])*a) + 1) / 32}[7..0] u16
    packuswb             m1, m0    ; {blendpx}[15..0] u8
    movd       [dstq+dsq*0], m1
    psrlq                m1, 32
    movd       [dstq+dsq*1], m1
    add               maskq, 8
    add                tmpq, 8
    lea                dstq, [dstq+dsq*2] ; dst_stride * 2
    sub                  hd, 2
    jg .w4
    RET
.w8:
    mova                 m0, [maskq]; m
    movq                 m1, [dstq+dsq*0] ; a
    movhps               m1, [dstq+dsq*1]
    mova                 m6, [tmpq] ; b
    BLEND                m1, m6
    movq       [dstq+dsq*0], m0
    movhps     [dstq+dsq*1], m0
    add               maskq, 16
    add                tmpq, 16
    lea                dstq, [dstq+dsq*2] ; dst_stride * 2
    sub                  hd, 2
    jg .w8
    RET
.w16:
    mova                 m0, [maskq]; m
    mova                 m1, [dstq] ; a
    mova                 m6, [tmpq] ; b
    BLEND                m1, m6
    mova             [dstq], m0
    add               maskq, 16
    add                tmpq, 16
    add                dstq, dsq ; dst_stride
    dec                  hd
    jg .w16
    RET
.w32:
    %assign i 0
    %rep 2
    mova                 m0, [maskq+16*i]; m
    mova                 m1, [dstq+16*i] ; a
    mova                 m6, [tmpq+16*i] ; b
    BLEND                m1, m6
    mova        [dstq+i*16], m0
    %assign i i+1
    %endrep
    add               maskq, 32
    add                tmpq, 32
    add                dstq, dsq ; dst_stride
    dec                  hd
    jg .w32
    RET

cglobal blend_v_8bpc, 3, 6, 6, dst, ds, tmp, w, h, mask
%define base r5-blend_v_ssse3_table
    LEA                  r5, blend_v_ssse3_table
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, dword [r5+wq*4]
    mova                 m5, [base+pw_512]
    add                  wq, r5
    add               maskq, obmc_masks-blend_v_ssse3_table
    jmp                  wq
.w2:
    movd                 m3, [maskq+4]
    punpckldq            m3, m3
    ; 2 mask blend is provided for 4 pixels / 2 lines
.w2_loop:
    movd                 m1, [dstq+dsq*0] ; a {..;a;a}
    pinsrw               m1, [dstq+dsq*1], 1
    movd                 m2, [tmpq] ; b
    punpcklbw            m0, m1, m2; {b;a}[7..0]
    pmaddubsw            m0, m3    ; {b*m + (64-m)*a}[7..0] u16
    pmulhrsw             m0, m5    ; {((b*m + (64-m)*a) + 1) / 32}[7..0] u16
    packuswb             m0, m1    ; {blendpx}[8..0] u8
    movd                r3d, m0
    mov        [dstq+dsq*0], r3w
    shr                 r3d, 16
    mov        [dstq+dsq*1], r3w
    add                tmpq, 2*2
    lea                dstq, [dstq + dsq * 2]
    sub                  hd, 2
    jg .w2_loop
    RET
.w4:
    movddup              m3, [maskq+8]
    ; 4 mask blend is provided for 8 pixels / 2 lines
.w4_loop:
    movd                 m1, [dstq+dsq*0] ; a
    movd                 m2, [dstq+dsq*1] ;
    punpckldq            m1, m2
    movq                 m2, [tmpq] ; b
    punpcklbw            m1, m2    ; {b;a}[7..0]
    pmaddubsw            m1, m3    ; {b*m + (64-m)*a}[7..0] u16
    pmulhrsw             m1, m5    ; {((b*m + (64-m)*a) + 1) / 32}[7..0] u16
    packuswb             m1, m1    ; {blendpx}[8..0] u8
    movd             [dstq], m1
    psrlq                m1, 32
    movd       [dstq+dsq*1], m1
    add                tmpq, 2*4
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w4_loop
    RET
.w8:
    mova                 m3, [maskq+16]
    ; 8 mask blend is provided for 16 pixels
.w8_loop:
    movq                 m1, [dstq+dsq*0] ; a
    movhps               m1, [dstq+dsq*1]
    mova                 m2, [tmpq]; b
    BLEND_64M            m1, m2, m3, m3
    movq       [dstq+dsq*0], m0
    movhps     [dstq+dsq*1], m0
    add                tmpq, 16
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w8_loop
    RET
.w16:
    ; 16 mask blend is provided for 32 pixels
    mova                  m3, [maskq+32] ; obmc_masks_16[0] (64-m[0])
    mova                  m4, [maskq+48] ; obmc_masks_16[1] (64-m[1])
.w16_loop:
    mova                 m1, [dstq] ; a
    mova                 m2, [tmpq] ; b
    BLEND_64M            m1, m2, m3, m4
    mova             [dstq], m0
    add                tmpq, 16
    add                dstq, dsq
    dec                  hd
    jg .w16_loop
    RET
.w32:
%if WIN64
    mova            [rsp+8], xmm6
%endif
    mova                 m3, [maskq+64] ; obmc_masks_32[0] (64-m[0])
    mova                 m4, [maskq+80] ; obmc_masks_32[1] (64-m[1])
    mova                 m6, [maskq+96] ; obmc_masks_32[2] (64-m[2])
    ; 16 mask blend is provided for 64 pixels
.w32_loop:
    mova                 m1, [dstq+16*0] ; a
    mova                 m2, [tmpq+16*0] ; b
    BLEND_64M            m1, m2, m3, m4
    movq                 m1, [dstq+16*1] ; a
    punpcklbw            m1, [tmpq+16*1] ; b
    pmaddubsw            m1, m6
    pmulhrsw             m1, m5
    packuswb             m1, m1
    mova        [dstq+16*0], m0
    movq        [dstq+16*1], m1
    add                tmpq, 32
    add                dstq, dsq
    dec                  hd
    jg .w32_loop
%if WIN64
    mova               xmm6, [rsp+8]
%endif
    RET

cglobal blend_h_8bpc, 3, 7, 6, dst, ds, tmp, w, h, mask
%define base t0-blend_h_ssse3_table
%if ARCH_X86_32
    ; We need to keep the PIC pointer for w4, reload wd from stack instead
    DECLARE_REG_TMP 6
%else
    DECLARE_REG_TMP 5
    mov                 r6d, wd
%endif
    LEA                  t0, blend_h_ssse3_table
    tzcnt                wd, wm
    mov                  hd, hm
    movsxd               wq, dword [t0+wq*4]
    mova                 m5, [base+pw_512]
    add                  wq, t0
    lea               maskq, [base+obmc_masks+hq*2]
    lea                  hd, [hq*3]
    shr                  hd, 2 ; h * 3/4
    lea               maskq, [maskq+hq*2]
    neg                  hq
    jmp                  wq
.w2:
    movd                 m0, [dstq+dsq*0]
    pinsrw               m0, [dstq+dsq*1], 1
    movd                 m2, [maskq+hq*2]
    movd                 m1, [tmpq]
    punpcklwd            m2, m2
    punpcklbw            m0, m1
    pmaddubsw            m0, m2
    pmulhrsw             m0, m5
    packuswb             m0, m0
    movd                r3d, m0
    mov        [dstq+dsq*0], r3w
    shr                 r3d, 16
    mov        [dstq+dsq*1], r3w
    lea                dstq, [dstq+dsq*2]
    add                tmpq, 2*2
    add                  hq, 2
    jl .w2
    RET
.w4:
%if ARCH_X86_32
    mova                 m3, [base+blend_shuf]
%else
    mova                 m3, [blend_shuf]
%endif
.w4_loop:
    movd                 m0, [dstq+dsq*0]
    movd                 m2, [dstq+dsq*1]
    punpckldq            m0, m2 ; a
    movq                 m1, [tmpq] ; b
    movq                 m2, [maskq+hq*2] ; m
    pshufb               m2, m3
    punpcklbw            m0, m1
    pmaddubsw            m0, m2
    pmulhrsw             m0, m5
    packuswb             m0, m0
    movd       [dstq+dsq*0], m0
    psrlq                m0, 32
    movd       [dstq+dsq*1], m0
    lea                dstq, [dstq+dsq*2]
    add                tmpq, 4*2
    add                  hq, 2
    jl .w4_loop
    RET
.w8:
    movd                 m4, [maskq+hq*2]
    punpcklwd            m4, m4
    pshufd               m3, m4, q0000
    pshufd               m4, m4, q1111
    movq                 m1, [dstq+dsq*0] ; a
    movhps               m1, [dstq+dsq*1]
    mova                 m2, [tmpq]
    BLEND_64M            m1, m2, m3, m4
    movq       [dstq+dsq*0], m0
    movhps     [dstq+dsq*1], m0
    lea                dstq, [dstq+dsq*2]
    add                tmpq, 8*2
    add                  hq, 2
    jl .w8
    RET
; w16/w32/w64/w128
.w16:
%if ARCH_X86_32
    mov                 r6d, wm
%endif
    sub                 dsq, r6
.w16_loop0:
    movd                 m3, [maskq+hq*2]
    pshuflw              m3, m3, q0000
    punpcklqdq           m3, m3
    mov                  wd, r6d
.w16_loop:
    mova                 m1, [dstq] ; a
    mova                 m2, [tmpq] ; b
    BLEND_64M            m1, m2, m3, m3
    mova             [dstq], m0
    add                dstq, 16
    add                tmpq, 16
    sub                  wd, 16
    jg .w16_loop
    add                dstq, dsq
    inc                  hq
    jl .w16_loop0
    RET

; emu_edge args:
; const intptr_t bw, const intptr_t bh, const intptr_t iw, const intptr_t ih,
; const intptr_t x, const intptr_t y, pixel *dst, const ptrdiff_t dst_stride,
; const pixel *ref, const ptrdiff_t ref_stride
;
; bw, bh total filled size
; iw, ih, copied block -> fill bottom, right
; x, y, offset in bw/bh -> fill top, left
cglobal emu_edge_8bpc, 10, 13, 2, bw, bh, iw, ih, x, \
                                  y, dst, dstride, src, sstride, \
                                  bottomext, rightext, blk
    ; we assume that the buffer (stride) is larger than width, so we can
    ; safely overwrite by a few bytes
    pxor                 m1, m1

%if ARCH_X86_64
 %define reg_zero       r12q
 %define reg_tmp        r10
 %define reg_src        srcq
 %define reg_bottomext  bottomextq
 %define reg_rightext   rightextq
 %define reg_blkm       r9m
%else
 %define reg_zero       r6
 %define reg_tmp        r0
 %define reg_src        r1
 %define reg_bottomext  r0
 %define reg_rightext   r1
 %define reg_blkm       r2m
%endif
    ;
    ; ref += iclip(y, 0, ih - 1) * PXSTRIDE(ref_stride)
    xor            reg_zero, reg_zero
    lea             reg_tmp, [ihq-1]
    cmp                  yq, ihq
    cmovs           reg_tmp, yq
    test                 yq, yq
    cmovs           reg_tmp, reg_zero
%if ARCH_X86_64
    imul            reg_tmp, sstrideq
    add                srcq, reg_tmp
%else
    imul            reg_tmp, sstridem
    mov             reg_src, srcm
    add             reg_src, reg_tmp
%endif
    ;
    ; ref += iclip(x, 0, iw - 1)
    lea             reg_tmp, [iwq-1]
    cmp                  xq, iwq
    cmovs           reg_tmp, xq
    test                 xq, xq
    cmovs           reg_tmp, reg_zero
    add             reg_src, reg_tmp
%if ARCH_X86_32
    mov                srcm, reg_src
%endif
    ;
    ; bottom_ext = iclip(y + bh - ih, 0, bh - 1)
%if ARCH_X86_32
    mov                  r1, r1m ; restore bh
%endif
    lea       reg_bottomext, [yq+bhq]
    sub       reg_bottomext, ihq
    lea                  r3, [bhq-1]
    cmovs     reg_bottomext, reg_zero
    ;

    DEFINE_ARGS bw, bh, iw, ih, x, \
                topext, dst, dstride, src, sstride, \
                bottomext, rightext, blk

    ; top_ext = iclip(-y, 0, bh - 1)
    neg             topextq
    cmovs           topextq, reg_zero
    cmp       reg_bottomext, bhq
    cmovns    reg_bottomext, r3
    cmp             topextq, bhq
    cmovg           topextq, r3
 %if ARCH_X86_32
    mov                 r4m, reg_bottomext
    ;
    ; right_ext = iclip(x + bw - iw, 0, bw - 1)
    mov                  r0, r0m ; restore bw
 %endif
    lea        reg_rightext, [xq+bwq]
    sub        reg_rightext, iwq
    lea                  r2, [bwq-1]
    cmovs      reg_rightext, reg_zero

    DEFINE_ARGS bw, bh, iw, ih, leftext, \
                topext, dst, dstride, src, sstride, \
                bottomext, rightext, blk

    ; left_ext = iclip(-x, 0, bw - 1)
    neg            leftextq
    cmovs          leftextq, reg_zero
    cmp        reg_rightext, bwq
    cmovns     reg_rightext, r2
 %if ARCH_X86_32
    mov                 r3m, r1
 %endif
    cmp            leftextq, bwq
    cmovns         leftextq, r2

%undef reg_zero
%undef reg_tmp
%undef reg_src
%undef reg_bottomext
%undef reg_rightext

    DEFINE_ARGS bw, centerh, centerw, dummy, leftext, \
                topext, dst, dstride, src, sstride, \
                bottomext, rightext, blk

    ; center_h = bh - top_ext - bottom_ext
%if ARCH_X86_64
    lea                  r3, [bottomextq+topextq]
    sub            centerhq, r3
%else
    mov                   r1, centerhm ; restore r1
    sub             centerhq, topextq
    sub             centerhq, r4m
    mov                  r1m, centerhq
%endif
    ;
    ; blk += top_ext * PXSTRIDE(dst_stride)
    mov                  r2, topextq
%if ARCH_X86_64
    imul                 r2, dstrideq
%else
    mov                  r6, r6m ; restore dstq
    imul                 r2, dstridem
%endif
    add                dstq, r2
    mov            reg_blkm, dstq ; save pointer for ext
    ;
    ; center_w = bw - left_ext - right_ext
    mov            centerwq, bwq
%if ARCH_X86_64
    lea                  r3, [rightextq+leftextq]
    sub            centerwq, r3
%else
    sub            centerwq, r3m
    sub            centerwq, leftextq
%endif

; vloop Macro
%macro v_loop 3 ; need_left_ext, need_right_ext, suffix
  %if ARCH_X86_64
    %define reg_tmp        r12
  %else
    %define reg_tmp        r0
  %endif
.v_loop_%3:
  %if ARCH_X86_32
    mov                  r0, r0m
    mov                  r1, r1m
  %endif
%if %1
    ; left extension
  %if ARCH_X86_64
    movd                 m0, [srcq]
  %else
    mov                  r3, srcm
    movd                 m0, [r3]
  %endif
    pshufb               m0, m1
    xor                  r3, r3
.left_loop_%3:
    mova          [dstq+r3], m0
    add                  r3, mmsize
    cmp                  r3, leftextq
    jl .left_loop_%3
    ; body
    lea             reg_tmp, [dstq+leftextq]
%endif
    xor                  r3, r3
.body_loop_%3:
  %if ARCH_X86_64
    movu                 m0, [srcq+r3]
  %else
    mov                  r1, srcm
    movu                 m0, [r1+r3]
  %endif
%if %1
    movu       [reg_tmp+r3], m0
%else
    movu          [dstq+r3], m0
%endif
    add                  r3, mmsize
    cmp                  r3, centerwq
    jl .body_loop_%3
%if %2
    ; right extension
%if %1
    add             reg_tmp, centerwq
%else
    lea             reg_tmp, [dstq+centerwq]
%endif
  %if ARCH_X86_64
    movd                 m0, [srcq+centerwq-1]
  %else
    mov                  r3, srcm
    movd                 m0, [r3+centerwq-1]
  %endif
    pshufb               m0, m1
    xor                  r3, r3
.right_loop_%3:
    movu       [reg_tmp+r3], m0
    add                  r3, mmsize
  %if ARCH_X86_64
    cmp                  r3, rightextq
  %else
    cmp                  r3, r3m
  %endif
    jl .right_loop_%3
%endif
  %if ARCH_X86_64
    add                dstq, dstrideq
    add                srcq, sstrideq
    dec            centerhq
    jg .v_loop_%3
  %else
    add                dstq, dstridem
    mov                  r0, sstridem
    add                srcm, r0
    sub       dword centerhm, 1
    jg .v_loop_%3
    mov                  r0, r0m ; restore r0
  %endif
%endmacro ; vloop MACRO

    test           leftextq, leftextq
    jnz .need_left_ext
 %if ARCH_X86_64
    test          rightextq, rightextq
    jnz .need_right_ext
 %else
    cmp            leftextq, r3m ; leftextq == 0
    jne .need_right_ext
 %endif
    v_loop                0, 0, 0
    jmp .body_done

    ;left right extensions
.need_left_ext:
 %if ARCH_X86_64
    test          rightextq, rightextq
 %else
    mov                  r3, r3m
    test                 r3, r3
 %endif
    jnz .need_left_right_ext
    v_loop                1, 0, 1
    jmp .body_done

.need_left_right_ext:
    v_loop                1, 1, 2
    jmp .body_done

.need_right_ext:
    v_loop                0, 1, 3

.body_done:
; r0 ; bw
; r1 ;; x loop
; r4 ;; y loop
; r5 ; topextq
; r6 ;dstq
; r7 ;dstrideq
; r8 ; srcq
%if ARCH_X86_64
 %define reg_dstride    dstrideq
%else
 %define reg_dstride    r2
%endif
    ;
    ; bottom edge extension
 %if ARCH_X86_64
    test         bottomextq, bottomextq
    jz .top
 %else
    xor                  r1, r1
    cmp                  r1, r4m
    je .top
 %endif
    ;
 %if ARCH_X86_64
    mov                srcq, dstq
    sub                srcq, dstrideq
    xor                  r1, r1
 %else
    mov                  r3, dstq
    mov         reg_dstride, dstridem
    sub                  r3, reg_dstride
    mov                srcm, r3
 %endif
    ;
.bottom_x_loop:
 %if ARCH_X86_64
    mova                 m0, [srcq+r1]
    lea                  r3, [dstq+r1]
    mov                  r4, bottomextq
 %else
    mov                  r3, srcm
    mova                 m0, [r3+r1]
    lea                  r3, [dstq+r1]
    mov                  r4, r4m
 %endif
    ;
.bottom_y_loop:
    mova               [r3], m0
    add                  r3, reg_dstride
    dec                  r4
    jg .bottom_y_loop
    add                  r1, mmsize
    cmp                  r1, bwq
    jl .bottom_x_loop

.top:
    ; top edge extension
    test            topextq, topextq
    jz .end
%if ARCH_X86_64
    mov                srcq, reg_blkm
%else
    mov                  r3, reg_blkm
    mov         reg_dstride, dstridem
%endif
    mov                dstq, dstm
    xor                  r1, r1
    ;
.top_x_loop:
%if ARCH_X86_64
    mova                 m0, [srcq+r1]
%else
    mov                  r3, reg_blkm
    mova                 m0, [r3+r1]
%endif
    lea                  r3, [dstq+r1]
    mov                  r4, topextq
    ;
.top_y_loop:
    mova               [r3], m0
    add                  r3, reg_dstride
    dec                  r4
    jg .top_y_loop
    add                  r1, mmsize
    cmp                  r1, bwq
    jl .top_x_loop

.end:
    RET

%undef reg_dstride
%undef reg_blkm
%undef reg_tmp

cextern resize_filter

%macro SCRATCH 3
%if ARCH_X86_32
    mova [rsp+%3*mmsize], m%1
%define m%2 [rsp+%3*mmsize]
%else
    SWAP             %1, %2
%endif
%endmacro

%if ARCH_X86_64
cglobal resize_8bpc, 0, 12, 14, dst, dst_stride, src, src_stride, \
                                dst_w, h, src_w, dx, mx0
%elif STACK_ALIGNMENT >= 16
cglobal resize_8bpc, 0, 7, 8, 3 * 16, dst, dst_stride, src, src_stride, \
                                      dst_w, h, src_w, dx, mx0
%else
cglobal resize_8bpc, 0, 6, 8, 3 * 16, dst, dst_stride, src, src_stride, \
                                      dst_w, h, src_w, dx, mx0
%endif
    movifnidn          dstq, dstmp
    movifnidn          srcq, srcmp
%if STACK_ALIGNMENT >= 16
    movifnidn        dst_wd, dst_wm
%endif
%if ARCH_X86_64
    movifnidn            hd, hm
%endif
    sub          dword mx0m, 4<<14
    sub        dword src_wm, 8
    movd                 m7, dxm
    movd                 m6, mx0m
    movd                 m5, src_wm
    pshufd               m7, m7, q0000
    pshufd               m6, m6, q0000
    pshufd               m5, m5, q0000

%if ARCH_X86_64
    DEFINE_ARGS dst, dst_stride, src, src_stride, dst_w, h, x
    LEA                  r7, $$
%define base r7-$$
%else
    DEFINE_ARGS dst, dst_stride, src, src_stride, dst_w, x
%define hd dword r5m
%if STACK_ALIGNMENT >= 16
    LEA                  r6, $$
%define base r6-$$
%else
    LEA                  r4, $$
%define base r4-$$
%endif
%endif

%if ARCH_X86_64
    mova                m10, [base+pw_m256]
    mova                 m9, [base+pd_63]
    mova                 m8, [base+pb_8x0_8x8]
%else
%define m10 [base+pw_m256]
%define m9  [base+pd_63]
%define m8  [base+pb_8x0_8x8]
%endif
    pmaddwd              m4, m7, [base+rescale_mul] ; dx*[0,1,2,3]
    pslld                m7, 2                      ; dx*4
    pslld                m5, 14
    paddd                m6, m4                     ; mx+[0..3]*dx
    SCRATCH               7, 13, 0
    SCRATCH               6, 12, 1
    SCRATCH               5, 11, 2

    ; m10 = pmulhrsw constant for x=(x+64)>>7
    ; m12 = mx+[0..3]*dx, m13 = dx*4, m11 = src_w, m9 = 0x3f, m8=0,8

.loop_y:
    xor                  xd, xd
    mova                 m0, m12                    ; per-line working version of mx

.loop_x:
    pxor                 m1, m1
    pcmpgtd              m1, m0
    pandn                m1, m0
    psrad                m2, m0, 8                  ; filter offset (unmasked)
    pcmpgtd              m3, m11, m1
    pand                 m1, m3
    pandn                m3, m11
    por                  m1, m3
    psubd                m3, m0, m1                 ; pshufb offset
    psrad                m1, 14                     ; clipped src_x offset
    psrad                m3, 14                     ; pshufb edge_emu offset
    pand                 m2, m9                     ; filter offset (masked)

    ; load source pixels
%if ARCH_X86_64
    movd                r8d, m1
    pshuflw              m1, m1, q3232
    movd                r9d, m1
    punpckhqdq           m1, m1
    movd               r10d, m1
    psrlq                m1, 32
    movd               r11d, m1
    movq                 m4, [srcq+r8]
    movq                 m5, [srcq+r10]
    movhps               m4, [srcq+r9]
    movhps               m5, [srcq+r11]
%else
    movd                r3d,  m1
    pshufd               m1,  m1, q3312
    movd                r1d,  m1
    pshuflw              m1,  m1, q3232
    movq                 m4, [srcq+r3]
    movq                 m5, [srcq+r1]
    movd                r3d,  m1
    punpckhqdq           m1,  m1
    movd                r1d,  m1
    movhps               m4, [srcq+r3]
    movhps               m5, [srcq+r1]
%endif

    ; if no emulation is required, we don't need to shuffle or emulate edges
    ; this also saves 2 quasi-vpgatherdqs
    pxor                 m6, m6
    pcmpeqb              m6, m3
%if ARCH_X86_64
    pmovmskb            r8d, m6
    cmp                 r8d, 0xffff
%else
    pmovmskb            r3d, m6
    cmp                 r3d, 0xffff
%endif
    je .filter

%if ARCH_X86_64
    movd                r8d, m3
    pshuflw              m3, m3, q3232
    movd                r9d, m3
    punpckhqdq           m3, m3
    movd               r10d, m3
    psrlq                m3, 32
    movd               r11d, m3
    movsxd               r8, r8d
    movsxd               r9, r9d
    movsxd              r10, r10d
    movsxd              r11, r11d
    movq                 m6, [base+resize_shuf+4+r8]
    movq                 m7, [base+resize_shuf+4+r10]
    movhps               m6, [base+resize_shuf+4+r9]
    movhps               m7, [base+resize_shuf+4+r11]
%else
    movd                r3d, m3
    pshufd               m3, m3, q3312
    movd                r1d, m3
    pshuflw              m3, m3, q3232
    movq                 m6, [base+resize_shuf+4+r3]
    movq                 m7, [base+resize_shuf+4+r1]
    movd                r3d, m3
    punpckhqdq           m3, m3
    movd                r1d, m3
    movhps               m6, [base+resize_shuf+4+r3]
    movhps               m7, [base+resize_shuf+4+r1]
%endif

    paddb                m6, m8
    paddb                m7, m8
    pshufb               m4, m6
    pshufb               m5, m7

.filter:
%if ARCH_X86_64
    movd                r8d, m2
    pshuflw              m2, m2, q3232
    movd                r9d, m2
    punpckhqdq           m2, m2
    movd               r10d, m2
    psrlq                m2, 32
    movd               r11d, m2
    movq                 m6, [base+resize_filter+r8*8]
    movq                 m7, [base+resize_filter+r10*8]
    movhps               m6, [base+resize_filter+r9*8]
    movhps               m7, [base+resize_filter+r11*8]
%else
    movd                r3d, m2
    pshufd               m2, m2, q3312
    movd                r1d, m2
    pshuflw              m2, m2, q3232
    movq                 m6, [base+resize_filter+r3*8]
    movq                 m7, [base+resize_filter+r1*8]
    movd                r3d, m2
    punpckhqdq           m2, m2
    movd                r1d, m2
    movhps               m6, [base+resize_filter+r3*8]
    movhps               m7, [base+resize_filter+r1*8]
%endif

    pmaddubsw            m4, m6
    pmaddubsw            m5, m7
    phaddw               m4, m5
    phaddsw              m4, m4
    pmulhrsw             m4, m10                    ; x=(x+64)>>7
    packuswb             m4, m4
    movd          [dstq+xq], m4

    paddd                m0, m13
    add                  xd, 4
%if STACK_ALIGNMENT >= 16
    cmp                  xd, dst_wd
%else
    cmp                  xd, dst_wm
%endif
    jl .loop_x

    add                dstq, dst_stridemp
    add                srcq, src_stridemp
    dec                  hd
    jg .loop_y
    RET

INIT_XMM ssse3
PREP_BILIN
PREP_8TAP
WARP_AFFINE_8X8
WARP_AFFINE_8X8T

INIT_XMM sse4
WARP_AFFINE_8X8
WARP_AFFINE_8X8T

INIT_XMM sse2
PREP_BILIN
PREP_8TAP
WARP_AFFINE_8X8
WARP_AFFINE_8X8T
