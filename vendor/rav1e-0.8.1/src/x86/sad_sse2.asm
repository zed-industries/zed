;
; Copyright (c) 2016, Alliance for Open Media. All rights reserved
;
; This source code is subject to the terms of the BSD 2 Clause License and
; the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
; was not distributed with this source code in the LICENSE file, you can
; obtain it at www.aomedia.org/license/software. If the Alliance for Open
; Media Patent License 1.0 was not distributed with this source code in the
; PATENTS file, you can obtain it at www.aomedia.org/license/patent.
;

;

%include "config.asm"
%include "ext/x86/x86inc.asm"

SECTION .text

%macro SAD_FN 4
%if %4 == 0
%if %3 == 5
cglobal sad%1x%2, 4, %3, 5, src, src_stride, ref, ref_stride, n_rows
%else ; %3 == 7
cglobal sad%1x%2, 4, %3, 6, src, src_stride, ref, ref_stride, \
                            src_stride3, ref_stride3, n_rows
%endif ; %3 == 5/7
%else ; avg
%if %3 == 5
cglobal sad%1x%2_avg, 5, 1 + %3, 5, src, src_stride, ref, ref_stride, \
                                    second_pred, n_rows
%else ; %3 == 7
cglobal sad%1x%2_avg, 5, ARCH_X86_64 + %3, 6, src, src_stride, \
                                              ref, ref_stride, \
                                              second_pred, \
                                              src_stride3, ref_stride3
%if ARCH_X86_64
%define n_rowsd r7d
%else ; x86-32
%define n_rowsd dword r0m
%endif ; x86-32/64
%endif ; %3 == 5/7
%endif ; avg/sad
  movsxdifnidn src_strideq, src_strided
  movsxdifnidn ref_strideq, ref_strided
%if %3 == 7
  lea         src_stride3q, [src_strideq*3]
  lea         ref_stride3q, [ref_strideq*3]
%endif ; %3 == 7
%endmacro

; unsigned int aom_sad128x128_sse2(uint8_t *src, int src_stride,
;                                  uint8_t *ref, int ref_stride);
%macro SAD128XN 1-2 0
  SAD_FN 128, %1, 5, %2
  mov              n_rowsd, %1
  pxor                  m0, m0

.loop:
  movu                  m1, [refq]
  movu                  m2, [refq+16]
  movu                  m3, [refq+32]
  movu                  m4, [refq+48]
%if %2 == 1
  pavgb                 m1, [second_predq+mmsize*0]
  pavgb                 m2, [second_predq+mmsize*1]
  pavgb                 m3, [second_predq+mmsize*2]
  pavgb                 m4, [second_predq+mmsize*3]
%endif
  psadbw                m1, [srcq]
  psadbw                m2, [srcq+16]
  psadbw                m3, [srcq+32]
  psadbw                m4, [srcq+48]

  paddd                 m1, m2
  paddd                 m3, m4
  paddd                 m0, m1
  paddd                 m0, m3

  movu                  m1, [refq+64]
  movu                  m2, [refq+80]
  movu                  m3, [refq+96]
  movu                  m4, [refq+112]
%if %2 == 1
  pavgb                 m1, [second_predq+mmsize*4]
  pavgb                 m2, [second_predq+mmsize*5]
  pavgb                 m3, [second_predq+mmsize*6]
  pavgb                 m4, [second_predq+mmsize*7]
  lea         second_predq, [second_predq+mmsize*8]
%endif
  psadbw                m1, [srcq+64]
  psadbw                m2, [srcq+80]
  psadbw                m3, [srcq+96]
  psadbw                m4, [srcq+112]

  add                 refq, ref_strideq
  add                 srcq, src_strideq

  paddd                 m1, m2
  paddd                 m3, m4
  paddd                 m0, m1
  paddd                 m0, m3

  sub              n_rowsd, 1
  jg .loop

  movhlps               m1, m0
  paddd                 m0, m1
  movd                 eax, m0
  RET
%endmacro

INIT_XMM sse2
SAD128XN 128     ; sad128x128_sse2
SAD128XN 128, 1  ; sad128x128_avg_sse2
SAD128XN 64      ; sad128x64_sse2
SAD128XN 64, 1   ; sad128x64_avg_sse2


; unsigned int aom_sad64x64_sse2(uint8_t *src, int src_stride,
;                                uint8_t *ref, int ref_stride);
%macro SAD64XN 1-2 0
  SAD_FN 64, %1, 5, %2
  mov              n_rowsd, %1
  pxor                  m0, m0
