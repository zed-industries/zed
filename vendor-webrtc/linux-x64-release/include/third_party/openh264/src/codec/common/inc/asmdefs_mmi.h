/*!
 * \copy
 *     Copyright (c)  2013, Loongson Technology Co.,Ltd.
 *     All rights reserved.
 *
 *     Redistribution and use in source and binary forms, with or without
 *     modification, are permitted provided that the following conditions
 *     are met:
 *
 *        * Redistributions of source code must retain the above copyright
 *          notice, this list of conditions and the following disclaimer.
 *
 *        * Redistributions in binary form must reproduce the above copyright
 *          notice, this list of conditions and the following disclaimer in
 *          the documentation and/or other materials provided with the
 *          distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *     "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *     LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *     FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 *     COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 *     INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 *     BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 *     CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 *     LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 *     ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 *     POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef ASMDEFS_MMI_H_
#define ASMDEFS_MMI_H_

#define CACHE_LINE_SIZE 32

#if defined(_ABI64) && _MIPS_SIM == _ABI64
# define mips_reg       int64_t
# define PTRSIZE        " 8 "
# define PTRLOG         " 3 "
# define PTR_ADDU       "daddu "
# define PTR_ADDIU      "daddiu "
# define PTR_ADDI       "daddi "
# define PTR_SUBU       "dsubu "
# define PTR_L          "ld "
# define PTR_S          "sd "
# define PTR_SRA        "dsra "
# define PTR_SRL        "dsrl "
# define PTR_SLL        "dsll "
#else
# define mips_reg       int32_t
# define PTRSIZE        " 4 "
# define PTRLOG         " 2 "
# define PTR_ADDU       "addu "
# define PTR_ADDIU      "addiu "
# define PTR_ADDI       "addi "
# define PTR_SUBU       "subu "
# define PTR_L          "lw "
# define PTR_S          "sw "
# define PTR_SRA        "sra "
# define PTR_SRL        "srl "
# define PTR_SLL        "sll "
#endif

#define MMI_XSawp_BH(f0, f2, f4, f6, f8, f10) \
  "mov.d      "#f8", "#f2"                \n\t" \
  "punpckhbh  "#f2", "#f0", "#f4"         \n\t" \
  "punpcklbh  "#f0", "#f0", "#f4"         \n\t" \
  "punpckhbh  "#f10", "#f8", "#f6"        \n\t" \
  "punpcklbh  "#f8", "#f8", "#f6"         \n\t"

#define MMI_XSawp_HW(f0, f2, f4, f6, f8, f10) \
  "mov.d      "#f8", "#f2"                \n\t" \
  "punpckhhw  "#f2", "#f0", "#f4"         \n\t" \
  "punpcklhw  "#f0", "#f0", "#f4"         \n\t" \
  "punpckhhw  "#f10", "#f8", "#f6"        \n\t" \
  "punpcklhw  "#f8", "#f8", "#f6"         \n\t"

#define MMI_XSawp_WD(f0, f2, f4, f6, f8, f10) \
  "mov.d      "#f8", "#f2"                \n\t" \
  "punpckhwd  "#f2", "#f0", "#f4"         \n\t" \
  "punpcklwd  "#f0", "#f0", "#f4"         \n\t" \
  "punpckhwd  "#f10", "#f8", "#f6"        \n\t" \
  "punpcklwd  "#f8", "#f8", "#f6"         \n\t"

#define MMI_XSawp_DQ(f0, f2, f4, f6, f8, f10) \
  "mov.d      "#f8", "#f2"                \n\t" \
  "mov.d      "#f2", "#f4"                \n\t" \
  "mov.d      "#f10", "#f6"               \n\t"

#define WELS_AbsH(f0, f2, f4, f6, f8, f10) \
  "xor        "#f8", "#f8", "#f8"         \n\t" \
  "psubh      "#f10", "#f8", "#f6"        \n\t" \
  "psubh      "#f8", "#f8", "#f4"         \n\t" \
  "pmaxsh     "#f0", "#f4", "#f8"         \n\t" \
  "pmaxsh     "#f2", "#f6", "#f10"        \n\t"

#define MMI_SumSub(f0, f2, f4, f6, f8, f10) \
  "mov.d      "#f8", "#f4"                    \n\t" \
  "mov.d      "#f10", "#f6"                   \n\t" \
  "paddh      "#f4", "#f4", "#f0"             \n\t" \
  "paddh      "#f6", "#f6", "#f2"             \n\t" \
  "psubh      "#f0", "#f0", "#f8"             \n\t" \
  "psubh      "#f2", "#f2", "#f10"            \n\t"

#define MMI_LoadDiff8P(f0, f2, f4, f6, f8, r0, r1) \
  "gsldlc1    "#f0", 0x7("#r0")               \n\t" \
  "gsldlc1    "#f4", 0x7("#r1")               \n\t" \
  "gsldrc1    "#f0", 0x0("#r0")               \n\t" \
  "gsldrc1    "#f4", 0x0("#r1")               \n\t" \
  "punpckhbh  "#f2", "#f0", "#f8"             \n\t" \
  "punpcklbh  "#f0", "#f0", "#f8"             \n\t" \
  "punpckhbh  "#f6", "#f4", "#f8"             \n\t" \
  "punpcklbh  "#f4", "#f4", "#f8"             \n\t" \
  "psubh      "#f0", "#f0", "#f4"             \n\t" \
  "psubh      "#f2", "#f2", "#f6"             \n\t"

#define MMI_TransTwo4x4H(f0, f2, f4, f6, f8, f10, f12, f14, f16, f18) \
  MMI_XSawp_HW(f0, f2, f4, f6, f16, f18)  \
  MMI_XSawp_HW(f8, f10, f12, f14, f4, f6) \
  MMI_XSawp_WD(f0, f2, f8, f10, f12, f14) \
  MMI_XSawp_WD(f16, f18, f4, f6, f8, f10) \
  MMI_XSawp_DQ(f0, f2, f16, f18, f4, f6)  \
  MMI_XSawp_DQ(f12, f14, f8, f10, f16, f18)

#define MMI_TransTwo8x8B(f0, f2, f4, f6, f8, f10, f12, f14, f16, f18, f20, f22, f24, f26, f28, f30, r0, r1) \
  "dmfc1      "#r0", "#f28"                   \n\t" \
  "dmfc1      "#r1", "#f30"                   \n\t" \
  MMI_XSawp_BH(f0, f2, f4, f6, f28, f30)            \
  MMI_XSawp_BH(f8, f10, f12, f14, f4, f6)           \
  MMI_XSawp_BH(f16, f18, f20, f22, f12, f14)        \
  "dmtc1      "#r0", "#f20"                   \n\t" \
  "dmtc1      "#r1", "#f22"                   \n\t" \
  "dmfc1      "#r0", "#f12"                   \n\t" \
  "dmfc1      "#r1", "#f14"                   \n\t" \
  MMI_XSawp_BH(f24, f26, f20, f22, f12, f14)        \
  MMI_XSawp_HW(f0, f2, f8, f10, f20, f22)           \
  MMI_XSawp_HW(f28, f30, f4, f6, f8, f10)           \
  MMI_XSawp_HW(f16, f18, f24, f26, f4, f6)          \
  "dmtc1      "#r0", "#f24"                   \n\t" \
  "dmtc1      "#r1", "#f26"                   \n\t" \
  "dmfc1      "#r0", "#f8"                    \n\t" \
  "dmfc1      "#r1", "#f10"                   \n\t" \
  MMI_XSawp_HW(f24, f26, f12, f14, f8, f10)         \
  MMI_XSawp_WD(f0, f2, f16, f18, f12, f14)          \
  MMI_XSawp_WD(f20, f22, f4, f6, f16, f18)          \
  MMI_XSawp_WD(f28, f30, f24, f26, f4, f6)          \
  "dmtc1      "#r0", "#f24"                   \n\t" \
  "dmtc1      "#r1", "#f26"                   \n\t" \
  "dmfc1      "#r0", "#f16"                   \n\t" \
  "dmfc1      "#r1", "#f18"                   \n\t" \
  MMI_XSawp_WD(f24, f26, f8, f10, f16, f18)         \
  MMI_XSawp_DQ(f0, f2, f28, f30, f8, f10)           \
  MMI_XSawp_DQ(f12, f14, f4, f6, f28, f30)          \
  MMI_XSawp_DQ(f20, f22, f24, f26, f4, f6)          \
  "dmtc1      "#r0", "#f24"                   \n\t" \
  "dmtc1      "#r1", "#f26"                   \n\t" \
  "dmfc1      "#r0", "#f0"                    \n\t" \
  "dmfc1      "#r1", "#f2"                    \n\t" \
  MMI_XSawp_DQ(f24, f26, f16, f18, f0, f2)          \
  "dmtc1      "#r0", "#f16"                   \n\t" \
  "dmtc1      "#r1", "#f18"                   \n\t"

#define MMI_XSwap_HW_SINGLE(f0, f2, f4) \
  "punpckhhw  "#f4", "#f0", "#f2"             \n\t" \
  "punpcklhw  "#f0", "#f0", "#f2"             \n\t"

#define MMI_XSwap_WD_SINGLE(f0, f2, f4) \
  "punpckhwd  "#f4", "#f0", "#f2"             \n\t" \
  "punpcklwd  "#f0", "#f0", "#f2"             \n\t"

#define MMI_Trans4x4H_SINGLE(f0, f2, f4, f6, f8) \
  MMI_XSwap_HW_SINGLE(f0, f2, f8)              \
  MMI_XSwap_HW_SINGLE(f4, f6, f2)              \
  MMI_XSwap_WD_SINGLE(f0, f4, f6)              \
  MMI_XSwap_WD_SINGLE(f8, f2, f4)

#define MMI_SumSub_SINGLE(f0, f2, f4) \
  "mov.d      "#f4", "#f2"                    \n\t" \
  "psubh      "#f2", "#f2", "#f0"             \n\t" \
  "paddh      "#f0", "#f0", "#f4"             \n\t"

#define MMI_SumSubMul2_SINGLE(f0, f2, f4, f6) \
  "mov.d      "#f4", "#f0"                    \n\t" \
  "psllh      "#f0", "#f0", "#f6"             \n\t" \
  "paddh      "#f0", "#f0", "#f2"             \n\t" \
  "psllh      "#f2", "#f2", "#f6"             \n\t" \
  "psubh      "#f4", "#f4", "#f2"             \n\t"

//f4 should be 0x0
#define MMI_Copy8Times(f0, f2, f4, r0) \
  "dmtc1      "#r0", "#f0"                    \n\t" \
  "pshufh     "#f0", "#f0", "#f4"             \n\t" \
  "mov.d      "#f2", "#f0"                    \n\t"

//f4 should be 0x0
#define MMI_Copy16Times(f0, f2, f4, r0) \
  "dmtc1      "#r0", "#f0"                    \n\t" \
  "punpcklbh  "#f0", "#f0", "#f0"             \n\t" \
  "pshufh     "#f0", "#f0", "#f4"             \n\t" \
  "mov.d      "#f2", "#f0"                    \n\t"

#define MMI_SumSubDiv2_SINGLE(f0, f2, f4, f6) \
  "psrah      "#f4", "#f2", "#f6"             \n\t" \
  "paddh      "#f4", "#f4", "#f0"             \n\t" \
  "psrah      "#f0", "#f0", "#f6"             \n\t" \
  "psubh      "#f0", "#f0", "#f2"             \n\t"

#define MMI_IDCT_SINGLE(f0, f2, f4, f6, f8, f10, f12) \
  MMI_SumSub_SINGLE(f6, f8, f10)             \
  MMI_SumSubDiv2_SINGLE(f4, f2, f0, f12)     \
  MMI_SumSub_SINGLE(f0, f6, f10)             \
  MMI_SumSub_SINGLE(f4, f8, f10)

#define MMI_StoreDiff4P_SINGLE(f0, f2, f4, f6, r0, r1, f8) \
  "gsldlc1    "#f2", 0x7("#r1")               \n\t" \
  "gsldrc1    "#f2", 0x0("#r1")               \n\t" \
  "punpcklbh  "#f2", "#f2", "#f6"             \n\t" \
  "paddh      "#f0", "#f0", "#f4"             \n\t" \
  "psrah      "#f0", "#f0", "#f8"             \n\t" \
  "paddsh     "#f0", "#f0", "#f2"             \n\t" \
  "packushb   "#f0", "#f0", "#f2"             \n\t" \
  "gsswlc1    "#f0", 0x3("#r0")               \n\t" \
  "gsswrc1    "#f0", 0x0("#r0")               \n\t"

#define SUMH_HORIZON(f0, f2, f4, f6, f8) \
  "paddh      "#f0", "#f0", "#f2"                       \n\t" \
  "punpckhhw  "#f2", "#f0", "#f8"                       \n\t" \
  "punpcklhw  "#f0", "#f0", "#f8"                       \n\t" \
  "paddw      "#f0", "#f0", "#f2"                       \n\t" \
  "punpckhwd  "#f2", "#f0", "#f0"                       \n\t" \
  "paddw      "#f0", "#f0", "#f2"                       \n\t"

#define LOAD_COLUMN(f0, f2, f4, f6, f8, f10, f12, f14, r0, r1, r2) \
  "daddu      "#r2", "#r0", "#r1"                       \n\t" \
  "gsldlc1    "#f0", 0x7("#r0")                         \n\t" \
  "gsldlc1    "#f4", 0x7("#r2")                         \n\t" \
  "gsldrc1    "#f0", 0x0("#r0")                         \n\t" \
  "gsldrc1    "#f4", 0x0("#r2")                         \n\t" \
  "punpcklbh  "#f0", "#f0", "#f4"                       \n\t" \
  "daddu      "#r0", "#r2", "#r1"                       \n\t" \
  "daddu      "#r2", "#r0", "#r1"                       \n\t" \
  "gsldlc1    "#f8", 0x7("#r0")                         \n\t" \
  "gsldlc1    "#f4", 0x7("#r2")                         \n\t" \
  "gsldrc1    "#f8", 0x0("#r0")                         \n\t" \
  "gsldrc1    "#f4", 0x0("#r2")                         \n\t" \
  "punpcklbh  "#f8", "#f8", "#f4"                       \n\t" \
  "punpckhhw  "#f2", "#f0", "#f8"                       \n\t" \
  "punpcklhw  "#f0", "#f0", "#f8"                       \n\t" \
  "daddu      "#r0", "#r2", "#r1"                       \n\t" \
  "daddu      "#r2", "#r0", "#r1"                       \n\t" \
  "gsldlc1    "#f12", 0x7("#r0")                        \n\t" \
  "gsldlc1    "#f4", 0x7("#r2")                         \n\t" \
  "gsldrc1    "#f12", 0x0("#r0")                        \n\t" \
  "gsldrc1    "#f4", 0x0("#r2")                         \n\t" \
  "punpcklbh  "#f12", "#f12", "#f4"                     \n\t" \
  "daddu      "#r0", "#r2", "#r1"                       \n\t" \
  "daddu      "#r2", "#r0", "#r1"                       \n\t" \
  "gsldlc1    "#f8", 0x7("#r0")                         \n\t" \
  "gsldlc1    "#f4", 0x7("#r2")                         \n\t" \
  "gsldrc1    "#f8", 0x0("#r0")                         \n\t" \
  "gsldrc1    "#f4", 0x0("#r2")                         \n\t" \
  "punpcklbh  "#f8", "#f8", "#f4"                       \n\t" \
  "punpckhhw  "#f14", "#f12", "#f8"                     \n\t" \
  "punpcklhw  "#f12", "#f12", "#f8"                     \n\t" \
  "daddu      "#r0", "#r2", "#r1"                       \n\t" \
  "punpcklwd  "#f0", "#f2", "#f14"                      \n\t" \
  "punpckhwd  "#f2", "#f2", "#f14"                      \n\t"

#define LOAD_COLUMN_C(f0, f2, f4, f6, r0, r1, r2) \
  "daddu      "#r2", "#r0", "#r1"                       \n\t" \
  "gsldlc1    "#f0", 0x7("#r0")                         \n\t" \
  "gsldlc1    "#f2", 0x7("#r2")                         \n\t" \
  "gsldrc1    "#f0", 0x0("#r0")                         \n\t" \
  "gsldrc1    "#f2", 0x0("#r2")                         \n\t" \
  "punpcklbh  "#f0", "#f0", "#f2"                       \n\t" \
  "daddu      "#r0", "#r2", "#r1"                       \n\t" \
  "daddu      "#r2", "#r0", "#r1"                       \n\t" \
  "gsldlc1    "#f4", 0x7("#r0")                         \n\t" \
  "gsldlc1    "#f2", 0x7("#r2")                         \n\t" \
  "gsldrc1    "#f4", 0x0("#r0")                         \n\t" \
  "gsldrc1    "#f2", 0x0("#r2")                         \n\t" \
  "punpcklbh  "#f4", "#f4", "#f2"                       \n\t" \
  "punpckhhw  "#f0", "#f0", "#f4"                       \n\t" \
  "daddu      "#r0", "#r2", "#r1"                       \n\t"

/**
 * backup register
 */
