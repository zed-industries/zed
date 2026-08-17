
/*---------------------------------------------------------------*/
/*--- begin                                          libvex.h ---*/
/*---------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2004-2017 OpenWorks LLP
      info@open-works.net

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, see <http://www.gnu.org/licenses/>.

   The GNU General Public License is contained in the file COPYING.

   Neither the names of the U.S. Department of Energy nor the
   University of California nor the names of its contributors may be
   used to endorse or promote products derived from this software
   without prior written permission.
*/

#ifndef __LIBVEX_H
#define __LIBVEX_H


#include "libvex_basictypes.h"
#include "libvex_ir.h"


/*---------------------------------------------------------------*/
/*--- This file defines the top-level interface to LibVEX.    ---*/
/*---------------------------------------------------------------*/

/*-------------------------------------------------------*/
/*--- Architectures, variants, and other arch info    ---*/
/*-------------------------------------------------------*/

typedef 
   enum { 
      VexArch_INVALID=0x400,
      VexArchX86, 
      VexArchAMD64, 
      VexArchARM,
      VexArchARM64,
      VexArchPPC32,
      VexArchPPC64,
      VexArchS390X,
      VexArchMIPS32,
      VexArchMIPS64,
      VexArchNANOMIPS,
   }
   VexArch;


/* Information about endianness. */
typedef
   enum {
      VexEndness_INVALID=0x600, /* unknown endianness */
      VexEndnessLE,             /* little endian */
      VexEndnessBE              /* big endian */
   }
   VexEndness;


/* For a given architecture, these specify extra capabilities beyond
   the minimum supported (baseline) capabilities.  They may be OR'd
   together, although some combinations don't make sense.  (eg, SSE2
   but not SSE1).  LibVEX_Translate will check for nonsensical
   combinations. */

/* x86: baseline capability is Pentium-1 (FPU, MMX, but no SSE), with
   cmpxchg8b. MMXEXT is a special AMD only subset of SSE1 (Integer SSE). */
#define VEX_HWCAPS_X86_MMXEXT  (1<<1)  /* A subset of SSE1 on early AMD */
#define VEX_HWCAPS_X86_SSE1    (1<<2)  /* SSE1 support (Pentium III) */
#define VEX_HWCAPS_X86_SSE2    (1<<3)  /* SSE2 support (Pentium 4) */
#define VEX_HWCAPS_X86_SSE3    (1<<4)  /* SSE3 support (>= Prescott) */
#define VEX_HWCAPS_X86_LZCNT   (1<<5)  /* SSE4a LZCNT insn */

/* amd64: baseline capability is SSE2, with cmpxchg8b but not
   cmpxchg16b. */
#define VEX_HWCAPS_AMD64_SSE3   (1<<5)  /* SSE3 support */
#define VEX_HWCAPS_AMD64_SSSE3  (1<<12) /* Supplemental SSE3 support */
#define VEX_HWCAPS_AMD64_CX16   (1<<6)  /* cmpxchg16b support */
#define VEX_HWCAPS_AMD64_LZCNT  (1<<7)  /* SSE4a LZCNT insn */
#define VEX_HWCAPS_AMD64_AVX    (1<<8)  /* AVX instructions */
#define VEX_HWCAPS_AMD64_RDTSCP (1<<9)  /* RDTSCP instruction */
#define VEX_HWCAPS_AMD64_BMI    (1<<10) /* BMI1 instructions */
#define VEX_HWCAPS_AMD64_AVX2   (1<<11) /* AVX2 instructions */
#define VEX_HWCAPS_AMD64_RDRAND (1<<13) /* RDRAND instructions */
#define VEX_HWCAPS_AMD64_F16C   (1<<14) /* F16C instructions */

/* ppc32: baseline capability is integer only */
#define VEX_HWCAPS_PPC32_F     (1<<8)  /* basic (non-optional) FP */
#define VEX_HWCAPS_PPC32_V     (1<<9)  /* Altivec (VMX) */
#define VEX_HWCAPS_PPC32_FX    (1<<10) /* FP extns (fsqrt, fsqrts) */
#define VEX_HWCAPS_PPC32_GX    (1<<11) /* Graphics extns
                                          (fres,frsqrte,fsel,stfiwx) */
#define VEX_HWCAPS_PPC32_VX    (1<<12) /* Vector-scalar floating-point (VSX); implies ISA 2.06 or higher  */
#define VEX_HWCAPS_PPC32_DFP   (1<<17) /* Decimal Floating Point (DFP) -- e.g., dadd */
#define VEX_HWCAPS_PPC32_ISA2_07   (1<<19) /* ISA 2.07 -- e.g., mtvsrd */
#define VEX_HWCAPS_PPC32_ISA3_0    (1<<21) /* ISA 3.0  -- e.g., cnttzw */

/* ppc64: baseline capability is integer and basic FP insns */
#define VEX_HWCAPS_PPC64_V     (1<<13) /* Altivec (VMX) */
#define VEX_HWCAPS_PPC64_FX    (1<<14) /* FP extns (fsqrt, fsqrts) */
#define VEX_HWCAPS_PPC64_GX    (1<<15) /* Graphics extns
                                          (fres,frsqrte,fsel,stfiwx) */
