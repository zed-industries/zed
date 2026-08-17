/*
 * Copyright (C) 2007-2020 Free Software Foundation, Inc.
 *
 * This file is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 3, or (at your option) any
 * later version.
 * 
 * This file is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * Under Section 7 of GPL version 3, you are granted additional
 * permissions described in the GCC Runtime Library Exception, version
 * 3.1, as published by the Free Software Foundation.
 * 
 * You should have received a copy of the GNU General Public License and
 * a copy of the GCC Runtime Library Exception along with this program;
 * see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
 * <http://www.gnu.org/licenses/>.
 */

/* %eax */
#define bit_AVX512BF16	(1 << 5)

/* %ecx */
#define bit_SSE3	(1 << 0)
#define bit_PCLMUL	(1 << 1)
#define bit_LZCNT	(1 << 5)
#define bit_SSSE3	(1 << 9)
#define bit_FMA		(1 << 12)
#define bit_CMPXCHG16B	(1 << 13)
#define bit_SSE4_1	(1 << 19)
#define bit_SSE4_2	(1 << 20)
#define bit_MOVBE	(1 << 22)
#define bit_POPCNT	(1 << 23)
#define bit_AES		(1 << 25)
#define bit_XSAVE	(1 << 26)
#define bit_OSXSAVE	(1 << 27)
#define bit_AVX		(1 << 28)
#define bit_F16C	(1 << 29)
#define bit_RDRND	(1 << 30)

/* %edx */
#define bit_CMPXCHG8B	(1 << 8)
#define bit_CMOV	(1 << 15)
#define bit_MMX		(1 << 23)
#define bit_FXSAVE	(1 << 24)
#define bit_SSE		(1 << 25)
#define bit_SSE2	(1 << 26)

/* Extended Features (%eax == 0x80000001) */
/* %ecx */
#define bit_LAHF_LM	(1 << 0)
#define bit_ABM		(1 << 5)
#define bit_SSE4a	(1 << 6)
#define bit_PRFCHW	(1 << 8)
#define bit_XOP         (1 << 11)
#define bit_LWP 	(1 << 15)
#define bit_FMA4        (1 << 16)
#define bit_TBM         (1 << 21)
#define bit_MWAITX      (1 << 29)

/* %edx */
#define bit_MMXEXT	(1 << 22)
#define bit_LM		(1 << 29)
#define bit_3DNOWP	(1 << 30)
#define bit_3DNOW	(1u << 31)

/* %ebx  */
#define bit_CLZERO	(1 << 0)
#define bit_WBNOINVD	(1 << 9)

/* Extended Features (%eax == 7) */
/* %ebx */
#define bit_FSGSBASE	(1 << 0)
#define bit_SGX (1 << 2)
#define bit_BMI	(1 << 3)
#define bit_HLE	(1 << 4)
#define bit_AVX2	(1 << 5)
#define bit_BMI2	(1 << 8)
#define bit_RTM	(1 << 11)
#define bit_MPX	(1 << 14)
#define bit_AVX512F	(1 << 16)
#define bit_AVX512DQ	(1 << 17)
#define bit_RDSEED	(1 << 18)
#define bit_ADX	(1 << 19)
#define bit_AVX512IFMA	(1 << 21)
#define bit_CLFLUSHOPT	(1 << 23)
#define bit_CLWB	(1 << 24)
#define bit_AVX512PF	(1 << 26)
#define bit_AVX512ER	(1 << 27)
#define bit_AVX512CD	(1 << 28)
#define bit_SHA		(1 << 29)
#define bit_AVX512BW	(1 << 30)
#define bit_AVX512VL	(1u << 31)

