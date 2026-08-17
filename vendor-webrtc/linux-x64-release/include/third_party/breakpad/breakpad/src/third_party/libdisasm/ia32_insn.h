#ifndef IA32_INSN_H
#define IA32_INSN_H
/* this file contains the structure of opcode definitions and the
 * constants they use */

#include <sys/types.h>
#include "libdis.h"


#define GET_BYTE( buf, buf_len ) buf_len ? *buf : 0

#define OP_SIZE_16	1
#define OP_SIZE_32	2
#define ADDR_SIZE_16	4
#define ADDR_SIZE_32	8

#define MAX_INSTRUCTION_SIZE 20

/* invalid instructions are handled by returning 0 [error] from the
 * function, setting the size of the insn to 1 byte, and copying
 * the byte at the start of the invalid insn into the x86_insn_t.
 * if the caller is saving the x86_insn_t for invalid instructions,
 * instead of discarding them, this will maintain a consistent
 * address space in the x86_insn_ts */

#define INVALID_INSN ((size_t) -1)	/* return value for invalid insn */
#define MAKE_INVALID( i, buf )                          \
                strcpy( i->mnemonic, "invalid" );       \
                x86_oplist_free( i );                   \
                i->size = 1;                            \
                i->group = insn_none;                   \
                i->type = insn_invalid;                 \
                memcpy( i->bytes, buf, 1 );


size_t ia32_disasm_addr( unsigned char * buf, size_t buf_len, 
		x86_insn_t *insn);


/* --------------------------------------------------------- Table Lookup */
/* IA32 Instruction defintion for ia32_opcodes.c */
typedef struct {
   unsigned int table;          /* escape to this sub-table */
   unsigned int mnem_flag;      /* Flags referring to mnemonic */
   unsigned int notes;          /* Notes for this instruction */
   unsigned int dest_flag, src_flag, aux_flag; /* and for specific operands */
   unsigned int cpu;            /* minimumCPU [AND with clocks?? */
   char mnemonic[16];           /* buffers for building instruction */
   char mnemonic_att[16];       /* at&t style mnemonic name */
   int32_t dest;
   int32_t src;
   int32_t aux;
   unsigned int flags_effected;
   unsigned int implicit_ops;	/* implicit operands */
} ia32_insn_t;



/* --------------------------------------------------------- Prefixes */
/* Prefix Flags */
/* Prefixes, same order as in the manual */
/* had to reverse the values of the first three as they were entered into
 * libdis.h incorrectly. */
#define PREFIX_LOCK       0x0004
#define PREFIX_REPNZ      0x0002
#define PREFIX_REPZ       0x0001
#define PREFIX_OP_SIZE    0x0010
#define PREFIX_ADDR_SIZE  0x0020
#define PREFIX_CS         0x0100
#define PREFIX_SS         0x0200
#define PREFIX_DS         0x0300
#define PREFIX_ES         0x0400
#define PREFIX_FS         0x0500
#define PREFIX_GS         0x0600
#define PREFIX_TAKEN      0x1000	/* branch taken */
#define PREFIX_NOTTAKEN   0x2000	/* branch not taken */
#define PREFIX_REG_MASK   0x0F00
#define BRANCH_HINT_MASK  0x3000 
#define PREFIX_PRINT_MASK 0x000F	/* printable prefixes */
#define PREFIX_MASK       0xFFFF

/* ---------------------------------------------------------- CPU Type */

#define cpu_8086         0x0001
#define cpu_80286        0x0002
#define cpu_80386        0x0003
#define cpu_80387        0x0004 /* originally these were a co-proc */
#define cpu_80486        0x0005
#define cpu_PENTIUM      0x0006
#define cpu_PENTPRO      0x0007
#define cpu_PENTIUM2     0x0008
#define cpu_PENTIUM3     0x0009
#define cpu_PENTIUM4     0x000A
#define cpu_K6		 0x0010
#define cpu_K7		 0x0020
#define cpu_ATHLON	 0x0030
#define CPU_MODEL_MASK	 0xFFFF
#define CPU_MODEL(cpu)	 (cpu & CPU_MODEL_MASK)
/* intel instruction subsets */
#define isa_GP		 0x10000	/* General Purpose Instructions */
#define isa_FPU		 0x20000	/* FPU instructions */
#define isa_FPUMGT	 0x30000	/* FPU/SIMD Management */
#define isa_MMX		 0x40000	/* MMX */
#define isa_SSE1	 0x50000	/* SSE */
#define isa_SSE2	 0x60000	/* SSE 2 */
#define isa_SSE3	 0x70000	/* SSE 3 */
#define isa_3DNOW	 0x80000	/* AMD 3d Now */
#define isa_SYS		 0x90000	/* System Instructions */
#define ISA_SUBSET_MASK	 0xFFFF0000
#define ISA_SUBSET(isa)	(isa & ISA_SUBSET_MASK)