#define VEX_HWCAPS_PPC64_VX    (1<<16) /* Vector-scalar floating-point (VSX); implies ISA 2.06 or higher  */
#define VEX_HWCAPS_PPC64_DFP   (1<<18) /* Decimal Floating Point (DFP) -- e.g., dadd */
#define VEX_HWCAPS_PPC64_ISA2_07   (1<<20) /* ISA 2.07 -- e.g., mtvsrd */
#define VEX_HWCAPS_PPC64_ISA3_0    (1<<22) /* ISA 3.0  -- e.g., cnttzw */

/* s390x: Hardware capability encoding

   Bits [26:31] encode the machine model (see VEX_S390X_MODEL... below)
   Bits [0:20]  encode specific hardware capabilities
                (see VEX_HWAPS_S390X_... below)
*/

/* Model numbers must be assigned in chronological order.
   They are used as array index. */
#define VEX_S390X_MODEL_Z900     0
#define VEX_S390X_MODEL_Z800     1
#define VEX_S390X_MODEL_Z990     2
#define VEX_S390X_MODEL_Z890     3
#define VEX_S390X_MODEL_Z9_EC    4
#define VEX_S390X_MODEL_Z9_BC    5
#define VEX_S390X_MODEL_Z10_EC   6
#define VEX_S390X_MODEL_Z10_BC   7
#define VEX_S390X_MODEL_Z196     8
#define VEX_S390X_MODEL_Z114     9
#define VEX_S390X_MODEL_ZEC12    10
#define VEX_S390X_MODEL_ZBC12    11
#define VEX_S390X_MODEL_Z13      12
#define VEX_S390X_MODEL_Z13S     13
#define VEX_S390X_MODEL_Z14      14
#define VEX_S390X_MODEL_Z14_ZR1  15
#define VEX_S390X_MODEL_Z15      16
#define VEX_S390X_MODEL_UNKNOWN  17     /* always last in list */
#define VEX_S390X_MODEL_MASK     0x3F

#define VEX_HWCAPS_S390X_LDISP (1<<6)   /* Long-displacement facility */
#define VEX_HWCAPS_S390X_EIMM  (1<<7)   /* Extended-immediate facility */
#define VEX_HWCAPS_S390X_GIE   (1<<8)   /* General-instruction-extension facility */
#define VEX_HWCAPS_S390X_DFP   (1<<9)   /* Decimal floating point facility */
#define VEX_HWCAPS_S390X_FGX   (1<<10)  /* FPR-GR transfer facility */
#define VEX_HWCAPS_S390X_ETF2  (1<<11)  /* ETF2-enhancement facility */
#define VEX_HWCAPS_S390X_STFLE (1<<12)  /* STFLE facility */
#define VEX_HWCAPS_S390X_ETF3  (1<<13)  /* ETF3-enhancement facility */
#define VEX_HWCAPS_S390X_STCKF (1<<14)  /* STCKF facility */
#define VEX_HWCAPS_S390X_FPEXT (1<<15)  /* Floating point extension facility */
#define VEX_HWCAPS_S390X_LSC   (1<<16)  /* Conditional load/store facility */
#define VEX_HWCAPS_S390X_PFPO  (1<<17)  /* Perform floating point ops facility */
#define VEX_HWCAPS_S390X_VX    (1<<18)  /* Vector facility */
#define VEX_HWCAPS_S390X_MSA5  (1<<19)  /* message security assistance facility */
#define VEX_HWCAPS_S390X_MI2   (1<<20)  /* miscellaneous-instruction-extensions facility 2 */
#define VEX_HWCAPS_S390X_LSC2  (1<<21)  /* Conditional load/store facility2 */


/* Special value representing all available s390x hwcaps */
#define VEX_HWCAPS_S390X_ALL   (VEX_HWCAPS_S390X_LDISP | \
                                VEX_HWCAPS_S390X_EIMM  | \
                                VEX_HWCAPS_S390X_GIE   | \
                                VEX_HWCAPS_S390X_DFP   | \
                                VEX_HWCAPS_S390X_FGX   | \
                                VEX_HWCAPS_S390X_STFLE | \
                                VEX_HWCAPS_S390X_STCKF | \
                                VEX_HWCAPS_S390X_FPEXT | \
                                VEX_HWCAPS_S390X_LSC   | \
                                VEX_HWCAPS_S390X_ETF3  | \
                                VEX_HWCAPS_S390X_ETF2  | \
                                VEX_HWCAPS_S390X_PFPO  | \
                                VEX_HWCAPS_S390X_VX    | \
                                VEX_HWCAPS_S390X_MSA5  | \
                                VEX_HWCAPS_S390X_MI2   | \
                                VEX_HWCAPS_S390X_LSC2)

#define VEX_HWCAPS_S390X(x)  ((x) & ~VEX_S390X_MODEL_MASK)
#define VEX_S390X_MODEL(x)   ((x) &  VEX_S390X_MODEL_MASK)

/* arm: baseline capability is ARMv4 */
/* Bits 5:0 - architecture level (e.g. 5 for v5, 6 for v6 etc) */
#define VEX_HWCAPS_ARM_VFP    (1<<6)  /* VFP extension */
#define VEX_HWCAPS_ARM_VFP2   (1<<7)  /* VFPv2 */
#define VEX_HWCAPS_ARM_VFP3   (1<<8)  /* VFPv3 */
/* Bits 15:10 reserved for (possible) future VFP revisions */
#define VEX_HWCAPS_ARM_NEON   (1<<16) /* Advanced SIMD also known as NEON */

