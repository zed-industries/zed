/*
 * Copyright (c) 2015 Imagination Technologies Ltd
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

/**
 * @file
 * MIPS assembly defines from sys/asm.h but rewritten for use with C inline
 * assembly (rather than from within .s files).
 */

#ifndef AVUTIL_MIPS_ASMDEFS_H
#define AVUTIL_MIPS_ASMDEFS_H

#include <stdint.h>

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

/*
 * parse_r var, r - Helper assembler macro for parsing register names.
 *
 * This converts the register name in $n form provided in \r to the
 * corresponding register number, which is assigned to the variable \var. It is
 * needed to allow explicit encoding of instructions in inline assembly where
 * registers are chosen by the compiler in $n form, allowing us to avoid using
 * fixed register numbers.
 *
 * It also allows newer instructions (not implemented by the assembler) to be
 * transparently implemented using assembler macros, instead of needing separate
 * cases depending on toolchain support.
 *
 * Simple usage example:
 * __asm__ __volatile__("parse_r __rt, %0\n\t"
 *                      ".insn\n\t"
 *                      "# di    %0\n\t"
 *                      ".word   (0x41606000 | (__rt << 16))"
 *                      : "=r" (status);
 */

/* Match an individual register number and assign to \var */
#define _IFC_REG(n)                                \
        ".ifc        \\r, $" #n "\n\t"             \
        "\\var        = " #n "\n\t"                \
        ".endif\n\t"

__asm__(".macro        parse_r var r\n\t"
        "\\var        = -1\n\t"
        _IFC_REG(0)  _IFC_REG(1)  _IFC_REG(2)  _IFC_REG(3)
        _IFC_REG(4)  _IFC_REG(5)  _IFC_REG(6)  _IFC_REG(7)
        _IFC_REG(8)  _IFC_REG(9)  _IFC_REG(10) _IFC_REG(11)
        _IFC_REG(12) _IFC_REG(13) _IFC_REG(14) _IFC_REG(15)
        _IFC_REG(16) _IFC_REG(17) _IFC_REG(18) _IFC_REG(19)
        _IFC_REG(20) _IFC_REG(21) _IFC_REG(22) _IFC_REG(23)
        _IFC_REG(24) _IFC_REG(25) _IFC_REG(26) _IFC_REG(27)
        _IFC_REG(28) _IFC_REG(29) _IFC_REG(30) _IFC_REG(31)
        ".iflt        \\var\n\t"
        ".error        \"Unable to parse register name \\r\"\n\t"
        ".endif\n\t"
        ".endm");

/* General union structure for clang adaption */
union mmi_intfloat64 {
    int64_t i;
    double  f;
};

#endif /* AVCODEC_MIPS_ASMDEFS_H */