/* ------------------------------------------------------ Operand Decoding */
#define ARG_NONE         0

/* Using a mask allows us to store info such as OP_SIGNED in the
 * operand flags field */
#define   OPFLAGS_MASK 	0x0000FFFF

/* Operand Addressing Methods, per intel manual */
#define   ADDRMETH_MASK	0x00FF0000

/* note: for instructions with implied operands, use no ADDRMETH */
#define   ADDRMETH_A  	0x00010000   
#define   ADDRMETH_C   	0x00020000
#define   ADDRMETH_D   	0x00030000
#define   ADDRMETH_E   	0x00040000
#define   ADDRMETH_F   	0x00050000
#define   ADDRMETH_G   	0x00060000
#define   ADDRMETH_I   	0x00070000
#define   ADDRMETH_J   	0x00080000
#define   ADDRMETH_M   	0x00090000
#define   ADDRMETH_O   	0x000A0000
#define   ADDRMETH_P   	0x000B0000
#define   ADDRMETH_Q   	0x000C0000
#define   ADDRMETH_R   	0x000D0000
#define   ADDRMETH_S   	0x000E0000
#define   ADDRMETH_T   	0x000F0000
#define   ADDRMETH_V   	0x00100000
#define   ADDRMETH_W   	0x00110000
#define   ADDRMETH_X   	0x00120000
#define   ADDRMETH_Y   	0x00130000
#define	  ADDRMETH_RR  	0x00140000	/* gen reg hard-coded in opcode */
#define	  ADDRMETH_RS  	0x00150000	/* seg reg hard-coded in opcode */
#define	  ADDRMETH_RT  	0x00160000	/* test reg hard-coded in opcode */
#define	  ADDRMETH_RF  	0x00170000	/* fpu reg hard-coded in opcode */
#define	  ADDRMETH_II  	0x00180000	/* immediate hard-coded in opcode */
#define   ADDRMETH_PP   0x00190000	/* mm reg ONLY in modr/m field */
#define   ADDRMETH_VV   0x001A0000	/* xmm reg ONLY in mod/rm field */

/* Operand Types, per intel manual */
#define OPTYPE_MASK	0xFF000000

#define OPTYPE_a	0x01000000 /* BOUND: h:h or w:w */
#define OPTYPE_b   	0x02000000 /* byte */
#define OPTYPE_c   	0x03000000 /* byte or word */
#define OPTYPE_d   	0x04000000 /* word */
#define OPTYPE_dq   	0x05000000 /* qword */
#define OPTYPE_p   	0x06000000 /* 16:16 or 16:32 pointer */
#define OPTYPE_pi   	0x07000000 /* dword MMX reg */
#define OPTYPE_ps   	0x08000000 /* 128-bit single fp */
#define OPTYPE_q   	0x09000000 /* dword */
#define OPTYPE_s   	0x0A000000 /* 6-byte descriptor */
#define OPTYPE_ss   	0x0B000000 /* scalar of 128-bit single fp */
#define OPTYPE_si   	0x0C000000 /* word general register */
#define OPTYPE_v   	0x0D000000 /* hword or word */
#define OPTYPE_w   	0x0E000000 /* hword */
#define OPTYPE_m   	0x0F000000	/* to handle LEA */
#define OPTYPE_none 0xFF000000 /* no valid operand size, INVLPG */

/* custom ones for FPU instructions */
#define OPTYPE_fs	0x10000000	/* pointer to single-real*/
#define OPTYPE_fd	0x20000000	/* pointer to double real */
#define OPTYPE_fe	0x30000000	/* pointer to extended real */
#define OPTYPE_fb	0x40000000	/* pointer to packed BCD */
#define OPTYPE_fv	0x50000000	/* pointer to FPU env: 14|28-bytes */
#define OPTYPE_ft	0x60000000	/* pointer to FPU state: 94|108-bytes */
#define OPTYPE_fx       0x70000000      /* pointer to FPU regs: 512 bites */
#define OPTYPE_fp       0x80000000      /* general fpu register: dbl ext */