/* Get an ARM architecure level from HWCAPS */
#define VEX_ARM_ARCHLEVEL(x) ((x) & 0x3f)

/* ARM64: baseline capability is AArch64 v8. */
/* (no definitions since no variants so far) */

/* MIPS baseline capability */
/* Assigned Company values for bits 23:16 of the PRId Register
   (CP0 register 15, select 0).  As of the MIPS32 and MIPS64 specs from
   MTI, the PRId register is defined in this (backwards compatible)
   way:

  +----------------+----------------+----------------+----------------+
  | Company Options| Company ID     | Processor ID   | Revision       |
  +----------------+----------------+----------------+----------------+
   31            24 23            16 15             8 7

*/

#define VEX_PRID_COMP_LEGACY      0x00000000
#define VEX_PRID_COMP_MIPS        0x00010000
#define VEX_PRID_COMP_BROADCOM    0x00020000
#define VEX_PRID_COMP_NETLOGIC    0x000C0000
#define VEX_PRID_COMP_CAVIUM      0x000D0000
#define VEX_PRID_COMP_INGENIC_E1  0x00E10000        /* JZ4780 */

/*
 * These are valid when 23:16 == PRID_COMP_LEGACY
 */
#define VEX_PRID_IMP_LOONGSON_64        0x6300  /* Loongson-2/3 */

/*
 * These are the PRID's for when 23:16 == PRID_COMP_MIPS
 */
#define VEX_PRID_IMP_34K                0x9500
#define VEX_PRID_IMP_74K                0x9700
#define VEX_PRID_IMP_P5600              0xa800

/*
 * Instead of Company Options values, bits 31:24 will be packed with
 * additional information, such as isa level and FP mode.
 */
#define VEX_MIPS_CPU_ISA_M32R1      0x01000000
#define VEX_MIPS_CPU_ISA_M32R2      0x02000000
#define VEX_MIPS_CPU_ISA_M64R1      0x04000000
#define VEX_MIPS_CPU_ISA_M64R2      0x08000000
#define VEX_MIPS_CPU_ISA_M32R6      0x10000000
#define VEX_MIPS_CPU_ISA_M64R6      0x20000000
/* FP mode is FR = 1 (32 dbl. prec. FP registers) */
#define VEX_MIPS_HOST_FR            0x40000000
/* Get MIPS Extended Information */
#define VEX_MIPS_EX_INFO(x) ((x) & 0xFF000000)
/* Get MIPS Company ID from HWCAPS */
#define VEX_MIPS_COMP_ID(x) ((x) & 0x00FF0000)
/* Get MIPS Processor ID from HWCAPS */
#define VEX_MIPS_PROC_ID(x) ((x) & 0x0000FF00)
/* Get MIPS Revision from HWCAPS */
#define VEX_MIPS_REV(x) ((x) & 0x000000FF)
/* Get host FP mode */
#define VEX_MIPS_HOST_FP_MODE(x) (!!(VEX_MIPS_EX_INFO(x) & VEX_MIPS_HOST_FR))
/* Check if the processor supports MIPS32R2. */
#define VEX_MIPS_CPU_HAS_MIPS32R2(x) (VEX_MIPS_EX_INFO(x) & \
                                      VEX_MIPS_CPU_ISA_M32R2)
/* Check if the processor supports MIPS64R2. */
#define VEX_MIPS_CPU_HAS_MIPS64R2(x) (VEX_MIPS_EX_INFO(x) & \
                                      VEX_MIPS_CPU_ISA_M64R2)
/* Check if the processor supports MIPSR6. */
#define VEX_MIPS_CPU_HAS_MIPSR6(x) (VEX_MIPS_EX_INFO(x) & \
                                    (VEX_MIPS_CPU_ISA_M32R6 | \
                                    VEX_MIPS_CPU_ISA_M64R6))
/* Check if the processor supports DSP ASE Rev 2. */
#define VEX_MIPS_PROC_DSP2(x) ((VEX_MIPS_COMP_ID(x) == VEX_PRID_COMP_MIPS) && \
                               (VEX_MIPS_PROC_ID(x) == VEX_PRID_IMP_74K))
/* Check if the processor supports DSP ASE Rev 1. */
#define VEX_MIPS_PROC_DSP(x)  (VEX_MIPS_PROC_DSP2(x) || \
                               ((VEX_MIPS_COMP_ID(x) == VEX_PRID_COMP_MIPS) && \
                               (VEX_MIPS_PROC_ID(x) == VEX_PRID_IMP_34K)))

/* Check if the processor supports MIPS MSA (SIMD)*/
#define VEX_MIPS_PROC_MSA(x) ((VEX_MIPS_COMP_ID(x) == VEX_PRID_COMP_MIPS) && \
                              (VEX_MIPS_PROC_ID(x) == VEX_PRID_IMP_P5600) && \
                              (VEX_MIPS_HOST_FP_MODE(x)))

/* These return statically allocated strings. */

extern const HChar* LibVEX_ppVexArch    ( VexArch );
extern const HChar* LibVEX_ppVexEndness ( VexEndness endness );
extern const HChar* LibVEX_ppVexHwCaps  ( VexArch, UInt );


/* The various kinds of caches */
typedef enum {
   DATA_CACHE=0x500,
   INSN_CACHE,
   UNIFIED_CACHE
} VexCacheKind;

