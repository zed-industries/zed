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

/*
 * opflags.h - operand flags
 */

#ifndef NASM_OPFLAGS_H
#define NASM_OPFLAGS_H

#include "compiler.h"
#include "tables.h"     /* for opflags_t and nasm_reg_flags[] */
#include "regs.h"

/*
 * Here we define the operand types. These are implemented as bit
 * masks, since some are subsets of others; e.g. AX in a MOV
 * instruction is a special operand type, whereas AX in other
 * contexts is just another 16-bit register. (Also, consider CL in
 * shift instructions, DX in OUT, etc.)
 *
 * The basic concept here is that
 *    (class & ~operand) == 0
 *
 * if and only if "operand" belongs to class type "class".
 */

#define OP_GENMASK(bits, shift)         (((UINT64_C(1) << (bits)) - 1) << (shift))
#define OP_GENBIT(bit, shift)           (UINT64_C(1) << ((shift) + (bit)))

/*
 * Type of operand: memory reference, register, etc.
 *
 * Bits: 0 - 3
 */
#define OPTYPE_SHIFT            (0)
#define OPTYPE_BITS             (4)
#define OPTYPE_MASK             OP_GENMASK(OPTYPE_BITS, OPTYPE_SHIFT)
#define GEN_OPTYPE(bit)         OP_GENBIT(bit, OPTYPE_SHIFT)

/*
 * Modifiers.
 *
 * Bits: 4 - 6
 */
#define MODIFIER_SHIFT          (4)
#define MODIFIER_BITS           (3)
#define MODIFIER_MASK           OP_GENMASK(MODIFIER_BITS, MODIFIER_SHIFT)
#define GEN_MODIFIER(bit)       OP_GENBIT(bit, MODIFIER_SHIFT)

/*
 * Register classes.
 *
 * Bits: 7 - 17
 */
#define REG_CLASS_SHIFT         (7)
#define REG_CLASS_BITS          (11)
#define REG_CLASS_MASK          OP_GENMASK(REG_CLASS_BITS, REG_CLASS_SHIFT)
#define GEN_REG_CLASS(bit)      OP_GENBIT(bit, REG_CLASS_SHIFT)

/*
 * Subclasses. Depends on type of operand.
 *
 * Bits: 18 - 25
 */
#define SUBCLASS_SHIFT          (18)
#define SUBCLASS_BITS           (8)
#define SUBCLASS_MASK           OP_GENMASK(SUBCLASS_BITS, SUBCLASS_SHIFT)
#define GEN_SUBCLASS(bit)       OP_GENBIT(bit, SUBCLASS_SHIFT)

/*
 * Special flags. Context dependent.
 *
 * Bits: 26 - 32
 */
#define SPECIAL_SHIFT           (26)
#define SPECIAL_BITS            (7)
#define SPECIAL_MASK            OP_GENMASK(SPECIAL_BITS, SPECIAL_SHIFT)
#define GEN_SPECIAL(bit)        OP_GENBIT(bit, SPECIAL_SHIFT)

/*
 * Sizes of the operands and attributes.
 *
 * Bits: 33 - 43
 */
#define SIZE_SHIFT              (33)
#define SIZE_BITS               (11)
#define SIZE_MASK               OP_GENMASK(SIZE_BITS, SIZE_SHIFT)
#define GEN_SIZE(bit)           OP_GENBIT(bit, SIZE_SHIFT)

/*
 * Register set count
 *
 * Bits: 44 - 48
 */
#define REGSET_SHIFT            (44)
#define REGSET_BITS             (5)
#define REGSET_MASK             OP_GENMASK(REGSET_BITS, REGSET_SHIFT)
#define GEN_REGSET(bit)         OP_GENBIT(bit, REGSET_SHIFT)