.loop:
  movu                  m1, [refq]
  movu                  m2, [refq+16]
  movu                  m3, [refq+32]
  movu                  m4, [refq+48]
%if %2 == 1
  pavgb                 m1, [second_predq+mmsize*0]
  pavgb                 m2, [second_predq+mmsize*1]
  pavgb                 m3, [second_predq+mmsize*2]
  pavgb                 m4, [second_predq+mmsize*3]
  lea         second_predq, [second_predq+mmsize*4]
%endif
  psadbw                m1, [srcq]
  psadbw                m2, [srcq+16]
  psadbw                m3, [srcq+32]
  psadbw                m4, [srcq+48]
  paddd                 m1, m2
  paddd                 m3, m4
  add                 refq, ref_strideq
  paddd                 m0, m1
  add                 srcq, src_strideq
  paddd                 m0, m3
  dec              n_rowsd
  jg .loop

  movhlps               m1, m0
  paddd                 m0, m1
  movd                 eax, m0
  RET
%endmacro

INIT_XMM sse2
SAD64XN 128     ; sad64x128_sse2
SAD64XN 128, 1  ; sad64x128_avg_sse2
SAD64XN 64 ; sad64x64_sse2
SAD64XN 32 ; sad64x32_sse2
SAD64XN 64, 1 ; sad64x64_avg_sse2
SAD64XN 32, 1 ; sad64x32_avg_sse2
SAD64XN 16 ; sad64x16_sse2
SAD64XN 16, 1 ; sad64x16_avg_sse2

; unsigned int aom_sad32x32_sse2(uint8_t *src, int src_stride,
;                                uint8_t *ref, int ref_stride);
%macro SAD32XN 1-2 0
  SAD_FN 32, %1, 5, %2
  mov              n_rowsd, %1/2
  pxor                  m0, m0
.loop:
  movu                  m1, [refq]
  movu                  m2, [refq+16]
  movu                  m3, [refq+ref_strideq]
  movu                  m4, [refq+ref_strideq+16]
%if %2 == 1
  pavgb                 m1, [second_predq+mmsize*0]
  pavgb                 m2, [second_predq+mmsize*1]
  pavgb                 m3, [second_predq+mmsize*2]
  pavgb                 m4, [second_predq+mmsize*3]
  lea         second_predq, [second_predq+mmsize*4]
%endif
  psadbw                m1, [srcq]
  psadbw                m2, [srcq+16]
  psadbw                m3, [srcq+src_strideq]
  psadbw                m4, [srcq+src_strideq+16]
  paddd                 m1, m2
  paddd                 m3, m4
  lea                 refq, [refq+ref_strideq*2]
  paddd                 m0, m1
  lea                 srcq, [srcq+src_strideq*2]
  paddd                 m0, m3
  dec              n_rowsd
  jg .loop

  movhlps               m1, m0
  paddd                 m0, m1
  movd                 eax, m0
  RET
%endmacro

INIT_XMM sse2
SAD32XN 64 ; sad32x64_sse2
SAD32XN 32 ; sad32x32_sse2
SAD32XN 16 ; sad32x16_sse2
SAD32XN 64, 1 ; sad32x64_avg_sse2
SAD32XN 32, 1 ; sad32x32_avg_sse2
SAD32XN 16, 1 ; sad32x16_avg_sse2
SAD32XN 8 ; sad_32x8_sse2
SAD32XN 8, 1 ; sad_32x8_avg_sse2

; unsigned int aom_sad16x{8,16}_sse2(uint8_t *src, int src_stride,
;                                    uint8_t *ref, int ref_stride);
%macro SAD16XN 1-2 0
  SAD_FN 16, %1, 7, %2
  mov              n_rowsd, %1/4
  pxor                  m0, m0

.loop:
  movu                  m1, [refq]
  movu                  m2, [refq+ref_strideq]
  movu                  m3, [refq+ref_strideq*2]
  movu                  m4, [refq+ref_stride3q]
%if %2 == 1
  pavgb                 m1, [second_predq+mmsize*0]
  pavgb                 m2, [second_predq+mmsize*1]
  pavgb                 m3, [second_predq+mmsize*2]
  pavgb                 m4, [second_predq+mmsize*3]
  lea         second_predq, [second_predq+mmsize*4]
%endif
  psadbw                m1, [srcq]
  psadbw                m2, [srcq+src_strideq]
  psadbw                m3, [srcq+src_strideq*2]
  psadbw                m4, [srcq+src_stride3q]
  paddd                 m1, m2
  paddd                 m3, m4
  lea                 refq, [refq+ref_strideq*4]
  paddd                 m0, m1
  lea                 srcq, [srcq+src_strideq*4]
  paddd                 m0, m3
  dec              n_rowsd
  jg .loop

  movhlps               m1, m0
  paddd                 m0, m1
  movd                 eax, m0
  RET