/* Information about a particular cache */
typedef struct {
   VexCacheKind kind;
   UInt level;         /* level this cache is at, e.g. 1 for L1 cache */
   UInt sizeB;         /* size of this cache in bytes */
   UInt line_sizeB;    /* cache line size in bytes */
   UInt assoc;         /* set associativity */
   Bool is_trace_cache;  /* False, except for certain Pentium 4 models */
} VexCache;

/* Convenience macro to initialise a VexCache */
#define VEX_CACHE_INIT(_kind, _level, _size, _line_size, _assoc)         \
         ({ (VexCache) { .kind = _kind, .level = _level, .sizeB = _size, \
               .line_sizeB = _line_size, .assoc = _assoc, \
               .is_trace_cache = False }; })

/* Information about the cache system as a whole */
typedef struct {
   UInt num_levels;
   UInt num_caches;
   /* Unordered array of caches for this host. NULL if there are
      no caches. The following can always be assumed:
      (1) There is at most one cache of a given kind per cache level.
      (2) If there exists a unified cache at a particular level then
          no other cache exists at that level.
      (3) The existence of a cache at level N > 1 implies the existence of
          at least one cache at level N-1. */
   VexCache *caches;
   Bool icaches_maintain_coherence;
} VexCacheInfo;


/* This struct is a bit of a hack, but is needed to carry misc
   important bits of info about an arch.  Fields which are meaningless
   or ignored for the platform in question should be set to zero.
   Nb: if you add fields to the struct make sure to update function
   LibVEX_default_VexArchInfo. */

typedef
   struct {
      /* The following three fields are mandatory. */
      UInt         hwcaps;
      VexEndness   endness;
      VexCacheInfo hwcache_info;
      /* PPC32/PPC64 only: size of instruction cache line */
      Int ppc_icache_line_szB;
      /* PPC32/PPC64 only: sizes zeroed by the dcbz/dcbzl instructions
         (bug#135264) */
      UInt ppc_dcbz_szB;
      UInt ppc_dcbzl_szB; /* 0 means unsupported (SIGILL) */
      /* ARM64: I- and D- minimum line sizes in log2(bytes), as
         obtained from ctr_el0.DminLine and .IminLine.  For example, a
         line size of 64 bytes would be encoded here as 6. */
      UInt arm64_dMinLine_lg2_szB;
      UInt arm64_iMinLine_lg2_szB;
      /* ARM64: does the host require us to use the fallback LLSC
         implementation? */
      Bool arm64_requires_fallback_LLSC;
   }
   VexArchInfo;

/* Write default settings info *vai. */
extern 
void LibVEX_default_VexArchInfo ( /*OUT*/VexArchInfo* vai );


/* This struct carries guest and host ABI variant information that may
   be needed.  Fields which are meaningless or ignored for the
   platform in question should be set to zero.

   Settings which are believed to be correct are:

   guest_stack_redzone_size
      guest is ppc32-linux                ==> 0
      guest is ppc64-linux                ==> 288
      guest is amd64-linux                ==> 128
      guest is other                      ==> inapplicable

   guest_amd64_assume_fs_is_const
      guest is amd64-linux                ==> True
      guest is amd64-darwin               ==> False
      guest is amd64-solaris              ==> True
      guest is other                      ==> inapplicable

   guest_amd64_assume_gs_is_const
      guest is amd64-darwin               ==> True
      guest is amd64-linux                ==> True
      guest is amd64-solaris              ==> False
      guest is other                      ==> inapplicable

   guest_ppc_zap_RZ_at_blr
      guest is ppc64-linux                ==> True
      guest is ppc32-linux                ==> False
      guest is other                      ==> inapplicable

   guest_ppc_zap_RZ_at_bl
      guest is ppc64-linux                ==> const True
      guest is ppc32-linux                ==> const False
      guest is other                      ==> inapplicable

   guest__use_fallback_LLSC
      guest is mips32                     ==> applicable, default True
      guest is mips64                     ==> applicable, default True
      guest is arm64                      ==> applicable, default False

   host_ppc_calls_use_fndescrs:
      host is ppc32-linux                 ==> False
      host is ppc64-linux                 ==> True
      host is other                       ==> inapplicable
*/

typedef
   struct {
      /* PPC and AMD64 GUESTS only: how many bytes below the 
         stack pointer are validly addressible? */
      Int guest_stack_redzone_size;

      /* AMD64 GUESTS only: should we translate %fs-prefixed
         instructions using the assumption that %fs always contains
         the same value? (typically zero on linux and solaris) */
      Bool guest_amd64_assume_fs_is_const;

      /* AMD64 GUESTS only: should we translate %gs-prefixed
         instructions using the assumption that %gs always contains
         the same value? (typically 0x60 on darwin)? */
      Bool guest_amd64_assume_gs_is_const;

      /* PPC GUESTS only: should we zap the stack red zone at a 'blr'
         (function return) ? */
      Bool guest_ppc_zap_RZ_at_blr;

      /* PPC GUESTS only: should we zap the stack red zone at a 'bl'
         (function call) ?  Is supplied with the guest address of the
         target of the call since that may be significant.  If NULL,
         is assumed equivalent to a fn which always returns False. */
      Bool (*guest_ppc_zap_RZ_at_bl)(Addr);

      /* Potentially for all guests that use LL/SC: use the fallback
         (synthesised) implementation rather than passing LL/SC on to
         the host? */
      Bool guest__use_fallback_LLSC;

      /* PPC32/PPC64 HOSTS only: does '&f' give us a pointer to a
         function descriptor on the host, or to the function code
         itself?  True => descriptor, False => code. */
      Bool host_ppc_calls_use_fndescrs;

      /* MIPS32/MIPS64 GUESTS only: emulated FPU mode. */
      UInt guest_mips_fp_mode;
   }
   VexAbiInfo;

