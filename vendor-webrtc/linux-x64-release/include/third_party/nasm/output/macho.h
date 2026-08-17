/* ----------------------------------------------------------------------- *
 *
 *   Copyright 1996-2018 The NASM Authors - All Rights Reserved
 *   See the file AUTHORS included with the NASM distribution for
 *   the specific copyright holders.
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following
 *   conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above
 *     copyright notice, this list of conditions and the following
 *     disclaimer in the documentation and/or other materials provided
 *     with the distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
 *     CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *     INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *     MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *     DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 *     CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *     SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *     NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 *     HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 *     CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 *     OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 *     EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * ----------------------------------------------------------------------- */

#ifndef OUTPUT_MACHO_H
#define OUTPUT_MACHO_H

#include "compiler.h"

/* Magics */
#define MH_MAGIC			0xfeedface
#define MH_MAGIC_64			0xfeedfacf

/* File types */
#define MH_OBJECT			0x1

/* CPUs */
#define CPU_ARCH_MASK			0xff000000
#define CPU_ARCH_ABI64			0x01000000
#define CPU_TYPE_X86			7
#define CPU_TYPE_I386			CPU_TYPE_X86
#define CPU_TYPE_X86_64			(CPU_TYPE_X86 | CPU_ARCH_ABI64)

#define CPU_SUBTYPE_MASK		0xff000000
#define CPU_SUBTYPE_I386_ALL		3

/* Header flags */
#define MH_SUBSECTIONS_VIA_SYMBOLS	0x00002000

/* Load commands */
#define LC_SEGMENT			0x1
#define LC_SEGMENT_64			0x19
#define LC_SYMTAB			0x2
#define LC_BUILD_VERSION		0x32

/* Symbol type bits */
#define N_STAB				0xe0
#define	N_PEXT				0x10
#define N_TYPE				0x0e
#define N_EXT				0x01

/* To mask with N_TYPE */
#define N_UNDF				0x00
#define N_ABS				0x02
#define N_INDR				0x0a
#define N_PBUD				0x0c
#define N_SECT				0x0e

/* Section ordinals */
#define NO_SECT				0x00
#define MAX_SECT			0xff

/* Section bits */
#define SECTION_TYPE			0x000000ff
#define SECTION_ATTRIBUTES		0xffffff00
#define SECTION_ATTRIBUTES_USR		0xff000000
#define SECTION_ATTRIBUTES_SYS		0x00ffff00

#define S_REGULAR				0x00
#define S_ZEROFILL				0x01
#define S_CSTRING_LITERALS			0x02
#define S_4BYTE_LITERALS			0x03
#define S_8BYTE_LITERALS			0x04
#define S_LITERAL_POINTERS			0x05
#define S_NON_LAZY_SYMBOL_POINTERS		0x06
#define S_LAZY_SYMBOL_POINTERS			0x07
#define S_SYMBOL_STUBS				0x08
#define S_MOD_INIT_FUNC_POINTERS		0x09
#define S_MOD_TERM_FUNC_POINTERS		0x0a
#define S_COALESCED				0x0b
#define S_GB_ZEROFILL				0x0c
#define S_INTERPOSING				0x0d
#define S_16BYTE_LITERALS			0x0e
#define S_DTRACE_DOF				0x0f
#define S_LAZY_DYLIB_SYMBOL_POINTERS		0x10
#define S_THREAD_LOCAL_REGULAR			0x11
#define S_THREAD_LOCAL_ZEROFILL			0x12
#define S_THREAD_LOCAL_VARIABLES		0x13
#define S_THREAD_LOCAL_VARIABLE_POINTERS	0x14
#define S_THREAD_LOCAL_INIT_FUNCTION_POINTERS	0x15

#define S_ATTR_PURE_INSTRUCTIONS		0x80000000
#define S_ATTR_NO_TOC				0x40000000
#define S_ATTR_STRIP_STATIC_SYMS		0x20000000
#define S_ATTR_NO_DEAD_STRIP			0x10000000
#define S_ATTR_LIVE_SUPPORT			0x08000000
#define S_ATTR_SELF_MODIFYING_CODE		0x04000000
#define S_ATTR_DEBUG				0x02000000

#define S_ATTR_SOME_INSTRUCTIONS		0x00000400
#define S_ATTR_EXT_RELOC			0x00000200
#define S_ATTR_LOC_RELOC			0x00000100
#define INDIRECT_SYMBOL_LOCAL			0x80000000
#define INDIRECT_SYMBOL_ABS			0x40000000

/* Relocation info type */
#define GENERIC_RELOC_VANILLA		0
#define GENERIC_RELOC_PAIR		1
#define GENERIC_RELOC_SECTDIFF		2
#define GENERIC_RELOC_PB_LA_PTR		3
#define GENERIC_RELOC_LOCAL_SECTDIFF	4
#define GENERIC_RELOC_TLV		5