/* %ecx */
#define bit_PREFETCHWT1	  (1 << 0)
#define bit_AVX512VBMI	(1 << 1)
#define bit_PKU	(1 << 3)
#define bit_OSPKE	(1 << 4)
#define bit_WAITPKG	(1 << 5)
#define bit_AVX512VBMI2	(1 << 6)
#define bit_SHSTK	(1 << 7)
#define bit_GFNI	(1 << 8)
#define bit_VAES	(1 << 9)
#define bit_AVX512VNNI	(1 << 11)
#define bit_VPCLMULQDQ	(1 << 10)
#define bit_AVX512BITALG	(1 << 12)
#define bit_AVX512VPOPCNTDQ	(1 << 14)
#define bit_RDPID	(1 << 22)
#define bit_MOVDIRI	(1 << 27)
#define bit_MOVDIR64B	(1 << 28)
#define bit_ENQCMD	(1 << 29)
#define bit_CLDEMOTE	(1 << 25)

/* %edx */
#define bit_AVX5124VNNIW (1 << 2)
#define bit_AVX5124FMAPS (1 << 3)
#define bit_AVX512VP2INTERSECT	(1 << 8)
#define bit_IBT	(1 << 20)
#define bit_PCONFIG	(1 << 18)
/* XFEATURE_ENABLED_MASK register bits (%eax == 13, %ecx == 0) */
#define bit_BNDREGS     (1 << 3)
#define bit_BNDCSR      (1 << 4)

/* Extended State Enumeration Sub-leaf (%eax == 13, %ecx == 1) */
#define bit_XSAVEOPT	(1 << 0)
#define bit_XSAVEC	(1 << 1)
#define bit_XSAVES	(1 << 3)

/* PT sub leaf (%eax == 14, %ecx == 0) */
/* %ebx */
#define bit_PTWRITE	(1 << 4)

/* Signatures for different CPU implementations as returned in uses
   of cpuid with level 0.  */
#define signature_AMD_ebx	0x68747541
#define signature_AMD_ecx	0x444d4163
#define signature_AMD_edx	0x69746e65

#define signature_CENTAUR_ebx	0x746e6543
#define signature_CENTAUR_ecx	0x736c7561
#define signature_CENTAUR_edx	0x48727561

#define signature_CYRIX_ebx	0x69727943
#define signature_CYRIX_ecx	0x64616574
#define signature_CYRIX_edx	0x736e4978

#define signature_INTEL_ebx	0x756e6547
#define signature_INTEL_ecx	0x6c65746e
#define signature_INTEL_edx	0x49656e69

#define signature_TM1_ebx	0x6e617254
#define signature_TM1_ecx	0x55504361
#define signature_TM1_edx	0x74656d73

#define signature_TM2_ebx	0x756e6547
#define signature_TM2_ecx	0x3638784d
#define signature_TM2_edx	0x54656e69

#define signature_NSC_ebx	0x646f6547
#define signature_NSC_ecx	0x43534e20
#define signature_NSC_edx	0x79622065

#define signature_NEXGEN_ebx	0x4778654e
#define signature_NEXGEN_ecx	0x6e657669
#define signature_NEXGEN_edx	0x72446e65

#define signature_RISE_ebx	0x65736952
#define signature_RISE_ecx	0x65736952
#define signature_RISE_edx	0x65736952

#define signature_SIS_ebx	0x20536953
#define signature_SIS_ecx	0x20536953
#define signature_SIS_edx	0x20536953

#define signature_UMC_ebx	0x20434d55
#define signature_UMC_ecx	0x20434d55
#define signature_UMC_edx	0x20434d55

#define signature_VIA_ebx	0x20414956
#define signature_VIA_ecx	0x20414956
#define signature_VIA_edx	0x20414956

#define signature_VORTEX_ebx	0x74726f56
#define signature_VORTEX_ecx	0x436f5320
#define signature_VORTEX_edx	0x36387865

#ifndef __x86_64__
/* At least one cpu (Winchip 2) does not set %ebx and %ecx
   for cpuid leaf 1. Forcibly zero the two registers before
   calling cpuid as a precaution.  */
#define __cpuid(level, a, b, c, d)			\
  do {							\
    if (__builtin_constant_p (level) && (level) != 1)	\
      __asm__ ("cpuid\n\t"				\
	      : "=a" (a), "=b" (b), "=c" (c), "=d" (d)	\
	      : "0" (level));				\
    else						\
      __asm__ ("cpuid\n\t"				\
	      : "=a" (a), "=b" (b), "=c" (c), "=d" (d)	\
	      : "0" (level), "1" (0), "2" (0));		\
  } while (0)