/* Write default settings info *vbi. */
extern 
void LibVEX_default_VexAbiInfo ( /*OUT*/VexAbiInfo* vbi );


/*-------------------------------------------------------*/
/*--- Control of Vex's optimiser (iropt).             ---*/
/*-------------------------------------------------------*/


/* VexRegisterUpdates specifies when to ensure that the guest state is
   up to date, in order of increasing accuracy but increasing expense.

     VexRegUpdSpAtMemAccess: all registers are updated at superblock
     exits, and SP is also up to date at memory exception points.  The
     SP is described by the arch specific functions
     guest_<arch>_state_requires_precise_mem_exns.

     VexRegUpdUnwindregsAtMemAccess: registers needed to make a stack
     trace are up to date at memory exception points.  Typically,
     these are PC/SP/FP.  The minimal registers are described by the
     arch specific functions guest_<arch>_state_requires_precise_mem_exns.
     This is what Valgrind sets as the default.

     VexRegUpdAllregsAtMemAccess: all registers up to date at memory
     exception points.  This is what normally might be considered as
     providing "precise exceptions for memory", but does not
     necessarily provide precise register values at any other kind of
     exception.

     VexRegUpdAllregsAtEachInsn: all registers up to date at each
     instruction. 
*/
typedef
   enum {
      VexRegUpd_INVALID=0x700,
      VexRegUpdSpAtMemAccess,
      VexRegUpdUnwindregsAtMemAccess,
      VexRegUpdAllregsAtMemAccess,
      VexRegUpdAllregsAtEachInsn
   }
   VexRegisterUpdates;

/* Control of Vex's optimiser. */

typedef
   struct {
      /* Controls verbosity of iropt.  0 = no output. */
      Int iropt_verbosity;
      /* Control aggressiveness of iropt.  0 = no opt, 1 = simple
         opts, 2 (default) = max optimisation. */
      Int iropt_level;
      /* Controls when registers are updated in guest state.  Note
         that this is the default value.  The VEX client can override
         this on a per-IRSB basis if it wants.  bb_to_IR() will query
         the client to ask if it wants a different setting for the
         block under construction, and that new setting is transported
         back to LibVEX_Translate, which feeds it to iropt via the
         various do_iropt_BB calls. */
      VexRegisterUpdates iropt_register_updates_default;
      /* How aggressive should iropt be in unrolling loops?  Higher
         numbers make it more enthusiastic about loop unrolling.
         Default=120.  A setting of zero disables unrolling.  */
      Int iropt_unroll_thresh;
      /* What's the maximum basic block length the front end(s) allow?
         BBs longer than this are split up.  Default=60 (guest
         insns). */
      Int guest_max_insns;
      /* Should Vex try to construct superblocks, by chasing unconditional
         branches/calls to known destinations, and performing AND/OR idiom
         recognition?  It is recommended to set this to True as that possibly
         improves performance a bit, and also is important for avoiding certain
         kinds of false positives in Memcheck.  Default=True.  */
      Bool guest_chase;
      /* Register allocator version. Allowed values are:
         - '2': previous, good and slow implementation.
         - '3': current, faster implementation; perhaps producing slightly worse
                spilling decisions. */
      UInt regalloc_version;
   }
   VexControl;


/* Write the default settings into *vcon. */

extern 
void LibVEX_default_VexControl ( /*OUT*/ VexControl* vcon );


/*-------------------------------------------------------*/
/*--- Storage management control                      ---*/
/*-------------------------------------------------------*/

/* Allocate in Vex's temporary allocation area.  Be careful with this.
   You can only call it inside an instrumentation or optimisation
   callback that you have previously specified in a call to
   LibVEX_Translate.  The storage allocated will only stay alive until
   translation of the current basic block is complete. */
extern void* LibVEX_Alloc ( SizeT nbytes );

/* Show Vex allocation statistics. */
extern void LibVEX_ShowAllocStats ( void );


/*-------------------------------------------------------*/
/*--- Describing guest state layout                   ---*/
/*-------------------------------------------------------*/

/* Describe the guest state enough that the instrumentation
   functions can work. */

/* The max number of guest state chunks which we can describe as
   always defined (for the benefit of Memcheck). */
#define VEXGLO_N_ALWAYSDEFD  24

typedef
   struct {
      /* Total size of the guest state, in bytes.  Must be
         16-aligned. */
      Int total_sizeB;
      /* Whereabouts is the stack pointer? */
      Int offset_SP;
      Int sizeof_SP; /* 4 or 8 */
      /* Whereabouts is the frame pointer? */
      Int offset_FP;
      Int sizeof_FP; /* 4 or 8 */
      /* Whereabouts is the instruction pointer? */
      Int offset_IP;
      Int sizeof_IP; /* 4 or 8 */
      /* Describe parts of the guest state regarded as 'always
         defined'. */
      Int n_alwaysDefd;
      struct {
         Int offset;
         Int size;
      } alwaysDefd[VEXGLO_N_ALWAYSDEFD];
   }
   VexGuestLayout;