%endmacro

INIT_XMM sse2
SAD16XN 32 ; sad16x32_sse2
SAD16XN 16 ; sad16x16_sse2
SAD16XN  8 ; sad16x8_sse2
SAD16XN 32, 1 ; sad16x32_avg_sse2
SAD16XN 16, 1 ; sad16x16_avg_sse2
SAD16XN  8, 1 ; sad16x8_avg_sse2
SAD16XN 4 ; sad_16x4_sse2
SAD16XN 4, 1 ; sad_16x4_avg_sse2
SAD16XN 64 ; sad_16x64_sse2
SAD16XN 64, 1 ; sad_16x64_avg_sse2

; unsigned int aom_sad8x{8,16}_sse2(uint8_t *src, int src_stride,
;                                   uint8_t *ref, int ref_stride);
%macro SAD8XN 1-2 0
  SAD_FN 8, %1, 7, %2
  mov              n_rowsd, %1/4
  pxor                  m0, m0

.loop:
  movh                  m1, [refq]
  movhps                m1, [refq+ref_strideq]
  movh                  m2, [refq+ref_strideq*2]
  movhps                m2, [refq+ref_stride3q]
%if %2 == 1
  pavgb                 m1, [second_predq+mmsize*0]
  pavgb                 m2, [second_predq+mmsize*1]
  lea         second_predq, [second_predq+mmsize*2]
%endif
  movh                  m3, [srcq]
  movhps                m3, [srcq+src_strideq]
  movh                  m4, [srcq+src_strideq*2]
  movhps                m4, [srcq+src_stride3q]
  psadbw                m1, m3
  psadbw                m2, m4
  lea                 refq, [refq+ref_strideq*4]
  paddd                 m0, m1
  lea                 srcq, [srcq+src_strideq*4]
  paddd                 m0, m2
  dec              n_rowsd
  jg .loop

  movhlps               m1, m0
  paddd                 m0, m1
  movd                 eax, m0
  RET
%endmacro

INIT_XMM sse2
SAD8XN 16 ; sad8x16_sse2
SAD8XN  8 ; sad8x8_sse2
SAD8XN  4 ; sad8x4_sse2
SAD8XN 16, 1 ; sad8x16_avg_sse2
SAD8XN  8, 1 ; sad8x8_avg_sse2
SAD8XN  4, 1 ; sad8x4_avg_sse2
SAD8XN 32 ; sad_8x32_sse2
SAD8XN 32, 1 ; sad_8x32_avg_sse2

; unsigned int aom_sad4x{4, 8}_sse2(uint8_t *src, int src_stride,
;                                   uint8_t *ref, int ref_stride);
%macro SAD4XN 1-2 0
  SAD_FN 4, %1, 7, %2
  mov              n_rowsd, %1/4
  pxor                  m0, m0

.loop:
  movd                  m1, [refq]
  movd                  m2, [refq+ref_strideq]
  movd                  m3, [refq+ref_strideq*2]
  movd                  m4, [refq+ref_stride3q]
  punpckldq             m1, m2
  punpckldq             m3, m4
  movlhps               m1, m3
%if %2 == 1
  pavgb                 m1, [second_predq+mmsize*0]
  lea         second_predq, [second_predq+mmsize*1]
%endif
  movd                  m2, [srcq]
  movd                  m5, [srcq+src_strideq]
  movd                  m4, [srcq+src_strideq*2]
  movd                  m3, [srcq+src_stride3q]
  punpckldq             m2, m5
  punpckldq             m4, m3
  movlhps               m2, m4
  psadbw                m1, m2
  lea                 refq, [refq+ref_strideq*4]
  paddd                 m0, m1
  lea                 srcq, [srcq+src_strideq*4]
  dec              n_rowsd
  jg .loop

  movhlps               m1, m0
  paddd                 m0, m1
  movd                 eax, m0
  RET
%endmacro

INIT_XMM sse2
SAD4XN  8 ; sad4x8_sse
SAD4XN  4 ; sad4x4_sse
SAD4XN  8, 1 ; sad4x8_avg_sse
SAD4XN  4, 1 ; sad4x4_avg_sse
SAD4XN 16 ; sad_4x16_sse2
SAD4XN 16, 1 ; sad_4x16_avg_sse2
