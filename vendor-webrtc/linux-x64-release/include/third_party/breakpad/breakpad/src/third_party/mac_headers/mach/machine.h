/*
 * Copyright (c) 2007-2016 Apple, Inc. All rights reserved.
 * Copyright (c) 2000 Apple Computer, Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
/*
 * Mach Operating System
 * Copyright (c) 1991,1990,1989,1988,1987 Carnegie Mellon University
 * All Rights Reserved.
 *
 * Permission to use, copy, modify and distribute this software and its
 * documentation is hereby granted, provided that both the copyright
 * notice and this permission notice appear in all copies of the
 * software, derivative works or modified versions, and any portions
 * thereof, and that both notices appear in supporting documentation.
 *
 * CARNEGIE MELLON ALLOWS FREE USE OF THIS SOFTWARE IN ITS "AS IS"
 * CONDITION.  CARNEGIE MELLON DISCLAIMS ANY LIABILITY OF ANY KIND FOR
 * ANY DAMAGES WHATSOEVER RESULTING FROM THE USE OF THIS SOFTWARE.
 *
 * Carnegie Mellon requests users of this software to return to
 *
 *  Software Distribution Coordinator  or  Software.Distribution@CS.CMU.EDU
 *  School of Computer Science
 *  Carnegie Mellon University
 *  Pittsburgh PA 15213-3890
 *
 * any improvements or extensions that they make and grant Carnegie Mellon
 * the rights to redistribute these changes.
 */
/*	File:	machine.h
 *	Author:	Avadis Tevanian, Jr.
 *	Date:	1986
 *
 *	Machine independent machine abstraction.
 */

#ifndef _MACH_MACHINE_H_
#define _MACH_MACHINE_H_

#ifndef __ASSEMBLER__

#include <stdint.h>
#include <mach/machine/vm_types.h>
#include <mach/boolean.h>

typedef integer_t       cpu_type_t;
typedef integer_t       cpu_subtype_t;
typedef integer_t       cpu_threadtype_t;

#define CPU_STATE_MAX           4

#define CPU_STATE_USER          0
#define CPU_STATE_SYSTEM        1
#define CPU_STATE_IDLE          2
#define CPU_STATE_NICE          3

#ifdef  KERNEL_PRIVATE

#include <sys/cdefs.h>

__BEGIN_DECLS
cpu_type_t                      cpu_type(void);

cpu_subtype_t           cpu_subtype(void);

cpu_threadtype_t        cpu_threadtype(void);
__END_DECLS

#ifdef  MACH_KERNEL_PRIVATE

struct machine_info {
	integer_t       major_version;          /* kernel major version id */
	integer_t       minor_version;          /* kernel minor version id */
	integer_t       max_cpus;                       /* max number of CPUs possible */
	uint32_t        memory_size;            /* size of memory in bytes, capped at 2 GB */
	uint64_t        max_mem;                        /* actual size of physical memory */
	uint32_t        physical_cpu;           /* number of physical CPUs now available */
	integer_t       physical_cpu_max;       /* max number of physical CPUs possible */
	uint32_t        logical_cpu;            /* number of logical cpu now available */
	integer_t       logical_cpu_max;        /* max number of physical CPUs possible */
};

typedef struct machine_info     *machine_info_t;
typedef struct machine_info     machine_info_data_t;

extern struct machine_info      machine_info;

__BEGIN_DECLS
cpu_type_t                      slot_type(
	int             slot_num);

cpu_subtype_t           slot_subtype(
	int             slot_num);

cpu_threadtype_t        slot_threadtype(
	int             slot_num);
__END_DECLS

#endif  /* MACH_KERNEL_PRIVATE */
#endif  /* KERNEL_PRIVATE */


/*
 * Capability bits used in the definition of cpu_type.
 */
#define CPU_ARCH_MASK           0xff000000      /* mask for architecture bits */
#define CPU_ARCH_ABI64          0x01000000      /* 64 bit ABI */
#define CPU_ARCH_ABI64_32       0x02000000      /* ABI for 64-bit hardware with 32-bit types; LP32 */