#define X86_64_RELOC_UNSIGNED		0
#define X86_64_RELOC_SIGNED		1
#define X86_64_RELOC_BRANCH		2
#define X86_64_RELOC_GOT_LOAD		3
#define X86_64_RELOC_GOT		4
#define X86_64_RELOC_SUBTRACTOR		5
#define X86_64_RELOC_SIGNED_1		6
#define X86_64_RELOC_SIGNED_2		7
#define X86_64_RELOC_SIGNED_4		8
#define X86_64_RELOC_TLV		9

/* Relocation info */
#define R_ABS		0
#define R_SCATTERED	0x80000000

/* Known values for the platform field in LC_BUILD_VERSION */
#define PLATFORM_MACOS				1
#define PLATFORM_IOS				2
#define PLATFORM_TVOS				3
#define PLATFORM_WATCHOS			4
#define PLATFORM_BRIDGEOS			5
#define PLATFORM_MACCATALYST		6
#define PLATFORM_IOSSIMULATOR		7
#define PLATFORM_TVOSSIMULATOR		8
#define PLATFORM_WATCHOSSIMULATOR	9
#define PLATFORM_DRIVERKIT			10

#define PLATFORM_INVALID			0

/* VM permission constants */
#define	VM_PROT_NONE			0x00
#define VM_PROT_READ			0x01
#define VM_PROT_WRITE			0x02
#define VM_PROT_EXECUTE			0x04

typedef struct {
	uint32_t	magic;
	uint32_t	cputype;
	uint32_t	cpusubtype;
	uint32_t	filetype;
	uint32_t	ncmds;
	uint32_t	sizeofcmds;
	uint32_t	flags;
} macho_header_t;

typedef struct {
	uint32_t	magic;
	uint32_t	cputype;
	uint32_t	cpusubtype;
	uint32_t	filetype;
	uint32_t	ncmds;
	uint32_t	sizeofcmds;
	uint32_t	flags;
	uint32_t	reserved;
} macho_header_64_t;

typedef struct {
	uint32_t	cmd;
	uint32_t	cmdsize;
} macho_load_command_t;

typedef struct {
	uint32_t	cmd;
	uint32_t	cmdsize;
	char		segname[16];
	uint32_t	vmaddr;
	uint32_t	vmsize;
	uint32_t	fileoff;
	uint32_t	filesize;
	uint32_t	maxprot;
	uint32_t	initprot;
	uint32_t	nsects;
	uint32_t	flags;
} macho_segment_command_t;

typedef struct {
	uint32_t	cmd;
	uint32_t	cmdsize;
	char		segname[16];
	uint64_t	vmaddr;
	uint64_t	vmsize;
	uint64_t	fileoff;
	uint64_t	filesize;
	uint32_t	maxprot;
	uint32_t	initprot;
	uint32_t	nsects;
	uint32_t	flags;
} macho_segment_command_64_t;

typedef struct {
	char		sectname[16];
	char		segname[16];
	uint32_t	addr;
	uint32_t	size;
	uint32_t	offset;
	uint32_t	align;
	uint32_t	reloff;
	uint32_t	nreloc;
	uint32_t	flags;
	uint32_t	reserved1;
	uint32_t	reserved2;
} macho_section_t;

typedef struct {
	char		sectname[16];
	char		segname[16];
	uint64_t	addr;
	uint64_t	size;
	uint32_t	offset;
	uint32_t	align;
	uint32_t	reloff;
	uint32_t	nreloc;
	uint32_t	flags;
	uint32_t	reserved1;
	uint32_t	reserved2;
	uint32_t	reserved3;
} macho_section_64_t;

typedef struct {
	uint32_t	cmd;
	uint32_t	cmdsize;
	uint32_t	symoff;
	uint32_t	nsyms;
	uint32_t	stroff;
	uint32_t	strsize;
} macho_symtab_command_t;

typedef struct {
	int32_t		r_address;
	union {
		struct {
			uint32_t	r_symbolnum:	24,
					r_pcrel:	1,
					r_length:	2,
					r_extern:	1,
					r_type:		4;
		} s;
		uint32_t	r_raw;
	} u;
} macho_relocation_info_t;

typedef struct nlist_base {
	uint32_t	n_strx;
	uint8_t		n_type;
	uint8_t		n_sect;
	uint16_t	n_desc;
} macho_nlist_base_t;

typedef struct nlist {
	uint32_t	n_strx;
	uint8_t		n_type;
	uint8_t		n_sect;
	int16_t		n_desc;
	uint32_t	n_value;
} macho_nlist_t;

typedef struct {
	uint32_t	n_strx;
	uint8_t		n_type;
	uint8_t		n_sect;
	uint16_t	n_desc;
	uint64_t	n_value;
} macho_nlist_64_t;

#endif /* OUTPUT_MACHO_H */