/* SSE2 operand types */
#define OPTYPE_sd	0x90000000	/* scalar of 128-bit double fp */
#define OPTYPE_pd	0xA0000000	/* 128-bit double fp */



/* ---------------------------------------------- Opcode Table Descriptions */
/* the table type describes how to handle byte/size increments before 
 * and after lookup. Some tables re-use the current byte, others
 * consume a byte only if the ModR/M encodes no operands, etc */
enum ia32_tbl_type_id {
	tbl_opcode = 0,	/* standard opcode table: no surprises */
	tbl_prefix,	/* Prefix Override, e.g. 66/F2/F3 */
	tbl_suffix,	/* 3D Now style */
	tbl_extension,	/* ModR/M extension: 00-FF -> 00-07 */
	tbl_ext_ext,	/* extension of modr/m using R/M field */
	tbl_fpu,	/* fpu table: 00-BF -> 00-0F */
	tbl_fpu_ext	/* fpu extension : C0-FF -> 00-1F */
 };

/* How it works:
 * Bytes are 'consumed' if the next table lookup requires that the byte
 * pointer be advanced in the instruction stream. 'Does not consume' means
 * that, when the lookup function recurses, the same byte it re-used in the
 * new table. It also means that size is not decremented, for example when
 * a ModR/M byte is used. Note that tbl_extension (ModR/M) instructions that
 * do not increase the size of an insn with their operands have a forced
 3 size increase in the lookup algo. Weird, yes, confusing, yes, welcome
 * to the Intel ISA. Another note: tbl_prefix is used as an override, so an
 * empty insn in a prefix table causes the instruction in the original table
 * to be used, rather than an invalid insn being generated.
 * 	tbl_opcode uses current byte and consumes it
 * 	tbl_prefix uses current byte but does not consume it
 * 	tbl_suffix uses and consumes last byte in insn
 * 	tbl_extension uses current byte but does not consume it
 * 	tbl_ext_ext uses current byte but does not consume it
 * 	tbl_fpu uses current byte and consumes it
 * 	tbl_fpu_ext uses current byte but does not consume it 
 */

/* Convenience struct for opcode tables : these will be stored in a 
 * 'table of tables' so we can use a table index instead of a pointer */
typedef struct {		/* Assembly instruction tables */
   ia32_insn_t *table;		/* Pointer to table of instruction encodings */
   enum ia32_tbl_type_id type;
   unsigned char shift;		/* amount to shift modrm byte */
   unsigned char mask;		/* bit mask for look up */
   unsigned char minlim,maxlim;	/* limits on min/max entries. */
} ia32_table_desc_t;


/* ---------------------------------------------- 'Cooked' Operand Type Info */
/*                   Permissions: */
#define OP_R         0x001      /* operand is READ */
#define OP_W         0x002      /* operand is WRITTEN */
#define OP_RW        0x003	/* (OP_R|OP_W): convenience macro */
#define OP_X         0x004      /* operand is EXECUTED */

#define OP_PERM_MASK 0x0000007  /* perms are NOT mutually exclusive */
#define OP_PERM( type )       (type & OP_PERM_MASK)

/* Flags */
#define OP_SIGNED    0x010   	/* operand is signed */

#define OP_FLAG_MASK  0x0F0  /* mods are NOT mutually exclusive */
#define OP_FLAGS( type )        (type & OP_FLAG_MASK)

#define OP_REG_MASK    0x0000FFFF /* lower WORD is register ID */
#define OP_REGTBL_MASK 0xFFFF0000 /* higher word is register type [gen/dbg] */
#define OP_REGID( type )      (type & OP_REG_MASK)
#define OP_REGTYPE( type )    (type & OP_REGTBL_MASK)

/* ------------------------------------------'Cooked' Instruction Type Info */
/* high-bit opcode types/insn meta-types */
#define INS_FLAG_PREFIX		0x10000000	/* insn is a prefix */
#define INS_FLAG_SUFFIX		0x20000000	/* followed by a suffix byte */
#define INS_FLAG_MASK    	0xFF000000

/* insn notes */
#define INS_NOTE_RING0		0x00000001	/* insn is privileged */
#define INS_NOTE_SMM		0x00000002	/* Sys Mgt Mode only */
#define INS_NOTE_SERIAL		0x00000004	/* serializes */
#define INS_NOTE_NONSWAP    0x00000008  /* insn is not swapped in att format */ // could be separate field?
#define INS_NOTE_NOSUFFIX   0x00000010  /* insn has no size suffix in att format */ // could be separate field?
//#define INS_NOTE_NMI		

