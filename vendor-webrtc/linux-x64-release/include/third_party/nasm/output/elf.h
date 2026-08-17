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

#ifndef OUTPUT_ELF_H
#define OUTPUT_ELF_H

/*
 * Since NASM support both Elf32/64 file formats
 * we need to cover all types, structures, typedefs and etc
 */

#include "compiler.h"

/* Segment types */
#define PT_NULL		0
#define PT_LOAD		1
#define PT_DYNAMIC	2
#define PT_INTERP	3
#define PT_NOTE		4
#define PT_SHLIB	5
#define PT_PHDR		6
#define PT_LOOS		0x60000000
#define PT_HIOS		0x6fffffff
#define PT_LOPROC	0x70000000
#define PT_HIPROC	0x7fffffff
#define PT_GNU_EH_FRAME	0x6474e550	/* Extension, eh? */

/* ELF file types */
#define ET_NONE		0
#define ET_REL		1
#define ET_EXEC		2
#define ET_DYN		3
#define ET_CORE		4
#define ET_LOPROC	0xff00
#define ET_HIPROC	0xffff

/* ELF machine types */
#define EM_NONE		0
#define EM_M32		1
#define EM_SPARC	2
#define EM_386		3
#define EM_68K		4
#define EM_88K		5
#define EM_486		6	/* Not used in Linux at least */
#define EM_860		7
#define EM_MIPS		8	/* R3k, bigendian(?) */
#define EM_MIPS_RS4_BE	10	/* R4k BE */
#define EM_PARISC	15
#define EM_SPARC32PLUS	18
#define EM_PPC		20
#define EM_PPC64	21
#define EM_S390		22
#define EM_SH		42
#define EM_SPARCV9	43	/* v9 = SPARC64 */
#define EM_H8_300H	47
#define EM_H8S		48
#define EM_IA_64	50
#define EM_X86_64	62
#define EM_CRIS		76
#define EM_V850		87
#define EM_ALPHA	0x9026	/* Interim Alpha that stuck around */
#define EM_CYGNUS_V850	0x9080	/* Old v850 ID used by Cygnus */
#define EM_S390_OLD	0xA390	/* Obsolete interim value for S/390 */

/* Dynamic type values */
#define DT_NULL		0
#define DT_NEEDED	1
#define DT_PLTRELSZ	2
#define DT_PLTGOT	3
#define DT_HASH		4
#define DT_STRTAB	5
#define DT_SYMTAB	6
#define DT_RELA		7
#define DT_RELASZ	8
#define DT_RELAENT	9
#define DT_STRSZ	10
#define DT_SYMENT	11
#define DT_INIT		12
#define DT_FINI		13
#define DT_SONAME	14
#define DT_RPATH	15
#define DT_SYMBOLIC	16
#define DT_REL		17
#define DT_RELSZ	18
#define DT_RELENT	19
#define DT_PLTREL	20
#define DT_DEBUG	21
#define DT_TEXTREL	22
#define DT_JMPREL	23
#define DT_LOPROC	0x70000000
#define DT_HIPROC	0x7fffffff

/* Auxiliary table entries */
#define AT_NULL		0	/* end of vector */
#define AT_IGNORE	1	/* entry should be ignored */
#define AT_EXECFD	2	/* file descriptor of program */
#define AT_PHDR		3	/* program headers for program */
#define AT_PHENT	4	/* size of program header entry */
#define AT_PHNUM	5	/* number of program headers */
#define AT_PAGESZ	6	/* system page size */
#define AT_BASE		7	/* base address of interpreter */
#define AT_FLAGS	8	/* flags */
#define AT_ENTRY	9	/* entry point of program */
#define AT_NOTELF	10	/* program is not ELF */
#define AT_UID		11	/* real uid */
#define AT_EUID		12	/* effective uid */
#define AT_GID		13	/* real gid */
#define AT_EGID		14	/* effective gid */
#define AT_PLATFORM	15	/* string identifying CPU for optimizations */
#define AT_HWCAP	16	/* arch dependent hints at CPU capabilities */
#define AT_CLKTCK	17	/* frequency at which times() increments */
/* 18..22 = ? */
#define AT_SECURE	23	/* secure mode boolean */

/* Program header permission flags */
#define PF_X		0x1
#define PF_W		0x2
#define PF_R		0x4