/* A note about guest state layout.

   LibVEX defines the layout for the guest state, in the file
   pub/libvex_guest_<arch>.h.  The struct will have an 16-aligned
   size.  Each translated bb is assumed to be entered with a specified
   register pointing at such a struct.  Beyond that is two copies of
   the shadow state area with the same size as the struct.  Beyond
   that is a spill area that LibVEX may spill into.  It must have size
   LibVEX_N_SPILL_BYTES, and this must be a 16-aligned number.

   On entry, the baseblock pointer register must be 16-aligned.

   There must be no holes in between the primary guest state, its two
   copies, and the spill area.  In short, all 4 areas must have a
   16-aligned size and be 16-aligned, and placed back-to-back.
*/

#define LibVEX_N_SPILL_BYTES 4096

/* The size of the guest state must be a multiple of this number. */
#define LibVEX_GUEST_STATE_ALIGN 16

/*-------------------------------------------------------*/
/*--- Initialisation of the library                   ---*/
/*-------------------------------------------------------*/

/* Initialise the library.  You must call this first. */

extern void LibVEX_Init (

   /* failure exit function */
#  if __cplusplus == 1 && __GNUC__ && __GNUC__ <= 3
   /* g++ 3.x doesn't understand attributes on function parameters.
      See #265762. */
#  else
   __attribute__ ((noreturn))
#  endif
   void (*failure_exit) ( void ),

   /* logging output function */
   void (*log_bytes) ( const HChar*, SizeT nbytes ),

   /* debug paranoia level */
   Int debuglevel,

   /* Control ... */
   const VexControl* vcon
);


/*-------------------------------------------------------*/
/*--- Make a translation                              ---*/
/*-------------------------------------------------------*/

/* Describes the outcome of a translation attempt. */
typedef
   struct {
      /* overall status */
      enum { VexTransOK=0x800,
             VexTransAccessFail, VexTransOutputFull } status;
      /* The number of extents that have a self-check (0 to 3) */
      UInt n_sc_extents;
      /* Offset in generated code of the profile inc, or -1 if
         none.  Needed for later patching. */
      Int offs_profInc;
      /* Stats only: the number of guest insns included in the
         translation.  It may be zero (!). */
      UInt n_guest_instrs;
      /* Stats only: the number of unconditional branches incorporated into the
         trace. */
      UShort n_uncond_in_trace;
      /* Stats only: the number of conditional branches incorporated into the
         trace. */
      UShort n_cond_in_trace;
   }
   VexTranslateResult;


/* Describes precisely the pieces of guest code that a translation
   covers.  Now that Vex can chase across BB boundaries, the old
   scheme of describing a chunk of guest code merely by its start
   address and length is inadequate.

   This struct uses 20 bytes on a 32-bit archtecture and 32 bytes on a
   64-bit architecture.  Space is important as clients will have to store
   one of these for each translation made.
*/
typedef
   struct {
      Addr   base[3];
      UShort len[3];
      UShort n_used;
   }
   VexGuestExtents;


/* A structure to carry arguments for LibVEX_Translate.  There are so
   many of them, it seems better to have a structure. */
