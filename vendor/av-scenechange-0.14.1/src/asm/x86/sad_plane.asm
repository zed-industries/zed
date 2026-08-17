; Copyright (c) 2022, The rav1e contributors. All rights reserved
;
; This source code is subject to the terms of the BSD 2 Clause License and
; the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
; was not distributed with this source code in the LICENSE file, you can
; obtain it at www.aomedia.org/license/software. If the Alliance for Open
; Media Patent License 1.0 was not distributed with this source code in the
; PATENTS file, you can obtain it at www.aomedia.org/license/patent.

%include "config.asm"
%include "src/asm/x86/x86inc.asm"

%if ARCH_X86_64

SECTION_RODATA

align 32
mask_lut: db \
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0, 0, \
-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0,

%macro JMP_TABLE 3-*
  %xdefine %%func mangle(private_prefix %+ _%1_%2)
  %xdefine %%table %1_%2_table
  %%table:
  %rep %0 - 2
      dd (%%func %+ .%3) - (%%table)
      %rotate 1
  %endrep
%endmacro

JMP_TABLE sad_plane_8bpc, avx2, vec0, vec1, vec2, vec3
JMP_TABLE sad_plane_8bpc, sse2, vec0, vec1, vec2, vec3

%use ifunc

SECTION .text

%macro SAD_PLANE_FN 0
cglobal sad_plane_8bpc, 5, 9, 9, p1, p2, stride, width, rows, \
                      resid_simd, resid, width_unrll, tmp0
  mov     resid_simdq, widthq
  mov     residd, widthd
  and     residd, mmsize - 1
  and     resid_simdq, -(mmsize)
  and     widthq, -(4*mmsize)
  ; LUT row size is always 32 regardless of mmsize (because the
  ; start of the rows would be the same, so we reuse the same LUT)
  shl     residd, ilog2(32)
  pxor    xm0, xm0
  pxor    xm1, xm1
  pxor    xm2, xm2
  pxor    xm3, xm3
  ; load mask from lookup table into m8
  lea   tmp0q, [mask_lut]
  mova     m8, [tmp0q + residq]

  DEFINE_ARGS p1, p2, stride, width, rows, \
                      resid_simd, resid, width_unrll, skip_ptr

  sub     resid_simdq, widthq
  ; need to divide by mmsize to load skip pointer
  shr     resid_simdq, ilog2(mmsize)
%if mmsize == 32
  %define jmp_table sad_plane_8bpc_avx2_table
%elif mmsize == 16
  %define jmp_table sad_plane_8bpc_sse2_table
%endif
  lea        r6, [jmp_table]
  movsxd     skip_ptrq, [r6 + 4*resid_simdq]
  add        skip_ptrq, r6

  ; shift back (for residual to load correct number of bytes)
  shl     resid_simdq, ilog2(mmsize)
  ; set pointer to point after end of width of first row
  add     p1q, widthq
  add     p2q, widthq
  mov     width_unrllq, widthq
  neg     widthq
.loop_row:
  test    widthq, widthq
  jz     .skip
.loop:
  mova        m4,     [p1q + widthq + 0*mmsize]
  mova        m5,     [p1q + widthq + 1*mmsize]
  mova        m6,     [p1q + widthq + 2*mmsize]
  mova        m7,     [p1q + widthq + 3*mmsize]

  psadbw      m4, m4, [p2q + widthq + 0*mmsize]
  psadbw      m5, m5, [p2q + widthq + 1*mmsize]
  psadbw      m6, m6, [p2q + widthq + 2*mmsize]
  psadbw      m7, m7, [p2q + widthq + 3*mmsize]

  paddq       m0, m4
  paddq       m1, m5
  paddq       m2, m6
  paddq       m3, m7

  add         widthq, 4*mmsize
  jnz        .loop
.skip:
  jmp         skip_ptrq
.vec3:
  mova        m6,     [p1q + 2*mmsize]
  psadbw      m6, m6, [p2q + 2*mmsize]
  paddq       m2, m6
.vec2:
  mova        m5,     [p1q + 1*mmsize]
  psadbw      m5, m5, [p2q + 1*mmsize]
  paddq       m1, m5
.vec1:
  mova        m4,     [p1q + 0*mmsize]
  psadbw      m4, m4, [p2q + 0*mmsize]
  paddq       m0, m4
.vec0:
  ; skip residual element add if necessary
  test        residd, residd
  jz         .next_row
  ; load residual elements and mask out elements past the width
  pand        m4, m8, [p1q + resid_simdq]
  pand        m5, m8, [p2q + resid_simdq]
  psadbw      m4, m4, m5
  paddq       m2, m4
.next_row:
  ; width is 0 after the unrolled loop, so subtracting is basically a mov + neg
  sub        widthq, width_unrllq
  ; since we started with p1+width, adding stride will get the
  ; pointer at the end of the next row
  add           p1q, strideq
  add           p2q, strideq
  dec           rowsd
  jnz          .loop_row
  ; final horizontal reduction
  paddq         m2, m3
  paddq         m0, m1
  paddq         m0, m2
%if mmsize == 32
  vextracti128 xm1, ym0, 1
  paddq        xm0, xm1
%endif
  pshufd       xm1, xm0, q0032
  paddq        xm0, xm1
  movq         rax, xm0
  RET
%endmacro

INIT_XMM sse2
SAD_PLANE_FN

INIT_YMM avx2
SAD_PLANE_FN

%endif ; ARCH_X86_64