/*
 *	Machine types known by all.
 */

#define CPU_TYPE_ANY            ((cpu_type_t) -1)

#define CPU_TYPE_VAX            ((cpu_type_t) 1)
/* skip				((cpu_type_t) 2)	*/
/* skip				((cpu_type_t) 3)	*/
/* skip				((cpu_type_t) 4)	*/
/* skip				((cpu_type_t) 5)	*/
#define CPU_TYPE_MC680x0        ((cpu_type_t) 6)
#define CPU_TYPE_X86            ((cpu_type_t) 7)
#define CPU_TYPE_I386           CPU_TYPE_X86            /* compatibility */
#define CPU_TYPE_X86_64         (CPU_TYPE_X86 | CPU_ARCH_ABI64)

/* skip CPU_TYPE_MIPS		((cpu_type_t) 8)	*/
/* skip                         ((cpu_type_t) 9)	*/
#define CPU_TYPE_MC98000        ((cpu_type_t) 10)
#define CPU_TYPE_HPPA           ((cpu_type_t) 11)
#define CPU_TYPE_ARM            ((cpu_type_t) 12)
#define CPU_TYPE_ARM64          (CPU_TYPE_ARM | CPU_ARCH_ABI64)
#define CPU_TYPE_ARM64_32       (CPU_TYPE_ARM | CPU_ARCH_ABI64_32)
#define CPU_TYPE_MC88000        ((cpu_type_t) 13)
#define CPU_TYPE_SPARC          ((cpu_type_t) 14)
#define CPU_TYPE_I860           ((cpu_type_t) 15)
/* skip	CPU_TYPE_ALPHA		((cpu_type_t) 16)	*/
/* skip				((cpu_type_t) 17)	*/
#define CPU_TYPE_POWERPC                ((cpu_type_t) 18)
#define CPU_TYPE_POWERPC64              (CPU_TYPE_POWERPC | CPU_ARCH_ABI64)
/* skip				((cpu_type_t) 19)	*/
/* skip				((cpu_type_t) 20 */
/* skip				((cpu_type_t) 21 */
/* skip				((cpu_type_t) 22 */
/* skip				((cpu_type_t) 23 */

/*
 *	Machine subtypes (these are defined here, instead of in a machine
 *	dependent directory, so that any program can get all definitions
 *	regardless of where is it compiled).
 */

/*
 * Capability bits used in the definition of cpu_subtype.
 */
#define CPU_SUBTYPE_MASK        0xff000000      /* mask for feature flags */
#define CPU_SUBTYPE_LIB64       0x80000000      /* 64 bit libraries */
#define CPU_SUBTYPE_PTRAUTH_ABI 0x80000000      /* pointer authentication with versioned ABI */

/*
 *      When selecting a slice, ANY will pick the slice with the best
 *      grading for the selected cpu_type_t, unlike the "ALL" subtypes,
 *      which are the slices that can run on any hardware for that cpu type.
 */
#define CPU_SUBTYPE_ANY         ((cpu_subtype_t) -1)

/*
 *	Object files that are hand-crafted to run on any
 *	implementation of an architecture are tagged with
 *	CPU_SUBTYPE_MULTIPLE.  This functions essentially the same as
 *	the "ALL" subtype of an architecture except that it allows us
 *	to easily find object files that may need to be modified
 *	whenever a new implementation of an architecture comes out.
 *
 *	It is the responsibility of the implementor to make sure the
 *	software handles unsupported implementations elegantly.
 */
#define CPU_SUBTYPE_MULTIPLE            ((cpu_subtype_t) -1)
#define CPU_SUBTYPE_LITTLE_ENDIAN       ((cpu_subtype_t) 0)
#define CPU_SUBTYPE_BIG_ENDIAN          ((cpu_subtype_t) 1)

/*
 *     Machine threadtypes.
 *     This is none - not defined - for most machine types/subtypes.
 */