/*
 * Bits distribution (counted from 0)
 *
 *    6         5         4         3         2         1
 * 3210987654321098765432109876543210987654321098765432109876543210
 *                                 |
 *                                 | dword bound
 *
 * ............................................................1111 optypes
 * .........................................................111.... modifiers
 * ..............................................11111111111....... register classes
 * ......................................11111111.................. subclasses
 * ...............................1111111.......................... specials
 * ....................11111111111................................. sizes
 * ...............11111............................................ regset count
 */

#define REGISTER                GEN_OPTYPE(0)                   /* register number in 'basereg' */
#define IMMEDIATE               GEN_OPTYPE(1)
#define REGMEM                  GEN_OPTYPE(2)                   /* for r/m, ie EA, operands */
#define MEMORY                  (GEN_OPTYPE(3) | REGMEM)

#define BITS8                   GEN_SIZE(0)                     /*   8 bits (BYTE) */
#define BITS16                  GEN_SIZE(1)                     /*  16 bits (WORD) */
#define BITS32                  GEN_SIZE(2)                     /*  32 bits (DWORD) */
#define BITS64                  GEN_SIZE(3)                     /*  64 bits (QWORD), x64 and FPU only */
#define BITS80                  GEN_SIZE(4)                     /*  80 bits (TWORD), FPU only */
#define BITS128                 GEN_SIZE(5)                     /* 128 bits (OWORD) */
#define BITS256                 GEN_SIZE(6)                     /* 256 bits (YWORD) */
#define BITS512                 GEN_SIZE(7)                     /* 512 bits (ZWORD) */
#define FAR                     GEN_SIZE(8)                     /* grotty: this means 16:16 or 16:32, like in CALL/JMP */
#define NEAR                    GEN_SIZE(9)
#define SHORT                   GEN_SIZE(10)                    /* and this means what it says :) */

#define TO                      GEN_MODIFIER(0)                 /* reverse effect in FADD, FSUB &c */
#define COLON                   GEN_MODIFIER(1)                 /* operand is followed by a colon */
#define STRICT                  GEN_MODIFIER(2)                 /* do not optimize this operand */

#define REG_CLASS_CDT           GEN_REG_CLASS(0)
#define REG_CLASS_GPR           GEN_REG_CLASS(1)
#define REG_CLASS_SREG          GEN_REG_CLASS(2)
#define REG_CLASS_FPUREG        GEN_REG_CLASS(3)
#define REG_CLASS_RM_MMX        GEN_REG_CLASS(4)
#define REG_CLASS_RM_XMM        GEN_REG_CLASS(5)
#define REG_CLASS_RM_YMM        GEN_REG_CLASS(6)
#define REG_CLASS_RM_ZMM        GEN_REG_CLASS(7)
#define REG_CLASS_OPMASK        GEN_REG_CLASS(8)
#define REG_CLASS_BND           GEN_REG_CLASS(9)
#define REG_CLASS_RM_TMM	GEN_REG_CLASS(10)

static inline bool is_class(opflags_t class, opflags_t op)
{
	return !(class & ~op);
}

static inline bool is_reg_class(opflags_t class, opflags_t reg)
{
	if (reg >= EXPR_REG_START && reg <= EXPR_REG_END)
		return is_class(class, nasm_reg_flags[reg]);
	return false;
}

#define IS_SREG(reg)                is_reg_class(REG_SREG, (reg))
#define IS_FSGS(reg)                is_reg_class(REG_FSGS, (reg))