/* Section header types */
#define SHT_NULL        0
#define SHT_PROGBITS    1
#define SHT_SYMTAB      2
#define SHT_STRTAB      3
#define SHT_RELA        4
#define SHT_HASH        5
#define SHT_DYNAMIC     6
#define SHT_NOTE        7
#define SHT_NOBITS      8
#define SHT_REL         9
#define SHT_SHLIB       10
#define SHT_DYNSYM      11
#define SHT_INIT_ARRAY	14
#define SHT_FINI_ARRAY	15
#define SHT_PREINIT_ARRAY 16
#define SHT_GROUP	17
#define SHT_SYMTAB_SHNDX 18
#define SHT_LOPROC      0x70000000
#define SHT_HIPROC      0x7fffffff
#define SHT_LOUSER      0x80000000
#define SHT_HIUSER      0xffffffff

/* Section header flags */
#define SHF_WRITE		(1 << 0)	/* Writable */
#define SHF_ALLOC		(1 << 1)	/* Occupies memory during execution */
#define SHF_EXECINSTR		(1 << 2)	/* Executable */
#define SHF_MERGE		(1 << 4)	/* Might be merged */
#define SHF_STRINGS		(1 << 5)	/* Contains nul-terminated strings */
#define SHF_INFO_LINK		(1 << 6)	/* `sh_info' contains SHT index */
#define SHF_LINK_ORDER		(1 << 7)	/* Preserve order after combining */
#define SHF_OS_NONCONFORMING	(1 << 8)	/* Non-standard OS specific handling required */
#define SHF_GROUP		(1 << 9)	/* Section is member of a group */
#define SHF_TLS			(1 << 10)	/* Section hold thread-local data */

/* Special section numbers */
#define SHN_UNDEF       0x0000
#define SHN_LORESERVE   0xff00
#define SHN_LOPROC      0xff00
#define SHN_HIPROC      0xff1f
#define SHN_ABS         0xfff1
#define SHN_COMMON      0xfff2
#define SHN_XINDEX	0xffff
#define SHN_HIRESERVE   0xffff

/* Same, but signed/sign-extended */
#define XSHN_UNDEF      ((int16_t)SHN_UNDEF)
#define XSHN_LORESERVE  ((int16_t)SHN_LORESERVE)
#define XSHN_LOPROC     ((int16_t)SHN_LOPROC)
#define XSHN_HIPROC     ((int16_t)SHN_HIPROC)
#define XSHN_ABS        ((int16_t)SHN_ABS)
#define XSHN_COMMON     ((int16_t)SHN_COMMON)
#define XSHN_XINDEX     ((int16_t)SHN_XINDEX)
#define XSHN_HIRESERVE  ((int16_t)SHN_HIRESERVE)

/* Section align flag */
#define SHA_ANY		1	/* No alignment constraint */

/* Length of magic at the start of a file */
#define EI_NIDENT	16

/* Magic number constants... */
#define EI_MAG0		0	/* e_ident[] indexes */
#define EI_MAG1		1
#define EI_MAG2		2
#define EI_MAG3		3
#define EI_CLASS	4
#define EI_DATA		5
#define EI_VERSION	6
#define EI_OSABI	7
#define EI_ABIVERSION	8
#define EI_NINDENT	16

#define ELFMAG0		0x7f	/* EI_MAG */
#define ELFMAG1		'E'
#define ELFMAG2		'L'
#define ELFMAG3		'F'
#define ELFMAG		"\177ELF"
#define SELFMAG		4

#define ELFCLASSNONE	0	/* EI_CLASS */
#define ELFCLASS32	1
#define ELFCLASS64	2
#define ELFCLASSNUM	3

#define ELFDATANONE	0	/* e_ident[EI_DATA] */
#define ELFDATA2LSB	1
#define ELFDATA2MSB	2

#define EV_NONE		0	/* e_version, EI_VERSION */
#define EV_CURRENT	1
#define EV_NUM		2

#define ELFOSABI_NONE	0
#define ELFOSABI_LINUX	3

/* Legal values for ST_BIND subfield of st_info (symbol binding) */
#define STB_LOCAL	0	/* Local symbol */
#define STB_GLOBAL	1	/* Global symbol */
#define STB_WEAK	2	/* Weak symbol */
#define STB_NUM		3	/* Number of defined types */
#define STB_LOOS	10	/* Start of OS-specific */
#define STB_HIOS	12	/* End of OS-specific */
#define STB_LOPROC	13	/* Start of processor-specific */
#define STB_HIPROC	15	/* End of processor-specific */