#define CPU_THREADTYPE_NONE             ((cpu_threadtype_t) 0)

/*
 *	VAX subtypes (these do *not* necessary conform to the actual cpu
 *	ID assigned by DEC available via the SID register).
 */

#define CPU_SUBTYPE_VAX_ALL     ((cpu_subtype_t) 0)
#define CPU_SUBTYPE_VAX780      ((cpu_subtype_t) 1)
#define CPU_SUBTYPE_VAX785      ((cpu_subtype_t) 2)
#define CPU_SUBTYPE_VAX750      ((cpu_subtype_t) 3)
#define CPU_SUBTYPE_VAX730      ((cpu_subtype_t) 4)
#define CPU_SUBTYPE_UVAXI       ((cpu_subtype_t) 5)
#define CPU_SUBTYPE_UVAXII      ((cpu_subtype_t) 6)
#define CPU_SUBTYPE_VAX8200     ((cpu_subtype_t) 7)
#define CPU_SUBTYPE_VAX8500     ((cpu_subtype_t) 8)
#define CPU_SUBTYPE_VAX8600     ((cpu_subtype_t) 9)
#define CPU_SUBTYPE_VAX8650     ((cpu_subtype_t) 10)
#define CPU_SUBTYPE_VAX8800     ((cpu_subtype_t) 11)
#define CPU_SUBTYPE_UVAXIII     ((cpu_subtype_t) 12)

/*
 *      680x0 subtypes
 *
 * The subtype definitions here are unusual for historical reasons.
 * NeXT used to consider 68030 code as generic 68000 code.  For
 * backwards compatability:
 *
 *	CPU_SUBTYPE_MC68030 symbol has been preserved for source code
 *	compatability.
 *
 *	CPU_SUBTYPE_MC680x0_ALL has been defined to be the same
 *	subtype as CPU_SUBTYPE_MC68030 for binary comatability.
 *
 *	CPU_SUBTYPE_MC68030_ONLY has been added to allow new object
 *	files to be tagged as containing 68030-specific instructions.
 */

#define CPU_SUBTYPE_MC680x0_ALL         ((cpu_subtype_t) 1)
#define CPU_SUBTYPE_MC68030             ((cpu_subtype_t) 1) /* compat */
#define CPU_SUBTYPE_MC68040             ((cpu_subtype_t) 2)
#define CPU_SUBTYPE_MC68030_ONLY        ((cpu_subtype_t) 3)

/*
 *	I386 subtypes
 */

#define CPU_SUBTYPE_INTEL(f, m) ((cpu_subtype_t) (f) + ((m) << 4))

#define CPU_SUBTYPE_I386_ALL                    CPU_SUBTYPE_INTEL(3, 0)
#define CPU_SUBTYPE_386                                 CPU_SUBTYPE_INTEL(3, 0)
#define CPU_SUBTYPE_486                                 CPU_SUBTYPE_INTEL(4, 0)
#define CPU_SUBTYPE_486SX                               CPU_SUBTYPE_INTEL(4, 8) // 8 << 4 = 128
#define CPU_SUBTYPE_586                                 CPU_SUBTYPE_INTEL(5, 0)
#define CPU_SUBTYPE_PENT        CPU_SUBTYPE_INTEL(5, 0)
#define CPU_SUBTYPE_PENTPRO     CPU_SUBTYPE_INTEL(6, 1)
#define CPU_SUBTYPE_PENTII_M3   CPU_SUBTYPE_INTEL(6, 3)
#define CPU_SUBTYPE_PENTII_M5   CPU_SUBTYPE_INTEL(6, 5)
#define CPU_SUBTYPE_CELERON                             CPU_SUBTYPE_INTEL(7, 6)
#define CPU_SUBTYPE_CELERON_MOBILE              CPU_SUBTYPE_INTEL(7, 7)
#define CPU_SUBTYPE_PENTIUM_3                   CPU_SUBTYPE_INTEL(8, 0)
#define CPU_SUBTYPE_PENTIUM_3_M                 CPU_SUBTYPE_INTEL(8, 1)
#define CPU_SUBTYPE_PENTIUM_3_XEON              CPU_SUBTYPE_INTEL(8, 2)
#define CPU_SUBTYPE_PENTIUM_M                   CPU_SUBTYPE_INTEL(9, 0)
#define CPU_SUBTYPE_PENTIUM_4                   CPU_SUBTYPE_INTEL(10, 0)
#define CPU_SUBTYPE_PENTIUM_4_M                 CPU_SUBTYPE_INTEL(10, 1)
#define CPU_SUBTYPE_ITANIUM                             CPU_SUBTYPE_INTEL(11, 0)
#define CPU_SUBTYPE_ITANIUM_2                   CPU_SUBTYPE_INTEL(11, 1)
#define CPU_SUBTYPE_XEON                                CPU_SUBTYPE_INTEL(12, 0)
#define CPU_SUBTYPE_XEON_MP                             CPU_SUBTYPE_INTEL(12, 1)