/* Register classes */
#define REG_EA                  (                                               REGMEM | REGISTER)      /* 'normal' reg, qualifies as EA */
#define RM_GPR                  (                  REG_CLASS_GPR              | REGMEM)                 /* integer operand */
#define REG_GPR                 (                  REG_CLASS_GPR              | REGMEM | REGISTER)      /* integer register */
#define REG8                    (                  REG_CLASS_GPR    | BITS8   | REGMEM | REGISTER)      /*  8-bit GPR  */
#define REG16                   (                  REG_CLASS_GPR    | BITS16  | REGMEM | REGISTER)      /* 16-bit GPR */
#define REG32                   (                  REG_CLASS_GPR    | BITS32  | REGMEM | REGISTER)      /* 32-bit GPR */
#define REG64                   (                  REG_CLASS_GPR    | BITS64  | REGMEM | REGISTER)      /* 64-bit GPR */
#define FPUREG                  (                  REG_CLASS_FPUREG                    | REGISTER)      /* floating point stack registers */
#define FPU0                    (GEN_SUBCLASS(1) | REG_CLASS_FPUREG                    | REGISTER)      /* FPU stack register zero */
#define RM_MMX                  (                  REG_CLASS_RM_MMX           | REGMEM)                 /* MMX operand */
#define MMXREG                  (                  REG_CLASS_RM_MMX           | REGMEM | REGISTER)      /* MMX register */
#define RM_XMM                  (                  REG_CLASS_RM_XMM           | REGMEM)                 /* XMM (SSE) operand */
#define XMMREG                  (                  REG_CLASS_RM_XMM           | REGMEM | REGISTER)      /* XMM (SSE) register */
#define RM_YMM                  (                  REG_CLASS_RM_YMM           | REGMEM)                 /* YMM (AVX) operand */
#define YMMREG                  (                  REG_CLASS_RM_YMM           | REGMEM | REGISTER)      /* YMM (AVX) register */
#define RM_ZMM                  (                  REG_CLASS_RM_ZMM           | REGMEM)                 /* ZMM (AVX512) operand */
#define ZMMREG                  (                  REG_CLASS_RM_ZMM           | REGMEM | REGISTER)      /* ZMM (AVX512) register */
#define RM_OPMASK               (                  REG_CLASS_OPMASK           | REGMEM)                 /* Opmask operand */
#define OPMASKREG               (                  REG_CLASS_OPMASK           | REGMEM | REGISTER)      /* Opmask register */
#define OPMASK0                 (GEN_SUBCLASS(1) | REG_CLASS_OPMASK           | REGMEM | REGISTER)      /* Opmask register zero (k0) */
#define RM_K                    RM_OPMASK
#define KREG                    OPMASKREG
#define RM_BND                  (                  REG_CLASS_BND              | REGMEM)                 /* Bounds operand */
#define BNDREG                  (                  REG_CLASS_BND              | REGMEM | REGISTER)      /* Bounds register */
#define TMMREG                  (                  REG_CLASS_RM_TMM           | REGMEM | REGISTER)      /* TMM (AMX) register */
#define REG_CDT                 (                  REG_CLASS_CDT    | BITS32           | REGISTER)      /* CRn, DRn and TRn */
#define REG_CREG                (GEN_SUBCLASS(1) | REG_CLASS_CDT    | BITS32           | REGISTER)      /* CRn */
#define REG_DREG                (GEN_SUBCLASS(2) | REG_CLASS_CDT    | BITS32           | REGISTER)      /* DRn */
#define REG_TREG                (GEN_SUBCLASS(3) | REG_CLASS_CDT    | BITS32           | REGISTER)      /* TRn */
#define REG_SREG                (                  REG_CLASS_SREG   | BITS16           | REGISTER)      /* any segment register */