/* Symbol types */
#define STT_NOTYPE	0	/* Symbol type is unspecified */
#define STT_OBJECT	1	/* Symbol is a data object */
#define STT_FUNC	2	/* Symbol is a code object */
#define STT_SECTION	3	/* Symbol associated with a section */
#define STT_FILE	4	/* Symbol's name is file name */
#define STT_COMMON	5	/* Symbol is a common data object */
#define STT_TLS		6	/* Symbol is thread-local data object */
#define STT_NUM		7	/* Number of defined types */

/* Symbol visibilities */
#define STV_DEFAULT	0	/* Default symbol visibility rules */
#define STV_INTERNAL	1	/* Processor specific hidden class */
#define STV_HIDDEN	2	/* Sym unavailable in other modules */
#define STV_PROTECTED	3	/* Not preemptible, not exported */

/* Both Elf32_Sym and Elf64_Sym use the same one-byte st_info field  */
#define ELF32_ST_BIND(i)	((i) >> 4)
#define ELF32_ST_MKBIND(i)	((i) << 4)	/* just a helper */
#define ELF32_ST_TYPE(i)	((i) & 0xf)
#define ELF32_ST_INFO(b, i)	(ELF32_ST_MKBIND(b) + ELF32_ST_TYPE(i))

#define ELF64_ST_BIND(i)	ELF32_ST_BIND(i)
#define ELF64_ST_MKBIND(i)	ELF32_ST_MKBIND(i)
#define ELF64_ST_TYPE(i)	ELF32_ST_TYPE(i)
#define ELF64_ST_INFO(b, i)	ELF32_ST_INFO(b, i)

/*
 * ELF standard typedefs (yet more proof that <stdint.h> was way overdue)
 */

typedef uint16_t Elf32_Half;
typedef int16_t Elf32_SHalf;
typedef uint32_t Elf32_Word;
typedef int32_t Elf32_Sword;
typedef uint64_t Elf32_Xword;
typedef int64_t Elf32_Sxword;

typedef uint32_t Elf32_Off;
typedef uint32_t Elf32_Addr;
typedef uint16_t Elf32_Section;

typedef uint16_t Elf64_Half;
typedef int16_t Elf64_SHalf;
typedef uint32_t Elf64_Word;
typedef int32_t Elf64_Sword;
typedef uint64_t Elf64_Xword;
typedef int64_t Elf64_Sxword;

typedef uint64_t Elf64_Off;
typedef uint64_t Elf64_Addr;
typedef uint16_t Elf64_Section;

/*
 * Dynamic header
 */

typedef struct elf32_dyn {
	Elf32_Sword		d_tag;
	union {
		Elf32_Sword	d_val;
		Elf32_Addr	d_ptr;
	} d_un;
} Elf32_Dyn;

typedef struct elf64_dyn {
	Elf64_Sxword		d_tag;
	union {
		Elf64_Xword	d_val;
		Elf64_Addr	d_ptr;
	} d_un;
} Elf64_Dyn;

/*
 * Relocations
 */

#define ELF32_R_SYM(x)		((x) >> 8)
#define ELF32_R_TYPE(x)		((x) & 0xff)
#define ELF32_R_INFO(s,t)	(((Elf32_Word)(s) << 8) + ELF32_R_TYPE(t))

typedef struct elf32_rel {
	Elf32_Addr		r_offset;
	Elf32_Word		r_info;
} Elf32_Rel;

typedef struct elf32_rela {
	Elf32_Addr		r_offset;
	Elf32_Word		r_info;
	Elf32_Sword		r_addend;
} Elf32_Rela;