#define CPU_SUBTYPE_INTEL_FAMILY(x)     ((x) & 15)
#define CPU_SUBTYPE_INTEL_FAMILY_MAX    15

#define CPU_SUBTYPE_INTEL_MODEL(x)      ((x) >> 4)
#define CPU_SUBTYPE_INTEL_MODEL_ALL     0

/*
 *	X86 subtypes.
 */

#define CPU_SUBTYPE_X86_ALL             ((cpu_subtype_t)3)
#define CPU_SUBTYPE_X86_64_ALL          ((cpu_subtype_t)3)
#define CPU_SUBTYPE_X86_ARCH1           ((cpu_subtype_t)4)
#define CPU_SUBTYPE_X86_64_H            ((cpu_subtype_t)8)      /* Haswell feature subset */


#define CPU_THREADTYPE_INTEL_HTT        ((cpu_threadtype_t) 1)

/*
 *	Mips subtypes.
 */

#define CPU_SUBTYPE_MIPS_ALL    ((cpu_subtype_t) 0)
#define CPU_SUBTYPE_MIPS_R2300  ((cpu_subtype_t) 1)
#define CPU_SUBTYPE_MIPS_R2600  ((cpu_subtype_t) 2)
#define CPU_SUBTYPE_MIPS_R2800  ((cpu_subtype_t) 3)
#define CPU_SUBTYPE_MIPS_R2000a ((cpu_subtype_t) 4)     /* pmax */
#define CPU_SUBTYPE_MIPS_R2000  ((cpu_subtype_t) 5)
#define CPU_SUBTYPE_MIPS_R3000a ((cpu_subtype_t) 6)     /* 3max */
#define CPU_SUBTYPE_MIPS_R3000  ((cpu_subtype_t) 7)

/*
 *	MC98000 (PowerPC) subtypes
 */
#define CPU_SUBTYPE_MC98000_ALL ((cpu_subtype_t) 0)
#define CPU_SUBTYPE_MC98601     ((cpu_subtype_t) 1)

/*
 *	HPPA subtypes for Hewlett-Packard HP-PA family of
 *	risc processors. Port by NeXT to 700 series.
 */

#define CPU_SUBTYPE_HPPA_ALL            ((cpu_subtype_t) 0)
#define CPU_SUBTYPE_HPPA_7100           ((cpu_subtype_t) 0) /* compat */
#define CPU_SUBTYPE_HPPA_7100LC         ((cpu_subtype_t) 1)

/*
 *	MC88000 subtypes.
 */
#define CPU_SUBTYPE_MC88000_ALL ((cpu_subtype_t) 0)
#define CPU_SUBTYPE_MC88100     ((cpu_subtype_t) 1)
#define CPU_SUBTYPE_MC88110     ((cpu_subtype_t) 2)

/*
 *	SPARC subtypes
 */
#define CPU_SUBTYPE_SPARC_ALL           ((cpu_subtype_t) 0)

/*
 *	I860 subtypes
 */