/* Segment registers */
#define REG_ES                  (GEN_SUBCLASS(0) | GEN_SUBCLASS(2) | REG_CLASS_SREG | BITS16 | REGISTER)      /* ES */
#define REG_CS                  (GEN_SUBCLASS(1) | GEN_SUBCLASS(2) | REG_CLASS_SREG | BITS16 | REGISTER)      /* CS */
#define REG_SS                  (GEN_SUBCLASS(0) | GEN_SUBCLASS(3) | REG_CLASS_SREG | BITS16 | REGISTER)      /* SS */
#define REG_DS                  (GEN_SUBCLASS(1) | GEN_SUBCLASS(3) | REG_CLASS_SREG | BITS16 | REGISTER)      /* DS */
#define REG_FS                  (GEN_SUBCLASS(0) | GEN_SUBCLASS(4) | REG_CLASS_SREG | BITS16 | REGISTER)      /* FS */
#define REG_GS                  (GEN_SUBCLASS(1) | GEN_SUBCLASS(4) | REG_CLASS_SREG | BITS16 | REGISTER)      /* GS */
#define REG_FSGS                (                  GEN_SUBCLASS(4) | REG_CLASS_SREG | BITS16 | REGISTER)      /* FS or GS */
#define REG_SEG67               (                  GEN_SUBCLASS(5) | REG_CLASS_SREG | BITS16 | REGISTER)      /* Unimplemented segment registers */

/* Special GPRs */
#define REG_SMASK               SUBCLASS_MASK                                                                           /* a mask for the following */
#define REG_ACCUM               (GEN_SUBCLASS(1)                   | REG_CLASS_GPR           | REGMEM | REGISTER)       /* accumulator: AL, AX, EAX, RAX */
#define REG_AL                  (GEN_SUBCLASS(1)                   | REG_CLASS_GPR | BITS8   | REGMEM | REGISTER)
#define REG_AX                  (GEN_SUBCLASS(1)                   | REG_CLASS_GPR | BITS16  | REGMEM | REGISTER)
#define REG_EAX                 (GEN_SUBCLASS(1)                   | REG_CLASS_GPR | BITS32  | REGMEM | REGISTER)
#define REG_RAX                 (GEN_SUBCLASS(1)                   | REG_CLASS_GPR | BITS64  | REGMEM | REGISTER)
#define REG_COUNT               (GEN_SUBCLASS(5) | GEN_SUBCLASS(2) | REG_CLASS_GPR           | REGMEM | REGISTER)       /* counter: CL, CX, ECX, RCX */
#define REG_CL                  (GEN_SUBCLASS(5) | GEN_SUBCLASS(2) | REG_CLASS_GPR | BITS8   | REGMEM | REGISTER)
#define REG_CX                  (GEN_SUBCLASS(5) | GEN_SUBCLASS(2) | REG_CLASS_GPR | BITS16  | REGMEM | REGISTER)
#define REG_ECX                 (GEN_SUBCLASS(5) | GEN_SUBCLASS(2) | REG_CLASS_GPR | BITS32  | REGMEM | REGISTER)
#define REG_RCX                 (GEN_SUBCLASS(5) | GEN_SUBCLASS(2) | REG_CLASS_GPR | BITS64  | REGMEM | REGISTER)
#define REG_DL                  (GEN_SUBCLASS(5) | GEN_SUBCLASS(3) | REG_CLASS_GPR | BITS8   | REGMEM | REGISTER)       /* data: DL, DX, EDX, RDX */
#define REG_DX                  (GEN_SUBCLASS(5) | GEN_SUBCLASS(3) | REG_CLASS_GPR | BITS16  | REGMEM | REGISTER)
#define REG_EDX                 (GEN_SUBCLASS(5) | GEN_SUBCLASS(3) | REG_CLASS_GPR | BITS32  | REGMEM | REGISTER)
#define REG_RDX                 (GEN_SUBCLASS(5) | GEN_SUBCLASS(3) | REG_CLASS_GPR | BITS64  | REGMEM | REGISTER)
#define REG_HIGH                (GEN_SUBCLASS(5) | GEN_SUBCLASS(4) | REG_CLASS_GPR | BITS8   | REGMEM | REGISTER)       /* high regs: AH, CH, DH, BH */
#define REG_NOTACC              GEN_SUBCLASS(5)                                                                         /* non-accumulator register */
#define REG8NA                  (GEN_SUBCLASS(5)                   | REG_CLASS_GPR | BITS8   | REGMEM | REGISTER)       /*  8-bit non-acc GPR  */
#define REG16NA                 (GEN_SUBCLASS(5)                   | REG_CLASS_GPR | BITS16  | REGMEM | REGISTER)       /* 16-bit non-acc GPR */
#define REG32NA                 (GEN_SUBCLASS(5)                   | REG_CLASS_GPR | BITS32  | REGMEM | REGISTER)       /* 32-bit non-acc GPR */
#define REG64NA                 (GEN_SUBCLASS(5)                   | REG_CLASS_GPR | BITS64  | REGMEM | REGISTER)       /* 64-bit non-acc GPR */

