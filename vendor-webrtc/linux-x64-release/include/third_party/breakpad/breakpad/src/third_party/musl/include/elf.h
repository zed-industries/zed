#ifndef _ELF_H
#define _ELF_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

typedef uint16_t Elf32_Half;
typedef uint16_t Elf64_Half;

typedef uint32_t Elf32_Word;
typedef	int32_t  Elf32_Sword;
typedef uint32_t Elf64_Word;
typedef	int32_t  Elf64_Sword;

typedef uint64_t Elf32_Xword;
typedef	int64_t  Elf32_Sxword;
typedef uint64_t Elf64_Xword;
typedef	int64_t  Elf64_Sxword;

typedef uint32_t Elf32_Addr;
typedef uint64_t Elf64_Addr;

typedef uint32_t Elf32_Off;
typedef uint64_t Elf64_Off;

typedef uint16_t Elf32_Section;
typedef uint16_t Elf64_Section;

typedef Elf32_Half Elf32_Versym;
typedef Elf64_Half Elf64_Versym;

#define EI_NIDENT (16)

typedef struct {
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

typedef struct {
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

#define EI_MAG0		0
#define ELFMAG0		0x7f

#define EI_MAG1		1
#define ELFMAG1		'E'

#define EI_MAG2		2
#define ELFMAG2		'L'

#define EI_MAG3		3
#define ELFMAG3		'F'


#define	ELFMAG		"\177ELF"
#define	SELFMAG		4

#define EI_CLASS	4
#define ELFCLASSNONE	0
#define ELFCLASS32	1
#define ELFCLASS64	2
#define ELFCLASSNUM	3

#define EI_DATA		5
#define ELFDATANONE	0
#define ELFDATA2LSB	1
#define ELFDATA2MSB	2
#define ELFDATANUM	3

#define EI_VERSION	6


#define EI_OSABI	7
#define ELFOSABI_NONE		0
#define ELFOSABI_SYSV		0
#define ELFOSABI_HPUX		1
#define ELFOSABI_NETBSD		2
#define ELFOSABI_LINUX		3
#define ELFOSABI_GNU		3
#define ELFOSABI_SOLARIS	6
#define ELFOSABI_AIX		7
#define ELFOSABI_IRIX		8
#define ELFOSABI_FREEBSD	9
#define ELFOSABI_TRU64		10
#define ELFOSABI_MODESTO	11
#define ELFOSABI_OPENBSD	12
#define ELFOSABI_ARM		97
#define ELFOSABI_STANDALONE	255

#define EI_ABIVERSION	8

#define EI_PAD		9



#define ET_NONE		0
#define ET_REL		1
#define ET_EXEC		2
#define ET_DYN		3
#define ET_CORE		4
#define	ET_NUM		5
#define ET_LOOS		0xfe00
#define ET_HIOS		0xfeff
#define ET_LOPROC	0xff00
#define ET_HIPROC	0xffff



#define EM_NONE		 0
#define EM_M32		 1
#define EM_SPARC	 2
#define EM_386		 3
#define EM_68K		 4
#define EM_88K		 5
#define EM_860		 7
#define EM_MIPS		 8
#define EM_S370		 9
#define EM_MIPS_RS3_LE	10

#define EM_PARISC	15
#define EM_VPP500	17
#define EM_SPARC32PLUS	18
#define EM_960		19
#define EM_PPC		20
#define EM_PPC64	21
#define EM_S390		22

#define EM_V800		36
#define EM_FR20		37
#define EM_RH32		38
#define EM_RCE		39
#define EM_ARM		40
#define EM_FAKE_ALPHA	41
#define EM_SH		42
#define EM_SPARCV9	43
#define EM_TRICORE	44
#define EM_ARC		45
#define EM_H8_300	46
#define EM_H8_300H	47
#define EM_H8S		48
#define EM_H8_500	49
#define EM_IA_64	50
#define EM_MIPS_X	51
#define EM_COLDFIRE	52
#define EM_68HC12	53
#define EM_MMA		54
#define EM_PCP		55
#define EM_NCPU		56
#define EM_NDR1		57
#define EM_STARCORE	58
#define EM_ME16		59
#define EM_ST100	60
#define EM_TINYJ	61
#define EM_X86_64	62
#define EM_PDSP		63

#define EM_FX66		66
#define EM_ST9PLUS	67
#define EM_ST7		68
#define EM_68HC16	69
#define EM_68HC11	70
#define EM_68HC08	71
#define EM_68HC05	72
#define EM_SVX		73
#define EM_ST19		74
#define EM_VAX		75
#define EM_CRIS		76
#define EM_JAVELIN	77
#define EM_FIREPATH	78
#define EM_ZSP		79
#define EM_MMIX		80
#define EM_HUANY	81
#define EM_PRISM	82
#define EM_AVR		83
#define EM_FR30		84
#define EM_D10V		85
#define EM_D30V		86
#define EM_V850		87
#define EM_M32R		88
#define EM_MN10300	89
#define EM_MN10200	90
#define EM_PJ		91
#define EM_OR1K		92
#define EM_ARC_A5	93
#define EM_XTENSA	94
#define EM_AARCH64	183
#define EM_TILEPRO	188
#define EM_MICROBLAZE	189
#define EM_TILEGX	191
#define EM_NUM		192
#define EM_ALPHA	0x9026

#define EV_NONE		0
#define EV_CURRENT	1
#define EV_NUM		2

typedef struct {
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

typedef struct {
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



#define SHN_UNDEF	0
#define SHN_LORESERVE	0xff00
#define SHN_LOPROC	0xff00
#define SHN_BEFORE	0xff00

#define SHN_AFTER	0xff01

#define SHN_HIPROC	0xff1f
#define SHN_LOOS	0xff20
#define SHN_HIOS	0xff3f
#define SHN_ABS		0xfff1
#define SHN_COMMON	0xfff2
#define SHN_XINDEX	0xffff
#define SHN_HIRESERVE	0xffff



#define SHT_NULL	  0
#define SHT_PROGBITS	  1
#define SHT_SYMTAB	  2
#define SHT_STRTAB	  3
#define SHT_RELA	  4
#define SHT_HASH	  5
#define SHT_DYNAMIC	  6
#define SHT_NOTE	  7
#define SHT_NOBITS	  8
#define SHT_REL		  9
#define SHT_SHLIB	  10
#define SHT_DYNSYM	  11
#define SHT_INIT_ARRAY	  14
#define SHT_FINI_ARRAY	  15
#define SHT_PREINIT_ARRAY 16
#define SHT_GROUP	  17
#define SHT_SYMTAB_SHNDX  18
#define	SHT_NUM		  19
#define SHT_LOOS	  0x60000000
#define SHT_GNU_ATTRIBUTES 0x6ffffff5
#define SHT_GNU_HASH	  0x6ffffff6
#define SHT_GNU_LIBLIST	  0x6ffffff7
#define SHT_CHECKSUM	  0x6ffffff8
#define SHT_LOSUNW	  0x6ffffffa
#define SHT_SUNW_move	  0x6ffffffa
#define SHT_SUNW_COMDAT   0x6ffffffb
#define SHT_SUNW_syminfo  0x6ffffffc
#define SHT_GNU_verdef	  0x6ffffffd
#define SHT_GNU_verneed	  0x6ffffffe
#define SHT_GNU_versym	  0x6fffffff
#define SHT_HISUNW	  0x6fffffff
#define SHT_HIOS	  0x6fffffff
#define SHT_LOPROC	  0x70000000
#define SHT_HIPROC	  0x7fffffff
#define SHT_LOUSER	  0x80000000
#define SHT_HIUSER	  0x8fffffff

#define SHF_WRITE	     (1 << 0)
#define SHF_ALLOC	     (1 << 1)
#define SHF_EXECINSTR	     (1 << 2)
#define SHF_MERGE	     (1 << 4)
#define SHF_STRINGS	     (1 << 5)
#define SHF_INFO_LINK	     (1 << 6)
#define SHF_LINK_ORDER	     (1 << 7)
#define SHF_OS_NONCONFORMING (1 << 8)

#define SHF_GROUP	     (1 << 9)
#define SHF_TLS		     (1 << 10)
#define SHF_MASKOS	     0x0ff00000
#define SHF_MASKPROC	     0xf0000000
#define SHF_ORDERED	     (1 << 30)
#define SHF_EXCLUDE	     (1U << 31)

#define GRP_COMDAT	0x1

typedef struct {
  Elf32_Word	st_name;
  Elf32_Addr	st_value;
  Elf32_Word	st_size;
  unsigned char	st_info;
  unsigned char	st_other;
  Elf32_Section	st_shndx;
} Elf32_Sym;

typedef struct {
  Elf64_Word	st_name;
  unsigned char	st_info;
  unsigned char st_other;
  Elf64_Section	st_shndx;
  Elf64_Addr	st_value;
  Elf64_Xword	st_size;
} Elf64_Sym;

typedef struct {
  Elf32_Half si_boundto;
  Elf32_Half si_flags;
} Elf32_Syminfo;

typedef struct {
  Elf64_Half si_boundto;
  Elf64_Half si_flags;
} Elf64_Syminfo;

#define SYMINFO_BT_SELF		0xffff
#define SYMINFO_BT_PARENT	0xfffe
#define SYMINFO_BT_LOWRESERVE	0xff00

#define SYMINFO_FLG_DIRECT	0x0001
#define SYMINFO_FLG_PASSTHRU	0x0002
#define SYMINFO_FLG_COPY	0x0004
#define SYMINFO_FLG_LAZYLOAD	0x0008

#define SYMINFO_NONE		0
#define SYMINFO_CURRENT		1
#define SYMINFO_NUM		2

#define ELF32_ST_BIND(val)		(((unsigned char) (val)) >> 4)
#define ELF32_ST_TYPE(val)		((val) & 0xf)
#define ELF32_ST_INFO(bind, type)	(((bind) << 4) + ((type) & 0xf))

#define ELF64_ST_BIND(val)		ELF32_ST_BIND (val)
#define ELF64_ST_TYPE(val)		ELF32_ST_TYPE (val)
#define ELF64_ST_INFO(bind, type)	ELF32_ST_INFO ((bind), (type))

#define STB_LOCAL	0
#define STB_GLOBAL	1
#define STB_WEAK	2
#define	STB_NUM		3
#define STB_LOOS	10
#define STB_GNU_UNIQUE	10
#define STB_HIOS	12
#define STB_LOPROC	13
#define STB_HIPROC	15

#define STT_NOTYPE	0
#define STT_OBJECT	1
#define STT_FUNC	2
#define STT_SECTION	3
#define STT_FILE	4
#define STT_COMMON	5
#define STT_TLS		6
#define	STT_NUM		7
#define STT_LOOS	10
#define STT_GNU_IFUNC	10
#define STT_HIOS	12
#define STT_LOPROC	13
#define STT_HIPROC	15

#define STN_UNDEF	0

#define ELF32_ST_VISIBILITY(o)	((o) & 0x03)
#define ELF64_ST_VISIBILITY(o)	ELF32_ST_VISIBILITY (o)

#define STV_DEFAULT	0
#define STV_INTERNAL	1
#define STV_HIDDEN	2
#define STV_PROTECTED	3




typedef struct
{
  Elf32_Addr	r_offset;
  Elf32_Word	r_info;
} Elf32_Rel;

typedef struct {
  Elf64_Addr	r_offset;
  Elf64_Xword	r_info;
} Elf64_Rel;



typedef struct {
  Elf32_Addr	r_offset;
  Elf32_Word	r_info;
  Elf32_Sword	r_addend;
} Elf32_Rela;

typedef struct {
  Elf64_Addr	r_offset;
  Elf64_Xword	r_info;
  Elf64_Sxword	r_addend;
} Elf64_Rela;



#define ELF32_R_SYM(val)		((val) >> 8)
#define ELF32_R_TYPE(val)		((val) & 0xff)
#define ELF32_R_INFO(sym, type)		(((sym) << 8) + ((type) & 0xff))

#define ELF64_R_SYM(i)			((i) >> 32)
#define ELF64_R_TYPE(i)			((i) & 0xffffffff)
#define ELF64_R_INFO(sym,type)		((((Elf64_Xword) (sym)) << 32) + (type))



typedef struct {
  Elf32_Word	p_type;
  Elf32_Off	p_offset;
  Elf32_Addr	p_vaddr;
  Elf32_Addr	p_paddr;
  Elf32_Word	p_filesz;
  Elf32_Word	p_memsz;
  Elf32_Word	p_flags;
  Elf32_Word	p_align;
} Elf32_Phdr;

typedef struct {
  Elf64_Word	p_type;
  Elf64_Word	p_flags;
  Elf64_Off	p_offset;
  Elf64_Addr	p_vaddr;
  Elf64_Addr	p_paddr;
  Elf64_Xword	p_filesz;
  Elf64_Xword	p_memsz;
  Elf64_Xword	p_align;
} Elf64_Phdr;



#define	PT_NULL		0
#define PT_LOAD		1
#define PT_DYNAMIC	2
#define PT_INTERP	3
#define PT_NOTE		4
#define PT_SHLIB	5
#define PT_PHDR		6
#define PT_TLS		7
#define	PT_NUM		8
#define PT_LOOS		0x60000000
#define PT_GNU_EH_FRAME	0x6474e550
#define PT_GNU_STACK	0x6474e551
#define PT_GNU_RELRO	0x6474e552
#define PT_LOSUNW	0x6ffffffa
#define PT_SUNWBSS	0x6ffffffa
#define PT_SUNWSTACK	0x6ffffffb
#define PT_HISUNW	0x6fffffff
#define PT_HIOS		0x6fffffff
#define PT_LOPROC	0x70000000
#define PT_HIPROC	0x7fffffff


#define PN_XNUM 0xffff


#define PF_X		(1 << 0)
#define PF_W		(1 << 1)
#define PF_R		(1 << 2)
#define PF_MASKOS	0x0ff00000
#define PF_MASKPROC	0xf0000000



#define NT_PRSTATUS	1
#define NT_FPREGSET	2
#define NT_PRPSINFO	3
#define NT_PRXREG	4
#define NT_TASKSTRUCT	4
#define NT_PLATFORM	5
#define NT_AUXV		6
#define NT_GWINDOWS	7
#define NT_ASRS		8
#define NT_PSTATUS	10
#define NT_PSINFO	13
#define NT_PRCRED	14
#define NT_UTSNAME	15
#define NT_LWPSTATUS	16
#define NT_LWPSINFO	17
#define NT_PRFPXREG	20
#define NT_SIGINFO	0x53494749
#define NT_FILE		0x46494c45
#define NT_PRXFPREG	0x46e62b7f
#define NT_PPC_VMX	0x100
#define NT_PPC_SPE	0x101
#define NT_PPC_VSX	0x102
#define NT_386_TLS	0x200
#define NT_386_IOPERM	0x201
#define NT_X86_XSTATE	0x202
#define NT_S390_HIGH_GPRS	0x300
#define NT_S390_TIMER	0x301
#define NT_S390_TODCMP	0x302
#define NT_S390_TODPREG	0x303
#define NT_S390_CTRS	0x304
#define NT_S390_PREFIX	0x305
#define NT_S390_LAST_BREAK	0x306
#define NT_S390_SYSTEM_CALL	0x307
#define NT_S390_TDB	0x308
#define NT_ARM_VFP	0x400
#define NT_ARM_TLS	0x401
#define NT_ARM_HW_BREAK	0x402
#define NT_ARM_HW_WATCH	0x403
#define NT_METAG_CBUF	0x500
#define NT_METAG_RPIPE	0x501
#define NT_METAG_TLS	0x502
#define NT_VERSION	1




typedef struct {
  Elf32_Sword d_tag;
  union {
      Elf32_Word d_val;
      Elf32_Addr d_ptr;
  } d_un;
} Elf32_Dyn;

typedef struct {
  Elf64_Sxword d_tag;
  union {
      Elf64_Xword d_val;
      Elf64_Addr d_ptr;
  } d_un;
} Elf64_Dyn;



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
#define	DT_BIND_NOW	24
#define	DT_INIT_ARRAY	25
#define	DT_FINI_ARRAY	26
#define	DT_INIT_ARRAYSZ	27
#define	DT_FINI_ARRAYSZ	28
#define DT_RUNPATH	29
#define DT_FLAGS	30
#define DT_ENCODING	32
#define DT_PREINIT_ARRAY 32
#define DT_PREINIT_ARRAYSZ 33
#define	DT_NUM		34
#define DT_LOOS		0x6000000d
#define DT_HIOS		0x6ffff000
#define DT_LOPROC	0x70000000
#define DT_HIPROC	0x7fffffff
#define	DT_PROCNUM	DT_MIPS_NUM

#define DT_VALRNGLO	0x6ffffd00
#define DT_GNU_PRELINKED 0x6ffffdf5
#define DT_GNU_CONFLICTSZ 0x6ffffdf6
#define DT_GNU_LIBLISTSZ 0x6ffffdf7
#define DT_CHECKSUM	0x6ffffdf8
#define DT_PLTPADSZ	0x6ffffdf9
#define DT_MOVEENT	0x6ffffdfa
#define DT_MOVESZ	0x6ffffdfb
#define DT_FEATURE_1	0x6ffffdfc
#define DT_POSFLAG_1	0x6ffffdfd

#define DT_SYMINSZ	0x6ffffdfe
#define DT_SYMINENT	0x6ffffdff
#define DT_VALRNGHI	0x6ffffdff
#define DT_VALTAGIDX(tag)	(DT_VALRNGHI - (tag))
#define DT_VALNUM 12

#define DT_ADDRRNGLO	0x6ffffe00
#define DT_GNU_HASH	0x6ffffef5
#define DT_TLSDESC_PLT	0x6ffffef6
#define DT_TLSDESC_GOT	0x6ffffef7
#define DT_GNU_CONFLICT	0x6ffffef8
#define DT_GNU_LIBLIST	0x6ffffef9
#define DT_CONFIG	0x6ffffefa
#define DT_DEPAUDIT	0x6ffffefb
#define DT_AUDIT	0x6ffffefc
#define	DT_PLTPAD	0x6ffffefd
#define	DT_MOVETAB	0x6ffffefe
#define DT_SYMINFO	0x6ffffeff
#define DT_ADDRRNGHI	0x6ffffeff
#define DT_ADDRTAGIDX(tag)	(DT_ADDRRNGHI - (tag))
#define DT_ADDRNUM 11



#define DT_VERSYM	0x6ffffff0

#define DT_RELACOUNT	0x6ffffff9
#define DT_RELCOUNT	0x6ffffffa


#define DT_FLAGS_1	0x6ffffffb
#define	DT_VERDEF	0x6ffffffc

#define	DT_VERDEFNUM	0x6ffffffd
#define	DT_VERNEED	0x6ffffffe

#define	DT_VERNEEDNUM	0x6fffffff
#define DT_VERSIONTAGIDX(tag)	(DT_VERNEEDNUM - (tag))
#define DT_VERSIONTAGNUM 16



#define DT_AUXILIARY    0x7ffffffd
#define DT_FILTER       0x7fffffff
#define DT_EXTRATAGIDX(tag)	((Elf32_Word)-((Elf32_Sword) (tag) <<1>>1)-1)
#define DT_EXTRANUM	3


#define DF_ORIGIN	0x00000001
#define DF_SYMBOLIC	0x00000002
#define DF_TEXTREL	0x00000004
#define DF_BIND_NOW	0x00000008
#define DF_STATIC_TLS	0x00000010



#define DF_1_NOW	0x00000001
#define DF_1_GLOBAL	0x00000002
#define DF_1_GROUP	0x00000004
#define DF_1_NODELETE	0x00000008
#define DF_1_LOADFLTR	0x00000010
#define DF_1_INITFIRST	0x00000020
#define DF_1_NOOPEN	0x00000040
#define DF_1_ORIGIN	0x00000080
#define DF_1_DIRECT	0x00000100
#define DF_1_TRANS	0x00000200
#define DF_1_INTERPOSE	0x00000400
#define DF_1_NODEFLIB	0x00000800
#define DF_1_NODUMP	0x00001000
#define DF_1_CONFALT	0x00002000
#define DF_1_ENDFILTEE	0x00004000
#define	DF_1_DISPRELDNE	0x00008000
#define	DF_1_DISPRELPND	0x00010000
#define	DF_1_NODIRECT	0x00020000
#define	DF_1_IGNMULDEF	0x00040000
#define	DF_1_NOKSYMS	0x00080000
#define	DF_1_NOHDR	0x00100000
#define	DF_1_EDITED	0x00200000
#define	DF_1_NORELOC	0x00400000
#define	DF_1_SYMINTPOSE	0x00800000
#define	DF_1_GLOBAUDIT	0x01000000
#define	DF_1_SINGLETON	0x02000000

#define DTF_1_PARINIT	0x00000001
#define DTF_1_CONFEXP	0x00000002


#define DF_P1_LAZYLOAD	0x00000001
#define DF_P1_GROUPPERM	0x00000002




typedef struct {
  Elf32_Half	vd_version;
  Elf32_Half	vd_flags;
  Elf32_Half	vd_ndx;
  Elf32_Half	vd_cnt;
  Elf32_Word	vd_hash;
  Elf32_Word	vd_aux;
  Elf32_Word	vd_next;
} Elf32_Verdef;

typedef struct {
  Elf64_Half	vd_version;
  Elf64_Half	vd_flags;
  Elf64_Half	vd_ndx;
  Elf64_Half	vd_cnt;
  Elf64_Word	vd_hash;
  Elf64_Word	vd_aux;
  Elf64_Word	vd_next;
} Elf64_Verdef;



#define VER_DEF_NONE	0
#define VER_DEF_CURRENT	1
#define VER_DEF_NUM	2


#define VER_FLG_BASE	0x1
#define VER_FLG_WEAK	0x2


#define	VER_NDX_LOCAL		0
#define	VER_NDX_GLOBAL		1
#define	VER_NDX_LORESERVE	0xff00
#define	VER_NDX_ELIMINATE	0xff01



typedef struct {
  Elf32_Word	vda_name;
  Elf32_Word	vda_next;
} Elf32_Verdaux;

typedef struct {
  Elf64_Word	vda_name;
  Elf64_Word	vda_next;
} Elf64_Verdaux;




typedef struct {
  Elf32_Half	vn_version;
  Elf32_Half	vn_cnt;
  Elf32_Word	vn_file;
  Elf32_Word	vn_aux;
  Elf32_Word	vn_next;
} Elf32_Verneed;

typedef struct {
  Elf64_Half	vn_version;
  Elf64_Half	vn_cnt;
  Elf64_Word	vn_file;
  Elf64_Word	vn_aux;
  Elf64_Word	vn_next;
} Elf64_Verneed;



#define VER_NEED_NONE	 0
#define VER_NEED_CURRENT 1
#define VER_NEED_NUM	 2



typedef struct {
  Elf32_Word	vna_hash;
  Elf32_Half	vna_flags;
  Elf32_Half	vna_other;
  Elf32_Word	vna_name;
  Elf32_Word	vna_next;
} Elf32_Vernaux;

typedef struct {
  Elf64_Word	vna_hash;
  Elf64_Half	vna_flags;
  Elf64_Half	vna_other;
  Elf64_Word	vna_name;
  Elf64_Word	vna_next;
} Elf64_Vernaux;



#define VER_FLG_WEAK	0x2



typedef struct {
  uint32_t a_type;
  union {
      uint32_t a_val;
  } a_un;
} Elf32_auxv_t;

typedef struct {
  uint64_t a_type;
  union {
      uint64_t a_val;
  } a_un;
} Elf64_auxv_t;



#define AT_NULL		0
#define AT_IGNORE	1
#define AT_EXECFD	2
#define AT_PHDR		3
#define AT_PHENT	4
#define AT_PHNUM	5
#define AT_PAGESZ	6
#define AT_BASE		7
#define AT_FLAGS	8
#define AT_ENTRY	9
#define AT_NOTELF	10
#define AT_UID		11
#define AT_EUID		12
#define AT_GID		13
#define AT_EGID		14
#define AT_CLKTCK	17


#define AT_PLATFORM	15
#define AT_HWCAP	16




#define AT_FPUCW	18


#define AT_DCACHEBSIZE	19
#define AT_ICACHEBSIZE	20
#define AT_UCACHEBSIZE	21



#define AT_IGNOREPPC	22

#define	AT_SECURE	23

#define AT_BASE_PLATFORM 24

#define AT_RANDOM	25

#define AT_HWCAP2	26

#define AT_EXECFN	31



#define AT_SYSINFO	32
#define AT_SYSINFO_EHDR	33



#define AT_L1I_CACHESHAPE	34
#define AT_L1D_CACHESHAPE	35
#define AT_L2_CACHESHAPE	36
#define AT_L3_CACHESHAPE	37




typedef struct {
  Elf32_Word n_namesz;
  Elf32_Word n_descsz;
  Elf32_Word n_type;
} Elf32_Nhdr;

typedef struct {
  Elf64_Word n_namesz;
  Elf64_Word n_descsz;
  Elf64_Word n_type;
} Elf64_Nhdr;




#define ELF_NOTE_SOLARIS	"SUNW Solaris"


#define ELF_NOTE_GNU		"GNU"





#define ELF_NOTE_PAGESIZE_HINT	1


#define NT_GNU_ABI_TAG	1
#define ELF_NOTE_ABI	NT_GNU_ABI_TAG



#define ELF_NOTE_OS_LINUX	0
#define ELF_NOTE_OS_GNU		1
#define ELF_NOTE_OS_SOLARIS2	2
#define ELF_NOTE_OS_FREEBSD	3

#define NT_GNU_BUILD_ID	3
#define NT_GNU_GOLD_VERSION	4



typedef struct {
  Elf32_Xword m_value;
  Elf32_Word m_info;
  Elf32_Word m_poffset;
  Elf32_Half m_repeat;
  Elf32_Half m_stride;
} Elf32_Move;

typedef struct {
  Elf64_Xword m_value;
  Elf64_Xword m_info;
  Elf64_Xword m_poffset;
  Elf64_Half m_repeat;
  Elf64_Half m_stride;
} Elf64_Move;


#define ELF32_M_SYM(info)	((info) >> 8)
#define ELF32_M_SIZE(info)	((unsigned char) (info))
#define ELF32_M_INFO(sym, size)	(((sym) << 8) + (unsigned char) (size))

#define ELF64_M_SYM(info)	ELF32_M_SYM (info)
#define ELF64_M_SIZE(info)	ELF32_M_SIZE (info)
#define ELF64_M_INFO(sym, size)	ELF32_M_INFO (sym, size)

#define EF_CPU32	0x00810000

#define R_68K_NONE	0
#define R_68K_32	1
#define R_68K_16	2
#define R_68K_8		3
#define R_68K_PC32	4
#define R_68K_PC16	5
#define R_68K_PC8	6
#define R_68K_GOT32	7
#define R_68K_GOT16	8
#define R_68K_GOT8	9
#define R_68K_GOT32O	10
#define R_68K_GOT16O	11
#define R_68K_GOT8O	12
#define R_68K_PLT32	13
#define R_68K_PLT16	14
#define R_68K_PLT8	15
#define R_68K_PLT32O	16
#define R_68K_PLT16O	17
#define R_68K_PLT8O	18
#define R_68K_COPY	19
#define R_68K_GLOB_DAT	20
#define R_68K_JMP_SLOT	21
#define R_68K_RELATIVE	22
#define R_68K_NUM	23

#define R_386_NONE	   0
#define R_386_32	   1
#define R_386_PC32	   2
#define R_386_GOT32	   3
#define R_386_PLT32	   4
#define R_386_COPY	   5
#define R_386_GLOB_DAT	   6
#define R_386_JMP_SLOT	   7
#define R_386_RELATIVE	   8
#define R_386_GOTOFF	   9
#define R_386_GOTPC	   10
#define R_386_32PLT	   11
#define R_386_TLS_TPOFF	   14
#define R_386_TLS_IE	   15
#define R_386_TLS_GOTIE	   16
#define R_386_TLS_LE	   17
#define R_386_TLS_GD	   18
#define R_386_TLS_LDM	   19
#define R_386_16	   20
#define R_386_PC16	   21
#define R_386_8		   22
#define R_386_PC8	   23
#define R_386_TLS_GD_32	   24
#define R_386_TLS_GD_PUSH  25
#define R_386_TLS_GD_CALL  26
#define R_386_TLS_GD_POP   27
#define R_386_TLS_LDM_32   28
#define R_386_TLS_LDM_PUSH 29
#define R_386_TLS_LDM_CALL 30
#define R_386_TLS_LDM_POP  31
#define R_386_TLS_LDO_32   32
#define R_386_TLS_IE_32	   33
#define R_386_TLS_LE_32	   34
#define R_386_TLS_DTPMOD32 35
#define R_386_TLS_DTPOFF32 36
#define R_386_TLS_TPOFF32  37
#define R_386_SIZE32       38
#define R_386_TLS_GOTDESC  39
#define R_386_TLS_DESC_CALL 40
#define R_386_TLS_DESC     41
#define R_386_IRELATIVE	   42
#define R_386_NUM	   43





#define STT_SPARC_REGISTER	13



#define EF_SPARCV9_MM		3
#define EF_SPARCV9_TSO		0
#define EF_SPARCV9_PSO		1
#define EF_SPARCV9_RMO		2
#define EF_SPARC_LEDATA		0x800000
#define EF_SPARC_EXT_MASK	0xFFFF00
#define EF_SPARC_32PLUS		0x000100
#define EF_SPARC_SUN_US1	0x000200
#define EF_SPARC_HAL_R1		0x000400
#define EF_SPARC_SUN_US3	0x000800



#define R_SPARC_NONE		0
#define R_SPARC_8		1
#define R_SPARC_16		2
#define R_SPARC_32		3
#define R_SPARC_DISP8		4
#define R_SPARC_DISP16		5
#define R_SPARC_DISP32		6
#define R_SPARC_WDISP30		7
#define R_SPARC_WDISP22		8
#define R_SPARC_HI22		9
#define R_SPARC_22		10
#define R_SPARC_13		11
#define R_SPARC_LO10		12
#define R_SPARC_GOT10		13
#define R_SPARC_GOT13		14
#define R_SPARC_GOT22		15
#define R_SPARC_PC10		16
#define R_SPARC_PC22		17
#define R_SPARC_WPLT30		18
#define R_SPARC_COPY		19
#define R_SPARC_GLOB_DAT	20
#define R_SPARC_JMP_SLOT	21
#define R_SPARC_RELATIVE	22
#define R_SPARC_UA32		23



#define R_SPARC_PLT32		24
#define R_SPARC_HIPLT22		25
#define R_SPARC_LOPLT10		26
#define R_SPARC_PCPLT32		27
#define R_SPARC_PCPLT22		28
#define R_SPARC_PCPLT10		29
#define R_SPARC_10		30
#define R_SPARC_11		31
#define R_SPARC_64		32
#define R_SPARC_OLO10		33
#define R_SPARC_HH22		34
#define R_SPARC_HM10		35
#define R_SPARC_LM22		36
#define R_SPARC_PC_HH22		37
#define R_SPARC_PC_HM10		38
#define R_SPARC_PC_LM22		39
#define R_SPARC_WDISP16		40
#define R_SPARC_WDISP19		41
#define R_SPARC_GLOB_JMP	42
#define R_SPARC_7		43
#define R_SPARC_5		44
#define R_SPARC_6		45
#define R_SPARC_DISP64		46
#define R_SPARC_PLT64		47
#define R_SPARC_HIX22		48
#define R_SPARC_LOX10		49
#define R_SPARC_H44		50
#define R_SPARC_M44		51
#define R_SPARC_L44		52
#define R_SPARC_REGISTER	53
#define R_SPARC_UA64		54
#define R_SPARC_UA16		55
#define R_SPARC_TLS_GD_HI22	56
#define R_SPARC_TLS_GD_LO10	57
#define R_SPARC_TLS_GD_ADD	58
#define R_SPARC_TLS_GD_CALL	59
#define R_SPARC_TLS_LDM_HI22	60
#define R_SPARC_TLS_LDM_LO10	61
#define R_SPARC_TLS_LDM_ADD	62
#define R_SPARC_TLS_LDM_CALL	63
#define R_SPARC_TLS_LDO_HIX22	64
#define R_SPARC_TLS_LDO_LOX10	65
#define R_SPARC_TLS_LDO_ADD	66
#define R_SPARC_TLS_IE_HI22	67
#define R_SPARC_TLS_IE_LO10	68
#define R_SPARC_TLS_IE_LD	69
#define R_SPARC_TLS_IE_LDX	70
#define R_SPARC_TLS_IE_ADD	71
#define R_SPARC_TLS_LE_HIX22	72
#define R_SPARC_TLS_LE_LOX10	73
#define R_SPARC_TLS_DTPMOD32	74
#define R_SPARC_TLS_DTPMOD64	75
#define R_SPARC_TLS_DTPOFF32	76
#define R_SPARC_TLS_DTPOFF64	77
#define R_SPARC_TLS_TPOFF32	78
#define R_SPARC_TLS_TPOFF64	79
#define R_SPARC_GOTDATA_HIX22	80
#define R_SPARC_GOTDATA_LOX10	81
#define R_SPARC_GOTDATA_OP_HIX22	82
#define R_SPARC_GOTDATA_OP_LOX10	83
#define R_SPARC_GOTDATA_OP	84
#define R_SPARC_H34		85
#define R_SPARC_SIZE32		86
#define R_SPARC_SIZE64		87
#define R_SPARC_GNU_VTINHERIT	250
#define R_SPARC_GNU_VTENTRY	251
#define R_SPARC_REV32		252

#define R_SPARC_NUM		253



#define DT_SPARC_REGISTER 0x70000001
#define DT_SPARC_NUM	2


#define EF_MIPS_NOREORDER   1
#define EF_MIPS_PIC	    2
#define EF_MIPS_CPIC	    4
#define EF_MIPS_XGOT	    8
#define EF_MIPS_64BIT_WHIRL 16
#define EF_MIPS_ABI2	    32
#define EF_MIPS_ABI_ON32    64
#define EF_MIPS_NAN2008     1024
#define EF_MIPS_ARCH	    0xf0000000



#define EF_MIPS_ARCH_1	    0x00000000
#define EF_MIPS_ARCH_2	    0x10000000
#define EF_MIPS_ARCH_3	    0x20000000
#define EF_MIPS_ARCH_4	    0x30000000
#define EF_MIPS_ARCH_5	    0x40000000
#define EF_MIPS_ARCH_32     0x50000000
#define EF_MIPS_ARCH_64     0x60000000
#define EF_MIPS_ARCH_32R2   0x70000000
#define EF_MIPS_ARCH_64R2   0x80000000


#define E_MIPS_ARCH_1	  0x00000000
#define E_MIPS_ARCH_2	  0x10000000
#define E_MIPS_ARCH_3	  0x20000000
#define E_MIPS_ARCH_4	  0x30000000
#define E_MIPS_ARCH_5	  0x40000000
#define E_MIPS_ARCH_32	  0x50000000
#define E_MIPS_ARCH_64	  0x60000000



#define SHN_MIPS_ACOMMON    0xff00
#define SHN_MIPS_TEXT	    0xff01
#define SHN_MIPS_DATA	    0xff02
#define SHN_MIPS_SCOMMON    0xff03
#define SHN_MIPS_SUNDEFINED 0xff04



#define SHT_MIPS_LIBLIST       0x70000000
#define SHT_MIPS_MSYM	       0x70000001
#define SHT_MIPS_CONFLICT      0x70000002
#define SHT_MIPS_GPTAB	       0x70000003
#define SHT_MIPS_UCODE	       0x70000004
#define SHT_MIPS_DEBUG	       0x70000005
#define SHT_MIPS_REGINFO       0x70000006
#define SHT_MIPS_PACKAGE       0x70000007
#define SHT_MIPS_PACKSYM       0x70000008
#define SHT_MIPS_RELD	       0x70000009
#define SHT_MIPS_IFACE         0x7000000b
#define SHT_MIPS_CONTENT       0x7000000c
#define SHT_MIPS_OPTIONS       0x7000000d
#define SHT_MIPS_SHDR	       0x70000010
#define SHT_MIPS_FDESC	       0x70000011
#define SHT_MIPS_EXTSYM	       0x70000012
#define SHT_MIPS_DENSE	       0x70000013
#define SHT_MIPS_PDESC	       0x70000014
#define SHT_MIPS_LOCSYM	       0x70000015
#define SHT_MIPS_AUXSYM	       0x70000016
#define SHT_MIPS_OPTSYM	       0x70000017
#define SHT_MIPS_LOCSTR	       0x70000018
#define SHT_MIPS_LINE	       0x70000019
#define SHT_MIPS_RFDESC	       0x7000001a
#define SHT_MIPS_DELTASYM      0x7000001b
#define SHT_MIPS_DELTAINST     0x7000001c
#define SHT_MIPS_DELTACLASS    0x7000001d
#define SHT_MIPS_DWARF         0x7000001e
#define SHT_MIPS_DELTADECL     0x7000001f
#define SHT_MIPS_SYMBOL_LIB    0x70000020
#define SHT_MIPS_EVENTS	       0x70000021
#define SHT_MIPS_TRANSLATE     0x70000022
#define SHT_MIPS_PIXIE	       0x70000023
#define SHT_MIPS_XLATE	       0x70000024
#define SHT_MIPS_XLATE_DEBUG   0x70000025
#define SHT_MIPS_WHIRL	       0x70000026
#define SHT_MIPS_EH_REGION     0x70000027
#define SHT_MIPS_XLATE_OLD     0x70000028
#define SHT_MIPS_PDR_EXCEPTION 0x70000029



#define SHF_MIPS_GPREL	 0x10000000
#define SHF_MIPS_MERGE	 0x20000000
#define SHF_MIPS_ADDR	 0x40000000
#define SHF_MIPS_STRINGS 0x80000000
#define SHF_MIPS_NOSTRIP 0x08000000
#define SHF_MIPS_LOCAL	 0x04000000
#define SHF_MIPS_NAMES	 0x02000000
#define SHF_MIPS_NODUPE	 0x01000000





#define STO_MIPS_DEFAULT		0x0
#define STO_MIPS_INTERNAL		0x1
#define STO_MIPS_HIDDEN			0x2
#define STO_MIPS_PROTECTED		0x3
#define STO_MIPS_PLT			0x8
#define STO_MIPS_SC_ALIGN_UNUSED	0xff


#define STB_MIPS_SPLIT_COMMON		13



typedef union {
  struct {
      Elf32_Word gt_current_g_value;
      Elf32_Word gt_unused;
  } gt_header;
  struct {
      Elf32_Word gt_g_value;
      Elf32_Word gt_bytes;
  } gt_entry;
} Elf32_gptab;



typedef struct {
  Elf32_Word	ri_gprmask;
  Elf32_Word	ri_cprmask[4];
  Elf32_Sword	ri_gp_value;
} Elf32_RegInfo;



typedef struct {
  unsigned char kind;

  unsigned char size;
  Elf32_Section section;

  Elf32_Word info;
} Elf_Options;



#define ODK_NULL	0
#define ODK_REGINFO	1
#define ODK_EXCEPTIONS	2
#define ODK_PAD		3
#define ODK_HWPATCH	4
#define ODK_FILL	5
#define ODK_TAGS	6
#define ODK_HWAND	7
#define ODK_HWOR	8



#define OEX_FPU_MIN	0x1f
#define OEX_FPU_MAX	0x1f00
#define OEX_PAGE0	0x10000
#define OEX_SMM		0x20000
#define OEX_FPDBUG	0x40000
#define OEX_PRECISEFP	OEX_FPDBUG
#define OEX_DISMISS	0x80000

#define OEX_FPU_INVAL	0x10
#define OEX_FPU_DIV0	0x08
#define OEX_FPU_OFLO	0x04
#define OEX_FPU_UFLO	0x02
#define OEX_FPU_INEX	0x01



#define OHW_R4KEOP	0x1
#define OHW_R8KPFETCH	0x2
#define OHW_R5KEOP	0x4
#define OHW_R5KCVTL	0x8

#define OPAD_PREFIX	0x1
#define OPAD_POSTFIX	0x2
#define OPAD_SYMBOL	0x4



typedef struct {
  Elf32_Word hwp_flags1;
  Elf32_Word hwp_flags2;
} Elf_Options_Hw;



#define OHWA0_R4KEOP_CHECKED	0x00000001
#define OHWA1_R4KEOP_CLEAN	0x00000002



#define R_MIPS_NONE		0
#define R_MIPS_16		1
#define R_MIPS_32		2
#define R_MIPS_REL32		3
#define R_MIPS_26		4
#define R_MIPS_HI16		5
#define R_MIPS_LO16		6
#define R_MIPS_GPREL16		7
#define R_MIPS_LITERAL		8
#define R_MIPS_GOT16		9
#define R_MIPS_PC16		10
#define R_MIPS_CALL16		11
#define R_MIPS_GPREL32		12

#define R_MIPS_SHIFT5		16
#define R_MIPS_SHIFT6		17
#define R_MIPS_64		18
#define R_MIPS_GOT_DISP		19
#define R_MIPS_GOT_PAGE		20
#define R_MIPS_GOT_OFST		21
#define R_MIPS_GOT_HI16		22
#define R_MIPS_GOT_LO16		23
#define R_MIPS_SUB		24
#define R_MIPS_INSERT_A		25
#define R_MIPS_INSERT_B		26
#define R_MIPS_DELETE		27
#define R_MIPS_HIGHER		28
#define R_MIPS_HIGHEST		29
#define R_MIPS_CALL_HI16	30
#define R_MIPS_CALL_LO16	31
#define R_MIPS_SCN_DISP		32
#define R_MIPS_REL16		33
#define R_MIPS_ADD_IMMEDIATE	34
#define R_MIPS_PJUMP		35
#define R_MIPS_RELGOT		36
#define R_MIPS_JALR		37
#define R_MIPS_TLS_DTPMOD32	38
#define R_MIPS_TLS_DTPREL32	39
#define R_MIPS_TLS_DTPMOD64	40
#define R_MIPS_TLS_DTPREL64	41
#define R_MIPS_TLS_GD		42
#define R_MIPS_TLS_LDM		43
#define R_MIPS_TLS_DTPREL_HI16	44
#define R_MIPS_TLS_DTPREL_LO16	45
#define R_MIPS_TLS_GOTTPREL	46
#define R_MIPS_TLS_TPREL32	47
#define R_MIPS_TLS_TPREL64	48
#define R_MIPS_TLS_TPREL_HI16	49
#define R_MIPS_TLS_TPREL_LO16	50
#define R_MIPS_GLOB_DAT		51
#define R_MIPS_COPY		126
#define R_MIPS_JUMP_SLOT        127

#define R_MIPS_NUM		128



#define PT_MIPS_REGINFO	0x70000000
#define PT_MIPS_RTPROC  0x70000001
#define PT_MIPS_OPTIONS 0x70000002



#define PF_MIPS_LOCAL	0x10000000



#define DT_MIPS_RLD_VERSION  0x70000001
#define DT_MIPS_TIME_STAMP   0x70000002
#define DT_MIPS_ICHECKSUM    0x70000003
#define DT_MIPS_IVERSION     0x70000004
#define DT_MIPS_FLAGS	     0x70000005
#define DT_MIPS_BASE_ADDRESS 0x70000006
#define DT_MIPS_MSYM	     0x70000007
#define DT_MIPS_CONFLICT     0x70000008
#define DT_MIPS_LIBLIST	     0x70000009
#define DT_MIPS_LOCAL_GOTNO  0x7000000a
#define DT_MIPS_CONFLICTNO   0x7000000b
#define DT_MIPS_LIBLISTNO    0x70000010
#define DT_MIPS_SYMTABNO     0x70000011
#define DT_MIPS_UNREFEXTNO   0x70000012
#define DT_MIPS_GOTSYM	     0x70000013
#define DT_MIPS_HIPAGENO     0x70000014
#define DT_MIPS_RLD_MAP	     0x70000016
#define DT_MIPS_DELTA_CLASS  0x70000017
#define DT_MIPS_DELTA_CLASS_NO    0x70000018

#define DT_MIPS_DELTA_INSTANCE    0x70000019
#define DT_MIPS_DELTA_INSTANCE_NO 0x7000001a

#define DT_MIPS_DELTA_RELOC  0x7000001b
#define DT_MIPS_DELTA_RELOC_NO 0x7000001c

#define DT_MIPS_DELTA_SYM    0x7000001d

#define DT_MIPS_DELTA_SYM_NO 0x7000001e

#define DT_MIPS_DELTA_CLASSSYM 0x70000020

#define DT_MIPS_DELTA_CLASSSYM_NO 0x70000021

#define DT_MIPS_CXX_FLAGS    0x70000022
#define DT_MIPS_PIXIE_INIT   0x70000023
#define DT_MIPS_SYMBOL_LIB   0x70000024
#define DT_MIPS_LOCALPAGE_GOTIDX 0x70000025
#define DT_MIPS_LOCAL_GOTIDX 0x70000026
#define DT_MIPS_HIDDEN_GOTIDX 0x70000027
#define DT_MIPS_PROTECTED_GOTIDX 0x70000028
#define DT_MIPS_OPTIONS	     0x70000029
#define DT_MIPS_INTERFACE    0x7000002a
#define DT_MIPS_DYNSTR_ALIGN 0x7000002b
#define DT_MIPS_INTERFACE_SIZE 0x7000002c
#define DT_MIPS_RLD_TEXT_RESOLVE_ADDR 0x7000002d

#define DT_MIPS_PERF_SUFFIX  0x7000002e

#define DT_MIPS_COMPACT_SIZE 0x7000002f
#define DT_MIPS_GP_VALUE     0x70000030
#define DT_MIPS_AUX_DYNAMIC  0x70000031

#define DT_MIPS_PLTGOT	     0x70000032

#define DT_MIPS_RWPLT        0x70000034
#define DT_MIPS_NUM	     0x35



#define RHF_NONE		   0
#define RHF_QUICKSTART		   (1 << 0)
#define RHF_NOTPOT		   (1 << 1)
#define RHF_NO_LIBRARY_REPLACEMENT (1 << 2)
#define RHF_NO_MOVE		   (1 << 3)
#define RHF_SGI_ONLY		   (1 << 4)
#define RHF_GUARANTEE_INIT	   (1 << 5)
#define RHF_DELTA_C_PLUS_PLUS	   (1 << 6)
#define RHF_GUARANTEE_START_INIT   (1 << 7)
#define RHF_PIXIE		   (1 << 8)
#define RHF_DEFAULT_DELAY_LOAD	   (1 << 9)
#define RHF_REQUICKSTART	   (1 << 10)
#define RHF_REQUICKSTARTED	   (1 << 11)
#define RHF_CORD		   (1 << 12)
#define RHF_NO_UNRES_UNDEF	   (1 << 13)
#define RHF_RLD_ORDER_SAFE	   (1 << 14)



typedef struct
{
  Elf32_Word l_name;
  Elf32_Word l_time_stamp;
  Elf32_Word l_checksum;
  Elf32_Word l_version;
  Elf32_Word l_flags;
} Elf32_Lib;

typedef struct
{
  Elf64_Word l_name;
  Elf64_Word l_time_stamp;
  Elf64_Word l_checksum;
  Elf64_Word l_version;
  Elf64_Word l_flags;
} Elf64_Lib;




#define LL_NONE		  0
#define LL_EXACT_MATCH	  (1 << 0)
#define LL_IGNORE_INT_VER (1 << 1)
#define LL_REQUIRE_MINOR  (1 << 2)
#define LL_EXPORTS	  (1 << 3)
#define LL_DELAY_LOAD	  (1 << 4)
#define LL_DELTA	  (1 << 5)



typedef Elf32_Addr Elf32_Conflict;






#define EF_PARISC_TRAPNIL	0x00010000
#define EF_PARISC_EXT		0x00020000
#define EF_PARISC_LSB		0x00040000
#define EF_PARISC_WIDE		0x00080000
#define EF_PARISC_NO_KABP	0x00100000

#define EF_PARISC_LAZYSWAP	0x00400000
#define EF_PARISC_ARCH		0x0000ffff



#define EFA_PARISC_1_0		    0x020b
#define EFA_PARISC_1_1		    0x0210
#define EFA_PARISC_2_0		    0x0214



#define SHN_PARISC_ANSI_COMMON	0xff00

#define SHN_PARISC_HUGE_COMMON	0xff01



#define SHT_PARISC_EXT		0x70000000
#define SHT_PARISC_UNWIND	0x70000001
#define SHT_PARISC_DOC		0x70000002



#define SHF_PARISC_SHORT	0x20000000
#define SHF_PARISC_HUGE		0x40000000
#define SHF_PARISC_SBP		0x80000000



#define STT_PARISC_MILLICODE	13

#define STT_HP_OPAQUE		(STT_LOOS + 0x1)
#define STT_HP_STUB		(STT_LOOS + 0x2)



#define R_PARISC_NONE		0
#define R_PARISC_DIR32		1
#define R_PARISC_DIR21L		2
#define R_PARISC_DIR17R		3
#define R_PARISC_DIR17F		4
#define R_PARISC_DIR14R		6
#define R_PARISC_PCREL32	9
#define R_PARISC_PCREL21L	10
#define R_PARISC_PCREL17R	11
#define R_PARISC_PCREL17F	12
#define R_PARISC_PCREL14R	14
#define R_PARISC_DPREL21L	18
#define R_PARISC_DPREL14R	22
#define R_PARISC_GPREL21L	26
#define R_PARISC_GPREL14R	30
#define R_PARISC_LTOFF21L	34
#define R_PARISC_LTOFF14R	38
#define R_PARISC_SECREL32	41
#define R_PARISC_SEGBASE	48
#define R_PARISC_SEGREL32	49
#define R_PARISC_PLTOFF21L	50
#define R_PARISC_PLTOFF14R	54
#define R_PARISC_LTOFF_FPTR32	57
#define R_PARISC_LTOFF_FPTR21L	58
#define R_PARISC_LTOFF_FPTR14R	62
#define R_PARISC_FPTR64		64
#define R_PARISC_PLABEL32	65
#define R_PARISC_PLABEL21L	66
#define R_PARISC_PLABEL14R	70
#define R_PARISC_PCREL64	72
#define R_PARISC_PCREL22F	74
#define R_PARISC_PCREL14WR	75
#define R_PARISC_PCREL14DR	76
#define R_PARISC_PCREL16F	77
#define R_PARISC_PCREL16WF	78
#define R_PARISC_PCREL16DF	79
#define R_PARISC_DIR64		80
#define R_PARISC_DIR14WR	83
#define R_PARISC_DIR14DR	84
#define R_PARISC_DIR16F		85
#define R_PARISC_DIR16WF	86
#define R_PARISC_DIR16DF	87
#define R_PARISC_GPREL64	88
#define R_PARISC_GPREL14WR	91
#define R_PARISC_GPREL14DR	92
#define R_PARISC_GPREL16F	93
#define R_PARISC_GPREL16WF	94
#define R_PARISC_GPREL16DF	95
#define R_PARISC_LTOFF64	96
#define R_PARISC_LTOFF14WR	99
#define R_PARISC_LTOFF14DR	100
#define R_PARISC_LTOFF16F	101
#define R_PARISC_LTOFF16WF	102
#define R_PARISC_LTOFF16DF	103
#define R_PARISC_SECREL64	104
#define R_PARISC_SEGREL64	112
#define R_PARISC_PLTOFF14WR	115
#define R_PARISC_PLTOFF14DR	116
#define R_PARISC_PLTOFF16F	117
#define R_PARISC_PLTOFF16WF	118
#define R_PARISC_PLTOFF16DF	119
#define R_PARISC_LTOFF_FPTR64	120
#define R_PARISC_LTOFF_FPTR14WR	123
#define R_PARISC_LTOFF_FPTR14DR	124
#define R_PARISC_LTOFF_FPTR16F	125
#define R_PARISC_LTOFF_FPTR16WF	126
#define R_PARISC_LTOFF_FPTR16DF	127
#define R_PARISC_LORESERVE	128
#define R_PARISC_COPY		128
#define R_PARISC_IPLT		129
#define R_PARISC_EPLT		130
#define R_PARISC_TPREL32	153
#define R_PARISC_TPREL21L	154
#define R_PARISC_TPREL14R	158
#define R_PARISC_LTOFF_TP21L	162
#define R_PARISC_LTOFF_TP14R	166
#define R_PARISC_LTOFF_TP14F	167
#define R_PARISC_TPREL64	216
#define R_PARISC_TPREL14WR	219
#define R_PARISC_TPREL14DR	220
#define R_PARISC_TPREL16F	221
#define R_PARISC_TPREL16WF	222
#define R_PARISC_TPREL16DF	223
#define R_PARISC_LTOFF_TP64	224
#define R_PARISC_LTOFF_TP14WR	227
#define R_PARISC_LTOFF_TP14DR	228
#define R_PARISC_LTOFF_TP16F	229
#define R_PARISC_LTOFF_TP16WF	230
#define R_PARISC_LTOFF_TP16DF	231
#define R_PARISC_GNU_VTENTRY	232
#define R_PARISC_GNU_VTINHERIT	233
#define R_PARISC_TLS_GD21L	234
#define R_PARISC_TLS_GD14R	235
#define R_PARISC_TLS_GDCALL	236
#define R_PARISC_TLS_LDM21L	237
#define R_PARISC_TLS_LDM14R	238
#define R_PARISC_TLS_LDMCALL	239
#define R_PARISC_TLS_LDO21L	240
#define R_PARISC_TLS_LDO14R	241
#define R_PARISC_TLS_DTPMOD32	242
#define R_PARISC_TLS_DTPMOD64	243
#define R_PARISC_TLS_DTPOFF32	244
#define R_PARISC_TLS_DTPOFF64	245
#define R_PARISC_TLS_LE21L	R_PARISC_TPREL21L
#define R_PARISC_TLS_LE14R	R_PARISC_TPREL14R
#define R_PARISC_TLS_IE21L	R_PARISC_LTOFF_TP21L
#define R_PARISC_TLS_IE14R	R_PARISC_LTOFF_TP14R
#define R_PARISC_TLS_TPREL32	R_PARISC_TPREL32
#define R_PARISC_TLS_TPREL64	R_PARISC_TPREL64
#define R_PARISC_HIRESERVE	255



#define PT_HP_TLS		(PT_LOOS + 0x0)
#define PT_HP_CORE_NONE		(PT_LOOS + 0x1)
#define PT_HP_CORE_VERSION	(PT_LOOS + 0x2)
#define PT_HP_CORE_KERNEL	(PT_LOOS + 0x3)
#define PT_HP_CORE_COMM		(PT_LOOS + 0x4)
#define PT_HP_CORE_PROC		(PT_LOOS + 0x5)
#define PT_HP_CORE_LOADABLE	(PT_LOOS + 0x6)
#define PT_HP_CORE_STACK	(PT_LOOS + 0x7)
#define PT_HP_CORE_SHM		(PT_LOOS + 0x8)
#define PT_HP_CORE_MMF		(PT_LOOS + 0x9)
#define PT_HP_PARALLEL		(PT_LOOS + 0x10)
#define PT_HP_FASTBIND		(PT_LOOS + 0x11)
#define PT_HP_OPT_ANNOT		(PT_LOOS + 0x12)
#define PT_HP_HSL_ANNOT		(PT_LOOS + 0x13)
#define PT_HP_STACK		(PT_LOOS + 0x14)

#define PT_PARISC_ARCHEXT	0x70000000
#define PT_PARISC_UNWIND	0x70000001



#define PF_PARISC_SBP		0x08000000

#define PF_HP_PAGE_SIZE		0x00100000
#define PF_HP_FAR_SHARED	0x00200000
#define PF_HP_NEAR_SHARED	0x00400000
#define PF_HP_CODE		0x01000000
#define PF_HP_MODIFY		0x02000000
#define PF_HP_LAZYSWAP		0x04000000
#define PF_HP_SBP		0x08000000






#define EF_ALPHA_32BIT		1
#define EF_ALPHA_CANRELAX	2




#define SHT_ALPHA_DEBUG		0x70000001
#define SHT_ALPHA_REGINFO	0x70000002



#define SHF_ALPHA_GPREL		0x10000000


#define STO_ALPHA_NOPV		0x80
#define STO_ALPHA_STD_GPLOAD	0x88



#define R_ALPHA_NONE		0
#define R_ALPHA_REFLONG		1
#define R_ALPHA_REFQUAD		2
#define R_ALPHA_GPREL32		3
#define R_ALPHA_LITERAL		4
#define R_ALPHA_LITUSE		5
#define R_ALPHA_GPDISP		6
#define R_ALPHA_BRADDR		7
#define R_ALPHA_HINT		8
#define R_ALPHA_SREL16		9
#define R_ALPHA_SREL32		10
#define R_ALPHA_SREL64		11
#define R_ALPHA_GPRELHIGH	17
#define R_ALPHA_GPRELLOW	18
#define R_ALPHA_GPREL16		19
#define R_ALPHA_COPY		24
#define R_ALPHA_GLOB_DAT	25
#define R_ALPHA_JMP_SLOT	26
#define R_ALPHA_RELATIVE	27
#define R_ALPHA_TLS_GD_HI	28
#define R_ALPHA_TLSGD		29
#define R_ALPHA_TLS_LDM		30
#define R_ALPHA_DTPMOD64	31
#define R_ALPHA_GOTDTPREL	32
#define R_ALPHA_DTPREL64	33
#define R_ALPHA_DTPRELHI	34
#define R_ALPHA_DTPRELLO	35
#define R_ALPHA_DTPREL16	36
#define R_ALPHA_GOTTPREL	37
#define R_ALPHA_TPREL64		38
#define R_ALPHA_TPRELHI		39
#define R_ALPHA_TPRELLO		40
#define R_ALPHA_TPREL16		41

#define R_ALPHA_NUM		46


#define LITUSE_ALPHA_ADDR	0
#define LITUSE_ALPHA_BASE	1
#define LITUSE_ALPHA_BYTOFF	2
#define LITUSE_ALPHA_JSR	3
#define LITUSE_ALPHA_TLS_GD	4
#define LITUSE_ALPHA_TLS_LDM	5


#define DT_ALPHA_PLTRO		(DT_LOPROC + 0)
#define DT_ALPHA_NUM		1




#define EF_PPC_EMB		0x80000000


#define EF_PPC_RELOCATABLE	0x00010000
#define EF_PPC_RELOCATABLE_LIB	0x00008000



#define R_PPC_NONE		0
#define R_PPC_ADDR32		1
#define R_PPC_ADDR24		2
#define R_PPC_ADDR16		3
#define R_PPC_ADDR16_LO		4
#define R_PPC_ADDR16_HI		5
#define R_PPC_ADDR16_HA		6
#define R_PPC_ADDR14		7
#define R_PPC_ADDR14_BRTAKEN	8
#define R_PPC_ADDR14_BRNTAKEN	9
#define R_PPC_REL24		10
#define R_PPC_REL14		11
#define R_PPC_REL14_BRTAKEN	12
#define R_PPC_REL14_BRNTAKEN	13
#define R_PPC_GOT16		14
#define R_PPC_GOT16_LO		15
#define R_PPC_GOT16_HI		16
#define R_PPC_GOT16_HA		17
#define R_PPC_PLTREL24		18
#define R_PPC_COPY		19
#define R_PPC_GLOB_DAT		20
#define R_PPC_JMP_SLOT		21
#define R_PPC_RELATIVE		22
#define R_PPC_LOCAL24PC		23
#define R_PPC_UADDR32		24
#define R_PPC_UADDR16		25
#define R_PPC_REL32		26
#define R_PPC_PLT32		27
#define R_PPC_PLTREL32		28
#define R_PPC_PLT16_LO		29
#define R_PPC_PLT16_HI		30
#define R_PPC_PLT16_HA		31
#define R_PPC_SDAREL16		32
#define R_PPC_SECTOFF		33
#define R_PPC_SECTOFF_LO	34
#define R_PPC_SECTOFF_HI	35
#define R_PPC_SECTOFF_HA	36


#define R_PPC_TLS		67
#define R_PPC_DTPMOD32		68
#define R_PPC_TPREL16		69
#define R_PPC_TPREL16_LO	70
#define R_PPC_TPREL16_HI	71
#define R_PPC_TPREL16_HA	72
#define R_PPC_TPREL32		73
#define R_PPC_DTPREL16		74
#define R_PPC_DTPREL16_LO	75
#define R_PPC_DTPREL16_HI	76
#define R_PPC_DTPREL16_HA	77
#define R_PPC_DTPREL32		78
#define R_PPC_GOT_TLSGD16	79
#define R_PPC_GOT_TLSGD16_LO	80
#define R_PPC_GOT_TLSGD16_HI	81
#define R_PPC_GOT_TLSGD16_HA	82
#define R_PPC_GOT_TLSLD16	83
#define R_PPC_GOT_TLSLD16_LO	84
#define R_PPC_GOT_TLSLD16_HI	85
#define R_PPC_GOT_TLSLD16_HA	86
#define R_PPC_GOT_TPREL16	87
#define R_PPC_GOT_TPREL16_LO	88
#define R_PPC_GOT_TPREL16_HI	89
#define R_PPC_GOT_TPREL16_HA	90
#define R_PPC_GOT_DTPREL16	91
#define R_PPC_GOT_DTPREL16_LO	92
#define R_PPC_GOT_DTPREL16_HI	93
#define R_PPC_GOT_DTPREL16_HA	94



#define R_PPC_EMB_NADDR32	101
#define R_PPC_EMB_NADDR16	102
#define R_PPC_EMB_NADDR16_LO	103
#define R_PPC_EMB_NADDR16_HI	104
#define R_PPC_EMB_NADDR16_HA	105
#define R_PPC_EMB_SDAI16	106
#define R_PPC_EMB_SDA2I16	107
#define R_PPC_EMB_SDA2REL	108
#define R_PPC_EMB_SDA21		109
#define R_PPC_EMB_MRKREF	110
#define R_PPC_EMB_RELSEC16	111
#define R_PPC_EMB_RELST_LO	112
#define R_PPC_EMB_RELST_HI	113
#define R_PPC_EMB_RELST_HA	114
#define R_PPC_EMB_BIT_FLD	115
#define R_PPC_EMB_RELSDA	116


#define R_PPC_DIAB_SDA21_LO	180
#define R_PPC_DIAB_SDA21_HI	181
#define R_PPC_DIAB_SDA21_HA	182
#define R_PPC_DIAB_RELSDA_LO	183
#define R_PPC_DIAB_RELSDA_HI	184
#define R_PPC_DIAB_RELSDA_HA	185


#define R_PPC_IRELATIVE		248


#define R_PPC_REL16		249
#define R_PPC_REL16_LO		250
#define R_PPC_REL16_HI		251
#define R_PPC_REL16_HA		252



#define R_PPC_TOC16		255


#define DT_PPC_GOT		(DT_LOPROC + 0)
#define DT_PPC_NUM		1


#define R_PPC64_NONE		R_PPC_NONE
#define R_PPC64_ADDR32		R_PPC_ADDR32
#define R_PPC64_ADDR24		R_PPC_ADDR24
#define R_PPC64_ADDR16		R_PPC_ADDR16
#define R_PPC64_ADDR16_LO	R_PPC_ADDR16_LO
#define R_PPC64_ADDR16_HI	R_PPC_ADDR16_HI
#define R_PPC64_ADDR16_HA	R_PPC_ADDR16_HA
#define R_PPC64_ADDR14		R_PPC_ADDR14
#define R_PPC64_ADDR14_BRTAKEN	R_PPC_ADDR14_BRTAKEN
#define R_PPC64_ADDR14_BRNTAKEN	R_PPC_ADDR14_BRNTAKEN
#define R_PPC64_REL24		R_PPC_REL24
#define R_PPC64_REL14		R_PPC_REL14
#define R_PPC64_REL14_BRTAKEN	R_PPC_REL14_BRTAKEN
#define R_PPC64_REL14_BRNTAKEN	R_PPC_REL14_BRNTAKEN
#define R_PPC64_GOT16		R_PPC_GOT16
#define R_PPC64_GOT16_LO	R_PPC_GOT16_LO
#define R_PPC64_GOT16_HI	R_PPC_GOT16_HI
#define R_PPC64_GOT16_HA	R_PPC_GOT16_HA

#define R_PPC64_COPY		R_PPC_COPY
#define R_PPC64_GLOB_DAT	R_PPC_GLOB_DAT
#define R_PPC64_JMP_SLOT	R_PPC_JMP_SLOT
#define R_PPC64_RELATIVE	R_PPC_RELATIVE

#define R_PPC64_UADDR32		R_PPC_UADDR32
#define R_PPC64_UADDR16		R_PPC_UADDR16
#define R_PPC64_REL32		R_PPC_REL32
#define R_PPC64_PLT32		R_PPC_PLT32
#define R_PPC64_PLTREL32	R_PPC_PLTREL32
#define R_PPC64_PLT16_LO	R_PPC_PLT16_LO
#define R_PPC64_PLT16_HI	R_PPC_PLT16_HI
#define R_PPC64_PLT16_HA	R_PPC_PLT16_HA

#define R_PPC64_SECTOFF		R_PPC_SECTOFF
#define R_PPC64_SECTOFF_LO	R_PPC_SECTOFF_LO
#define R_PPC64_SECTOFF_HI	R_PPC_SECTOFF_HI
#define R_PPC64_SECTOFF_HA	R_PPC_SECTOFF_HA
#define R_PPC64_ADDR30		37
#define R_PPC64_ADDR64		38
#define R_PPC64_ADDR16_HIGHER	39
#define R_PPC64_ADDR16_HIGHERA	40
#define R_PPC64_ADDR16_HIGHEST	41
#define R_PPC64_ADDR16_HIGHESTA	42
#define R_PPC64_UADDR64		43
#define R_PPC64_REL64		44
#define R_PPC64_PLT64		45
#define R_PPC64_PLTREL64	46
#define R_PPC64_TOC16		47
#define R_PPC64_TOC16_LO	48
#define R_PPC64_TOC16_HI	49
#define R_PPC64_TOC16_HA	50
#define R_PPC64_TOC		51
#define R_PPC64_PLTGOT16	52
#define R_PPC64_PLTGOT16_LO	53
#define R_PPC64_PLTGOT16_HI	54
#define R_PPC64_PLTGOT16_HA	55

#define R_PPC64_ADDR16_DS	56
#define R_PPC64_ADDR16_LO_DS	57
#define R_PPC64_GOT16_DS	58
#define R_PPC64_GOT16_LO_DS	59
#define R_PPC64_PLT16_LO_DS	60
#define R_PPC64_SECTOFF_DS	61
#define R_PPC64_SECTOFF_LO_DS	62
#define R_PPC64_TOC16_DS	63
#define R_PPC64_TOC16_LO_DS	64
#define R_PPC64_PLTGOT16_DS	65
#define R_PPC64_PLTGOT16_LO_DS	66


#define R_PPC64_TLS		67
#define R_PPC64_DTPMOD64	68
#define R_PPC64_TPREL16		69
#define R_PPC64_TPREL16_LO	70
#define R_PPC64_TPREL16_HI	71
#define R_PPC64_TPREL16_HA	72
#define R_PPC64_TPREL64		73
#define R_PPC64_DTPREL16	74
#define R_PPC64_DTPREL16_LO	75
#define R_PPC64_DTPREL16_HI	76
#define R_PPC64_DTPREL16_HA	77
#define R_PPC64_DTPREL64	78
#define R_PPC64_GOT_TLSGD16	79
#define R_PPC64_GOT_TLSGD16_LO	80
#define R_PPC64_GOT_TLSGD16_HI	81
#define R_PPC64_GOT_TLSGD16_HA	82
#define R_PPC64_GOT_TLSLD16	83
#define R_PPC64_GOT_TLSLD16_LO	84
#define R_PPC64_GOT_TLSLD16_HI	85
#define R_PPC64_GOT_TLSLD16_HA	86
#define R_PPC64_GOT_TPREL16_DS	87
#define R_PPC64_GOT_TPREL16_LO_DS 88
#define R_PPC64_GOT_TPREL16_HI	89
#define R_PPC64_GOT_TPREL16_HA	90
#define R_PPC64_GOT_DTPREL16_DS	91
#define R_PPC64_GOT_DTPREL16_LO_DS 92
#define R_PPC64_GOT_DTPREL16_HI	93
#define R_PPC64_GOT_DTPREL16_HA	94
#define R_PPC64_TPREL16_DS	95
#define R_PPC64_TPREL16_LO_DS	96
#define R_PPC64_TPREL16_HIGHER	97
#define R_PPC64_TPREL16_HIGHERA	98
#define R_PPC64_TPREL16_HIGHEST	99
#define R_PPC64_TPREL16_HIGHESTA 100
#define R_PPC64_DTPREL16_DS	101
#define R_PPC64_DTPREL16_LO_DS	102
#define R_PPC64_DTPREL16_HIGHER	103
#define R_PPC64_DTPREL16_HIGHERA 104
#define R_PPC64_DTPREL16_HIGHEST 105
#define R_PPC64_DTPREL16_HIGHESTA 106


#define R_PPC64_JMP_IREL	247
#define R_PPC64_IRELATIVE	248
#define R_PPC64_REL16		249
#define R_PPC64_REL16_LO	250
#define R_PPC64_REL16_HI	251
#define R_PPC64_REL16_HA	252


#define DT_PPC64_GLINK  (DT_LOPROC + 0)
#define DT_PPC64_OPD	(DT_LOPROC + 1)
#define DT_PPC64_OPDSZ	(DT_LOPROC + 2)
#define DT_PPC64_NUM    3





#define EF_ARM_RELEXEC		0x01
#define EF_ARM_HASENTRY		0x02
#define EF_ARM_INTERWORK	0x04
#define EF_ARM_APCS_26		0x08
#define EF_ARM_APCS_FLOAT	0x10
#define EF_ARM_PIC		0x20
#define EF_ARM_ALIGN8		0x40
#define EF_ARM_NEW_ABI		0x80
#define EF_ARM_OLD_ABI		0x100
#define EF_ARM_SOFT_FLOAT	0x200
#define EF_ARM_VFP_FLOAT	0x400
#define EF_ARM_MAVERICK_FLOAT	0x800

#define EF_ARM_ABI_FLOAT_SOFT	0x200
#define EF_ARM_ABI_FLOAT_HARD	0x400


#define EF_ARM_SYMSARESORTED	0x04
#define EF_ARM_DYNSYMSUSESEGIDX	0x08
#define EF_ARM_MAPSYMSFIRST	0x10
#define EF_ARM_EABIMASK		0XFF000000


#define EF_ARM_BE8	    0x00800000
#define EF_ARM_LE8	    0x00400000

#define EF_ARM_EABI_VERSION(flags)	((flags) & EF_ARM_EABIMASK)
#define EF_ARM_EABI_UNKNOWN	0x00000000
#define EF_ARM_EABI_VER1	0x01000000
#define EF_ARM_EABI_VER2	0x02000000
#define EF_ARM_EABI_VER3	0x03000000
#define EF_ARM_EABI_VER4	0x04000000
#define EF_ARM_EABI_VER5	0x05000000


#define STT_ARM_TFUNC		STT_LOPROC
#define STT_ARM_16BIT		STT_HIPROC


#define SHF_ARM_ENTRYSECT	0x10000000
#define SHF_ARM_COMDEF		0x80000000



#define PF_ARM_SB		0x10000000

#define PF_ARM_PI		0x20000000
#define PF_ARM_ABS		0x40000000


#define PT_ARM_EXIDX		(PT_LOPROC + 1)


#define SHT_ARM_EXIDX		(SHT_LOPROC + 1)
#define SHT_ARM_PREEMPTMAP	(SHT_LOPROC + 2)
#define SHT_ARM_ATTRIBUTES	(SHT_LOPROC + 3)


#define R_AARCH64_NONE            0
#define R_AARCH64_ABS64         257
#define R_AARCH64_ABS32         258
#define R_AARCH64_ABS16		259
#define R_AARCH64_PREL64	260
#define R_AARCH64_PREL32	261
#define R_AARCH64_PREL16	262
#define R_AARCH64_MOVW_UABS_G0	263
#define R_AARCH64_MOVW_UABS_G0_NC 264
#define R_AARCH64_MOVW_UABS_G1	265
#define R_AARCH64_MOVW_UABS_G1_NC 266
#define R_AARCH64_MOVW_UABS_G2	267
#define R_AARCH64_MOVW_UABS_G2_NC 268
#define R_AARCH64_MOVW_UABS_G3	269
#define R_AARCH64_MOVW_SABS_G0	270
#define R_AARCH64_MOVW_SABS_G1	271
#define R_AARCH64_MOVW_SABS_G2	272
#define R_AARCH64_LD_PREL_LO19	273
#define R_AARCH64_ADR_PREL_LO21	274
#define R_AARCH64_ADR_PREL_PG_HI21 275
#define R_AARCH64_ADR_PREL_PG_HI21_NC 276
#define R_AARCH64_ADD_ABS_LO12_NC 277
#define R_AARCH64_LDST8_ABS_LO12_NC 278
#define R_AARCH64_TSTBR14	279
#define R_AARCH64_CONDBR19	280
#define R_AARCH64_JUMP26	282
#define R_AARCH64_CALL26	283
#define R_AARCH64_LDST16_ABS_LO12_NC 284
#define R_AARCH64_LDST32_ABS_LO12_NC 285
#define R_AARCH64_LDST64_ABS_LO12_NC 286
#define R_AARCH64_MOVW_PREL_G0	287
#define R_AARCH64_MOVW_PREL_G0_NC 288
#define R_AARCH64_MOVW_PREL_G1	289
#define R_AARCH64_MOVW_PREL_G1_NC 290
#define R_AARCH64_MOVW_PREL_G2	291
#define R_AARCH64_MOVW_PREL_G2_NC 292
#define R_AARCH64_MOVW_PREL_G3	293
#define R_AARCH64_LDST128_ABS_LO12_NC 299
#define R_AARCH64_MOVW_GOTOFF_G0 300
#define R_AARCH64_MOVW_GOTOFF_G0_NC 301
#define R_AARCH64_MOVW_GOTOFF_G1 302
#define R_AARCH64_MOVW_GOTOFF_G1_NC 303
#define R_AARCH64_MOVW_GOTOFF_G2 304
#define R_AARCH64_MOVW_GOTOFF_G2_NC 305
#define R_AARCH64_MOVW_GOTOFF_G3 306
#define R_AARCH64_GOTREL64	307
#define R_AARCH64_GOTREL32	308
#define R_AARCH64_GOT_LD_PREL19	309
#define R_AARCH64_LD64_GOTOFF_LO15 310
#define R_AARCH64_ADR_GOT_PAGE	311
#define R_AARCH64_LD64_GOT_LO12_NC 312
#define R_AARCH64_LD64_GOTPAGE_LO15 313
#define R_AARCH64_TLSGD_ADR_PREL21 512
#define R_AARCH64_TLSGD_ADR_PAGE21 513
#define R_AARCH64_TLSGD_ADD_LO12_NC 514
#define R_AARCH64_TLSGD_MOVW_G1	515
#define R_AARCH64_TLSGD_MOVW_G0_NC 516
#define R_AARCH64_TLSLD_ADR_PREL21 517
#define R_AARCH64_TLSLD_ADR_PAGE21 518
#define R_AARCH64_TLSLD_ADD_LO12_NC 519
#define R_AARCH64_TLSLD_MOVW_G1	520
#define R_AARCH64_TLSLD_MOVW_G0_NC 521
#define R_AARCH64_TLSLD_LD_PREL19 522
#define R_AARCH64_TLSLD_MOVW_DTPREL_G2 523
#define R_AARCH64_TLSLD_MOVW_DTPREL_G1 524
#define R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC 525
#define R_AARCH64_TLSLD_MOVW_DTPREL_G0 526
#define R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC 527
#define R_AARCH64_TLSLD_ADD_DTPREL_HI12 528
#define R_AARCH64_TLSLD_ADD_DTPREL_LO12 529
#define R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC 530
#define R_AARCH64_TLSLD_LDST8_DTPREL_LO12 531
#define R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC 532
#define R_AARCH64_TLSLD_LDST16_DTPREL_LO12 533
#define R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC 534
#define R_AARCH64_TLSLD_LDST32_DTPREL_LO12 535
#define R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC 536
#define R_AARCH64_TLSLD_LDST64_DTPREL_LO12 537
#define R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC 538
#define R_AARCH64_TLSIE_MOVW_GOTTPREL_G1 539
#define R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC 540
#define R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21 541
#define R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC 542
#define R_AARCH64_TLSIE_LD_GOTTPREL_PREL19 543
#define R_AARCH64_TLSLE_MOVW_TPREL_G2 544
#define R_AARCH64_TLSLE_MOVW_TPREL_G1 545
#define R_AARCH64_TLSLE_MOVW_TPREL_G1_NC 546
#define R_AARCH64_TLSLE_MOVW_TPREL_G0 547
#define R_AARCH64_TLSLE_MOVW_TPREL_G0_NC 548
#define R_AARCH64_TLSLE_ADD_TPREL_HI12 549
#define R_AARCH64_TLSLE_ADD_TPREL_LO12 550
#define R_AARCH64_TLSLE_ADD_TPREL_LO12_NC 551
#define R_AARCH64_TLSLE_LDST8_TPREL_LO12 552
#define R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC 553
#define R_AARCH64_TLSLE_LDST16_TPREL_LO12 554
#define R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC 555
#define R_AARCH64_TLSLE_LDST32_TPREL_LO12 556
#define R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC 557
#define R_AARCH64_TLSLE_LDST64_TPREL_LO12 558
#define R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC 559
#define R_AARCH64_TLSDESC_LD_PREL19 560
#define R_AARCH64_TLSDESC_ADR_PREL21 561
#define R_AARCH64_TLSDESC_ADR_PAGE21 562
#define R_AARCH64_TLSDESC_LD64_LO12 563
#define R_AARCH64_TLSDESC_ADD_LO12 564
#define R_AARCH64_TLSDESC_OFF_G1 565
#define R_AARCH64_TLSDESC_OFF_G0_NC 566
#define R_AARCH64_TLSDESC_LDR	567
#define R_AARCH64_TLSDESC_ADD	568
#define R_AARCH64_TLSDESC_CALL	569
#define R_AARCH64_TLSLE_LDST128_TPREL_LO12 570
#define R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC 571
#define R_AARCH64_TLSLD_LDST128_DTPREL_LO12 572
#define R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC 573
#define R_AARCH64_COPY         1024
#define R_AARCH64_GLOB_DAT     1025
#define R_AARCH64_JUMP_SLOT    1026
#define R_AARCH64_RELATIVE     1027
#define R_AARCH64_TLS_DTPMOD64 1028
#define R_AARCH64_TLS_DTPREL64 1029
#define R_AARCH64_TLS_TPREL64  1030
#define R_AARCH64_TLSDESC      1031


#define R_ARM_NONE		0
#define R_ARM_PC24		1
#define R_ARM_ABS32		2
#define R_ARM_REL32		3
#define R_ARM_PC13		4
#define R_ARM_ABS16		5
#define R_ARM_ABS12		6
#define R_ARM_THM_ABS5		7
#define R_ARM_ABS8		8
#define R_ARM_SBREL32		9
#define R_ARM_THM_PC22		10
#define R_ARM_THM_PC8		11
#define R_ARM_AMP_VCALL9	12
#define R_ARM_TLS_DESC		13
#define R_ARM_THM_SWI8		14
#define R_ARM_XPC25		15
#define R_ARM_THM_XPC22		16
#define R_ARM_TLS_DTPMOD32	17
#define R_ARM_TLS_DTPOFF32	18
#define R_ARM_TLS_TPOFF32	19
#define R_ARM_COPY		20
#define R_ARM_GLOB_DAT		21
#define R_ARM_JUMP_SLOT		22
#define R_ARM_RELATIVE		23
#define R_ARM_GOTOFF		24
#define R_ARM_GOTPC		25
#define R_ARM_GOT32		26
#define R_ARM_PLT32		27
#define R_ARM_CALL		28
#define R_ARM_JUMP24		29
#define R_ARM_THM_JUMP24	30
#define R_ARM_BASE_ABS		31
#define R_ARM_ALU_PCREL_7_0	32
#define R_ARM_ALU_PCREL_15_8	33
#define R_ARM_ALU_PCREL_23_15	34
#define R_ARM_LDR_SBREL_11_0	35
#define R_ARM_ALU_SBREL_19_12	36
#define R_ARM_ALU_SBREL_27_20	37
#define R_ARM_TARGET1		38
#define R_ARM_SBREL31		39
#define R_ARM_V4BX		40
#define R_ARM_TARGET2		41
#define R_ARM_PREL31		42
#define R_ARM_MOVW_ABS_NC	43
#define R_ARM_MOVT_ABS		44
#define R_ARM_MOVW_PREL_NC	45
#define R_ARM_MOVT_PREL		46
#define R_ARM_THM_MOVW_ABS_NC	47
#define R_ARM_THM_MOVT_ABS	48
#define R_ARM_THM_MOVW_PREL_NC	49
#define R_ARM_THM_MOVT_PREL	50
#define R_ARM_THM_JUMP19	51
#define R_ARM_THM_JUMP6		52
#define R_ARM_THM_ALU_PREL_11_0	53
#define R_ARM_THM_PC12		54
#define R_ARM_ABS32_NOI		55
#define R_ARM_REL32_NOI		56
#define R_ARM_ALU_PC_G0_NC	57
#define R_ARM_ALU_PC_G0		58
#define R_ARM_ALU_PC_G1_NC	59
#define R_ARM_ALU_PC_G1		60
#define R_ARM_ALU_PC_G2		61
#define R_ARM_LDR_PC_G1		62
#define R_ARM_LDR_PC_G2		63
#define R_ARM_LDRS_PC_G0	64
#define R_ARM_LDRS_PC_G1	65
#define R_ARM_LDRS_PC_G2	66
#define R_ARM_LDC_PC_G0		67
#define R_ARM_LDC_PC_G1		68
#define R_ARM_LDC_PC_G2		69
#define R_ARM_ALU_SB_G0_NC	70
#define R_ARM_ALU_SB_G0		71
#define R_ARM_ALU_SB_G1_NC	72
#define R_ARM_ALU_SB_G1		73
#define R_ARM_ALU_SB_G2		74
#define R_ARM_LDR_SB_G0		75
#define R_ARM_LDR_SB_G1		76
#define R_ARM_LDR_SB_G2		77
#define R_ARM_LDRS_SB_G0	78
#define R_ARM_LDRS_SB_G1	79
#define R_ARM_LDRS_SB_G2	80
#define R_ARM_LDC_SB_G0		81
#define R_ARM_LDC_SB_G1		82
#define R_ARM_LDC_SB_G2		83
#define R_ARM_MOVW_BREL_NC	84
#define R_ARM_MOVT_BREL		85
#define R_ARM_MOVW_BREL		86
#define R_ARM_THM_MOVW_BREL_NC	87
#define R_ARM_THM_MOVT_BREL	88
#define R_ARM_THM_MOVW_BREL	89
#define R_ARM_TLS_GOTDESC	90
#define R_ARM_TLS_CALL		91
#define R_ARM_TLS_DESCSEQ	92
#define R_ARM_THM_TLS_CALL	93
#define R_ARM_PLT32_ABS		94
#define R_ARM_GOT_ABS		95
#define R_ARM_GOT_PREL		96
#define R_ARM_GOT_BREL12	97
#define R_ARM_GOTOFF12		98
#define R_ARM_GOTRELAX		99
#define R_ARM_GNU_VTENTRY	100
#define R_ARM_GNU_VTINHERIT	101
#define R_ARM_THM_PC11		102
#define R_ARM_THM_PC9		103
#define R_ARM_TLS_GD32		104

#define R_ARM_TLS_LDM32		105

#define R_ARM_TLS_LDO32		106

#define R_ARM_TLS_IE32		107

#define R_ARM_TLS_LE32		108
#define R_ARM_TLS_LDO12		109
#define R_ARM_TLS_LE12		110
#define R_ARM_TLS_IE12GP	111
#define R_ARM_ME_TOO		128
#define R_ARM_THM_TLS_DESCSEQ	129
#define R_ARM_THM_TLS_DESCSEQ16	129
#define R_ARM_THM_TLS_DESCSEQ32	130
#define R_ARM_THM_GOT_BREL12	131
#define R_ARM_IRELATIVE		160
#define R_ARM_RXPC25		249
#define R_ARM_RSBREL32		250
#define R_ARM_THM_RPC22		251
#define R_ARM_RREL32		252
#define R_ARM_RABS22		253
#define R_ARM_RPC24		254
#define R_ARM_RBASE		255

#define R_ARM_NUM		256




#define EF_IA_64_MASKOS		0x0000000f
#define EF_IA_64_ABI64		0x00000010
#define EF_IA_64_ARCH		0xff000000


#define PT_IA_64_ARCHEXT	(PT_LOPROC + 0)
#define PT_IA_64_UNWIND		(PT_LOPROC + 1)
#define PT_IA_64_HP_OPT_ANOT	(PT_LOOS + 0x12)
#define PT_IA_64_HP_HSL_ANOT	(PT_LOOS + 0x13)
#define PT_IA_64_HP_STACK	(PT_LOOS + 0x14)


#define PF_IA_64_NORECOV	0x80000000


#define SHT_IA_64_EXT		(SHT_LOPROC + 0)
#define SHT_IA_64_UNWIND	(SHT_LOPROC + 1)


#define SHF_IA_64_SHORT		0x10000000
#define SHF_IA_64_NORECOV	0x20000000


#define DT_IA_64_PLT_RESERVE	(DT_LOPROC + 0)
#define DT_IA_64_NUM		1


#define R_IA64_NONE		0x00
#define R_IA64_IMM14		0x21
#define R_IA64_IMM22		0x22
#define R_IA64_IMM64		0x23
#define R_IA64_DIR32MSB		0x24
#define R_IA64_DIR32LSB		0x25
#define R_IA64_DIR64MSB		0x26
#define R_IA64_DIR64LSB		0x27
#define R_IA64_GPREL22		0x2a
#define R_IA64_GPREL64I		0x2b
#define R_IA64_GPREL32MSB	0x2c
#define R_IA64_GPREL32LSB	0x2d
#define R_IA64_GPREL64MSB	0x2e
#define R_IA64_GPREL64LSB	0x2f
#define R_IA64_LTOFF22		0x32
#define R_IA64_LTOFF64I		0x33
#define R_IA64_PLTOFF22		0x3a
#define R_IA64_PLTOFF64I	0x3b
#define R_IA64_PLTOFF64MSB	0x3e
#define R_IA64_PLTOFF64LSB	0x3f
#define R_IA64_FPTR64I		0x43
#define R_IA64_FPTR32MSB	0x44
#define R_IA64_FPTR32LSB	0x45
#define R_IA64_FPTR64MSB	0x46
#define R_IA64_FPTR64LSB	0x47
#define R_IA64_PCREL60B		0x48
#define R_IA64_PCREL21B		0x49
#define R_IA64_PCREL21M		0x4a
#define R_IA64_PCREL21F		0x4b
#define R_IA64_PCREL32MSB	0x4c
#define R_IA64_PCREL32LSB	0x4d
#define R_IA64_PCREL64MSB	0x4e
#define R_IA64_PCREL64LSB	0x4f
#define R_IA64_LTOFF_FPTR22	0x52
#define R_IA64_LTOFF_FPTR64I	0x53
#define R_IA64_LTOFF_FPTR32MSB	0x54
#define R_IA64_LTOFF_FPTR32LSB	0x55
#define R_IA64_LTOFF_FPTR64MSB	0x56
#define R_IA64_LTOFF_FPTR64LSB	0x57
#define R_IA64_SEGREL32MSB	0x5c
#define R_IA64_SEGREL32LSB	0x5d
#define R_IA64_SEGREL64MSB	0x5e
#define R_IA64_SEGREL64LSB	0x5f
#define R_IA64_SECREL32MSB	0x64
#define R_IA64_SECREL32LSB	0x65
#define R_IA64_SECREL64MSB	0x66
#define R_IA64_SECREL64LSB	0x67
#define R_IA64_REL32MSB		0x6c
#define R_IA64_REL32LSB		0x6d
#define R_IA64_REL64MSB		0x6e
#define R_IA64_REL64LSB		0x6f
#define R_IA64_LTV32MSB		0x74
#define R_IA64_LTV32LSB		0x75
#define R_IA64_LTV64MSB		0x76
#define R_IA64_LTV64LSB		0x77
#define R_IA64_PCREL21BI	0x79
#define R_IA64_PCREL22		0x7a
#define R_IA64_PCREL64I		0x7b
#define R_IA64_IPLTMSB		0x80
#define R_IA64_IPLTLSB		0x81
#define R_IA64_COPY		0x84
#define R_IA64_SUB		0x85
#define R_IA64_LTOFF22X		0x86
#define R_IA64_LDXMOV		0x87
#define R_IA64_TPREL14		0x91
#define R_IA64_TPREL22		0x92
#define R_IA64_TPREL64I		0x93
#define R_IA64_TPREL64MSB	0x96
#define R_IA64_TPREL64LSB	0x97
#define R_IA64_LTOFF_TPREL22	0x9a
#define R_IA64_DTPMOD64MSB	0xa6
#define R_IA64_DTPMOD64LSB	0xa7
#define R_IA64_LTOFF_DTPMOD22	0xaa
#define R_IA64_DTPREL14		0xb1
#define R_IA64_DTPREL22		0xb2
#define R_IA64_DTPREL64I	0xb3
#define R_IA64_DTPREL32MSB	0xb4
#define R_IA64_DTPREL32LSB	0xb5
#define R_IA64_DTPREL64MSB	0xb6
#define R_IA64_DTPREL64LSB	0xb7
#define R_IA64_LTOFF_DTPREL22	0xba




#define	R_SH_NONE		0
#define	R_SH_DIR32		1
#define	R_SH_REL32		2
#define	R_SH_DIR8WPN		3
#define	R_SH_IND12W		4
#define	R_SH_DIR8WPL		5
#define	R_SH_DIR8WPZ		6
#define	R_SH_DIR8BP		7
#define	R_SH_DIR8W		8
#define	R_SH_DIR8L		9
#define	R_SH_SWITCH16		25
#define	R_SH_SWITCH32		26
#define	R_SH_USES		27
#define	R_SH_COUNT		28
#define	R_SH_ALIGN		29
#define	R_SH_CODE		30
#define	R_SH_DATA		31
#define	R_SH_LABEL		32
#define	R_SH_SWITCH8		33
#define	R_SH_GNU_VTINHERIT	34
#define	R_SH_GNU_VTENTRY	35
#define	R_SH_TLS_GD_32		144
#define	R_SH_TLS_LD_32		145
#define	R_SH_TLS_LDO_32		146
#define	R_SH_TLS_IE_32		147
#define	R_SH_TLS_LE_32		148
#define	R_SH_TLS_DTPMOD32	149
#define	R_SH_TLS_DTPOFF32	150
#define	R_SH_TLS_TPOFF32	151
#define	R_SH_GOT32		160
#define	R_SH_PLT32		161
#define	R_SH_COPY		162
#define	R_SH_GLOB_DAT		163
#define	R_SH_JMP_SLOT		164
#define	R_SH_RELATIVE		165
#define	R_SH_GOTOFF		166
#define	R_SH_GOTPC		167
#define	R_SH_GOT20		201
#define	R_SH_GOTOFF20		202
#define	R_SH_GOTFUNCDESC	203
#define	R_SH_GOTFUNCDEST20	204
#define	R_SH_GOTOFFFUNCDESC	205
#define	R_SH_GOTOFFFUNCDEST20	206
#define	R_SH_FUNCDESC		207
#define	R_SH_FUNCDESC_VALUE	208

#define	R_SH_NUM		256



#define R_390_NONE		0
#define R_390_8			1
#define R_390_12		2
#define R_390_16		3
#define R_390_32		4
#define R_390_PC32		5
#define R_390_GOT12		6
#define R_390_GOT32		7
#define R_390_PLT32		8
#define R_390_COPY		9
#define R_390_GLOB_DAT		10
#define R_390_JMP_SLOT		11
#define R_390_RELATIVE		12
#define R_390_GOTOFF32		13
#define R_390_GOTPC		14
#define R_390_GOT16		15
#define R_390_PC16		16
#define R_390_PC16DBL		17
#define R_390_PLT16DBL		18
#define R_390_PC32DBL		19
#define R_390_PLT32DBL		20
#define R_390_GOTPCDBL		21
#define R_390_64		22
#define R_390_PC64		23
#define R_390_GOT64		24
#define R_390_PLT64		25
#define R_390_GOTENT		26
#define R_390_GOTOFF16		27
#define R_390_GOTOFF64		28
#define R_390_GOTPLT12		29
#define R_390_GOTPLT16		30
#define R_390_GOTPLT32		31
#define R_390_GOTPLT64		32
#define R_390_GOTPLTENT		33
#define R_390_PLTOFF16		34
#define R_390_PLTOFF32		35
#define R_390_PLTOFF64		36
#define R_390_TLS_LOAD		37
#define R_390_TLS_GDCALL	38

#define R_390_TLS_LDCALL	39

#define R_390_TLS_GD32		40

#define R_390_TLS_GD64		41

#define R_390_TLS_GOTIE12	42

#define R_390_TLS_GOTIE32	43

#define R_390_TLS_GOTIE64	44

#define R_390_TLS_LDM32		45

#define R_390_TLS_LDM64		46

#define R_390_TLS_IE32		47

#define R_390_TLS_IE64		48

#define R_390_TLS_IEENT		49

#define R_390_TLS_LE32		50

#define R_390_TLS_LE64		51

#define R_390_TLS_LDO32		52

#define R_390_TLS_LDO64		53

#define R_390_TLS_DTPMOD	54
#define R_390_TLS_DTPOFF	55
#define R_390_TLS_TPOFF		56

#define R_390_20		57
#define R_390_GOT20		58
#define R_390_GOTPLT20		59
#define R_390_TLS_GOTIE20	60


#define R_390_NUM		61



#define R_CRIS_NONE		0
#define R_CRIS_8		1
#define R_CRIS_16		2
#define R_CRIS_32		3
#define R_CRIS_8_PCREL		4
#define R_CRIS_16_PCREL		5
#define R_CRIS_32_PCREL		6
#define R_CRIS_GNU_VTINHERIT	7
#define R_CRIS_GNU_VTENTRY	8
#define R_CRIS_COPY		9
#define R_CRIS_GLOB_DAT		10
#define R_CRIS_JUMP_SLOT	11
#define R_CRIS_RELATIVE		12
#define R_CRIS_16_GOT		13
#define R_CRIS_32_GOT		14
#define R_CRIS_16_GOTPLT	15
#define R_CRIS_32_GOTPLT	16
#define R_CRIS_32_GOTREL	17
#define R_CRIS_32_PLT_GOTREL	18
#define R_CRIS_32_PLT_PCREL	19

#define R_CRIS_NUM		20



#define R_X86_64_NONE		0
#define R_X86_64_64		1
#define R_X86_64_PC32		2
#define R_X86_64_GOT32		3
#define R_X86_64_PLT32		4
#define R_X86_64_COPY		5
#define R_X86_64_GLOB_DAT	6
#define R_X86_64_JUMP_SLOT	7
#define R_X86_64_RELATIVE	8
#define R_X86_64_GOTPCREL	9

#define R_X86_64_32		10
#define R_X86_64_32S		11
#define R_X86_64_16		12
#define R_X86_64_PC16		13
#define R_X86_64_8		14
#define R_X86_64_PC8		15
#define R_X86_64_DTPMOD64	16
#define R_X86_64_DTPOFF64	17
#define R_X86_64_TPOFF64	18
#define R_X86_64_TLSGD		19

#define R_X86_64_TLSLD		20

#define R_X86_64_DTPOFF32	21
#define R_X86_64_GOTTPOFF	22

#define R_X86_64_TPOFF32	23
#define R_X86_64_PC64		24
#define R_X86_64_GOTOFF64	25
#define R_X86_64_GOTPC32	26
#define R_X86_64_GOT64		27
#define R_X86_64_GOTPCREL64	28
#define R_X86_64_GOTPC64	29
#define R_X86_64_GOTPLT64	30
#define R_X86_64_PLTOFF64	31
#define R_X86_64_SIZE32		32
#define R_X86_64_SIZE64		33

#define R_X86_64_GOTPC32_TLSDESC 34
#define R_X86_64_TLSDESC_CALL   35

#define R_X86_64_TLSDESC        36
#define R_X86_64_IRELATIVE	37
#define R_X86_64_RELATIVE64	38
#define R_X86_64_NUM		39



#define R_MN10300_NONE		0
#define R_MN10300_32		1
#define R_MN10300_16		2
#define R_MN10300_8		3
#define R_MN10300_PCREL32	4
#define R_MN10300_PCREL16	5
#define R_MN10300_PCREL8	6
#define R_MN10300_GNU_VTINHERIT	7
#define R_MN10300_GNU_VTENTRY	8
#define R_MN10300_24		9
#define R_MN10300_GOTPC32	10
#define R_MN10300_GOTPC16	11
#define R_MN10300_GOTOFF32	12
#define R_MN10300_GOTOFF24	13
#define R_MN10300_GOTOFF16	14
#define R_MN10300_PLT32		15
#define R_MN10300_PLT16		16
#define R_MN10300_GOT32		17
#define R_MN10300_GOT24		18
#define R_MN10300_GOT16		19
#define R_MN10300_COPY		20
#define R_MN10300_GLOB_DAT	21
#define R_MN10300_JMP_SLOT	22
#define R_MN10300_RELATIVE	23

#define R_MN10300_NUM		24



#define R_M32R_NONE		0
#define R_M32R_16		1
#define R_M32R_32		2
#define R_M32R_24		3
#define R_M32R_10_PCREL		4
#define R_M32R_18_PCREL		5
#define R_M32R_26_PCREL		6
#define R_M32R_HI16_ULO		7
#define R_M32R_HI16_SLO		8
#define R_M32R_LO16		9
#define R_M32R_SDA16		10
#define R_M32R_GNU_VTINHERIT	11
#define R_M32R_GNU_VTENTRY	12

#define R_M32R_16_RELA		33
#define R_M32R_32_RELA		34
#define R_M32R_24_RELA		35
#define R_M32R_10_PCREL_RELA	36
#define R_M32R_18_PCREL_RELA	37
#define R_M32R_26_PCREL_RELA	38
#define R_M32R_HI16_ULO_RELA	39
#define R_M32R_HI16_SLO_RELA	40
#define R_M32R_LO16_RELA	41
#define R_M32R_SDA16_RELA	42
#define R_M32R_RELA_GNU_VTINHERIT	43
#define R_M32R_RELA_GNU_VTENTRY	44
#define R_M32R_REL32		45

#define R_M32R_GOT24		48
#define R_M32R_26_PLTREL	49
#define R_M32R_COPY		50
#define R_M32R_GLOB_DAT		51
#define R_M32R_JMP_SLOT		52
#define R_M32R_RELATIVE		53
#define R_M32R_GOTOFF		54
#define R_M32R_GOTPC24		55
#define R_M32R_GOT16_HI_ULO	56

#define R_M32R_GOT16_HI_SLO	57

#define R_M32R_GOT16_LO		58
#define R_M32R_GOTPC_HI_ULO	59

#define R_M32R_GOTPC_HI_SLO	60

#define R_M32R_GOTPC_LO		61

#define R_M32R_GOTOFF_HI_ULO	62

#define R_M32R_GOTOFF_HI_SLO	63

#define R_M32R_GOTOFF_LO	64
#define R_M32R_NUM		256

#define R_MICROBLAZE_NONE 0
#define R_MICROBLAZE_32 1
#define R_MICROBLAZE_32_PCREL 2
#define R_MICROBLAZE_64_PCREL 3
#define R_MICROBLAZE_32_PCREL_LO 4
#define R_MICROBLAZE_64 5
#define R_MICROBLAZE_32_LO 6
#define R_MICROBLAZE_SRO32 7
#define R_MICROBLAZE_SRW32 8
#define R_MICROBLAZE_64_NONE 9
#define R_MICROBLAZE_32_SYM_OP_SYM 10
#define R_MICROBLAZE_GNU_VTINHERIT 11
#define R_MICROBLAZE_GNU_VTENTRY 12
#define R_MICROBLAZE_GOTPC_64 13
#define R_MICROBLAZE_GOT_64 14
#define R_MICROBLAZE_PLT_64 15
#define R_MICROBLAZE_REL 16
#define R_MICROBLAZE_JUMP_SLOT 17
#define R_MICROBLAZE_GLOB_DAT 18
#define R_MICROBLAZE_GOTOFF_64 19
#define R_MICROBLAZE_GOTOFF_32 20
#define R_MICROBLAZE_COPY 21
#define R_MICROBLAZE_TLS 22
#define R_MICROBLAZE_TLSGD 23
#define R_MICROBLAZE_TLSLD 24
#define R_MICROBLAZE_TLSDTPMOD32 25
#define R_MICROBLAZE_TLSDTPREL32 26
#define R_MICROBLAZE_TLSDTPREL64 27
#define R_MICROBLAZE_TLSGOTTPREL32 28
#define R_MICROBLAZE_TLSTPREL32	 29

#define R_OR1K_NONE		0
#define R_OR1K_32		1
#define R_OR1K_16		2
#define R_OR1K_8		3
#define R_OR1K_LO_16_IN_INSN	4
#define R_OR1K_HI_16_IN_INSN	5
#define R_OR1K_INSN_REL_26	6
#define R_OR1K_GNU_VTENTRY	7
#define R_OR1K_GNU_VTINHERIT	8
#define R_OR1K_32_PCREL		9
#define R_OR1K_16_PCREL		10
#define R_OR1K_8_PCREL		11
#define R_OR1K_GOTPC_HI16	12
#define R_OR1K_GOTPC_LO16	13
#define R_OR1K_GOT16		14
#define R_OR1K_PLT26		15
#define R_OR1K_GOTOFF_HI16	16
#define R_OR1K_GOTOFF_LO16	17
#define R_OR1K_COPY		18
#define R_OR1K_GLOB_DAT		19
#define R_OR1K_JMP_SLOT		20
#define R_OR1K_RELATIVE		21
#define R_OR1K_TLS_GD_HI16	22
#define R_OR1K_TLS_GD_LO16	23
#define R_OR1K_TLS_LDM_HI16	24
#define R_OR1K_TLS_LDM_LO16	25
#define R_OR1K_TLS_LDO_HI16	26
#define R_OR1K_TLS_LDO_LO16	27
#define R_OR1K_TLS_IE_HI16	28
#define R_OR1K_TLS_IE_LO16	29
#define R_OR1K_TLS_LE_HI16	30
#define R_OR1K_TLS_LE_LO16	31
#define R_OR1K_TLS_TPOFF	32
#define R_OR1K_TLS_DTPOFF	33
#define R_OR1K_TLS_DTPMOD	34

#ifdef __cplusplus
}
#endif


#endif