#define CPU_SUBTYPE_I860_ALL    ((cpu_subtype_t) 0)
#define CPU_SUBTYPE_I860_860    ((cpu_subtype_t) 1)

/*
 *	PowerPC subtypes
 */
#define CPU_SUBTYPE_POWERPC_ALL         ((cpu_subtype_t) 0)
#define CPU_SUBTYPE_POWERPC_601         ((cpu_subtype_t) 1)
#define CPU_SUBTYPE_POWERPC_602         ((cpu_subtype_t) 2)
#define CPU_SUBTYPE_POWERPC_603         ((cpu_subtype_t) 3)
#define CPU_SUBTYPE_POWERPC_603e        ((cpu_subtype_t) 4)
#define CPU_SUBTYPE_POWERPC_603ev       ((cpu_subtype_t) 5)
#define CPU_SUBTYPE_POWERPC_604         ((cpu_subtype_t) 6)
#define CPU_SUBTYPE_POWERPC_604e        ((cpu_subtype_t) 7)
#define CPU_SUBTYPE_POWERPC_620         ((cpu_subtype_t) 8)
#define CPU_SUBTYPE_POWERPC_750         ((cpu_subtype_t) 9)
#define CPU_SUBTYPE_POWERPC_7400        ((cpu_subtype_t) 10)
#define CPU_SUBTYPE_POWERPC_7450        ((cpu_subtype_t) 11)
#define CPU_SUBTYPE_POWERPC_970         ((cpu_subtype_t) 100)

/*
 *	ARM subtypes
 */
#define CPU_SUBTYPE_ARM_ALL             ((cpu_subtype_t) 0)
#define CPU_SUBTYPE_ARM_V4T             ((cpu_subtype_t) 5)
#define CPU_SUBTYPE_ARM_V6              ((cpu_subtype_t) 6)
#define CPU_SUBTYPE_ARM_V5TEJ           ((cpu_subtype_t) 7)
#define CPU_SUBTYPE_ARM_XSCALE          ((cpu_subtype_t) 8)
#define CPU_SUBTYPE_ARM_V7              ((cpu_subtype_t) 9)  /* ARMv7-A and ARMv7-R */
#define CPU_SUBTYPE_ARM_V7F             ((cpu_subtype_t) 10) /* Cortex A9 */
#define CPU_SUBTYPE_ARM_V7S             ((cpu_subtype_t) 11) /* Swift */
#define CPU_SUBTYPE_ARM_V7K             ((cpu_subtype_t) 12)
#define CPU_SUBTYPE_ARM_V8              ((cpu_subtype_t) 13)
#define CPU_SUBTYPE_ARM_V6M             ((cpu_subtype_t) 14) /* Not meant to be run under xnu */
#define CPU_SUBTYPE_ARM_V7M             ((cpu_subtype_t) 15) /* Not meant to be run under xnu */
#define CPU_SUBTYPE_ARM_V7EM            ((cpu_subtype_t) 16) /* Not meant to be run under xnu */
#define CPU_SUBTYPE_ARM_V8M             ((cpu_subtype_t) 17) /* Not meant to be run under xnu */

/*
 *  ARM64 subtypes
 */
#define CPU_SUBTYPE_ARM64_ALL           ((cpu_subtype_t) 0)
#define CPU_SUBTYPE_ARM64_V8            ((cpu_subtype_t) 1)
#define CPU_SUBTYPE_ARM64E              ((cpu_subtype_t) 2)

/* CPU subtype feature flags for ptrauth on arm64e platforms */
#define CPU_SUBTYPE_ARM64_PTR_AUTH_MASK 0x0f000000
#define CPU_SUBTYPE_ARM64_PTR_AUTH_VERSION(x) (((x) & CPU_SUBTYPE_ARM64_PTR_AUTH_MASK) >> 24)
#ifdef PRIVATE
#define CPU_SUBTYPE_ARM64_PTR_AUTH_CURRENT_VERSION 0
#endif /* PRIVATE */

/*
 *  ARM64_32 subtypes
 */