/* special types of EAs */
#define MEM_OFFS                (GEN_SUBCLASS(1) | MEMORY)      /* simple [address] offset - absolute! */
#define IP_REL                  (GEN_SUBCLASS(2) | MEMORY)      /* IP-relative offset */
#define XMEM                    (GEN_SUBCLASS(3) | MEMORY)      /* 128-bit vector SIB */
#define YMEM                    (GEN_SUBCLASS(4) | MEMORY)      /* 256-bit vector SIB */
#define ZMEM                    (GEN_SUBCLASS(5) | MEMORY)      /* 512-bit vector SIB */

/* memory which matches any type of r/m operand */
#define MEMORY_ANY              (MEMORY | RM_GPR | RM_MMX | RM_XMM_L16 | RM_YMM_L16 | RM_ZMM_L16 | RM_OPMASK | RM_BND)

/* special immediate values */
#define UNITY                   (GEN_SUBCLASS(0) | IMMEDIATE)   /* operand equals 1 */
#define SBYTEWORD               (GEN_SUBCLASS(1) | IMMEDIATE)   /* operand is in the range -128..127 mod 2^16 */
#define SBYTEDWORD              (GEN_SUBCLASS(2) | IMMEDIATE)   /* operand is in the range -128..127 mod 2^32 */
#define SDWORD                  (GEN_SUBCLASS(3) | IMMEDIATE)   /* operand is in the range -0x80000000..0x7FFFFFFF */
#define UDWORD                  (GEN_SUBCLASS(4) | IMMEDIATE)   /* operand is in the range 0..0xFFFFFFFF */

/*
 * Subset of vector registers: register 0 only and registers 0-15.
 * Avoid conflicts in subclass bitfield with any of special EA types!
 */
#define RM_XMM_L16              (GEN_SUBCLASS(6) | RM_XMM)                                              /* XMM r/m operand  0 ~ 15 */
#define XMM0                    (GEN_SUBCLASS(1) | GEN_SUBCLASS(6) | XMMREG)                            /* XMM register   zero  */
#define XMM_L16                 (                  GEN_SUBCLASS(6) | XMMREG)                            /* XMM register  0 ~ 15 */

#define RM_YMM_L16              (GEN_SUBCLASS(6) | RM_YMM)                                              /* YMM r/m operand  0 ~ 15 */
#define YMM0                    (GEN_SUBCLASS(1) | GEN_SUBCLASS(6) | YMMREG)                            /* YMM register   zero  */
#define YMM_L16                 (                  GEN_SUBCLASS(6) | YMMREG)                            /* YMM register  0 ~ 15 */

#define RM_ZMM_L16              (GEN_SUBCLASS(6) | RM_ZMM)                                              /* ZMM r/m operand  0 ~ 15 */
#define ZMM0                    (GEN_SUBCLASS(1) | GEN_SUBCLASS(6) | ZMMREG)                            /* ZMM register   zero  */
#define ZMM_L16                 (                  GEN_SUBCLASS(6) | ZMMREG)                            /* ZMM register  0 ~ 15 */

/* Register set sizes */
#define RS2                     GEN_REGSET(0)
#define RS4                     GEN_REGSET(1)
#define RS8                     GEN_REGSET(2)
#define RS16                    GEN_REGSET(3)
#define RS32                    GEN_REGSET(4)

#endif /* NASM_OPFLAGS_H */