#define INS_INVALID 	0

/* instruction groups */
#define INS_EXEC	0x1000
#define INS_ARITH	0x2000
#define INS_LOGIC	0x3000
#define INS_STACK	0x4000
#define INS_COND	0x5000
#define INS_LOAD	0x6000
#define INS_ARRAY	0x7000
#define INS_BIT		0x8000
#define INS_FLAG	0x9000
#define INS_FPU		0xA000
#define INS_TRAPS	0xD000
#define INS_SYSTEM	0xE000
#define INS_OTHER	0xF000

#define INS_GROUP_MASK	0xF000
#define INS_GROUP( type )     ( type & INS_GROUP_MASK )

/* INS_EXEC group */
#define INS_BRANCH	(INS_EXEC | 0x01)	/* Unconditional branch */
#define INS_BRANCHCC	(INS_EXEC | 0x02)	/* Conditional branch */
#define INS_CALL	(INS_EXEC | 0x03)	/* Jump to subroutine */
#define INS_CALLCC	(INS_EXEC | 0x04)	/* Jump to subroutine */
#define INS_RET		(INS_EXEC | 0x05)	/* Return from subroutine */

/* INS_ARITH group */
#define INS_ADD 	(INS_ARITH | 0x01)
#define INS_SUB		(INS_ARITH | 0x02)
#define INS_MUL		(INS_ARITH | 0x03)
#define INS_DIV		(INS_ARITH | 0x04)
#define INS_INC		(INS_ARITH | 0x05)	/* increment */
#define INS_DEC		(INS_ARITH | 0x06)	/* decrement */
#define INS_SHL		(INS_ARITH | 0x07)	/* shift right */
#define INS_SHR		(INS_ARITH | 0x08)	/* shift left */
#define INS_ROL		(INS_ARITH | 0x09)	/* rotate left */
#define INS_ROR		(INS_ARITH | 0x0A)	/* rotate right */
#define INS_MIN		(INS_ARITH | 0x0B)	/* min func */
#define INS_MAX		(INS_ARITH | 0x0C)	/* max func */
#define INS_AVG		(INS_ARITH | 0x0D)	/* avg func */
#define INS_FLR		(INS_ARITH | 0x0E)	/* floor func */
#define INS_CEIL	(INS_ARITH | 0x0F)	/* ceiling func */

/* INS_LOGIC group */
#define INS_AND		(INS_LOGIC | 0x01)
#define INS_OR		(INS_LOGIC | 0x02)
#define INS_XOR		(INS_LOGIC | 0x03)
#define INS_NOT		(INS_LOGIC | 0x04)
#define INS_NEG		(INS_LOGIC | 0x05)
#define INS_NAND	(INS_LOGIC | 0x06)

/* INS_STACK group */
#define INS_PUSH	(INS_STACK | 0x01)
#define INS_POP		(INS_STACK | 0x02)
#define INS_PUSHREGS	(INS_STACK | 0x03)	/* push register context */
#define INS_POPREGS	(INS_STACK | 0x04)	/* pop register context */
#define INS_PUSHFLAGS	(INS_STACK | 0x05)	/* push all flags */
#define INS_POPFLAGS	(INS_STACK | 0x06)	/* pop all flags */
#define INS_ENTER	(INS_STACK | 0x07)	/* enter stack frame */
#define INS_LEAVE	(INS_STACK | 0x08)	/* leave stack frame */

/* INS_COND group */
#define INS_TEST	(INS_COND | 0x01)
#define INS_CMP		(INS_COND | 0x02)

/* INS_LOAD group */
#define INS_MOV		(INS_LOAD | 0x01)
#define INS_MOVCC	(INS_LOAD | 0x02)
#define INS_XCHG	(INS_LOAD | 0x03)
#define INS_XCHGCC	(INS_LOAD | 0x04)
#define INS_CONV	(INS_LOAD | 0x05)	/* move and convert type */

/* INS_ARRAY group */
#define INS_STRCMP	(INS_ARRAY | 0x01)
#define INS_STRLOAD	(INS_ARRAY | 0x02)
#define INS_STRMOV	(INS_ARRAY | 0x03)
#define INS_STRSTOR	(INS_ARRAY | 0x04)
#define INS_XLAT	(INS_ARRAY | 0x05)