#else
#define __cpuid(level, a, b, c, d)			\
  __asm__ ("cpuid\n\t"					\
	   : "=a" (a), "=b" (b), "=c" (c), "=d" (d)	\
	   : "0" (level))
#endif

#define __cpuid_count(level, count, a, b, c, d)		\
  __asm__ ("cpuid\n\t"					\
	   : "=a" (a), "=b" (b), "=c" (c), "=d" (d)	\
	   : "0" (level), "2" (count))


/* Return highest supported input value for cpuid instruction.  ext can
   be either 0x0 or 0x80000000 to return highest supported value for
   basic or extended cpuid information.  Function returns 0 if cpuid
   is not supported or whatever cpuid returns in eax register.  If sig
   pointer is non-null, then first four bytes of the signature
   (as found in ebx register) are returned in location pointed by sig.  */

static __inline unsigned int
__get_cpuid_max (unsigned int __ext, unsigned int *__sig)
{
  unsigned int __eax, __ebx, __ecx, __edx;

#ifndef __x86_64__
  /* See if we can use cpuid.  On AMD64 we always can.  */
#if __GNUC__ >= 3
  __asm__ ("pushf{l|d}\n\t"
	   "pushf{l|d}\n\t"
	   "pop{l}\t%0\n\t"
	   "mov{l}\t{%0, %1|%1, %0}\n\t"
	   "xor{l}\t{%2, %0|%0, %2}\n\t"
	   "push{l}\t%0\n\t"
	   "popf{l|d}\n\t"
	   "pushf{l|d}\n\t"
	   "pop{l}\t%0\n\t"
	   "popf{l|d}\n\t"
	   : "=&r" (__eax), "=&r" (__ebx)
	   : "i" (0x00200000));
#else
/* Host GCCs older than 3.0 weren't supporting Intel asm syntax
   nor alternatives in i386 code.  */
  __asm__ ("pushfl\n\t"
	   "pushfl\n\t"
	   "popl\t%0\n\t"
	   "movl\t%0, %1\n\t"
	   "xorl\t%2, %0\n\t"
	   "pushl\t%0\n\t"
	   "popfl\n\t"
	   "pushfl\n\t"
	   "popl\t%0\n\t"
	   "popfl\n\t"
	   : "=&r" (__eax), "=&r" (__ebx)
	   : "i" (0x00200000));
#endif

  if (!((__eax ^ __ebx) & 0x00200000))
    return 0;
#endif

  /* Host supports cpuid.  Return highest supported cpuid input value.  */
  __cpuid (__ext, __eax, __ebx, __ecx, __edx);

  if (__sig)
    *__sig = __ebx;

  return __eax;
}

/* Return cpuid data for requested cpuid leaf, as found in returned
   eax, ebx, ecx and edx registers.  The function checks if cpuid is
   supported and returns 1 for valid cpuid information or 0 for
   unsupported cpuid leaf.  All pointers are required to be non-null.  */

static __inline int
__get_cpuid (unsigned int __leaf,
	     unsigned int *__eax, unsigned int *__ebx,
	     unsigned int *__ecx, unsigned int *__edx)
{
  unsigned int __ext = __leaf & 0x80000000;
  unsigned int __maxlevel = __get_cpuid_max (__ext, 0);

  if (__maxlevel == 0 || __maxlevel < __leaf)
    return 0;

  __cpuid (__leaf, *__eax, *__ebx, *__ecx, *__edx);
  return 1;
}

/* Same as above, but sub-leaf can be specified.  */

static __inline int
__get_cpuid_count (unsigned int __leaf, unsigned int __subleaf,
		   unsigned int *__eax, unsigned int *__ebx,
		   unsigned int *__ecx, unsigned int *__edx)
{
  unsigned int __ext = __leaf & 0x80000000;
  unsigned int __maxlevel = __get_cpuid_max (__ext, 0);

  if (__maxlevel == 0 || __maxlevel < __leaf)
    return 0;

  __cpuid_count (__leaf, __subleaf, *__eax, *__ebx, *__ecx, *__edx);
  return 1;
}