typedef
   struct {
      /* IN: The instruction sets we are translating from and to.  And
         guest/host misc info. */
      VexArch      arch_guest;
      VexArchInfo  archinfo_guest;
      VexArch      arch_host;
      VexArchInfo  archinfo_host;
      VexAbiInfo   abiinfo_both;

      /* IN: an opaque value which is passed as the first arg to all
         callback functions supplied in this struct.  Vex has no idea
         what's at the other end of this pointer. */
      void*   callback_opaque;

      /* IN: the block to translate, and its guest address. */
      /* where are the actual bytes in the host's address space? */
      const UChar*  guest_bytes;
      /* where do the bytes really come from in the guest's aspace?
         This is the post-redirection guest address.  Not that Vex
         understands anything about redirection; that is all done on
         the Valgrind side. */
      Addr    guest_bytes_addr;

      /* Is it OK to chase into this guest address?  May not be
	 NULL. */
      Bool    (*chase_into_ok) ( /*callback_opaque*/void*, Addr );

      /* OUT: which bits of guest code actually got translated */
      VexGuestExtents* guest_extents;

      /* IN: a place to put the resulting code, and its size */
      UChar*  host_bytes;
      Int     host_bytes_size;
      /* OUT: how much of the output area is used. */
      Int*    host_bytes_used;

      /* IN: optionally, two instrumentation functions.  May be
	 NULL. */
      IRSB*   (*instrument1) ( /*callback_opaque*/void*, 
                               IRSB*, 
                               const VexGuestLayout*, 
                               const VexGuestExtents*,
                               const VexArchInfo*,
                               IRType gWordTy, IRType hWordTy );
      IRSB*   (*instrument2) ( /*callback_opaque*/void*, 
                               IRSB*, 
                               const VexGuestLayout*, 
                               const VexGuestExtents*,
                               const VexArchInfo*,
                               IRType gWordTy, IRType hWordTy );

      IRSB* (*finaltidy) ( IRSB* );

      /* IN: a callback used to ask the caller which of the extents,
         if any, a self check is required for.  Must not be NULL.
         The returned value is a bitmask with a 1 in position i indicating
         that the i'th extent needs a check.  Since there can be at most
         3 extents, the returned values must be between 0 and 7.

         This call also gives the VEX client the opportunity to change
         the precision of register update preservation as performed by
         the IR optimiser.  Before the call, VEX will set *pxControl
         to hold the default register-update status value as specified
         by VexControl::iropt_register_updates_default as passed to
         LibVEX_Init at library initialisation time.  The client (in
         this callback) can if it wants, inspect the value and change
         it to something different, and that value will be used for
         subsequent IR optimisation of the block. */
      UInt (*needs_self_check)( /*callback_opaque*/void*,
                                /*MAYBE_MOD*/VexRegisterUpdates* pxControl,
                                const VexGuestExtents* );

      /* IN: optionally, a callback which allows the caller to add its
         own IR preamble following the self-check and any other
         VEX-generated preamble, if any.  May be NULL.  If non-NULL,
         the IRSB under construction is handed to this function, which
         presumably adds IR statements to it.  The callback may
         optionally complete the block and direct bb_to_IR not to
         disassemble any instructions into it; this is indicated by
         the callback returning True.
      */
      Bool    (*preamble_function)(/*callback_opaque*/void*, IRSB*);

      /* IN: debug: trace vex activity at various points */
      Int     traceflags;

      /* IN: debug: print diagnostics when an illegal instr is detected */
      Bool    sigill_diag;

      /* IN: profiling: add a 64 bit profiler counter increment to the
         translation? */
      Bool    addProfInc;

      /* IN: address of the dispatcher entry points.  Describes the
         places where generated code should jump to at the end of each
         bb.

         At the end of each translation, the next guest address is
         placed in the host's standard return register (x86: %eax,
         amd64: %rax, ppc32: %r3, ppc64: %r3).  Optionally, the guest
         state pointer register (on host x86: %ebp; amd64: %rbp;
         ppc32/64: r31) may be set to a VEX_TRC_ value to indicate any
         special action required before the next block is run.

         Control is then passed back to the dispatcher (beyond Vex's
         control; caller supplies this) in the following way:

         - On host archs which lack a link register (x86, amd64), by a
           jump to the host address specified in
           'dispatcher_assisted', if the guest state pointer has been
           changed so as to request some action before the next block
           is run, or 'dispatcher_unassisted' (the fast path), in
           which it is assumed that the guest state pointer is
           unchanged and we wish to continue directly with the next
           translation.  Both of these must be non-NULL.

         - On host archs which have a link register (ppc32, ppc64), by
           a branch to the link register (which is guaranteed to be
           unchanged from whatever it was at entry to the
           translation).  'dispatch_assisted' and
           'dispatch_unassisted' must be NULL.

         The aim is to get back and forth between translations and the
         dispatcher without creating memory traffic to store return
         addresses.

         FIXME: update this comment
      */
      const void* disp_cp_chain_me_to_slowEP;
      const void* disp_cp_chain_me_to_fastEP;
      const void* disp_cp_xindir;
      const void* disp_cp_xassisted;
   }
   VexTranslateArgs;


/* Runs the entire compilation pipeline. */
extern 
VexTranslateResult LibVEX_Translate ( /*MOD*/ VexTranslateArgs* );

/* Runs the first half of the compilation pipeline: lifts guest code to IR,
   optimises, instruments and optimises it some more. */
extern
IRSB* LibVEX_FrontEnd ( /*MOD*/ VexTranslateArgs*,
                        /*OUT*/ VexTranslateResult* res,
                        /*OUT*/ VexRegisterUpdates* pxControl );


/* A subtlety re interaction between self-checking translations and
   bb-chasing.  The supplied chase_into_ok function should say NO
   (False) when presented with any address for which you might want to
   make a self-checking translation.

   If it doesn't do that, you may end up with Vex chasing from BB #1
   to BB #2 (fine); but if you wanted checking for #2 and not #1, that
   would not be the result.  Therefore chase_into_ok should disallow
   following into #2.  That will force the caller to eventually
   request a new translation starting at #2, at which point Vex will
   correctly observe the make-a-self-check flag.

   FIXME: is this still up to date? */


/*-------------------------------------------------------*/
/*--- Patch existing translations                     ---*/
/*-------------------------------------------------------*/

/* A host address range that was modified by the functions below. 
   Callers must request I-cache syncing after the call as appropriate. */
typedef
   struct {
      HWord start;
      HWord len;     /* always > 0 */
   }
   VexInvalRange;

/* Chain an XDirect jump located at place_to_chain so it jumps to
   place_to_jump_to.  It is expected (and checked) that this site
   currently contains a call to the dispatcher specified by
   disp_cp_chain_me_EXPECTED. */
extern
VexInvalRange LibVEX_Chain ( VexArch     arch_host,
                             VexEndness  endhess_host,
                             void*       place_to_chain,
                             const void* disp_cp_chain_me_EXPECTED,
                             const void* place_to_jump_to );

/* Undo an XDirect jump located at place_to_unchain, so it is
   converted back into a call to disp_cp_chain_me.  It is expected
   (and checked) that this site currently contains a jump directly to
   the address specified by place_to_jump_to_EXPECTED. */
extern
VexInvalRange LibVEX_UnChain ( VexArch     arch_host,
                               VexEndness  endness_host,
                               void*       place_to_unchain,
                               const void* place_to_jump_to_EXPECTED,
                               const void* disp_cp_chain_me );