/* INS_BIT group */
#define INS_BITTEST	(INS_BIT | 0x01)
#define INS_BITSET	(INS_BIT | 0x02)
#define INS_BITCLR	(INS_BIT | 0x03)

/* INS_FLAG group */
#define INS_CLEARCF	(INS_FLAG | 0x01)	/* clear Carry flag */
#define INS_CLEARZF	(INS_FLAG | 0x02)	/* clear Zero flag */
#define INS_CLEAROF	(INS_FLAG | 0x03)	/* clear Overflow flag */
#define INS_CLEARDF	(INS_FLAG | 0x04)	/* clear Direction flag */
#define INS_CLEARSF	(INS_FLAG | 0x05)	/* clear Sign flag */
#define INS_CLEARPF	(INS_FLAG | 0x06)	/* clear Parity flag */
#define INS_SETCF	(INS_FLAG | 0x07)
#define INS_SETZF	(INS_FLAG | 0x08)
#define INS_SETOF	(INS_FLAG | 0x09)
#define INS_SETDF	(INS_FLAG | 0x0A)
#define INS_SETSF	(INS_FLAG | 0x0B)
#define INS_SETPF	(INS_FLAG | 0x0C)
#define INS_TOGCF	(INS_FLAG | 0x10)	/* toggle */
#define INS_TOGZF	(INS_FLAG | 0x20)
#define INS_TOGOF	(INS_FLAG | 0x30)
#define INS_TOGDF	(INS_FLAG | 0x40)
#define INS_TOGSF	(INS_FLAG | 0x50)
#define INS_TOGPF	(INS_FLAG | 0x60)

/* INS_FPU */
#define INS_FMOV       (INS_FPU | 0x1)
#define INS_FMOVCC     (INS_FPU | 0x2)
#define INS_FNEG       (INS_FPU | 0x3)
#define INS_FABS       (INS_FPU | 0x4)
#define INS_FADD       (INS_FPU | 0x5)
#define INS_FSUB       (INS_FPU | 0x6)
#define INS_FMUL       (INS_FPU | 0x7)
#define INS_FDIV       (INS_FPU | 0x8)
#define INS_FSQRT      (INS_FPU | 0x9)
#define INS_FCMP       (INS_FPU | 0xA)
#define INS_FCOS       (INS_FPU | 0xC)               /* cosine */
#define INS_FLDPI      (INS_FPU | 0xD)               /* load pi */
#define INS_FLDZ       (INS_FPU | 0xE)               /* load 0 */
#define INS_FTAN       (INS_FPU | 0xF)               /* tanget */
#define INS_FSINE      (INS_FPU | 0x10)              /* sine */
#define INS_FSYS       (INS_FPU | 0x20)              /* misc */

/* INS_TRAP */
#define INS_TRAP	(INS_TRAPS | 0x01)	/* generate trap */
#define INS_TRAPCC	(INS_TRAPS | 0x02)	/* conditional trap gen */
#define INS_TRET	(INS_TRAPS | 0x03)	/* return from trap */
#define INS_BOUNDS	(INS_TRAPS | 0x04)	/* gen bounds trap */
#define INS_DEBUG	(INS_TRAPS | 0x05)	/* gen breakpoint trap */
#define INS_TRACE	(INS_TRAPS | 0x06)	/* gen single step trap */
#define INS_INVALIDOP	(INS_TRAPS | 0x07)	/* gen invalid insn */
#define INS_OFLOW	(INS_TRAPS | 0x08)	/* gen overflow trap */
#define INS_ICEBP	(INS_TRAPS | 0x09)	/* ICE breakpoint */

/* INS_SYSTEM */
#define INS_HALT	(INS_SYSTEM | 0x01)	/* halt machine */
#define INS_IN		(INS_SYSTEM | 0x02)	/* input form port */
#define INS_OUT		(INS_SYSTEM | 0x03)	/* output to port */
#define INS_CPUID	(INS_SYSTEM | 0x04)	/* identify cpu */

/* INS_OTHER */
#define INS_NOP		(INS_OTHER | 0x01)
#define INS_BCDCONV	(INS_OTHER | 0x02)	/* convert to/from BCD */
#define INS_SZCONV	(INS_OTHER | 0x03)	/* convert size of operand */
#define INS_SALC	(INS_OTHER | 0x04)	/* set %al on carry */
#define INS_UNKNOWN	(INS_OTHER | 0x05)
 