enum reloc32_type {
	R_386_32		=  1,	/* ordinary absolute relocation */
	R_386_PC32		=  2,	/* PC-relative relocation */
	R_386_GOT32		=  3,	/* an offset into GOT */
	R_386_PLT32		=  4,	/* a PC-relative offset into PLT */
	R_386_COPY		=  5,	/* ??? */
	R_386_GLOB_DAT		=  6,	/* ??? */
	R_386_JUMP_SLOT		=  7,	/* ??? */
	R_386_RELATIVE		=  8,	/* ??? */
	R_386_GOTOFF		=  9,	/* an offset from GOT base */
	R_386_GOTPC		= 10,	/* a PC-relative offset _to_ GOT */
	R_386_TLS_TPOFF		= 14,	/* Offset in static TLS block */
	R_386_TLS_IE		= 15,	/* Address of GOT entry for static TLS block offset */
	/* These are GNU extensions, but useful */
	R_386_16		= 20,	/* A 16-bit absolute relocation */
	R_386_PC16		= 21,	/* A 16-bit PC-relative relocation */
	R_386_8			= 22,	/* An 8-bit absolute relocation */
	R_386_PC8		= 23,	/* An 8-bit PC-relative relocation */
        R_386_SEG16		= 45,   /* A 16-bit real-mode segment */
        R_386_SUB16		= 46,   /* Subtract 16-bit value */
        R_386_SUB32		= 47    /* Subtract 32-bit value */
};

#define ELF64_R_SYM(x)		((x) >> 32)
#define ELF64_R_TYPE(x)		((x) & 0xffffffff)
#define ELF64_R_INFO(s,t)	(((Elf64_Xword)(s) << 32) + ELF64_R_TYPE(t))

typedef struct elf64_rel {
	Elf64_Addr	r_offset;
	Elf64_Xword	r_info;
} Elf64_Rel;

typedef struct elf64_rela {
	Elf64_Addr	r_offset;
	Elf64_Xword	r_info;
	Elf64_Sxword	r_addend;
} Elf64_Rela;

enum reloc64_type {
	R_X86_64_NONE		=  0,	/* No reloc */
	R_X86_64_64		=  1,	/* Direct 64 bit  */
	R_X86_64_PC32		=  2,	/* PC relative 32 bit signed */
	R_X86_64_GOT32		=  3,	/* 32 bit GOT entry */
	R_X86_64_PLT32		=  4,	/* 32 bit PLT address */
	R_X86_64_COPY		=  5,	/* Copy symbol at runtime */
	R_X86_64_GLOB_DAT	=  6,	/* Create GOT entry */
	R_X86_64_JUMP_SLOT	=  7,	/* Create PLT entry */
	R_X86_64_RELATIVE	=  8,	/* Adjust by program base */
	R_X86_64_GOTPCREL	=  9,	/* 32 bit signed PC relative offset to GOT */
	R_X86_64_32		= 10,	/* Direct 32 bit zero extended */
	R_X86_64_32S		= 11,	/* Direct 32 bit sign extended */
	R_X86_64_16		= 12,	/* Direct 16 bit zero extended */
	R_X86_64_PC16		= 13,	/* 16 bit sign extended pc relative */
	R_X86_64_8		= 14,	/* Direct 8 bit sign extended  */
	R_X86_64_PC8		= 15,	/* 8 bit sign extended pc relative */
	R_X86_64_DTPMOD64	= 16,	/* ID of module containing symbol */
	R_X86_64_DTPOFF64	= 17,	/* Offset in module's TLS block */
	R_X86_64_TPOFF64	= 18,	/* Offset in initial TLS block */
	R_X86_64_TLSGD		= 19,	/* 32 bit signed PC relative offset to two GOT entries for GD symbol */
	R_X86_64_TLSLD		= 20,	/* 32 bit signed PC relative offset to two GOT entries for LD symbol */
	R_X86_64_DTPOFF32	= 21,	/* Offset in TLS block */
	R_X86_64_GOTTPOFF	= 22,	/* 32 bit signed PC relative offset to GOT entry for IE symbol */
	R_X86_64_TPOFF32	= 23,	/* Offset in initial TLS block */
	R_X86_64_PC64		= 24,	/* word64 S + A - P */
	R_X86_64_GOTOFF64	= 25,	/* word64 S + A - GOT */
	R_X86_64_GOTPC32	= 26,	/* word32 GOT + A - P */
	R_X86_64_GOT64		= 27,	/* word64 G + A */
	R_X86_64_GOTPCREL64	= 28,	/* word64 G + GOT - P + A */
	R_X86_64_GOTPC64	= 29,	/* word64 GOT - P + A */
	R_X86_64_GOTPLT64	= 30,	/* word64 G + A */
	R_X86_64_PLTOFF64	= 31,	/* word64 L - GOT + A */
	R_X86_64_SIZE32		= 32,	/* word32 Z + A */
	R_X86_64_SIZE64		= 33,	/* word64 Z + A */
	R_X86_64_GOTPC32_TLSDESC= 34,	/* word32 */
	R_X86_64_TLSDESC_CALL	= 35,	/* none */
	R_X86_64_TLSDESC	= 36	/* word64?2 */
};