/* Returns a constant -- the size of the event check that is put at
   the start of every translation.  This makes it possible to
   calculate the fast entry point address if the slow entry point
   address is known (the usual case), or vice versa. */
extern
Int LibVEX_evCheckSzB ( VexArch arch_host );


/* Patch the counter location into an existing ProfInc point.  The
   specified point is checked to make sure it is plausible. */
extern
VexInvalRange LibVEX_PatchProfInc ( VexArch      arch_host,
                                    VexEndness   endness_host,
                                    void*        place_to_patch,
                                    const ULong* location_of_counter );


/*-------------------------------------------------------*/
/*--- Show accumulated statistics                     ---*/
/*-------------------------------------------------------*/

extern void LibVEX_ShowStats ( void );

/*-------------------------------------------------------*/
/*-- IR injection                                      --*/
/*-------------------------------------------------------*/

/* IR Injection Control Block */

#define NO_ROUNDING_MODE (~0u)

typedef 
   struct {
      IROp  op;        // the operation to perform
      HWord result;    // address of the result
      HWord opnd1;     // address of 1st operand
      HWord opnd2;     // address of 2nd operand
      HWord opnd3;     // address of 3rd operand
      HWord opnd4;     // address of 4th operand
      IRType t_result; // type of result
      IRType t_opnd1;  // type of 1st operand
      IRType t_opnd2;  // type of 2nd operand
      IRType t_opnd3;  // type of 3rd operand
      IRType t_opnd4;  // type of 4th operand
      UInt  rounding_mode;
      UInt  num_operands; // excluding rounding mode, if any
      /* The following two members describe if this operand has immediate
       *  operands. There are a few restrictions:
       *    (1) An operator can have at most one immediate operand.
       * (2) If there is an immediate operand, it is the right-most operand
       *  An immediate_index of 0 means there is no immediate operand.
       */
      UInt immediate_type;  // size of immediate Ity_I8, Ity_16
      UInt immediate_index; // operand number: 1, 2
   }
   IRICB;

extern void LibVEX_InitIRI ( const IRICB * );

/*-------------------------------------------------------*/
/*--- Notes                                           ---*/
/*-------------------------------------------------------*/

/* Code generation conventions that need to be recorded somewhere.
   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

   x86
   ~~~
   Generated code should be entered using a JMP instruction.  On
   entry, %ebp should point to the guest state, and %esp should be a
   valid stack pointer.  The generated code may change %eax, %ebx,
   %ecx, %edx, %esi, %edi, all the FP registers and control state, and
   all the XMM registers.

   On entry, the FPU control word should be set to 0x027F, and the SSE
   control word (%mxcsr) should be set to 0x1F80.  On exit, they
   should still have those values (after masking off the lowest 6 bits
   of %mxcsr).  If they don't, there is a bug in VEX-generated code.

   Generated code returns to the scheduler using a JMP instruction, to
   the address specified in the .dispatch field of VexTranslateArgs.
   %eax (or %eax:%edx, if simulating a 64-bit target) will contain the
   guest address of the next block to execute.  %ebp may be changed
   to a VEX_TRC_ value, otherwise it should be as it was at entry.

   CRITICAL ISSUES in x86 code generation.  The only known critical
   issue is that the host FPU and SSE state is not properly saved
   across calls to helper functions.  If any helper references any
   such state, it is likely (1) to misbehave itself, since the FP
   stack tags will not be as expected, and (2) after returning to
   generated code, the generated code is likely to go wrong.  This
   really should be fixed.

   amd64
   ~~~~~
   Analogous to x86.

   ppc32
   ~~~~~
   On entry, guest state pointer is r31.  .dispatch must be NULL.
   Control is returned with a branch to the link register.  Generated
   code will not change lr.  At return, r3 holds the next guest addr
   (or r3:r4 ?).  r31 may be may be changed to a VEX_TRC_ value,
   otherwise it should be as it was at entry.

   ppc64
   ~~~~~
   Same as ppc32.

   arm32
   ~~~~~
   r8 is GSP.

   arm64
   ~~~~~
   r21 is GSP.

   ALL GUEST ARCHITECTURES
   ~~~~~~~~~~~~~~~~~~~~~~~
   The guest state must contain two pseudo-registers, guest_CMSTART
   and guest_CMLEN.  These are used to specify guest address ranges,
   either of code to be invalidated, when used in conjunction with
   Ijk_InvalICache, or of d-cache ranges to be flushed, when used in
   conjunction with Ijk_FlushDCache.  In such cases, the two _CM
   pseudo-regs should be filled in by the IR, and then an exit with
   one of the two abovementioned Ijk_ kinds should happen, so that the
   dispatcher can action them.  Both pseudo-regs must have size equal
   to the guest word size.

   The architecture must a third pseudo-register, guest_NRADDR, also
   guest-word-sized.  This is used to record the unredirected guest
   address at the start of a translation whose start has been
   redirected.  By reading this pseudo-register shortly afterwards,
   the translation can find out what the corresponding no-redirection
   address was.  Note, this is only set for wrap-style redirects, not
   for replace-style ones.
*/
#endif /* ndef __LIBVEX_H */

/*---------------------------------------------------------------*/
/*---                                                libvex.h ---*/
/*---------------------------------------------------------------*/