#define CPU_SUBTYPE_ARM64_32_ALL        ((cpu_subtype_t) 0)
#define CPU_SUBTYPE_ARM64_32_V8 ((cpu_subtype_t) 1)

#endif /* !__ASSEMBLER__ */

/*
 *	CPU families (sysctl hw.cpufamily)
 *
 * These are meant to identify the CPU's marketing name - an
 * application can map these to (possibly) localized strings.
 * NB: the encodings of the CPU families are intentionally arbitrary.
 * There is no ordering, and you should never try to deduce whether
 * or not some feature is available based on the family.
 * Use feature flags (eg, hw.optional.altivec) to test for optional
 * functionality.
 */
#define CPUFAMILY_UNKNOWN               0
#define CPUFAMILY_POWERPC_G3            0xcee41549
#define CPUFAMILY_POWERPC_G4            0x77c184ae
#define CPUFAMILY_POWERPC_G5            0xed76d8aa
#define CPUFAMILY_INTEL_6_13            0xaa33392b
#define CPUFAMILY_INTEL_PENRYN          0x78ea4fbc
#define CPUFAMILY_INTEL_NEHALEM         0x6b5a4cd2
#define CPUFAMILY_INTEL_WESTMERE        0x573b5eec
#define CPUFAMILY_INTEL_SANDYBRIDGE     0x5490b78c
#define CPUFAMILY_INTEL_IVYBRIDGE       0x1f65e835
#define CPUFAMILY_INTEL_HASWELL         0x10b282dc
#define CPUFAMILY_INTEL_BROADWELL       0x582ed09c
#define CPUFAMILY_INTEL_SKYLAKE         0x37fc219f
#define CPUFAMILY_INTEL_KABYLAKE        0x0f817246
#define CPUFAMILY_INTEL_ICELAKE         0x38435547
#define CPUFAMILY_ARM_9                 0xe73283ae
#define CPUFAMILY_ARM_11                0x8ff620d8
#define CPUFAMILY_ARM_XSCALE            0x53b005f5
#define CPUFAMILY_ARM_12                0xbd1b0ae9
#define CPUFAMILY_ARM_13                0x0cc90e64
#define CPUFAMILY_ARM_14                0x96077ef1
#define CPUFAMILY_ARM_15                0xa8511bca
#define CPUFAMILY_ARM_SWIFT             0x1e2d6381
#define CPUFAMILY_ARM_CYCLONE           0x37a09642
#define CPUFAMILY_ARM_TYPHOON           0x2c91a47e
#define CPUFAMILY_ARM_TWISTER           0x92fb37c8
#define CPUFAMILY_ARM_HURRICANE         0x67ceee93
#define CPUFAMILY_ARM_MONSOON_MISTRAL   0xe81e7ef6
#define CPUFAMILY_ARM_VORTEX_TEMPEST    0x07d34b9f
#define CPUFAMILY_ARM_LIGHTNING_THUNDER 0x462504d2
#ifndef RC_HIDE_XNU_FIRESTORM
#define CPUFAMILY_ARM_FIRESTORM_ICESTORM 0x1b588bb3
#endif /* !RC_HIDE_XNU_FIRESTORM */

/* Described in rdar://64125549 */
#define CPUSUBFAMILY_UNKNOWN            0
#define CPUSUBFAMILY_ARM_HP             1
#define CPUSUBFAMILY_ARM_HG             2
#define CPUSUBFAMILY_ARM_M              3
#ifndef RC_HIDE_XNU_FIRESTORM
#define CPUSUBFAMILY_ARM_HS             4
#define CPUSUBFAMILY_ARM_HC_HD          5
#endif /* !RC_HIDE_XNU_FIRESTORM */

/* The following synonyms are deprecated: */
#define CPUFAMILY_INTEL_6_23    CPUFAMILY_INTEL_PENRYN
#define CPUFAMILY_INTEL_6_26    CPUFAMILY_INTEL_NEHALEM


#endif  /* _MACH_MACHINE_H_ */