/*
 * Symbol
 */

typedef struct elf32_sym {
	Elf32_Word	st_name;
	Elf32_Addr	st_value;
	Elf32_Word	st_size;
	unsigned char	st_info;
	unsigned char	st_other;
	Elf32_Half	st_shndx;
} Elf32_Sym;

typedef struct elf64_sym {
	Elf64_Word	st_name;
	unsigned char	st_info;
	unsigned char	st_other;
	Elf64_Half	st_shndx;
	Elf64_Addr	st_value;
	Elf64_Xword	st_size;
} Elf64_Sym;

/*
 * Main file header
 */

typedef struct elf32_hdr {
	unsigned char	e_ident[EI_NIDENT];
	Elf32_Half	e_type;
	Elf32_Half	e_machine;
	Elf32_Word	e_version;
	Elf32_Addr	e_entry;
	Elf32_Off	e_phoff;
	Elf32_Off	e_shoff;
	Elf32_Word	e_flags;
	Elf32_Half	e_ehsize;
	Elf32_Half	e_phentsize;
	Elf32_Half	e_phnum;
	Elf32_Half	e_shentsize;
	Elf32_Half	e_shnum;
	Elf32_Half	e_shstrndx;
} Elf32_Ehdr;

typedef struct elf64_hdr {
	unsigned char	e_ident[EI_NIDENT];
	Elf64_Half	e_type;
	Elf64_Half	e_machine;
	Elf64_Word	e_version;
	Elf64_Addr	e_entry;
	Elf64_Off	e_phoff;
	Elf64_Off	e_shoff;
	Elf64_Word	e_flags;
	Elf64_Half	e_ehsize;
	Elf64_Half	e_phentsize;
	Elf64_Half	e_phnum;
	Elf64_Half	e_shentsize;
	Elf64_Half	e_shnum;
	Elf64_Half	e_shstrndx;
} Elf64_Ehdr;

/*
 * Program header
 */

typedef struct elf32_phdr {
	Elf32_Word	p_type;
	Elf32_Off	p_offset;
	Elf32_Addr	p_vaddr;
	Elf32_Addr	p_paddr;
	Elf32_Word	p_filesz;
	Elf32_Word	p_memsz;
	Elf32_Word	p_flags;
	Elf32_Word	p_align;
} Elf32_Phdr;

typedef struct elf64_phdr {
	Elf64_Word	p_type;
	Elf64_Word	p_flags;
	Elf64_Off	p_offset;
	Elf64_Addr	p_vaddr;
	Elf64_Addr	p_paddr;
	Elf64_Xword	p_filesz;
	Elf64_Xword	p_memsz;
	Elf64_Xword	p_align;
} Elf64_Phdr;

/*
 * Section header
 */

typedef struct elf32_shdr {
	Elf32_Word	sh_name;
	Elf32_Word	sh_type;
	Elf32_Word	sh_flags;
	Elf32_Addr	sh_addr;
	Elf32_Off	sh_offset;
	Elf32_Word	sh_size;
	Elf32_Word	sh_link;
	Elf32_Word	sh_info;
	Elf32_Word	sh_addralign;
	Elf32_Word	sh_entsize;
} Elf32_Shdr;

typedef struct elf64_shdr {
	Elf64_Word	sh_name;
	Elf64_Word	sh_type;
	Elf64_Xword	sh_flags;
	Elf64_Addr	sh_addr;
	Elf64_Off	sh_offset;
	Elf64_Xword	sh_size;
	Elf64_Word	sh_link;
	Elf64_Word	sh_info;
	Elf64_Xword	sh_addralign;
	Elf64_Xword	sh_entsize;
} Elf64_Shdr;

/*
 * Note header
 */
typedef struct elf32_note {
	Elf32_Word	n_namesz;	/* Name size */
	Elf32_Word	n_descsz;	/* Content size */
	Elf32_Word	n_type;		/* Content type */
} Elf32_Nhdr;

typedef struct elf64_note {
	Elf64_Word	n_namesz;	/* Name size */
	Elf64_Word	n_descsz;	/* Content size */
	Elf64_Word	n_type;		/* Content type */
} Elf64_Nhdr;

#endif /* OUTPUT_ELF_H */
