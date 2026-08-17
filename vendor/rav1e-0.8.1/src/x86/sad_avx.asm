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

; unsigned int aom_sad128x128_avx2(uint8_t *src, int src_stride,
;                                  uint8_t *ref, int ref_stride);
%macro SAD128XN 1-2 0
  SAD_FN 128, %1, 5, %2
  mov              n_rowsd, %1
  pxor                  m0, m0

.loop:
  movu                  m1, [refq]
  movu                  m2, [refq+32]
  movu                  m3, [refq+64]
  movu                  m4, [refq+96]
%if %2 == 1
  vpavgb                m1, [second_predq+mmsize*0]
  vpavgb                m2, [second_predq+mmsize*1]
  vpavgb                m3, [second_predq+mmsize*2]
  vpavgb                m4, [second_predq+mmsize*3]
  lea         second_predq, [second_predq+mmsize*4]
%endif
  vpsadbw               m1, [srcq]
  vpsadbw               m2, [srcq+32]
  vpsadbw               m3, [srcq+64]
  vpsadbw               m4, [srcq+96]

  add                 refq, ref_strideq
  add                 srcq, src_strideq

  vpaddd                m1, m2
  vpaddd                m3, m4
  vpaddd                m0, m1
  vpaddd                m0, m3

  dec              n_rowsd
  jg .loop

  vextracti128         xm1, m0, 1
  paddd                xm0, xm1

  movhlps              xm1, xm0
  paddd                xm0, xm1
  movd                 eax, xm0

  RET
%endmacro

INIT_YMM avx2
SAD128XN 128     ; sad128x128_avx2
SAD128XN 128, 1  ; sad128x128_avg_avx2
SAD128XN 64      ; sad128x64_avx2
SAD128XN 64, 1   ; sad128x64_avg_avx2


; unsigned int aom_sad64x64_avx2(uint8_t *src, int src_stride,
;                               uint8_t *ref, int ref_stride);
%macro SAD64XN 1-2 0
  SAD_FN 64, %1, 5, %2
  mov              n_rowsd, %1/2
  pxor                  m0, m0
.loop:
  movu                  m1, [refq]
  movu                  m2, [refq+32]
  movu                  m3, [refq+ref_strideq]
  movu                  m4, [refq+ref_strideq+32]
%if %2 == 1
  vpavgb                m1, [second_predq+mmsize*0]
  vpavgb                m2, [second_predq+mmsize*1]
  vpavgb                m3, [second_predq+mmsize*2]
  vpavgb                m4, [second_predq+mmsize*3]
  lea         second_predq, [second_predq+mmsize*4]
%endif
  vpsadbw               m1, [srcq]
  vpsadbw               m2, [srcq+32]
  vpsadbw               m3, [srcq+src_strideq]
  vpsadbw               m4, [srcq+src_strideq+32]

  vpaddd                m1, m2
  vpaddd                m3, m4
  lea                 refq, [refq+ref_strideq*2]
  vpaddd                m0, m1
  lea                 srcq, [srcq+src_strideq*2]
  vpaddd                m0, m3
  dec              n_rowsd
  jg .loop

  vextracti128         xm1, m0, 1
  paddd                xm0, xm1

  movhlps              xm1, xm0
  paddd                xm0, xm1
  movd                 eax, xm0
  RET
%endmacro

INIT_YMM avx2
SAD64XN 128     ; sad64x128_avx2
SAD64XN 128, 1  ; sad64x128_avg_avx2
SAD64XN 64 ; sad64x64_avx2
SAD64XN 32 ; sad64x32_avx2
SAD64XN 64, 1 ; sad64x64_avg_avx2
SAD64XN 32, 1 ; sad64x32_avg_avx2
SAD64XN 16 ; sad64x16_avx2
SAD64XN 16, 1 ; sad64x16_avg_avx2


; unsigned int aom_sad32x32_avx2(uint8_t *src, int src_stride,
;                               uint8_t *ref, int ref_stride);
%macro SAD32XN 1-2 0
  SAD_FN 32, %1, 7, %2
  mov              n_rowsd, %1/4
  pxor                  m0, m0
.loop:
  movu                  m1, [refq]
  movu                  m2, [refq+ref_strideq]
  movu                  m3, [refq+ref_strideq*2]
  movu                  m4, [refq+ref_stride3q]
%if %2 == 1
  vpavgb                m1, [second_predq+mmsize*0]
  vpavgb                m2, [second_predq+mmsize*1]
  vpavgb                m3, [second_predq+mmsize*2]
  vpavgb                m4, [second_predq+mmsize*3]
  lea         second_predq, [second_predq+mmsize*4]
%endif
  psadbw                m1, [srcq]
  psadbw                m2, [srcq+src_strideq]
  psadbw                m3, [srcq+src_strideq*2]
  psadbw                m4, [srcq+src_stride3q]

  vpaddd                m1, m2
  vpaddd                m3, m4
  lea                 refq, [refq+ref_strideq*4]
  vpaddd                m0, m1
  lea                 srcq, [srcq+src_strideq*4]
  vpaddd                m0, m3
  dec              n_rowsd
  jg .loop

  vextracti128         xm1, m0, 1
  paddd                xm0, xm1

  movhlps              xm1, xm0
  paddd                xm0, xm1
  movd                 eax, xm0
  RET
%endmacro

INIT_YMM avx2
SAD32XN 64 ; sad32x64_avx2
SAD32XN 32 ; sad32x32_avx2
SAD32XN 16 ; sad32x16_avx2
SAD32XN 64, 1 ; sad32x64_avg_avx2
SAD32XN 32, 1 ; sad32x32_avg_avx2
SAD32XN 16, 1 ; sad32x16_avg_avx2
SAD32XN 8 ; sad_32x8_avx2
SAD32XN 8, 1 ; sad_32x8_avg_avx2