#define INS_TYPE_MASK	0xFFFF
#define INS_TYPE( type )      ( type & INS_TYPE_MASK )

   /* flags effected by instruction */
#define INS_TEST_CARRY        0x01    /* carry */
#define INS_TEST_ZERO         0x02    /* zero/equal */
#define INS_TEST_OFLOW        0x04    /* overflow */
#define INS_TEST_DIR          0x08    /* direction */
#define INS_TEST_SIGN         0x10    /* negative */
#define INS_TEST_PARITY       0x20    /* parity */
#define INS_TEST_OR           0x40    /* used in jle */
#define INS_TEST_NCARRY       0x100	/* ! carry */
#define INS_TEST_NZERO        0x200	/* ! zero */
#define INS_TEST_NOFLOW       0x400	/* ! oflow */
#define INS_TEST_NDIR         0x800	/* ! dir */
#define INS_TEST_NSIGN        0x100	/* ! sign */
#define INS_TEST_NPARITY      0x2000	/* ! parity */
/* SF == OF */
#define INS_TEST_SFEQOF       0x4000
/* SF != OF */
#define INS_TEST_SFNEOF       0x8000

#define INS_TEST_ALL		INS_TEST_CARRY | INS_TEST_ZERO | \
				INS_TEST_OFLOW | INS_TEST_SIGN | \
				INS_TEST_PARITY

#define INS_SET_CARRY        0x010000    /* carry */
#define INS_SET_ZERO         0x020000    /* zero/equal */
#define INS_SET_OFLOW        0x040000    /* overflow */
#define INS_SET_DIR          0x080000    /* direction */
#define INS_SET_SIGN         0x100000    /* negative */
#define INS_SET_PARITY       0x200000    /* parity */
#define INS_SET_NCARRY       0x1000000 
#define INS_SET_NZERO        0x2000000
#define INS_SET_NOFLOW       0x4000000
#define INS_SET_NDIR         0x8000000
#define INS_SET_NSIGN        0x10000000
#define INS_SET_NPARITY      0x20000000
#define INS_SET_SFEQOF       0x40000000
#define INS_SET_SFNEOF       0x80000000

#define INS_SET_ALL		INS_SET_CARRY | INS_SET_ZERO | \
				INS_SET_OFLOW | INS_SET_SIGN | \
				INS_SET_PARITY

#define INS_TEST_MASK          0x0000FFFF
#define INS_FLAGS_TEST(x)      (x & INS_TEST_MASK)
#define INS_SET_MASK           0xFFFF0000
#define INS_FLAGS_SET(x)       (x & INS_SET_MASK)

#if 0
/* TODO: actually start using these */
#define X86_PAIR_NP	1		/* not pairable; execs in U */
#define X86_PAIR_PU	2		/* pairable in U pipe */
#define X86_PAIR_PV	3		/* pairable in V pipe */
#define X86_PAIR_UV	4		/* pairable in UV pipe */
#define X86_PAIR_FX	5		/* pairable with FXCH */

#define X86_EXEC_PORT_0	1
#define X86_EXEC_PORT_1	2
#define X86_EXEC_PORT_2	4
#define X86_EXEC_PORT_3	8
#define X86_EXEC_PORT_4	16

#define X86_EXEC_UNITS

typedef struct {	/* representation of an insn during decoding */
	uint32_t flags;		/* runtime settings */
	/* instruction prefixes and other foolishness */
	uint32_t prefix;		/* encoding of prefix */
	char prefix_str[16];		/* mnemonics for prefix */
	uint32_t branch_hint;	/* gah! */
	unsigned int cpu_ver;		/* TODO: cpu version */
	unsigned int clocks;		/* TODO: clock cycles: min/max */
	unsigned char last_prefix;
	/* runtime intruction decoding helpers */
	unsigned char mode;		/* 16, 32, 64 */
	unsigned char gen_regs;		/* offset of default general reg set */
	unsigned char sz_operand;	/* operand size for insn */
	unsigned char sz_address;	/* address size for insn */
	unsigned char uops;		/* uops per insn */
	unsigned char pairing;		/* np,pu,pv.lv */
	unsigned char exec_unit;
	unsigned char exec_port;
	unsigned char latency;
} ia32_info_t;
#define MODE_32 0	/* default */
#define MODE_16 1
#define MODE_64 2
#endif

#endif