#if defined(_ABI64) && _MIPS_SIM == _ABI64
#define BACKUP_REG \
   double __attribute__((aligned(16))) __back_temp[8];         \
   __asm__ volatile (                                          \
     "gssqc1       $f25,      $f24,       0x00(%[temp])  \n\t" \
     "gssqc1       $f27,      $f26,       0x10(%[temp])  \n\t" \
     "gssqc1       $f29,      $f28,       0x20(%[temp])  \n\t" \
     "gssqc1       $f31,      $f30,       0x30(%[temp])  \n\t" \
     :                                                         \
     : [temp]"r"(__back_temp)                                  \
     : "memory"                                                \
   );
#else
#define BACKUP_REG \
   double __attribute__((aligned(16))) __back_temp[8];         \
   __asm__ volatile (                                          \
     "gssqc1       $f22,      $f20,       0x00(%[temp])  \n\t" \
     "gssqc1       $f26,      $f24,       0x10(%[temp])  \n\t" \
     "gssqc1       $f30,      $f28,       0x20(%[temp])  \n\t" \
     :                                                         \
     : [temp]"r"(__back_temp)                                  \
     : "memory"                                                \
   );
#endif

/**
 * recover register
 */
#if defined(_ABI64) && _MIPS_SIM == _ABI64
#define RECOVER_REG \
   __asm__ volatile (                                          \
     "gslqc1       $f25,      $f24,       0x00(%[temp])  \n\t" \
     "gslqc1       $f27,      $f26,       0x10(%[temp])  \n\t" \
     "gslqc1       $f29,      $f28,       0x20(%[temp])  \n\t" \
     "gslqc1       $f31,      $f30,       0x30(%[temp])  \n\t" \
     :                                                         \
     : [temp]"r"(__back_temp)                                  \
     : "memory"                                                \
   );
#else
#define RECOVER_REG \
   __asm__ volatile (                                          \
     "gslqc1       $f22,      $f20,       0x00(%[temp])  \n\t" \
     "gslqc1       $f26,      $f24,       0x10(%[temp])  \n\t" \
     "gslqc1       $f30,      $f28,       0x20(%[temp])  \n\t" \
     :                                                         \
     : [temp]"r"(__back_temp)                                  \
     : "memory"                                                \
   );
#endif

# define OK             1
# define NOTOK          0

#endif  /* ASMDEFS_MMI_H_ */
