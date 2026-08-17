
/*---------------------------------------------------------------*/
/*--- begin                                       libvex_ir.h ---*/
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

#ifndef __LIBVEX_IR_H
#define __LIBVEX_IR_H

#include "libvex_basictypes.h"

   
/*---------------------------------------------------------------*/
/*--- High-level IR description                               ---*/
/*---------------------------------------------------------------*/

/* Vex IR is an architecture-neutral intermediate representation.
   Unlike some IRs in systems similar to Vex, it is not like assembly
   language (ie. a list of instructions).  Rather, it is more like the
   IR that might be used in a compiler.

   Code blocks
   ~~~~~~~~~~~
   The code is broken into small code blocks ("superblocks", type:
   'IRSB').  Each code block typically represents from 1 to perhaps 50
   instructions.  IRSBs are single-entry, multiple-exit code blocks.
   Each IRSB contains three things:
   - a type environment, which indicates the type of each temporary
     value present in the IRSB
   - a list of statements, which represent code
   - a jump that exits from the end the IRSB
   Because the blocks are multiple-exit, there can be additional
   conditional exit statements that cause control to leave the IRSB
   before the final exit.  Also because of this, IRSBs can cover
   multiple non-consecutive sequences of code (up to 3).  These are
   recorded in the type VexGuestExtents (see libvex.h).

   Statements and expressions
   ~~~~~~~~~~~~~~~~~~~~~~~~~~
   Statements (type 'IRStmt') represent operations with side-effects,
   eg.  guest register writes, stores, and assignments to temporaries.
   Expressions (type 'IRExpr') represent operations without
   side-effects, eg. arithmetic operations, loads, constants.
   Expressions can contain sub-expressions, forming expression trees,
   eg. (3 + (4 * load(addr1)).

   Storage of guest state
   ~~~~~~~~~~~~~~~~~~~~~~
   The "guest state" contains the guest registers of the guest machine
   (ie.  the machine that we are simulating).  It is stored by default
   in a block of memory supplied by the user of the VEX library,
   generally referred to as the guest state (area).  To operate on
   these registers, one must first read ("Get") them from the guest
   state into a temporary value.  Afterwards, one can write ("Put")
   them back into the guest state.

   Get and Put are characterised by a byte offset into the guest
   state, a small integer which effectively gives the identity of the
   referenced guest register, and a type, which indicates the size of
   the value to be transferred.

   The basic "Get" and "Put" operations are sufficient to model normal
   fixed registers on the guest.  Selected areas of the guest state
   can be treated as a circular array of registers (type:
   'IRRegArray'), which can be indexed at run-time.  This is done with
   the "GetI" and "PutI" primitives.  This is necessary to describe
   rotating register files, for example the x87 FPU stack, SPARC
   register windows, and the Itanium register files.

   Examples, and flattened vs. unflattened code
   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
   For example, consider this x86 instruction:
     
     addl %eax, %ebx

   One Vex IR translation for this code would be this:

     ------ IMark(0x24F275, 7, 0) ------
     t3 = GET:I32(0)             # get %eax, a 32-bit integer
     t2 = GET:I32(12)            # get %ebx, a 32-bit integer
     t1 = Add32(t3,t2)           # addl
     PUT(0) = t1                 # put %eax

   (For simplicity, this ignores the effects on the condition codes, and
   the update of the instruction pointer.)

   The "IMark" is an IR statement that doesn't represent actual code.
   Instead it indicates the address and length of the original
   instruction.  The numbers 0 and 12 are offsets into the guest state
   for %eax and %ebx.  The full list of offsets for an architecture
   <ARCH> can be found in the type VexGuest<ARCH>State in the file
   VEX/pub/libvex_guest_<ARCH>.h.

   The five statements in this example are:
   - the IMark
   - three assignments to temporaries
   - one register write (put)

   The six expressions in this example are:
   - two register reads (gets)
   - one arithmetic (add) operation
   - three temporaries (two nested within the Add32, one in the PUT)

   The above IR is "flattened", ie. all sub-expressions are "atoms",
   either constants or temporaries.  An equivalent, unflattened version
   would be:
   
     PUT(0) = Add32(GET:I32(0), GET:I32(12))

   IR is guaranteed to be flattened at instrumentation-time.  This makes
   instrumentation easier.  Equivalent flattened and unflattened IR
   typically results in the same generated code.

   Another example, this one showing loads and stores:

     addl %edx,4(%eax)

   This becomes (again ignoring condition code and instruction pointer
   updates):

     ------ IMark(0x4000ABA, 3, 0) ------
     t3 = Add32(GET:I32(0),0x4:I32)
     t2 = LDle:I32(t3)
     t1 = GET:I32(8)
     t0 = Add32(t2,t1)
     STle(t3) = t0

   The "le" in "LDle" and "STle" is short for "little-endian".

   No need for deallocations
   ~~~~~~~~~~~~~~~~~~~~~~~~~
   Although there are allocation functions for various data structures
   in this file, there are no deallocation functions.  This is because
   Vex uses a memory allocation scheme that automatically reclaims the
   memory used by allocated structures once translation is completed.
   This makes things easier for tools that instruments/transforms code
   blocks.

   SSAness and typing
   ~~~~~~~~~~~~~~~~~~
   The IR is fully typed.  For every IRSB (IR block) it is possible to
   say unambiguously whether or not it is correctly typed.
   Incorrectly typed IR has no meaning and the VEX will refuse to
   process it.  At various points during processing VEX typechecks the
   IR and aborts if any violations are found.  This seems overkill but
   makes it a great deal easier to build a reliable JIT.

   IR also has the SSA property.  SSA stands for Static Single
   Assignment, and what it means is that each IR temporary may be
   assigned to only once.  This idea became widely used in compiler
   construction in the mid to late 90s.  It makes many IR-level
   transformations/code improvements easier, simpler and faster.
   Whenever it typechecks an IR block, VEX also checks the SSA
   property holds, and will abort if not so.  So SSAness is
   mechanically and rigidly enforced.
*/

/*---------------------------------------------------------------*/
/*--- Type definitions for the IR                             ---*/
/*---------------------------------------------------------------*/

/* General comments about naming schemes:

   All publically visible functions contain the name of the primary
   type on which they operate (IRFoo, IRBar, etc).  Hence you should
   be able to identify these functions by grepping for "IR[A-Z]".

   For some type 'IRFoo':

   - ppIRFoo is the printing method for IRFoo, printing it to the
     output channel specified in the LibVEX_Initialise call.

   - eqIRFoo is a structural equality predicate for IRFoos.

   - deepCopyIRFoo is a deep copy constructor for IRFoos. 
     It recursively traverses the entire argument tree and
     produces a complete new tree.  All types have a deep copy
     constructor.

   - shallowCopyIRFoo is the shallow copy constructor for IRFoos.
     It creates a new top-level copy of the supplied object,
     but does not copy any sub-objects.  Only some types have a
     shallow copy constructor.
*/

/* ------------------ Types ------------------ */

/* A type indicates the size of a value, and whether it's an integer, a
   float, or a vector (SIMD) value. */
typedef 
   enum { 
      Ity_INVALID=0x1100,
      Ity_I1, 
      Ity_I8, 
      Ity_I16, 
      Ity_I32, 
      Ity_I64,
      Ity_I128,  /* 128-bit scalar */
      Ity_F16,   /* 16 bit float */
      Ity_F32,   /* IEEE 754 float */
      Ity_F64,   /* IEEE 754 double */
      Ity_D32,   /* 32-bit Decimal floating point */
      Ity_D64,   /* 64-bit Decimal floating point */
      Ity_D128,  /* 128-bit Decimal floating point */
      Ity_F128,  /* 128-bit floating point; implementation defined */
      Ity_V128,  /* 128-bit SIMD */
      Ity_V256   /* 256-bit SIMD */
   }
   IRType;

/* Pretty-print an IRType */
extern void ppIRType ( IRType );

/* Get the size (in bytes) of an IRType */ 
extern Int sizeofIRType ( IRType );

/* Translate 1/2/4/8 into Ity_I{8,16,32,64} respectively.  Asserts on
   any other input. */
extern IRType integerIRTypeOfSize ( Int szB );


/* ------------------ Endianness ------------------ */

/* IREndness is used in load IRExprs and store IRStmts. */
typedef
   enum { 
      Iend_LE=0x1200, /* little endian */
      Iend_BE          /* big endian */
   }
   IREndness;


/* ------------------ Constants ------------------ */

/* IRConsts are used within 'Const' and 'Exit' IRExprs. */

/* The various kinds of constant. */
typedef
   enum { 
      Ico_U1=0x1300,
      Ico_U8, 
      Ico_U16, 
      Ico_U32, 
      Ico_U64,
      Ico_F32,   /* 32-bit IEEE754 floating */
      Ico_F32i,  /* 32-bit unsigned int to be interpreted literally
                    as a IEEE754 single value. */
      Ico_F64,   /* 64-bit IEEE754 floating */
      Ico_F64i,  /* 64-bit unsigned int to be interpreted literally
                    as a IEEE754 double value. */
      Ico_V128,  /* 128-bit restricted vector constant, with 1 bit
                    (repeated 8 times) for each of the 16 x 1-byte lanes */
      Ico_V256   /* 256-bit restricted vector constant, with 1 bit
                    (repeated 8 times) for each of the 32 x 1-byte lanes */
   }
   IRConstTag;

/* A constant.  Stored as a tagged union.  'tag' indicates what kind of
   constant this is.  'Ico' is the union that holds the fields.  If an
   IRConst 'c' has c.tag equal to Ico_U32, then it's a 32-bit constant,
   and its value can be accessed with 'c.Ico.U32'. */
typedef
   struct _IRConst {
      IRConstTag tag;
      union {
         Bool   U1;
         UChar  U8;
         UShort U16;
         UInt   U32;
         ULong  U64;
         Float  F32;
         UInt   F32i;
         Double F64;
         ULong  F64i;
         UShort V128;   /* 16-bit value; see Ico_V128 comment above */
         UInt   V256;   /* 32-bit value; see Ico_V256 comment above */
      } Ico;
   }
   IRConst;

/* IRConst constructors */
extern IRConst* IRConst_U1   ( Bool );
extern IRConst* IRConst_U8   ( UChar );
extern IRConst* IRConst_U16  ( UShort );
extern IRConst* IRConst_U32  ( UInt );
extern IRConst* IRConst_U64  ( ULong );
extern IRConst* IRConst_F32  ( Float );
extern IRConst* IRConst_F32i ( UInt );
extern IRConst* IRConst_F64  ( Double );
extern IRConst* IRConst_F64i ( ULong );
extern IRConst* IRConst_V128 ( UShort );
extern IRConst* IRConst_V256 ( UInt );

/* Deep-copy an IRConst */
extern IRConst* deepCopyIRConst ( const IRConst* );

/* Pretty-print an IRConst */
extern void ppIRConst ( const IRConst* );

/* Compare two IRConsts for equality */
extern Bool eqIRConst ( const IRConst*, const IRConst* );


/* ------------------ Call targets ------------------ */

/* Describes a helper function to call.  The name part is purely for
   pretty printing and not actually used.  regparms=n tells the back
   end that the callee has been declared
   "__attribute__((regparm(n)))", although indirectly using the
   VEX_REGPARM(n) macro.  On some targets (x86) the back end will need
   to construct a non-standard sequence to call a function declared
   like this.

   mcx_mask is a sop to Memcheck.  It indicates which args should be
   considered 'always defined' when lazily computing definedness of
   the result.  Bit 0 of mcx_mask corresponds to args[0], bit 1 to
   args[1], etc.  If a bit is set, the corresponding arg is excluded
   (hence "x" in "mcx") from definedness checking.  
*/

typedef
   struct {
      Int          regparms;
      const HChar* name;
      void*        addr;
      UInt         mcx_mask;
   }
   IRCallee;

/* Create an IRCallee. */
extern IRCallee* mkIRCallee ( Int regparms, const HChar* name, void* addr );

/* Deep-copy an IRCallee. */
extern IRCallee* deepCopyIRCallee ( const IRCallee* );

/* Pretty-print an IRCallee. */
extern void ppIRCallee ( const IRCallee* );


/* ------------------ Guest state arrays ------------------ */

/* This describes a section of the guest state that we want to
   be able to index at run time, so as to be able to describe 
   indexed or rotating register files on the guest. */
typedef
   struct {
      Int    base;   /* guest state offset of start of indexed area */
      IRType elemTy; /* type of each element in the indexed area */
      Int    nElems; /* number of elements in the indexed area */
   }
   IRRegArray;

extern IRRegArray* mkIRRegArray ( Int, IRType, Int );

extern IRRegArray* deepCopyIRRegArray ( const IRRegArray* );

extern void ppIRRegArray ( const IRRegArray* );
extern Bool eqIRRegArray ( const IRRegArray*, const IRRegArray* );


/* ------------------ Temporaries ------------------ */

/* This represents a temporary, eg. t1.  The IR optimiser relies on the
   fact that IRTemps are 32-bit ints.  Do not change them to be ints of
   any other size. */
typedef UInt IRTemp;

/* Pretty-print an IRTemp. */
extern void ppIRTemp ( IRTemp );

#define IRTemp_INVALID ((IRTemp)0xFFFFFFFF)


/* --------------- Primops (arity 1,2,3 and 4) --------------- */

/* Primitive operations that are used in Unop, Binop, Triop and Qop
   IRExprs.  Once we take into account integer, floating point and SIMD
   operations of all the different sizes, there are quite a lot of them.
   Most instructions supported by the architectures that Vex supports
   (x86, PPC, etc) are represented.  Some more obscure ones (eg. cpuid)
   are not;  they are instead handled with dirty helpers that emulate
   their functionality.  Such obscure ones are thus not directly visible
   in the IR, but their effects on guest state (memory and registers) 
   are made visible via the annotations in IRDirty structures.

   2018-Dec-27: some of int<->fp conversion operations have been renamed so as
   to have a trailing _DEP, meaning "deprecated".  This is because they don't
   specify a rounding mode to be used for the conversion and so are
   underspecified.  Their use should be replaced with equivalents that do
   specify a rounding mode, either as a first argument or using a suffix on the
   name, that indicates the rounding mode to use.
*/
typedef
   enum { 
      /* -- Do not change this ordering.  The IR generators rely on
            (eg) Iop_Add64 == IopAdd8 + 3. -- */

      Iop_INVALID=0x1400,
      Iop_Add8,  Iop_Add16,  Iop_Add32,  Iop_Add64,
      Iop_Sub8,  Iop_Sub16,  Iop_Sub32,  Iop_Sub64,
      /* Signless mul.  MullS/MullU is elsewhere. */
      Iop_Mul8,  Iop_Mul16,  Iop_Mul32,  Iop_Mul64,
      Iop_Or8,   Iop_Or16,   Iop_Or32,   Iop_Or64,
      Iop_And8,  Iop_And16,  Iop_And32,  Iop_And64,
      Iop_Xor8,  Iop_Xor16,  Iop_Xor32,  Iop_Xor64,
      Iop_Shl8,  Iop_Shl16,  Iop_Shl32,  Iop_Shl64,
      Iop_Shr8,  Iop_Shr16,  Iop_Shr32,  Iop_Shr64,
      Iop_Sar8,  Iop_Sar16,  Iop_Sar32,  Iop_Sar64,
      /* Integer comparisons. */
      Iop_CmpEQ8,  Iop_CmpEQ16,  Iop_CmpEQ32,  Iop_CmpEQ64,
      Iop_CmpNE8,  Iop_CmpNE16,  Iop_CmpNE32,  Iop_CmpNE64,
      /* Tags for unary ops */
      Iop_Not8,  Iop_Not16,  Iop_Not32,  Iop_Not64,

      /* Exactly like CmpEQ8/16/32/64, but carrying the additional
         hint that these compute the success/failure of a CAS
         operation, and hence are almost certainly applied to two
         copies of the same value, which in turn has implications for
         Memcheck's instrumentation. */
      Iop_CasCmpEQ8, Iop_CasCmpEQ16, Iop_CasCmpEQ32, Iop_CasCmpEQ64,
      Iop_CasCmpNE8, Iop_CasCmpNE16, Iop_CasCmpNE32, Iop_CasCmpNE64,

      /* Exactly like CmpNE8/16/32/64, but carrying the additional
         hint that these needs expensive definedness tracking. */
      Iop_ExpCmpNE8, Iop_ExpCmpNE16, Iop_ExpCmpNE32, Iop_ExpCmpNE64,

      /* -- Ordering not important after here. -- */

      /* Widening multiplies */
      Iop_MullS8, Iop_MullS16, Iop_MullS32, Iop_MullS64,
      Iop_MullU8, Iop_MullU16, Iop_MullU32, Iop_MullU64,

      /* Counting bits */
      /* Ctz64/Ctz32/Clz64/Clz32 are UNDEFINED when given arguments of zero.
         You must ensure they are never given a zero argument.  As of
         2018-Nov-14 they are deprecated.  Try to use the Nat variants
         immediately below, if you can.
      */
      Iop_Clz64, Iop_Clz32,   /* count leading zeroes */
      Iop_Ctz64, Iop_Ctz32,   /* count trailing zeros */
      /* Count leading/trailing zeroes, with "natural" semantics for the
         case where the input is zero: then the result is the number of bits
         in the word. */
      Iop_ClzNat64, Iop_ClzNat32,
      Iop_CtzNat64, Iop_CtzNat32,
      /* Population count -- compute the number of 1 bits in the argument. */
      Iop_PopCount64, Iop_PopCount32,

      /* Standard integer comparisons */
      Iop_CmpLT32S, Iop_CmpLT64S,
      Iop_CmpLE32S, Iop_CmpLE64S,
      Iop_CmpLT32U, Iop_CmpLT64U,
      Iop_CmpLE32U, Iop_CmpLE64U,

      /* As a sop to Valgrind-Memcheck, the following are useful. */
      Iop_CmpNEZ8, Iop_CmpNEZ16,  Iop_CmpNEZ32,  Iop_CmpNEZ64,
      Iop_CmpwNEZ32, Iop_CmpwNEZ64, /* all-0s -> all-Os; other -> all-1s */
      Iop_Left8, Iop_Left16, Iop_Left32, Iop_Left64, /*  \x -> x | -x */
      Iop_Max32U, /* unsigned max */

      /* PowerPC-style 3-way integer comparisons.  Without them it is
         difficult to simulate PPC efficiently.
         op(x,y) | x < y  = 0x8 else 
                 | x > y  = 0x4 else
                 | x == y = 0x2
      */
      Iop_CmpORD32U, Iop_CmpORD64U,
      Iop_CmpORD32S, Iop_CmpORD64S,

      /* Division */
      /* TODO: clarify semantics wrt rounding, negative values, whatever */
      Iop_DivU32,   // :: I32,I32 -> I32 (simple div, no mod)
      Iop_DivS32,   // ditto, signed
      Iop_DivU64,   // :: I64,I64 -> I64 (simple div, no mod)
      Iop_DivS64,   // ditto, signed
      Iop_DivU64E,  // :: I64,I64 -> I64 (dividend is 64-bit arg (hi)
                    //                    concat with 64 0's (low))
      Iop_DivS64E,  // ditto, signed
      Iop_DivU32E,  // :: I32,I32 -> I32 (dividend is 32-bit arg (hi)
                    // concat with 32 0's (low))
      Iop_DivS32E,  // ditto, signed

      Iop_DivModU64to32, // :: I64,I32 -> I64
                         // of which lo half is div and hi half is mod
      Iop_DivModS64to32, // ditto, signed

      Iop_DivModU128to64, // :: V128,I64 -> V128
                          // of which lo half is div and hi half is mod
      Iop_DivModS128to64, // ditto, signed

      Iop_DivModS64to64, // :: I64,I64 -> I128
                         // of which lo half is div and hi half is mod
      Iop_DivModU64to64, // :: I64,I64 -> I128
                         // of which lo half is div and hi half is mod
      Iop_DivModS32to32, // :: I32,I32 -> I64
                         // of which lo half is div and hi half is mod
      Iop_DivModU32to32, // :: I32,I32 -> I64
                         // of which lo half is div and hi half is mod

      /* Integer conversions.  Some of these are redundant (eg
         Iop_64to8 is the same as Iop_64to32 and then Iop_32to8), but
         having a complete set reduces the typical dynamic size of IR
         and makes the instruction selectors easier to write. */

      /* Widening conversions */
      Iop_8Uto16, Iop_8Uto32,  Iop_8Uto64,
                  Iop_16Uto32, Iop_16Uto64,
                               Iop_32Uto64,
      Iop_8Sto16, Iop_8Sto32,  Iop_8Sto64,
                  Iop_16Sto32, Iop_16Sto64,
                               Iop_32Sto64,

      /* Narrowing conversions */
      Iop_64to8, Iop_32to8, Iop_64to16,
      /* 8 <-> 16 bit conversions */
      Iop_16to8,      // :: I16 -> I8, low half
      Iop_16HIto8,    // :: I16 -> I8, high half
      Iop_8HLto16,    // :: (I8,I8) -> I16
      /* 16 <-> 32 bit conversions */
      Iop_32to16,     // :: I32 -> I16, low half
      Iop_32HIto16,   // :: I32 -> I16, high half
      Iop_16HLto32,   // :: (I16,I16) -> I32
      /* 32 <-> 64 bit conversions */
      Iop_64to32,     // :: I64 -> I32, low half
      Iop_64HIto32,   // :: I64 -> I32, high half
      Iop_32HLto64,   // :: (I32,I32) -> I64
      /* 64 <-> 128 bit conversions */
      Iop_128to64,    // :: I128 -> I64, low half
      Iop_128HIto64,  // :: I128 -> I64, high half
      Iop_64HLto128,  // :: (I64,I64) -> I128
      /* 1-bit stuff */
      Iop_Not1,   /* :: Ity_Bit -> Ity_Bit */
      Iop_And1,   /* :: (Ity_Bit, Ity_Bit) -> Ity_Bit.  Evaluates both args! */
      Iop_Or1,    /* :: (Ity_Bit, Ity_Bit) -> Ity_Bit.  Evaluates both args! */
      Iop_32to1,  /* :: Ity_I32 -> Ity_Bit, just select bit[0] */
      Iop_64to1,  /* :: Ity_I64 -> Ity_Bit, just select bit[0] */
      Iop_1Uto8,  /* :: Ity_Bit -> Ity_I8,  unsigned widen */
      Iop_1Uto32, /* :: Ity_Bit -> Ity_I32, unsigned widen */
      Iop_1Uto64, /* :: Ity_Bit -> Ity_I64, unsigned widen */
      Iop_1Sto8,  /* :: Ity_Bit -> Ity_I8,  signed widen */
      Iop_1Sto16, /* :: Ity_Bit -> Ity_I16, signed widen */
      Iop_1Sto32, /* :: Ity_Bit -> Ity_I32, signed widen */
      Iop_1Sto64, /* :: Ity_Bit -> Ity_I64, signed widen */

      /* ------ Floating point.  We try to be IEEE754 compliant. ------ */

      /* --- Simple stuff as mandated by 754. --- */

      /* Binary operations, with rounding. */
      /* :: IRRoundingMode(I32) x F64 x F64 -> F64 */ 
      Iop_AddF64, Iop_SubF64, Iop_MulF64, Iop_DivF64,

      /* :: IRRoundingMode(I32) x F32 x F32 -> F32 */ 
      Iop_AddF32, Iop_SubF32, Iop_MulF32, Iop_DivF32,

      /* Variants of the above which produce a 64-bit result but which
         round their result to a IEEE float range first. */
      /* :: IRRoundingMode(I32) x F64 x F64 -> F64 */ 
      Iop_AddF64r32, Iop_SubF64r32, Iop_MulF64r32, Iop_DivF64r32, 

      /* Unary operations, without rounding. */
      /* :: F64 -> F64 */
      Iop_NegF64, Iop_AbsF64,

      /* :: F32 -> F32 */
      Iop_NegF32, Iop_AbsF32,

      /* Unary operations, with rounding. */
      /* :: IRRoundingMode(I32) x F64 -> F64 */
      Iop_SqrtF64,

      /* :: IRRoundingMode(I32) x F32 -> F32 */
      Iop_SqrtF32,

      /* Comparison, yielding GT/LT/EQ/UN(ordered), as per the following:
            0x45 Unordered
            0x01 LT
            0x00 GT
            0x40 EQ
         This just happens to be the Intel encoding.  The values
         are recorded in the type IRCmpF64Result.
      */
      /* :: F64 x F64 -> IRCmpF64Result(I32) */
      Iop_CmpF64,
      Iop_CmpF32,
      Iop_CmpF128,

      /* --- Int to/from FP conversions. --- */

      /* For the most part, these take a first argument :: Ity_I32 (as
         IRRoundingMode) which is an indication of the rounding mode
         to use, as per the following encoding ("the standard
         encoding"):
            00b  to nearest (the default)
            01b  to -infinity
            10b  to +infinity
            11b  to zero
         This just happens to be the Intel encoding.  For reference only,
         the PPC encoding is:
            00b  to nearest (the default)
            01b  to zero
            10b  to +infinity
            11b  to -infinity
         Any PPC -> IR front end will have to translate these PPC
         encodings, as encoded in the guest state, to the standard
         encodings, to pass to the primops.
         For reference only, the ARM VFP encoding is:
            00b  to nearest
            01b  to +infinity
            10b  to -infinity
            11b  to zero
         Again, this will have to be converted to the standard encoding
         to pass to primops.

         If one of these conversions gets an out-of-range condition,
         or a NaN, as an argument, the result is host-defined.  On x86
         the "integer indefinite" value 0x80..00 is produced.  On PPC
         it is either 0x80..00 or 0x7F..FF depending on the sign of
         the argument.

         On ARMvfp, when converting to a signed integer result, the
         overflow result is 0x80..00 for negative args and 0x7F..FF
         for positive args.  For unsigned integer results it is
         0x00..00 and 0xFF..FF respectively.

         Rounding is required whenever the destination type cannot
         represent exactly all values of the source type.
      */
      Iop_F64toI16S, /* IRRoundingMode(I32) x F64 -> signed I16 */
      Iop_F64toI32S, /* IRRoundingMode(I32) x F64 -> signed I32 */
      Iop_F64toI64S, /* IRRoundingMode(I32) x F64 -> signed I64 */
      Iop_F64toI64U, /* IRRoundingMode(I32) x F64 -> unsigned I64 */

      Iop_F64toI32U, /* IRRoundingMode(I32) x F64 -> unsigned I32 */

      Iop_I32StoF64, /*                       signed I32 -> F64 */
      Iop_I64StoF64, /* IRRoundingMode(I32) x signed I64 -> F64 */
      Iop_I64UtoF64, /* IRRoundingMode(I32) x unsigned I64 -> F64 */
      Iop_I64UtoF32, /* IRRoundingMode(I32) x unsigned I64 -> F32 */

      Iop_I32UtoF32, /* IRRoundingMode(I32) x unsigned I32 -> F32 */
      Iop_I32UtoF64, /*                       unsigned I32 -> F64 */

      Iop_F32toI32S, /* IRRoundingMode(I32) x F32 -> signed I32 */
      Iop_F32toI64S, /* IRRoundingMode(I32) x F32 -> signed I64 */
      Iop_F32toI32U, /* IRRoundingMode(I32) x F32 -> unsigned I32 */
      Iop_F32toI64U, /* IRRoundingMode(I32) x F32 -> unsigned I64 */

      Iop_I32StoF32, /* IRRoundingMode(I32) x signed I32 -> F32 */
      Iop_I64StoF32, /* IRRoundingMode(I32) x signed I64 -> F32 */

      /* Conversion between floating point formats */
      Iop_F32toF64,  /*                       F32 -> F64 */
      Iop_F64toF32,  /* IRRoundingMode(I32) x F64 -> F32 */

      /* Reinterpretation.  Take an F64 and produce an I64 with 
         the same bit pattern, or vice versa. */
      Iop_ReinterpF64asI64, Iop_ReinterpI64asF64,
      Iop_ReinterpF32asI32, Iop_ReinterpI32asF32,

      /* Support for 128-bit floating point */
      Iop_F64HLtoF128,/* (high half of F128,low half of F128) -> F128 */
      Iop_F128HItoF64,/* F128 -> high half of F128 into a F64 register */
      Iop_F128LOtoF64,/* F128 -> low  half of F128 into a F64 register */

      /* :: IRRoundingMode(I32) x F128 x F128 -> F128 */
      Iop_AddF128, Iop_SubF128, Iop_MulF128, Iop_DivF128,
      Iop_MAddF128,    // (A * B) + C
      Iop_MSubF128,    // (A * B) - C
      Iop_NegMAddF128, // -((A * B) + C)
      Iop_NegMSubF128, // -((A * B) - C)

      /* :: F128 -> F128 */
      Iop_NegF128, Iop_AbsF128,

      /* :: IRRoundingMode(I32) x F128 -> F128 */
      Iop_SqrtF128,

      Iop_I32StoF128, /*                signed I32  -> F128 */
      Iop_I64StoF128, /*                signed I64  -> F128 */
      Iop_I32UtoF128, /*              unsigned I32  -> F128 */
      Iop_I64UtoF128, /*              unsigned I64  -> F128 */
      Iop_F32toF128,  /*                       F32  -> F128 */
      Iop_F64toF128,  /*                       F64  -> F128 */

      Iop_F128toI32S, /* IRRoundingMode(I32) x F128 -> signed I32  */
      Iop_F128toI64S, /* IRRoundingMode(I32) x F128 -> signed I64  */
      Iop_F128toI32U, /* IRRoundingMode(I32) x F128 -> unsigned I32  */
      Iop_F128toI64U, /* IRRoundingMode(I32) x F128 -> unsigned I64  */
      Iop_F128toI128S,/* IRRoundingMode(I32) x F128 -> signed I128 */
      Iop_F128toF64,  /* IRRoundingMode(I32) x F128 -> F64         */
      Iop_F128toF32,  /* IRRoundingMode(I32) x F128 -> F32         */
      Iop_RndF128,    /* IRRoundingMode(I32) x F128 -> F128         */

      /* Truncate to the specified value, source and result
       * are stroed in a F128 register.
       */
      Iop_TruncF128toI32S,  /* truncate F128 -> I32         */
      Iop_TruncF128toI32U,  /* truncate F128 -> I32         */
      Iop_TruncF128toI64U,  /* truncate F128 -> I64         */
      Iop_TruncF128toI64S,  /* truncate F128 -> I64         */

      /* --- guest x86/amd64 specifics, not mandated by 754. --- */

      /* Binary ops, with rounding. */
      /* :: IRRoundingMode(I32) x F64 x F64 -> F64 */ 
      Iop_AtanF64,       /* FPATAN,  arctan(arg1/arg2)       */
      Iop_Yl2xF64,       /* FYL2X,   arg1 * log2(arg2)       */
      Iop_Yl2xp1F64,     /* FYL2XP1, arg1 * log2(arg2+1.0)   */
      Iop_PRemF64,       /* FPREM,   non-IEEE remainder(arg1/arg2)    */
      Iop_PRemC3210F64,  /* C3210 flags resulting from FPREM, :: I32 */
      Iop_PRem1F64,      /* FPREM1,  IEEE remainder(arg1/arg2)    */
      Iop_PRem1C3210F64, /* C3210 flags resulting from FPREM1, :: I32 */
      Iop_ScaleF64,      /* FSCALE,  arg1 * (2^RoundTowardsZero(arg2)) */
      /* Note that on x86 guest, PRem1{C3210} has the same behaviour
         as the IEEE mandated RemF64, except it is limited in the
         range of its operand.  Hence the partialness. */

      /* Unary ops, with rounding. */
      /* :: IRRoundingMode(I32) x F64 -> F64 */
      Iop_SinF64,    /* FSIN */
      Iop_CosF64,    /* FCOS */
      Iop_TanF64,    /* FTAN */
      Iop_2xm1F64,   /* (2^arg - 1.0) */
      Iop_RoundF128toInt, /* F128 value to nearest integral value (still
                             as F128) */
      Iop_RoundF64toInt, /* F64 value to nearest integral value (still
                            as F64) */
      Iop_RoundF32toInt, /* F32 value to nearest integral value (still
                            as F32) */

      /* --- guest s390 specifics, not mandated by 754. --- */

      /* Fused multiply-add/sub */
      /* :: IRRoundingMode(I32) x F32 x F32 x F32 -> F32
            (computes arg2 * arg3 +/- arg4) */ 
      Iop_MAddF32, Iop_MSubF32,

      /* --- guest ppc32/64 specifics, not mandated by 754. --- */

      /* Ternary operations, with rounding. */
      /* Fused multiply-add/sub, with 112-bit intermediate
         precision for ppc.
         Also used to implement fused multiply-add/sub for s390. */
      /* :: IRRoundingMode(I32) x F64 x F64 x F64 -> F64 
            (computes arg2 * arg3 +/- arg4) */ 
      Iop_MAddF64, Iop_MSubF64,

      /* Variants of the above which produce a 64-bit result but which
         round their result to a IEEE float range first. */
      /* :: IRRoundingMode(I32) x F64 x F64 x F64 -> F64 */ 
      Iop_MAddF64r32, Iop_MSubF64r32,

      /* :: F64 -> F64 */
      Iop_RSqrtEst5GoodF64, /* reciprocal square root estimate, 5 good bits */
      Iop_RoundF64toF64_NEAREST, /* frin */
      Iop_RoundF64toF64_NegINF,  /* frim */ 
      Iop_RoundF64toF64_PosINF,  /* frip */
      Iop_RoundF64toF64_ZERO,    /* friz */

      /* :: F64 -> F32 */
      Iop_TruncF64asF32, /* do F64->F32 truncation as per 'fsts' */

      /* :: IRRoundingMode(I32) x F64 -> F64 */
      Iop_RoundF64toF32, /* round F64 to nearest F32 value (still as F64) */
      /* NB: pretty much the same as Iop_F64toF32, except no change 
         of type. */

      /* --- guest arm64 specifics, not mandated by 754. --- */

      Iop_RecpExpF64,  /* FRECPX d  :: IRRoundingMode(I32) x F64 -> F64 */
      Iop_RecpExpF32,  /* FRECPX s  :: IRRoundingMode(I32) x F32 -> F32 */

      /* --------- Possibly required by IEEE 754-2008. --------- */

      Iop_MaxNumF64,  /* max, F64, numerical operand if other is a qNaN */
      Iop_MinNumF64,  /* min, F64, ditto */
      Iop_MaxNumF32,  /* max, F32, ditto */
      Iop_MinNumF32,  /* min, F32, ditto */

      /* ------------------ 16-bit scalar FP ------------------ */

      Iop_F16toF64,  /*                       F16 -> F64 */
      Iop_F64toF16,  /* IRRoundingMode(I32) x F64 -> F16 */

      Iop_F16toF32,  /*                       F16 -> F32 */
      Iop_F32toF16,  /* IRRoundingMode(I32) x F32 -> F16 */

      /* ------------------ 32-bit SIMD Integer ------------------ */

      /* 32x1 saturating add/sub (ok, well, not really SIMD :) */
      Iop_QAdd32S,
      Iop_QSub32S,

      /* 16x2 add/sub, also signed/unsigned saturating variants */
      Iop_Add16x2, Iop_Sub16x2,
      Iop_QAdd16Sx2, Iop_QAdd16Ux2,
      Iop_QSub16Sx2, Iop_QSub16Ux2,

      /* 16x2 signed/unsigned halving add/sub.  For each lane, these
         compute bits 16:1 of (eg) sx(argL) + sx(argR),
         or zx(argL) - zx(argR) etc. */
      Iop_HAdd16Ux2, Iop_HAdd16Sx2,
      Iop_HSub16Ux2, Iop_HSub16Sx2,

      /* 8x4 add/sub, also signed/unsigned saturating variants */
      Iop_Add8x4, Iop_Sub8x4,
      Iop_QAdd8Sx4, Iop_QAdd8Ux4,
      Iop_QSub8Sx4, Iop_QSub8Ux4,

      /* 8x4 signed/unsigned halving add/sub.  For each lane, these
         compute bits 8:1 of (eg) sx(argL) + sx(argR),
         or zx(argL) - zx(argR) etc. */
      Iop_HAdd8Ux4, Iop_HAdd8Sx4,
      Iop_HSub8Ux4, Iop_HSub8Sx4,

      /* 8x4 sum of absolute unsigned differences. */
      Iop_Sad8Ux4,

      /* MISC (vector integer cmp != 0) */
      Iop_CmpNEZ16x2, Iop_CmpNEZ8x4,

      /* Byte swap in a 32-bit word */
      Iop_Reverse8sIn32_x1,

      /* ------------------ 64-bit SIMD FP ------------------------ */

      /* Conversion to/from int */
      // Deprecated: these don't specify a rounding mode
      Iop_I32UtoF32x2_DEP,  Iop_I32StoF32x2_DEP,    /* I32x2 -> F32x2 */

      Iop_F32toI32Ux2_RZ,  Iop_F32toI32Sx2_RZ,    /* F32x2 -> I32x2 */

      /* Fixed32 format is floating-point number with fixed number of fraction
         bits. The number of fraction bits is passed as a second argument of
         type I8. */
      Iop_F32ToFixed32Ux2_RZ, Iop_F32ToFixed32Sx2_RZ, /* fp -> fixed-point */
      Iop_Fixed32UToF32x2_RN, Iop_Fixed32SToF32x2_RN, /* fixed-point -> fp */

      /* Binary operations */
      Iop_Max32Fx2,      Iop_Min32Fx2,
      /* Pairwise Min and Max. See integer pairwise operations for more
         details. */
      Iop_PwMax32Fx2,    Iop_PwMin32Fx2,
      /* Note: For the following compares, the arm front-end assumes a
         nan in a lane of either argument returns zero for that lane. */
      Iop_CmpEQ32Fx2, Iop_CmpGT32Fx2, Iop_CmpGE32Fx2,

      /* Vector Reciprocal Estimate finds an approximate reciprocal of each
      element in the operand vector, and places the results in the destination
      vector.  */
      Iop_RecipEst32Fx2,

      /* Vector Reciprocal Step computes (2.0 - arg1 * arg2).
         Note, that if one of the arguments is zero and another one is infinity
         of arbitrary sign the result of the operation is 2.0. */
      Iop_RecipStep32Fx2,

      /* Vector Reciprocal Square Root Estimate finds an approximate reciprocal
         square root of each element in the operand vector. */
      Iop_RSqrtEst32Fx2,

      /* Vector Reciprocal Square Root Step computes (3.0 - arg1 * arg2) / 2.0.
         Note, that of one of the arguments is zero and another one is infiinty
         of arbitrary sign the result of the operation is 1.5. */
      Iop_RSqrtStep32Fx2,

      /* Unary */
      Iop_Neg32Fx2, Iop_Abs32Fx2,

      /* ------------------ 64-bit SIMD Integer. ------------------ */

      /* MISC (vector integer cmp != 0) */
      Iop_CmpNEZ8x8, Iop_CmpNEZ16x4, Iop_CmpNEZ32x2,

      /* ADDITION (normal / unsigned sat / signed sat) */
      Iop_Add8x8,   Iop_Add16x4,   Iop_Add32x2,
      Iop_QAdd8Ux8, Iop_QAdd16Ux4, Iop_QAdd32Ux2, Iop_QAdd64Ux1,
      Iop_QAdd8Sx8, Iop_QAdd16Sx4, Iop_QAdd32Sx2, Iop_QAdd64Sx1,

      /* PAIRWISE operations */
      /* Iop_PwFoo16x4( [a,b,c,d], [e,f,g,h] ) =
            [Foo16(a,b), Foo16(c,d), Foo16(e,f), Foo16(g,h)] */
      Iop_PwAdd8x8,  Iop_PwAdd16x4,  Iop_PwAdd32x2,
      Iop_PwMax8Sx8, Iop_PwMax16Sx4, Iop_PwMax32Sx2,
      Iop_PwMax8Ux8, Iop_PwMax16Ux4, Iop_PwMax32Ux2,
      Iop_PwMin8Sx8, Iop_PwMin16Sx4, Iop_PwMin32Sx2,
      Iop_PwMin8Ux8, Iop_PwMin16Ux4, Iop_PwMin32Ux2,
      /* Longening variant is unary. The resulting vector contains two times
         less elements than operand, but they are two times wider.
         Example:
            Iop_PAddL16Ux4( [a,b,c,d] ) = [a+b,c+d]
               where a+b and c+d are unsigned 32-bit values. */
      Iop_PwAddL8Ux8, Iop_PwAddL16Ux4, Iop_PwAddL32Ux2,
      Iop_PwAddL8Sx8, Iop_PwAddL16Sx4, Iop_PwAddL32Sx2,

      /* SUBTRACTION (normal / unsigned sat / signed sat) */
      Iop_Sub8x8,   Iop_Sub16x4,   Iop_Sub32x2,
      Iop_QSub8Ux8, Iop_QSub16Ux4, Iop_QSub32Ux2, Iop_QSub64Ux1,
      Iop_QSub8Sx8, Iop_QSub16Sx4, Iop_QSub32Sx2, Iop_QSub64Sx1,

      /* ABSOLUTE VALUE */
      Iop_Abs8x8, Iop_Abs16x4, Iop_Abs32x2,

      /* MULTIPLICATION (normal / high half of signed/unsigned / plynomial ) */
      Iop_Mul8x8, Iop_Mul16x4, Iop_Mul32x2,
      Iop_Mul32Fx2,
      Iop_MulHi16Ux4,
      Iop_MulHi16Sx4,
      /* Plynomial multiplication treats it's arguments as coefficients of
         polynoms over {0, 1}. */
      Iop_PolynomialMul8x8,

      /* Vector Saturating Doubling Multiply Returning High Half and
         Vector Saturating Rounding Doubling Multiply Returning High Half */
      /* These IROp's multiply corresponding elements in two vectors, double
         the results, and place the most significant half of the final results
         in the destination vector. The results are truncated or rounded. If
         any of the results overflow, they are saturated. */
      Iop_QDMulHi16Sx4, Iop_QDMulHi32Sx2,
      Iop_QRDMulHi16Sx4, Iop_QRDMulHi32Sx2,

      /* AVERAGING: note: (arg1 + arg2 + 1) >>u 1 */
      Iop_Avg8Ux8,
      Iop_Avg16Ux4,

      /* MIN/MAX */
      Iop_Max8Sx8, Iop_Max16Sx4, Iop_Max32Sx2,
      Iop_Max8Ux8, Iop_Max16Ux4, Iop_Max32Ux2,
      Iop_Min8Sx8, Iop_Min16Sx4, Iop_Min32Sx2,
      Iop_Min8Ux8, Iop_Min16Ux4, Iop_Min32Ux2,

      /* COMPARISON */
      Iop_CmpEQ8x8,  Iop_CmpEQ16x4,  Iop_CmpEQ32x2,
      Iop_CmpGT8Ux8, Iop_CmpGT16Ux4, Iop_CmpGT32Ux2,
      Iop_CmpGT8Sx8, Iop_CmpGT16Sx4, Iop_CmpGT32Sx2,

      /* COUNT ones / leading zeroes / leading sign bits (not including topmost
         bit) */
      Iop_Cnt8x8,
      Iop_Clz8x8, Iop_Clz16x4, Iop_Clz32x2,
      Iop_Cls8x8, Iop_Cls16x4, Iop_Cls32x2,
      Iop_Clz64x2,

      /*Vector COUNT trailing zeros */
      Iop_Ctz8x16, Iop_Ctz16x8, Iop_Ctz32x4, Iop_Ctz64x2, 

      /* VECTOR x VECTOR SHIFT / ROTATE */
      Iop_Shl8x8, Iop_Shl16x4, Iop_Shl32x2,
      Iop_Shr8x8, Iop_Shr16x4, Iop_Shr32x2,
      Iop_Sar8x8, Iop_Sar16x4, Iop_Sar32x2,
      Iop_Sal8x8, Iop_Sal16x4, Iop_Sal32x2, Iop_Sal64x1,

      /* VECTOR x SCALAR SHIFT (shift amt :: Ity_I8) */
      Iop_ShlN8x8, Iop_ShlN16x4, Iop_ShlN32x2,
      Iop_ShrN8x8, Iop_ShrN16x4, Iop_ShrN32x2,
      Iop_SarN8x8, Iop_SarN16x4, Iop_SarN32x2,

      /* VECTOR x VECTOR SATURATING SHIFT */
      Iop_QShl8x8, Iop_QShl16x4, Iop_QShl32x2, Iop_QShl64x1,
      Iop_QSal8x8, Iop_QSal16x4, Iop_QSal32x2, Iop_QSal64x1,
      /* VECTOR x INTEGER SATURATING SHIFT */
      Iop_QShlNsatSU8x8,  Iop_QShlNsatSU16x4,
      Iop_QShlNsatSU32x2, Iop_QShlNsatSU64x1,
      Iop_QShlNsatUU8x8,  Iop_QShlNsatUU16x4,
      Iop_QShlNsatUU32x2, Iop_QShlNsatUU64x1,
      Iop_QShlNsatSS8x8,  Iop_QShlNsatSS16x4,
      Iop_QShlNsatSS32x2, Iop_QShlNsatSS64x1,

      /* NARROWING (binary) 
         -- narrow 2xI64 into 1xI64, hi half from left arg */
      /* For saturated narrowing, I believe there are 4 variants of
         the basic arithmetic operation, depending on the signedness
         of argument and result.  Here are examples that exemplify
         what I mean:

         QNarrow16Uto8U ( UShort x )  if (x >u 255) x = 255;
                                      return x[7:0];

         QNarrow16Sto8S ( Short x )   if (x <s -128) x = -128;
                                      if (x >s  127) x = 127;
                                      return x[7:0];

         QNarrow16Uto8S ( UShort x )  if (x >u 127) x = 127;
                                      return x[7:0];

         QNarrow16Sto8U ( Short x )   if (x <s 0)   x = 0;
                                      if (x >s 255) x = 255;
                                      return x[7:0];
      */
      Iop_QNarrowBin16Sto8Ux8,
      Iop_QNarrowBin16Sto8Sx8, Iop_QNarrowBin32Sto16Sx4,
      Iop_NarrowBin16to8x8,    Iop_NarrowBin32to16x4,

      /* INTERLEAVING */
      /* Interleave lanes from low or high halves of
         operands.  Most-significant result lane is from the left
         arg. */
      Iop_InterleaveHI8x8, Iop_InterleaveHI16x4, Iop_InterleaveHI32x2,
      Iop_InterleaveLO8x8, Iop_InterleaveLO16x4, Iop_InterleaveLO32x2,
      /* Interleave odd/even lanes of operands.  Most-significant result lane
         is from the left arg.  Note that Interleave{Odd,Even}Lanes32x2 are
         identical to Interleave{HI,LO}32x2 and so are omitted.*/
      Iop_InterleaveOddLanes8x8, Iop_InterleaveEvenLanes8x8,
      Iop_InterleaveOddLanes16x4, Iop_InterleaveEvenLanes16x4,

      /* CONCATENATION -- build a new value by concatenating either
         the even or odd lanes of both operands.  Note that
         Cat{Odd,Even}Lanes32x2 are identical to Interleave{HI,LO}32x2
         and so are omitted. */
      Iop_CatOddLanes8x8, Iop_CatOddLanes16x4,
      Iop_CatEvenLanes8x8, Iop_CatEvenLanes16x4,

      /* GET / SET elements of VECTOR
         GET is binop (I64, I8) -> I<elem_size>
         SET is triop (I64, I8, I<elem_size>) -> I64 */
      /* Note: the arm back-end handles only constant second argument */
      Iop_GetElem8x8, Iop_GetElem16x4, Iop_GetElem32x2,
      Iop_SetElem8x8, Iop_SetElem16x4, Iop_SetElem32x2,

      /* DUPLICATING -- copy value to all lanes */
      Iop_Dup8x8,   Iop_Dup16x4,   Iop_Dup32x2,

      /* SLICE -- produces the lowest 64 bits of (arg1:arg2) >> (8 * arg3).
         arg3 is a shift amount in bytes and may be between 0 and 8
         inclusive.  When 0, the result is arg2; when 8, the result is arg1.
         Not all back ends handle all values.  The arm32 and arm64 back
         ends handle only immediate arg3 values. */
      Iop_Slice64,  // (I64, I64, I8) -> I64

      /* REVERSE the order of chunks in vector lanes.  Chunks must be
         smaller than the vector lanes (obviously) and so may be 8-, 16- and
         32-bit in size.  Note that the degenerate case,
         Iop_Reverse8sIn64_x1, is a simply a vanilla byte-swap. */
      /* Examples:
            Reverse8sIn16_x4([a,b,c,d,e,f,g,h]) = [b,a,d,c,f,e,h,g]
            Reverse8sIn32_x2([a,b,c,d,e,f,g,h]) = [d,c,b,a,h,g,f,e]
            Reverse8sIn64_x1([a,b,c,d,e,f,g,h]) = [h,g,f,e,d,c,b,a] */
      Iop_Reverse8sIn16_x4,
      Iop_Reverse8sIn32_x2, Iop_Reverse16sIn32_x2,
      Iop_Reverse8sIn64_x1, Iop_Reverse16sIn64_x1, Iop_Reverse32sIn64_x1,

      /* PERMUTING -- copy src bytes to dst,
         as indexed by control vector bytes:
            for i in 0 .. 7 . result[i] = argL[ argR[i] ] 
         argR[i] values may only be in the range 0 .. 7, else behaviour
         is undefined.  That is, argR[i][7:3] must be zero. */
      Iop_Perm8x8,

      /* PERMUTING with optional zeroing:
            for i in 0 .. 7 . result[i] = if argR[i] bit 7 is set
                                          then zero else argL[ argR[i] ]
         argR[i][6:3] must be zero, else behaviour is undefined.
      */
      Iop_PermOrZero8x8,

      /* MISC CONVERSION -- get high bits of each byte lane, a la
         x86/amd64 pmovmskb */
      Iop_GetMSBs8x8, /* I64 -> I8 */

      /* Vector Reciprocal Estimate and Vector Reciprocal Square Root Estimate
         See floating-point equivalents for details. */
      Iop_RecipEst32Ux2, Iop_RSqrtEst32Ux2,

      /* ------------------ Decimal Floating Point ------------------ */

      /* ARITHMETIC INSTRUCTIONS   64-bit
	 ----------------------------------
	 IRRoundingMode(I32) X D64 X D64 -> D64
      */
      Iop_AddD64, Iop_SubD64, Iop_MulD64, Iop_DivD64,

      /* ARITHMETIC INSTRUCTIONS  128-bit
	 ----------------------------------
	 IRRoundingMode(I32) X D128 X D128 -> D128
      */
      Iop_AddD128, Iop_SubD128, Iop_MulD128, Iop_DivD128,

      /* SHIFT SIGNIFICAND INSTRUCTIONS
       *    The DFP significand is shifted by the number of digits specified
       *    by the U8 operand.  Digits shifted out of the leftmost digit are
       *    lost. Zeros are supplied to the vacated positions on the right.
       *    The sign of the result is the same as the sign of the original
       *    operand.
       *
       * D64 x U8  -> D64    left shift and right shift respectively */
      Iop_ShlD64, Iop_ShrD64,

      /* D128 x U8  -> D128  left shift and right shift respectively */
      Iop_ShlD128, Iop_ShrD128,


      /* FORMAT CONVERSION INSTRUCTIONS
       *   D32 -> D64
       */
      Iop_D32toD64,

      /*   D64 -> D128 */
      Iop_D64toD128, 

      /*   I32S -> D128 */
      Iop_I32StoD128,

      /*   I32U -> D128 */
      Iop_I32UtoD128,

      /*   I64S -> D128 */
      Iop_I64StoD128, 

      /*   I64U -> D128 */
      Iop_I64UtoD128,

      /*   IRRoundingMode(I32) x D64 -> D32 */
      Iop_D64toD32,

      /*   IRRoundingMode(I32) x D128 -> D64 */
      Iop_D128toD64,

      /*   I32S -> D64 */
      Iop_I32StoD64,

      /*   I32U -> D64 */
      Iop_I32UtoD64,

      /*   IRRoundingMode(I32) x I64 -> D64 */
      Iop_I64StoD64,

      /*   IRRoundingMode(I32) x I64 -> D64 */
      Iop_I64UtoD64,

      /*   IRRoundingMode(I32) x D64 -> I32 */
      Iop_D64toI32S,

      /*   IRRoundingMode(I32) x D64 -> I32 */
      Iop_D64toI32U,

      /*   IRRoundingMode(I32) x D64 -> I64 */
      Iop_D64toI64S,

      /*   IRRoundingMode(I32) x D64 -> I64 */
      Iop_D64toI64U,

      /*   IRRoundingMode(I32) x D128 -> I32 */
      Iop_D128toI32S,

      /*   IRRoundingMode(I32) x D128 -> I32 */
      Iop_D128toI32U,

      /*   IRRoundingMode(I32) x D128 -> I64 */
      Iop_D128toI64S,

      /*   IRRoundingMode(I32) x D128 -> I64 */
      Iop_D128toI64U,

      /*   IRRoundingMode(I32) x F32 -> D32 */
      Iop_F32toD32,

      /*   IRRoundingMode(I32) x F32 -> D64 */
      Iop_F32toD64,

      /*   IRRoundingMode(I32) x F32 -> D128 */
      Iop_F32toD128,

      /*   IRRoundingMode(I32) x F64 -> D32 */
      Iop_F64toD32,

      /*   IRRoundingMode(I32) x F64 -> D64 */
      Iop_F64toD64,

      /*   IRRoundingMode(I32) x F64 -> D128 */
      Iop_F64toD128,

      /*   IRRoundingMode(I32) x F128 -> D32 */
      Iop_F128toD32,

      /*   IRRoundingMode(I32) x F128 -> D64 */
      Iop_F128toD64,

      /*   IRRoundingMode(I32) x F128 -> D128 */
      Iop_F128toD128,

      /*   IRRoundingMode(I32) x D32 -> F32 */
      Iop_D32toF32,

      /*   IRRoundingMode(I32) x D32 -> F64 */
      Iop_D32toF64,

      /*   IRRoundingMode(I32) x D32 -> F128 */
      Iop_D32toF128,

      /*   IRRoundingMode(I32) x D64 -> F32 */
      Iop_D64toF32,

      /*   IRRoundingMode(I32) x D64 -> F64 */
      Iop_D64toF64,

      /*   IRRoundingMode(I32) x D64 -> F128 */
      Iop_D64toF128,

      /*   IRRoundingMode(I32) x D128 -> F32 */
      Iop_D128toF32,

      /*   IRRoundingMode(I32) x D128 -> F64 */
      Iop_D128toF64,

      /*   IRRoundingMode(I32) x D128 -> F128 */
      Iop_D128toF128,

      /* ROUNDING INSTRUCTIONS
       * IRRoundingMode(I32) x D64 -> D64
       * The D64 operand, if a finite number, it is rounded to a
       * floating point integer value, i.e. no fractional part.
       */
      Iop_RoundD64toInt,

      /* IRRoundingMode(I32) x D128 -> D128 */
      Iop_RoundD128toInt,

      /* COMPARE INSTRUCTIONS
       * D64 x D64 -> IRCmpD64Result(I32) */
      Iop_CmpD64,

      /* D128 x D128 -> IRCmpD128Result(I32) */
      Iop_CmpD128,

      /* COMPARE BIASED EXPONENET INSTRUCTIONS
       * D64 x D64 -> IRCmpD64Result(I32) */
      Iop_CmpExpD64,

      /* D128 x D128 -> IRCmpD128Result(I32) */
      Iop_CmpExpD128,

      /* QUANTIZE AND ROUND INSTRUCTIONS
       * The source operand is converted and rounded to the form with the 
       * immediate exponent specified by the rounding and exponent parameter.
       *
       * The second operand is converted and rounded to the form
       * of the first operand's exponent and the rounded based on the specified
       * rounding mode parameter.
       *
       * IRRoundingMode(I32) x D64 x D64-> D64 */
      Iop_QuantizeD64,

      /* IRRoundingMode(I32) x D128 x D128 -> D128 */
      Iop_QuantizeD128,

      /* IRRoundingMode(I32) x I8 x D64 -> D64
       *    The Decimal Floating point operand is rounded to the requested 
       *    significance given by the I8 operand as specified by the rounding 
       *    mode.
       */
      Iop_SignificanceRoundD64,

      /* IRRoundingMode(I32) x I8 x D128 -> D128 */
      Iop_SignificanceRoundD128,

      /* EXTRACT AND INSERT INSTRUCTIONS
       * D64 -> I64
       *    The exponent of the D32 or D64 operand is extracted.  The 
       *    extracted exponent is converted to a 64-bit signed binary integer.
       */
      Iop_ExtractExpD64,

      /* D128 -> I64 */
      Iop_ExtractExpD128,

      /* D64 -> I64
       * The number of significand digits of the D64 operand is extracted.
       * The number is stored as a 64-bit signed binary integer.
       */
      Iop_ExtractSigD64,

      /* D128 -> I64 */
      Iop_ExtractSigD128,

      /* I64 x D64  -> D64
       *    The exponent is specified by the first I64 operand the signed
       *    significand is given by the second I64 value.  The result is a D64
       *    value consisting of the specified significand and exponent whose 
       *    sign is that of the specified significand.
       */
      Iop_InsertExpD64,

      /* I64 x D128 -> D128 */
      Iop_InsertExpD128,

      /* Support for 128-bit DFP type */
      Iop_D64HLtoD128, Iop_D128HItoD64, Iop_D128LOtoD64,

      /*  I64 -> I64  
       *     Convert 50-bit densely packed BCD string to 60 bit BCD string
       */
      Iop_DPBtoBCD,

      /* I64 -> I64
       *     Convert 60 bit BCD string to 50-bit densely packed BCD string
       */
      Iop_BCDtoDPB,

      /* BCD arithmetic instructions, (V128, V128) -> V128
       * The BCD format is the same as that used in the BCD<->DPB conversion
       * routines, except using 124 digits (vs 60) plus the trailing 4-bit
       * signed code. */
      Iop_BCDAdd, Iop_BCDSub,

      /* Conversion signed 128-bit integer to signed BCD 128-bit */
      Iop_I128StoBCD128,

      /* Conversion signed BCD 128-bit to 128-bit integer */
      Iop_BCD128toI128S,

      /* Conversion I64 -> D64 */
      Iop_ReinterpI64asD64,

      /* Conversion D64 -> I64 */
      Iop_ReinterpD64asI64,

      /* ------------------ 128-bit SIMD FP. ------------------ */

      /* --- 32x4 vector FP --- */

      /* ternary :: IRRoundingMode(I32) x V128 x V128 -> V128 */
      Iop_Add32Fx4, Iop_Sub32Fx4, Iop_Mul32Fx4, Iop_Div32Fx4, 

      /* binary */
      Iop_Max32Fx4, Iop_Min32Fx4,
      Iop_Add32Fx2, Iop_Sub32Fx2,
      /* Note: For the following compares, the ppc and arm front-ends assume a
         nan in a lane of either argument returns zero for that lane. */
      Iop_CmpEQ32Fx4, Iop_CmpLT32Fx4, Iop_CmpLE32Fx4, Iop_CmpUN32Fx4,
      Iop_CmpGT32Fx4, Iop_CmpGE32Fx4,

      /* Pairwise Max and Min. See integer pairwise operations for details. */
      Iop_PwMax32Fx4, Iop_PwMin32Fx4,

      /* unary */
      Iop_Abs32Fx4,
      Iop_Neg32Fx4,

      /* binary :: IRRoundingMode(I32) x V128 -> V128 */
      Iop_Sqrt32Fx4,

      /* Vector Reciprocal Estimate finds an approximate reciprocal of each
         element in the operand vector, and places the results in the
         destination vector.  */
      Iop_RecipEst32Fx4,

      /* Vector Reciprocal Step computes (2.0 - arg1 * arg2).
         Note, that if one of the arguments is zero and another one is infinity
         of arbitrary sign the result of the operation is 2.0. */
      Iop_RecipStep32Fx4,

      /* Vector Reciprocal Square Root Estimate finds an approximate reciprocal
         square root of each element in the operand vector. */
      Iop_RSqrtEst32Fx4,

      /* Scaling of vector with a power of 2  (wd[i] <- ws[i] * 2^wt[i]) */
      Iop_Scale2_32Fx4,

      /* Vector floating-point base 2 logarithm */
      Iop_Log2_32Fx4,

      /* Vector floating-point exponential 2^x */
      Iop_Exp2_32Fx4,

      /* Vector Reciprocal Square Root Step computes (3.0 - arg1 * arg2) / 2.0.
         Note, that of one of the arguments is zero and another one is infiinty
         of arbitrary sign the result of the operation is 1.5. */
      Iop_RSqrtStep32Fx4,

      /* --- Int to/from FP conversion --- */
      /* Unlike the standard fp conversions, these irops take no
         rounding mode argument. Instead the irop trailers _R{M,P,N,Z}
         indicate the mode: {-inf, +inf, nearest, zero} respectively. */

      // These carry no rounding mode and are therefore deprecated
      Iop_I32UtoF32x4_DEP, Iop_I32StoF32x4_DEP,  /* I32x4 -> F32x4 */

      Iop_I32StoF32x4, /* IRRoundingMode(I32) x V128 -> V128 */
      Iop_F32toI32Sx4, /* IRRoundingMode(I32) x V128 -> V128 */

      Iop_F32toI32Ux4_RZ,  Iop_F32toI32Sx4_RZ,  /* F32x4 -> I32x4       */
      Iop_QF32toI32Ux4_RZ, Iop_QF32toI32Sx4_RZ, /* F32x4 -> I32x4 (saturating) */
      Iop_RoundF32x4_RM, Iop_RoundF32x4_RP,   /* round to fp integer  */
      Iop_RoundF32x4_RN, Iop_RoundF32x4_RZ,   /* round to fp integer  */
      /* Fixed32 format is floating-point number with fixed number of fraction
         bits. The number of fraction bits is passed as a second argument of
         type I8. */
      Iop_F32ToFixed32Ux4_RZ, Iop_F32ToFixed32Sx4_RZ, /* fp -> fixed-point */
      Iop_Fixed32UToF32x4_RN, Iop_Fixed32SToF32x4_RN, /* fixed-point -> fp */

      /* --- Single to/from half conversion --- */
      /* FIXME: what kind of rounding in F32x4 -> F16x4 case? */
      // FIXME these carry no rounding mode
      Iop_F32toF16x4_DEP, /* F32x4(==V128) -> F16x4(==I64), NO ROUNDING MODE */
      Iop_F32toF16x4,     /* IRRoundingMode(I32) x V128 -> I64 */
      Iop_F16toF32x4,     /* F16x4 -> F32x4 */

      /* -- Double to/from half conversion -- */
      Iop_F64toF16x2_DEP, // F64x2 -> F16x2, NO ROUNDING MODE
      Iop_F16toF64x2,

      /* Values from two registers converted in smaller type and put in one
       IRRoundingMode(I32) x (F32x4 | F32x4) -> Q16x8 */
      Iop_F32x4_2toQ16x8,


      /* --- 32x4 lowest-lane-only scalar FP --- */

      /* In binary cases, upper 3/4 is copied from first operand.  In
         unary cases, upper 3/4 is copied from the operand. */

      /* binary */
      Iop_Add32F0x4, Iop_Sub32F0x4, Iop_Mul32F0x4, Iop_Div32F0x4, 
      Iop_Max32F0x4, Iop_Min32F0x4,
      Iop_CmpEQ32F0x4, Iop_CmpLT32F0x4, Iop_CmpLE32F0x4, Iop_CmpUN32F0x4, 

      /* unary */
      Iop_RecipEst32F0x4, Iop_Sqrt32F0x4, Iop_RSqrtEst32F0x4,

      /* --- 64x2 vector FP --- */

      /* ternary :: IRRoundingMode(I32) x V128 x V128 -> V128 */
      Iop_Add64Fx2, Iop_Sub64Fx2, Iop_Mul64Fx2, Iop_Div64Fx2, 

      /* binary */
      Iop_Max64Fx2, Iop_Min64Fx2,
      Iop_CmpEQ64Fx2, Iop_CmpLT64Fx2, Iop_CmpLE64Fx2, Iop_CmpUN64Fx2, 

      /* unary */
      Iop_Abs64Fx2,
      Iop_Neg64Fx2,

      /* binary :: IRRoundingMode(I32) x V128 -> V128 */
      Iop_Sqrt64Fx2,

      /* Scaling of vector with a power of 2  (wd[i] <- ws[i] * 2^wt[i]) */
      Iop_Scale2_64Fx2,

      /* Vector floating-point base 2 logarithm */
      Iop_Log2_64Fx2,

      /* see 32Fx4 variants for description */
      Iop_RecipEst64Fx2,    // unary
      Iop_RecipStep64Fx2,   // binary
      Iop_RSqrtEst64Fx2,    // unary
      Iop_RSqrtStep64Fx2,   // binary


      /* Values from two registers converted in smaller type and put in one
       IRRoundingMode(I32) x (F64x2 | F64x2) -> Q32x4 */
      Iop_F64x2_2toQ32x4,

      /* --- 64x2 lowest-lane-only scalar FP --- */

      /* In binary cases, upper half is copied from first operand.  In
         unary cases, upper half is copied from the operand. */

      /* binary */
      Iop_Add64F0x2, Iop_Sub64F0x2, Iop_Mul64F0x2, Iop_Div64F0x2, 
      Iop_Max64F0x2, Iop_Min64F0x2,
      Iop_CmpEQ64F0x2, Iop_CmpLT64F0x2, Iop_CmpLE64F0x2, Iop_CmpUN64F0x2, 

      /* unary */
      Iop_Sqrt64F0x2,

      /* --- pack / unpack --- */

      /* 64 <-> 128 bit vector */
      Iop_V128to64,     // :: V128 -> I64, low half
      Iop_V128HIto64,   // :: V128 -> I64, high half
      Iop_64HLtoV128,   // :: (I64,I64) -> V128

      Iop_64UtoV128,
      Iop_SetV128lo64,

      /* Copies lower 64/32/16/8 bits, zeroes out the rest. */
      Iop_ZeroHI64ofV128,    // :: V128 -> V128
      Iop_ZeroHI96ofV128,    // :: V128 -> V128
      Iop_ZeroHI112ofV128,   // :: V128 -> V128
      Iop_ZeroHI120ofV128,   // :: V128 -> V128

      /* 32 <-> 128 bit vector */
      Iop_32UtoV128,
      Iop_V128to32,     // :: V128 -> I32, lowest lane
      Iop_SetV128lo32,  // :: (V128,I32) -> V128

      /* ------------------ 128-bit SIMD Integer. ------------------ */

      /* BITWISE OPS */
      Iop_NotV128,
      Iop_AndV128, Iop_OrV128, Iop_XorV128, 

      /* VECTOR SHIFT (shift amt :: Ity_I8) */
      Iop_ShlV128, Iop_ShrV128, Iop_SarV128,

      /* MISC (vector integer cmp != 0) */
      Iop_CmpNEZ8x16, Iop_CmpNEZ16x8, Iop_CmpNEZ32x4, Iop_CmpNEZ64x2,
      Iop_CmpNEZ128x1,

      /* ADDITION (normal / U->U sat / S->S sat) */
      Iop_Add8x16,    Iop_Add16x8,    Iop_Add32x4,    Iop_Add64x2,   Iop_Add128x1,
      Iop_QAdd8Ux16,  Iop_QAdd16Ux8,  Iop_QAdd32Ux4,  Iop_QAdd64Ux2,
      Iop_QAdd8Sx16,  Iop_QAdd16Sx8,  Iop_QAdd32Sx4,  Iop_QAdd64Sx2,

      /* ADDITION, ARM64 specific saturating variants. */
      /* Unsigned widen left arg, signed widen right arg, add, saturate S->S.
         This corresponds to SUQADD. */
      Iop_QAddExtUSsatSS8x16, Iop_QAddExtUSsatSS16x8,
      Iop_QAddExtUSsatSS32x4, Iop_QAddExtUSsatSS64x2,
      /* Signed widen left arg, unsigned widen right arg, add, saturate U->U.
         This corresponds to USQADD. */
      Iop_QAddExtSUsatUU8x16, Iop_QAddExtSUsatUU16x8,
      Iop_QAddExtSUsatUU32x4, Iop_QAddExtSUsatUU64x2,

      /* SUBTRACTION (normal / unsigned sat / signed sat) */
      Iop_Sub8x16,   Iop_Sub16x8,   Iop_Sub32x4,   Iop_Sub64x2,   Iop_Sub128x1,
      Iop_QSub8Ux16, Iop_QSub16Ux8, Iop_QSub32Ux4, Iop_QSub64Ux2,
      Iop_QSub8Sx16, Iop_QSub16Sx8, Iop_QSub32Sx4, Iop_QSub64Sx2,

      /* MULTIPLICATION (normal / high half of signed/unsigned) */
      Iop_Mul8x16,  Iop_Mul16x8,    Iop_Mul32x4,
      Iop_MulHi8Ux16, Iop_MulHi16Ux8, Iop_MulHi32Ux4,
      Iop_MulHi8Sx16, Iop_MulHi16Sx8, Iop_MulHi32Sx4,
      /* (widening signed/unsigned of even lanes, with lowest lane=zero) */
      Iop_MullEven8Ux16, Iop_MullEven16Ux8, Iop_MullEven32Ux4,
      Iop_MullEven8Sx16, Iop_MullEven16Sx8, Iop_MullEven32Sx4,

      /* Widening multiplies, all of the form (I64, I64) -> V128 */
      Iop_Mull8Ux8, Iop_Mull8Sx8,
      Iop_Mull16Ux4, Iop_Mull16Sx4,
      Iop_Mull32Ux2, Iop_Mull32Sx2,

      /* Signed doubling saturating widening multiplies, (I64, I64) -> V128 */
      Iop_QDMull16Sx4, Iop_QDMull32Sx2,

      /* Vector Saturating Doubling Multiply Returning High Half and
         Vector Saturating Rounding Doubling Multiply Returning High Half.
         These IROps multiply corresponding elements in two vectors, double
         the results, and place the most significant half of the final results
         in the destination vector.  The results are truncated or rounded.  If
         any of the results overflow, they are saturated.  To be more precise,
         for each lane, the computed result is: 
           QDMulHi:  
             hi-half( sign-extend(laneL) *q sign-extend(laneR) *q 2 )
           QRDMulHi:
             hi-half( sign-extend(laneL) *q sign-extend(laneR) *q 2
                      +q (1 << (lane-width-in-bits - 1)) )
      */
      Iop_QDMulHi16Sx8,  Iop_QDMulHi32Sx4,  /* (V128, V128) -> V128 */
      Iop_QRDMulHi16Sx8, Iop_QRDMulHi32Sx4, /* (V128, V128) -> V128 */

      /* Polynomial multiplication treats its arguments as
         coefficients of polynomials over {0, 1}. */
      Iop_PolynomialMul8x16, /* (V128, V128) -> V128 */
      Iop_PolynomialMull8x8, /*   (I64, I64) -> V128 */

      /* Vector Polynomial multiplication add.   (V128, V128) -> V128

       *** Below is the algorithm for the instructions. These Iops could
           be emulated to get this functionality, but the emulation would
           be long and messy.

        Example for polynomial multiply add for vector of bytes
        do i = 0 to 15
            prod[i].bit[0:14] <- 0
            srcA <- VR[argL].byte[i]
            srcB <- VR[argR].byte[i]
            do j = 0 to 7
                do k = 0 to j
                    gbit <- srcA.bit[k] & srcB.bit[j-k]
                    prod[i].bit[j] <- prod[i].bit[j] ^ gbit
                end
            end

            do j = 8 to 14
                do k = j-7 to 7
                     gbit <- (srcA.bit[k] & srcB.bit[j-k])
                     prod[i].bit[j] <- prod[i].bit[j] ^ gbit
                end
            end
        end

        do i = 0 to 7
            VR[dst].hword[i] <- 0b0 || (prod[2×i] ^ prod[2×i+1])
        end
      */
      Iop_PolynomialMulAdd8x16, Iop_PolynomialMulAdd16x8,
      Iop_PolynomialMulAdd32x4, Iop_PolynomialMulAdd64x2,

      /* PAIRWISE operations */
      /* Iop_PwFoo16x4( [a,b,c,d], [e,f,g,h] ) =
            [Foo16(a,b), Foo16(c,d), Foo16(e,f), Foo16(g,h)] */
      Iop_PwAdd8x16, Iop_PwAdd16x8, Iop_PwAdd32x4,
      Iop_PwAdd32Fx2,

      /* Longening variant is unary. The resulting vector contains two times
         less elements than operand, but they are two times wider.
         Example:
            Iop_PwAddL16Ux4( [a,b,c,d] ) = [a+b,c+d]
               where a+b and c+d are unsigned 32-bit values. */
      Iop_PwAddL8Ux16, Iop_PwAddL16Ux8, Iop_PwAddL32Ux4, Iop_PwAddL64Ux2,
      Iop_PwAddL8Sx16, Iop_PwAddL16Sx8, Iop_PwAddL32Sx4,

      /* This is amd64 PMADDUBSW, (V128, V128) -> V128.  For each adjacent pair
         of bytes [a,b] in the first arg and [c,d] in the second, computes:
            signed/signed sat to 16 bits ( zxTo16(a) * sxTo16(b) 
                                           + zxTo16(c) * sxTo16(d) )
         This exists because it's frequently used and there's no reasonably
         concise way to express it using other IROps.
      */
      Iop_PwExtUSMulQAdd8x16,

      /* Other unary pairwise ops */

      /* Vector bit matrix transpose.  (V128) -> V128 */
      /* For each doubleword element of the source vector, an 8-bit x 8-bit
       * matrix transpose is performed. */
      Iop_PwBitMtxXpose64x2,

      /* ABSOLUTE VALUE */
      Iop_Abs8x16, Iop_Abs16x8, Iop_Abs32x4, Iop_Abs64x2,

      /* AVERAGING: note: (arg1 + arg2 + 1) >>u 1 */
      Iop_Avg8Ux16, Iop_Avg16Ux8, Iop_Avg32Ux4, Iop_Avg64Ux2,
      Iop_Avg8Sx16, Iop_Avg16Sx8, Iop_Avg32Sx4, Iop_Avg64Sx2,

      /* MIN/MAX */
      Iop_Max8Sx16, Iop_Max16Sx8, Iop_Max32Sx4, Iop_Max64Sx2,
      Iop_Max8Ux16, Iop_Max16Ux8, Iop_Max32Ux4, Iop_Max64Ux2,
      Iop_Min8Sx16, Iop_Min16Sx8, Iop_Min32Sx4, Iop_Min64Sx2,
      Iop_Min8Ux16, Iop_Min16Ux8, Iop_Min32Ux4, Iop_Min64Ux2,

      /* COMPARISON */
      Iop_CmpEQ8x16,  Iop_CmpEQ16x8,  Iop_CmpEQ32x4,  Iop_CmpEQ64x2,
      Iop_CmpGT8Sx16, Iop_CmpGT16Sx8, Iop_CmpGT32Sx4, Iop_CmpGT64Sx2,
      Iop_CmpGT8Ux16, Iop_CmpGT16Ux8, Iop_CmpGT32Ux4, Iop_CmpGT64Ux2,

      /* COUNT ones / leading zeroes / leading sign bits (not including topmost
         bit) */
      Iop_Cnt8x16,
      Iop_Clz8x16, Iop_Clz16x8, Iop_Clz32x4,
      Iop_Cls8x16, Iop_Cls16x8, Iop_Cls32x4,

      /* VECTOR x SCALAR SHIFT (shift amt :: Ity_I8) */
      Iop_ShlN8x16, Iop_ShlN16x8, Iop_ShlN32x4, Iop_ShlN64x2,
      Iop_ShrN8x16, Iop_ShrN16x8, Iop_ShrN32x4, Iop_ShrN64x2,
      Iop_SarN8x16, Iop_SarN16x8, Iop_SarN32x4, Iop_SarN64x2,

      /* VECTOR x VECTOR SHIFT / ROTATE */
      /* FIXME: I'm pretty sure the ARM32 front/back ends interpret these
         differently from all other targets.  The intention is that
         the shift amount (2nd arg) is interpreted as unsigned and
         only the lowest log2(lane-bits) bits are relevant.  But the
         ARM32 versions treat the shift amount as an 8 bit signed
         number.  The ARM32 uses should be replaced by the relevant
         vector x vector bidirectional shifts instead. */
      Iop_Shl8x16, Iop_Shl16x8, Iop_Shl32x4, Iop_Shl64x2,
      Iop_Shr8x16, Iop_Shr16x8, Iop_Shr32x4, Iop_Shr64x2,
      Iop_Sar8x16, Iop_Sar16x8, Iop_Sar32x4, Iop_Sar64x2,
      Iop_Sal8x16, Iop_Sal16x8, Iop_Sal32x4, Iop_Sal64x2,
      Iop_Rol8x16, Iop_Rol16x8, Iop_Rol32x4, Iop_Rol64x2,

      /* VECTOR x VECTOR SATURATING SHIFT */
      Iop_QShl8x16, Iop_QShl16x8, Iop_QShl32x4, Iop_QShl64x2,
      Iop_QSal8x16, Iop_QSal16x8, Iop_QSal32x4, Iop_QSal64x2,
      /* VECTOR x INTEGER SATURATING SHIFT */
      Iop_QShlNsatSU8x16, Iop_QShlNsatSU16x8,
      Iop_QShlNsatSU32x4, Iop_QShlNsatSU64x2,
      Iop_QShlNsatUU8x16, Iop_QShlNsatUU16x8,
      Iop_QShlNsatUU32x4, Iop_QShlNsatUU64x2,
      Iop_QShlNsatSS8x16, Iop_QShlNsatSS16x8,
      Iop_QShlNsatSS32x4, Iop_QShlNsatSS64x2,

      /* VECTOR x VECTOR BIDIRECTIONAL SATURATING (& MAYBE ROUNDING) SHIFT */
      /* All of type (V128, V128) -> V256. */
      /* The least significant 8 bits of each lane of the second
         operand are used as the shift amount, and interpreted signedly.
         Positive values mean a shift left, negative a shift right.  The
         result is signedly or unsignedly saturated.  There are also
         rounding variants, which add 2^(shift_amount-1) to the value before
         shifting, but only in the shift-right case.  Vacated positions
         are filled with zeroes.  IOW, it's either SHR or SHL, but not SAR.

         These operations return 129 bits: one bit ("Q") indicating whether
         saturation occurred, and the shift result.  The result type is V256,
         of which the lower V128 is the shift result, and Q occupies the
         least significant bit of the upper V128.  All other bits of the
         upper V128 are zero. */
      // Unsigned saturation, no rounding
      Iop_QandUQsh8x16, Iop_QandUQsh16x8,
      Iop_QandUQsh32x4, Iop_QandUQsh64x2,
      // Signed saturation, no rounding
      Iop_QandSQsh8x16, Iop_QandSQsh16x8,
      Iop_QandSQsh32x4, Iop_QandSQsh64x2,

      // Unsigned saturation, rounding
      Iop_QandUQRsh8x16, Iop_QandUQRsh16x8,
      Iop_QandUQRsh32x4, Iop_QandUQRsh64x2,
      // Signed saturation, rounding
      Iop_QandSQRsh8x16, Iop_QandSQRsh16x8,
      Iop_QandSQRsh32x4, Iop_QandSQRsh64x2,

      /* VECTOR x VECTOR BIDIRECTIONAL (& MAYBE ROUNDING) SHIFT */
      /* All of type (V128, V128) -> V128 */
      /* The least significant 8 bits of each lane of the second
         operand are used as the shift amount, and interpreted signedly.
         Positive values mean a shift left, negative a shift right.
         There are also rounding variants, which add 2^(shift_amount-1)
         to the value before shifting, but only in the shift-right case.

         For left shifts, the vacated places are filled with zeroes.
         For right shifts, the vacated places are filled with zeroes
         for the U variants and sign bits for the S variants. */
      // Signed and unsigned, non-rounding
      Iop_Sh8Sx16, Iop_Sh16Sx8, Iop_Sh32Sx4, Iop_Sh64Sx2,
      Iop_Sh8Ux16, Iop_Sh16Ux8, Iop_Sh32Ux4, Iop_Sh64Ux2,

      // Signed and unsigned, rounding
      Iop_Rsh8Sx16, Iop_Rsh16Sx8, Iop_Rsh32Sx4, Iop_Rsh64Sx2,
      Iop_Rsh8Ux16, Iop_Rsh16Ux8, Iop_Rsh32Ux4, Iop_Rsh64Ux2,

      /* The least significant 8 bits of each lane of the second
         operand are used as the shift amount, and interpreted signedly.
         Positive values mean a shift left, negative a shift right.  The
         result is signedly or unsignedly saturated.  There are also
         rounding variants, which add 2^(shift_amount-1) to the value before
         shifting, but only in the shift-right case.  Vacated positions
         are filled with zeroes.  IOW, it's either SHR or SHL, but not SAR.
      */

      /* VECTOR x SCALAR SATURATING (& MAYBE ROUNDING) NARROWING SHIFT RIGHT */
      /* All of type (V128, I8) -> V128 */
      /* The first argument is shifted right, then narrowed to half the width
         by saturating it.  The second argument is a scalar shift amount that
         applies to all lanes, and must be a value in the range 1 to lane_width.
         The shift may be done signedly (Sar variants) or unsignedly (Shr
         variants).  The saturation is done according to the two signedness
         indicators at the end of the name.  For example 64Sto32U means a
         signed 64 bit value is saturated into an unsigned 32 bit value.
         Additionally, the QRS variants do rounding, that is, they add the
         value (1 << (shift_amount-1)) to each source lane before shifting.

         These operations return 65 bits: one bit ("Q") indicating whether
         saturation occurred, and the shift result.  The result type is V128,
         of which the lower half is the shift result, and Q occupies the
         least significant bit of the upper half.  All other bits of the
         upper half are zero. */
      // No rounding, sat U->U
      Iop_QandQShrNnarrow16Uto8Ux8,
      Iop_QandQShrNnarrow32Uto16Ux4, Iop_QandQShrNnarrow64Uto32Ux2,
      // No rounding, sat S->S
      Iop_QandQSarNnarrow16Sto8Sx8,
      Iop_QandQSarNnarrow32Sto16Sx4, Iop_QandQSarNnarrow64Sto32Sx2,
      // No rounding, sat S->U
      Iop_QandQSarNnarrow16Sto8Ux8,
      Iop_QandQSarNnarrow32Sto16Ux4, Iop_QandQSarNnarrow64Sto32Ux2,

      // Rounding, sat U->U
      Iop_QandQRShrNnarrow16Uto8Ux8,
      Iop_QandQRShrNnarrow32Uto16Ux4, Iop_QandQRShrNnarrow64Uto32Ux2,
      // Rounding, sat S->S
      Iop_QandQRSarNnarrow16Sto8Sx8,
      Iop_QandQRSarNnarrow32Sto16Sx4, Iop_QandQRSarNnarrow64Sto32Sx2,
      // Rounding, sat S->U
      Iop_QandQRSarNnarrow16Sto8Ux8,
      Iop_QandQRSarNnarrow32Sto16Ux4, Iop_QandQRSarNnarrow64Sto32Ux2,

      /* NARROWING (binary) 
         -- narrow 2xV128 into 1xV128, hi half from left arg */
      /* See comments above w.r.t. U vs S issues in saturated narrowing. */
      Iop_QNarrowBin16Sto8Ux16, Iop_QNarrowBin32Sto16Ux8,
      Iop_QNarrowBin16Sto8Sx16, Iop_QNarrowBin32Sto16Sx8,
      Iop_QNarrowBin16Uto8Ux16, Iop_QNarrowBin32Uto16Ux8,
      Iop_NarrowBin16to8x16, Iop_NarrowBin32to16x8,
      Iop_QNarrowBin64Sto32Sx4, Iop_QNarrowBin64Uto32Ux4,
      Iop_NarrowBin64to32x4,

      /* NARROWING (unary) -- narrow V128 into I64 */
      Iop_NarrowUn16to8x8, Iop_NarrowUn32to16x4, Iop_NarrowUn64to32x2,
      /* Saturating narrowing from signed source to signed/unsigned
         destination */
      Iop_QNarrowUn16Sto8Sx8, Iop_QNarrowUn32Sto16Sx4, Iop_QNarrowUn64Sto32Sx2,
      Iop_QNarrowUn16Sto8Ux8, Iop_QNarrowUn32Sto16Ux4, Iop_QNarrowUn64Sto32Ux2,
      /* Saturating narrowing from unsigned source to unsigned destination */
      Iop_QNarrowUn16Uto8Ux8, Iop_QNarrowUn32Uto16Ux4, Iop_QNarrowUn64Uto32Ux2,

      /* WIDENING -- sign or zero extend each element of the argument
         vector to the twice original size.  The resulting vector consists of
         the same number of elements but each element and the vector itself
         are twice as wide.
         All operations are I64->V128.
         Example
            Iop_Widen32Sto64x2( [a, b] ) = [c, d]
               where c = Iop_32Sto64(a) and d = Iop_32Sto64(b) */
      Iop_Widen8Uto16x8, Iop_Widen16Uto32x4, Iop_Widen32Uto64x2,
      Iop_Widen8Sto16x8, Iop_Widen16Sto32x4, Iop_Widen32Sto64x2,

      /* INTERLEAVING */
      /* Interleave lanes from low or high halves of
         operands.  Most-significant result lane is from the left
         arg. */
      Iop_InterleaveHI8x16, Iop_InterleaveHI16x8,
      Iop_InterleaveHI32x4, Iop_InterleaveHI64x2,
      Iop_InterleaveLO8x16, Iop_InterleaveLO16x8,
      Iop_InterleaveLO32x4, Iop_InterleaveLO64x2,
      /* Interleave odd/even lanes of operands.  Most-significant result lane
         is from the left arg. */
      Iop_InterleaveOddLanes8x16, Iop_InterleaveEvenLanes8x16,
      Iop_InterleaveOddLanes16x8, Iop_InterleaveEvenLanes16x8,
      Iop_InterleaveOddLanes32x4, Iop_InterleaveEvenLanes32x4,

      /* Pack even/odd lanes. */
      Iop_PackOddLanes8x16, Iop_PackEvenLanes8x16,
      Iop_PackOddLanes16x8, Iop_PackEvenLanes16x8,
      Iop_PackOddLanes32x4, Iop_PackEvenLanes32x4,

      /* CONCATENATION -- build a new value by concatenating either
         the even or odd lanes of both operands.  Note that
         Cat{Odd,Even}Lanes64x2 are identical to Interleave{HI,LO}64x2
         and so are omitted. */
      Iop_CatOddLanes8x16, Iop_CatOddLanes16x8, Iop_CatOddLanes32x4,
      Iop_CatEvenLanes8x16, Iop_CatEvenLanes16x8, Iop_CatEvenLanes32x4,

      /* GET elements of VECTOR
         GET is binop (V128, I8) -> I<elem_size>
         SET is triop (V128, I8, I<elem_size>) -> V128 */
      /* Note: the arm back-end handles only constant second argument. */
      Iop_GetElem8x16, Iop_GetElem16x8, Iop_GetElem32x4, Iop_GetElem64x2,
      Iop_SetElem8x16, Iop_SetElem16x8, Iop_SetElem32x4, Iop_SetElem64x2,

      /* DUPLICATING -- copy value to all lanes */
      Iop_Dup8x16,   Iop_Dup16x8,   Iop_Dup32x4,

      /* SLICE -- produces the lowest 128 bits of (arg1:arg2) >> (8 * arg3).
         arg3 is a shift amount in bytes and may be between 0 and 16
         inclusive.  When 0, the result is arg2; when 16, the result is arg1.
         Not all back ends handle all values.  The arm64 back
         end handles only immediate arg3 values. */
      Iop_SliceV128,  // (V128, V128, I8) -> V128

      /* REVERSE the order of chunks in vector lanes.  Chunks must be
         smaller than the vector lanes (obviously) and so may be 8-,
         16- and 32-bit in size.  See definitions of 64-bit SIMD
         versions above for examples. */
      Iop_Reverse8sIn16_x8,
      Iop_Reverse8sIn32_x4, Iop_Reverse16sIn32_x4,
      Iop_Reverse8sIn64_x2, Iop_Reverse16sIn64_x2, Iop_Reverse32sIn64_x2,
      Iop_Reverse1sIn8_x16, /* Reverse bits in each byte lane. */

      /* PERMUTING -- copy src bytes to dst,
         as indexed by control vector bytes:
            for i in 0 .. 15 . result[i] = argL[ argR[i] ] 
         argR[i] values may only be in the range 0 .. 15, else behaviour
         is undefined.  That is, argR[i][7:4] must be zero. */
      Iop_Perm8x16,
      Iop_Perm32x4, /* ditto, except argR values are restricted to 0 .. 3 */

      /* PERMUTING with optional zeroing:
            for i in 0 .. 15 . result[i] = if argR[i] bit 7 is set
                                           then zero else argL[ argR[i] ]
         argR[i][6:4] must be zero, else behaviour is undefined.
      */
      Iop_PermOrZero8x16,

      /* same, but Triop (argL consists of two 128-bit parts) */
      /* correct range for argR values is 0..31 */
      /* (V128, V128, V128) -> V128 */
      /* (ArgL_first, ArgL_second, ArgR) -> result */
      Iop_Perm8x16x2,

      /* MISC CONVERSION -- get high bits of each byte lane, a la
         x86/amd64 pmovmskb */
      Iop_GetMSBs8x16, /* V128 -> I16 */

      /* Vector Reciprocal Estimate and Vector Reciprocal Square Root Estimate
         See floating-point equivalents for details. */
      Iop_RecipEst32Ux4, Iop_RSqrtEst32Ux4,

      /* 128-bit multipy by 10 instruction, result is lower 128-bits */
      Iop_MulI128by10,

      /* 128-bit multipy by 10 instruction, result is carry out from the MSB */
      Iop_MulI128by10Carry,

      /* 128-bit multipy by 10 instruction, result is lower 128-bits of the
       * source times 10 plus the carry in
       */
      Iop_MulI128by10E,

      /* 128-bit multipy by 10 instruction, result is carry out from the MSB
       * of the source times 10 plus the carry in
       */
      Iop_MulI128by10ECarry,

      /* ------------------ 256-bit SIMD Integer. ------------------ */

      /* Pack/unpack */
      Iop_V256to64_0,  // V256 -> I64, extract least significant lane
      Iop_V256to64_1,
      Iop_V256to64_2,
      Iop_V256to64_3,  // V256 -> I64, extract most significant lane

      Iop_64x4toV256,  // (I64,I64,I64,I64)->V256
                       // first arg is most significant lane

      Iop_V256toV128_0, // V256 -> V128, less significant lane
      Iop_V256toV128_1, // V256 -> V128, more significant lane
      Iop_V128HLtoV256, // (V128,V128)->V256, first arg is most signif

      Iop_AndV256,
      Iop_OrV256,
      Iop_XorV256,
      Iop_NotV256,

      /* MISC (vector integer cmp != 0) */
      Iop_CmpNEZ8x32, Iop_CmpNEZ16x16, Iop_CmpNEZ32x8, Iop_CmpNEZ64x4,

      Iop_Add8x32,    Iop_Add16x16,    Iop_Add32x8,    Iop_Add64x4,
      Iop_Sub8x32,    Iop_Sub16x16,    Iop_Sub32x8,    Iop_Sub64x4,

      Iop_CmpEQ8x32,  Iop_CmpEQ16x16,  Iop_CmpEQ32x8,  Iop_CmpEQ64x4,
      Iop_CmpGT8Sx32, Iop_CmpGT16Sx16, Iop_CmpGT32Sx8, Iop_CmpGT64Sx4,

      Iop_ShlN16x16, Iop_ShlN32x8, Iop_ShlN64x4,
      Iop_ShrN16x16, Iop_ShrN32x8, Iop_ShrN64x4,
      Iop_SarN16x16, Iop_SarN32x8,

      Iop_Max8Sx32, Iop_Max16Sx16, Iop_Max32Sx8,
      Iop_Max8Ux32, Iop_Max16Ux16, Iop_Max32Ux8,
      Iop_Min8Sx32, Iop_Min16Sx16, Iop_Min32Sx8,
      Iop_Min8Ux32, Iop_Min16Ux16, Iop_Min32Ux8,

      Iop_Mul16x16, Iop_Mul32x8,
      Iop_MulHi16Ux16, Iop_MulHi16Sx16,

      Iop_QAdd8Ux32, Iop_QAdd16Ux16,
      Iop_QAdd8Sx32, Iop_QAdd16Sx16,
      Iop_QSub8Ux32, Iop_QSub16Ux16,
      Iop_QSub8Sx32, Iop_QSub16Sx16,

      Iop_Avg8Ux32, Iop_Avg16Ux16,

      Iop_Perm32x8,

      /* (V128, V128) -> V128 */
      Iop_CipherV128, Iop_CipherLV128, Iop_CipherSV128,
      Iop_NCipherV128, Iop_NCipherLV128,

      /* Hash instructions, Federal Information Processing Standards
       * Publication 180-3 Secure Hash Standard. */
      /* (V128, I8) -> V128; The I8 input arg is (ST | SIX), where ST and
       * SIX are fields from the insn. See ISA 2.07 description of
       * vshasigmad and vshasigmaw insns.*/
      Iop_SHA512, Iop_SHA256,

      /* ------------------ 256-bit SIMD FP. ------------------ */

      /* ternary :: IRRoundingMode(I32) x V256 x V256 -> V256 */
      Iop_Add64Fx4, Iop_Sub64Fx4, Iop_Mul64Fx4, Iop_Div64Fx4,
      Iop_Add32Fx8, Iop_Sub32Fx8, Iop_Mul32Fx8, Iop_Div32Fx8,

      Iop_I32StoF32x8, /* IRRoundingMode(I32) x V256 -> V256 */
      Iop_F32toI32Sx8, /* IRRoundingMode(I32) x V256 -> V256 */

      Iop_F32toF16x8,  /* IRRoundingMode(I32) x V256 -> V128 */
      Iop_F16toF32x8,  /* F16x8(==V128) -> F32x8(==V256) */

      Iop_Sqrt32Fx8,
      Iop_Sqrt64Fx4,
      Iop_RSqrtEst32Fx8,
      Iop_RecipEst32Fx8,

      Iop_Max32Fx8, Iop_Min32Fx8,
      Iop_Max64Fx4, Iop_Min64Fx4,
      Iop_Rotx32, Iop_Rotx64,
      Iop_LAST      /* must be the last enumerator */
   }
   IROp;

/* Pretty-print an op. */
extern void ppIROp ( IROp );

/* For a given operand return the types of its arguments and its result. */
extern void typeOfPrimop ( IROp op,
                           /*OUTs*/ IRType* t_dst, IRType* t_arg1,
                           IRType* t_arg2, IRType* t_arg3, IRType* t_arg4 );

/* Might the given primop trap (eg, attempt integer division by zero)?  If in
   doubt returns True.  However, the vast majority of primops will never
   trap. */
extern Bool primopMightTrap ( IROp op );

/* Encoding of IEEE754-specified rounding modes.
   Note, various front and back ends rely on the actual numerical
   values of these, so do not change them. */
typedef
   enum { 
      Irrm_NEAREST              = 0,  // Round to nearest, ties to even
      Irrm_NegINF               = 1,  // Round to negative infinity
      Irrm_PosINF               = 2,  // Round to positive infinity
      Irrm_ZERO                 = 3,  // Round toward zero
      Irrm_NEAREST_TIE_AWAY_0   = 4,  // Round to nearest, ties away from 0
      Irrm_PREPARE_SHORTER      = 5,  // Round to prepare for shorter 
                                      // precision
      Irrm_AWAY_FROM_ZERO       = 6,  // Round to away from 0
      Irrm_NEAREST_TIE_TOWARD_0 = 7   // Round to nearest, ties towards 0
   }
   IRRoundingMode;

/* Binary floating point comparison result values.
   This is also derived from what IA32 does. */
typedef
   enum {
      Ircr_UN = 0x45,
      Ircr_LT = 0x01,
      Ircr_GT = 0x00,
      Ircr_EQ = 0x40
   }
   IRCmpFResult;

typedef IRCmpFResult IRCmpF32Result;
typedef IRCmpFResult IRCmpF64Result;
typedef IRCmpFResult IRCmpF128Result;

/* Decimal floating point result values. */
typedef IRCmpFResult IRCmpDResult;
typedef IRCmpDResult IRCmpD64Result;
typedef IRCmpDResult IRCmpD128Result;

/* ------------------ Expressions ------------------ */

typedef struct _IRQop   IRQop;   /* forward declaration */
typedef struct _IRTriop IRTriop; /* forward declaration */


/* The different kinds of expressions.  Their meaning is explained below
   in the comments for IRExpr. */
typedef
   enum { 
      Iex_Binder=0x1900,
      Iex_Get,
      Iex_GetI,
      Iex_RdTmp,
      Iex_Qop,
      Iex_Triop,
      Iex_Binop,
      Iex_Unop,
      Iex_Load,
      Iex_Const,
      Iex_ITE,
      Iex_CCall,
      Iex_VECRET,
      Iex_GSPTR
   }
   IRExprTag;

/* An expression.  Stored as a tagged union.  'tag' indicates what kind
   of expression this is.  'Iex' is the union that holds the fields.  If
   an IRExpr 'e' has e.tag equal to Iex_Load, then it's a load
   expression, and the fields can be accessed with
   'e.Iex.Load.<fieldname>'.

   For each kind of expression, we show what it looks like when
   pretty-printed with ppIRExpr().
*/
typedef
   struct _IRExpr
   IRExpr;

struct _IRExpr {
   IRExprTag tag;
   union {
      /* Used only in pattern matching within Vex.  Should not be seen
         outside of Vex. */
      struct {
         Int binder;
      } Binder;

      /* Read a guest register, at a fixed offset in the guest state.
         ppIRExpr output: GET:<ty>(<offset>), eg. GET:I32(0)
      */
      struct {
         Int    offset;    /* Offset into the guest state */
         IRType ty;        /* Type of the value being read */
      } Get;

      /* Read a guest register at a non-fixed offset in the guest
         state.  This allows circular indexing into parts of the guest
         state, which is essential for modelling situations where the
         identity of guest registers is not known until run time.  One
         example is the x87 FP register stack.

         The part of the guest state to be treated as a circular array
         is described in the IRRegArray 'descr' field.  It holds the
         offset of the first element in the array, the type of each
         element, and the number of elements.

         The array index is indicated rather indirectly, in a way
         which makes optimisation easy: as the sum of variable part
         (the 'ix' field) and a constant offset (the 'bias' field).

         Since the indexing is circular, the actual array index to use
         is computed as (ix + bias) % num-of-elems-in-the-array.

         Here's an example.  The description

            (96:8xF64)[t39,-7]

         describes an array of 8 F64-typed values, the
         guest-state-offset of the first being 96.  This array is
         being indexed at (t39 - 7) % 8.

         It is important to get the array size/type exactly correct
         since IR optimisation looks closely at such info in order to
         establish aliasing/non-aliasing between seperate GetI and
         PutI events, which is used to establish when they can be
         reordered, etc.  Putting incorrect info in will lead to
         obscure IR optimisation bugs.

            ppIRExpr output: GETI<descr>[<ix>,<bias]
                         eg. GETI(128:8xI8)[t1,0]
      */
      struct {
         IRRegArray* descr; /* Part of guest state treated as circular */
         IRExpr*     ix;    /* Variable part of index into array */
         Int         bias;  /* Constant offset part of index into array */
      } GetI;

      /* The value held by a temporary.
         ppIRExpr output: t<tmp>, eg. t1
      */
      struct {
         IRTemp tmp;       /* The temporary number */
      } RdTmp;

      /* A quaternary operation.
         ppIRExpr output: <op>(<arg1>, <arg2>, <arg3>, <arg4>),
                      eg. MAddF64r32(t1, t2, t3, t4)
      */
      struct {
        IRQop* details;
      } Qop;

      /* A ternary operation.
         ppIRExpr output: <op>(<arg1>, <arg2>, <arg3>),
                      eg. MulF64(1, 2.0, 3.0)
      */
      struct {
        IRTriop* details;
      } Triop;

      /* A binary operation.
         ppIRExpr output: <op>(<arg1>, <arg2>), eg. Add32(t1,t2)
      */
      struct {
         IROp op;          /* op-code   */
         IRExpr* arg1;     /* operand 1 */
         IRExpr* arg2;     /* operand 2 */
      } Binop;

      /* A unary operation.
         ppIRExpr output: <op>(<arg>), eg. Neg8(t1)
      */
      struct {
         IROp    op;       /* op-code */
         IRExpr* arg;      /* operand */
      } Unop;

      /* A load from memory -- a normal load, not a load-linked.
         Load-Linkeds (and Store-Conditionals) are instead represented
         by IRStmt.LLSC since Load-Linkeds have side effects and so
         are not semantically valid IRExpr's.
         ppIRExpr output: LD<end>:<ty>(<addr>), eg. LDle:I32(t1)
      */
      struct {
         IREndness end;    /* Endian-ness of the load */
         IRType    ty;     /* Type of the loaded value */
         IRExpr*   addr;   /* Address being loaded from */
      } Load;

      /* A constant-valued expression.
         ppIRExpr output: <con>, eg. 0x4:I32
      */
      struct {
         IRConst* con;     /* The constant itself */
      } Const;

      /* A call to a pure (no side-effects) helper C function.

         With the 'cee' field, 'name' is the function's name.  It is
         only used for pretty-printing purposes.  The address to call
         (host address, of course) is stored in the 'addr' field
         inside 'cee'.

         The 'args' field is a NULL-terminated array of arguments.
         The stated return IRType, and the implied argument types,
         must match that of the function being called well enough so
         that the back end can actually generate correct code for the
         call.

         The called function **must** satisfy the following:

         * no side effects -- must be a pure function, the result of
           which depends only on the passed parameters.

         * it may not look at, nor modify, any of the guest state
           since that would hide guest state transitions from
           instrumenters

         * it may not access guest memory, since that would hide
           guest memory transactions from the instrumenters

         * it must not assume that arguments are being evaluated in a
           particular order. The oder of evaluation is unspecified.

         This is restrictive, but makes the semantics clean, and does
         not interfere with IR optimisation.

         If you want to call a helper which can mess with guest state
         and/or memory, instead use Ist_Dirty.  This is a lot more
         flexible, but you have to give a bunch of details about what
         the helper does (and you better be telling the truth,
         otherwise any derived instrumentation will be wrong).  Also
         Ist_Dirty inhibits various IR optimisations and so can cause
         quite poor code to be generated.  Try to avoid it.

         In principle it would be allowable to have the arg vector
         contain an IRExpr_VECRET(), although not IRExpr_GSPTR(). However,
         at the moment there is no requirement for clean helper calls to
         be able to return V128 or V256 values.  Hence this is not allowed.

         ppIRExpr output: <cee>(<args>):<retty>
                      eg. foo{0x80489304}(t1, t2):I32
      */
      struct {
         IRCallee* cee;    /* Function to call. */
         IRType    retty;  /* Type of return value. */
         IRExpr**  args;   /* Vector of argument expressions. */
      }  CCall;

      /* A ternary if-then-else operator.  It returns iftrue if cond is
         nonzero, iffalse otherwise.  Note that it is STRICT, ie. both
         iftrue and iffalse are evaluated in all cases.

         ppIRExpr output: ITE(<cond>,<iftrue>,<iffalse>),
                         eg. ITE(t6,t7,t8)
      */
      struct {
         IRExpr* cond;     /* Condition */
         IRExpr* iftrue;   /* True expression */
         IRExpr* iffalse;  /* False expression */
      } ITE;
   } Iex;
};

/* Expression auxiliaries: a ternary expression. */
struct _IRTriop {
   IROp op;          /* op-code   */
   IRExpr* arg1;     /* operand 1 */
   IRExpr* arg2;     /* operand 2 */
   IRExpr* arg3;     /* operand 3 */
};

/* Expression auxiliaries: a quarternary expression. */
struct _IRQop {
   IROp op;          /* op-code   */
   IRExpr* arg1;     /* operand 1 */
   IRExpr* arg2;     /* operand 2 */
   IRExpr* arg3;     /* operand 3 */
   IRExpr* arg4;     /* operand 4 */
};


/* Two special kinds of IRExpr, which can ONLY be used in
   argument lists for dirty helper calls (IRDirty.args) and in NO
   OTHER PLACES.  And then only in very limited ways.  */

/* Denotes an argument which (in the helper) takes a pointer to a
   (naturally aligned) V128 or V256, into which the helper is expected
   to write its result.  Use of IRExpr_VECRET() is strictly
   controlled.  If the helper returns a V128 or V256 value then
   IRExpr_VECRET() must appear exactly once in the arg list, although
   it can appear anywhere, and the helper must have a C 'void' return
   type.  If the helper returns any other type, IRExpr_VECRET() may
   not appear in the argument list. */

/* Denotes an void* argument which is passed to the helper, which at
   run time will point to the thread's guest state area.  This can
   only appear at most once in an argument list, and it may not appear
   at all in argument lists for clean helper calls. */

static inline Bool is_IRExpr_VECRET_or_GSPTR ( const IRExpr* e ) {
   return e->tag == Iex_VECRET || e->tag == Iex_GSPTR;
}


/* Expression constructors. */
extern IRExpr* IRExpr_Binder ( Int binder );
extern IRExpr* IRExpr_Get    ( Int off, IRType ty );
extern IRExpr* IRExpr_GetI   ( IRRegArray* descr, IRExpr* ix, Int bias );
extern IRExpr* IRExpr_RdTmp  ( IRTemp tmp );
extern IRExpr* IRExpr_Qop    ( IROp op, IRExpr* arg1, IRExpr* arg2, 
                                        IRExpr* arg3, IRExpr* arg4 );
extern IRExpr* IRExpr_Triop  ( IROp op, IRExpr* arg1, 
                                        IRExpr* arg2, IRExpr* arg3 );
extern IRExpr* IRExpr_Binop  ( IROp op, IRExpr* arg1, IRExpr* arg2 );
extern IRExpr* IRExpr_Unop   ( IROp op, IRExpr* arg );
extern IRExpr* IRExpr_Load   ( IREndness end, IRType ty, IRExpr* addr );
extern IRExpr* IRExpr_Const  ( IRConst* con );
extern IRExpr* IRExpr_CCall  ( IRCallee* cee, IRType retty, IRExpr** args );
extern IRExpr* IRExpr_ITE    ( IRExpr* cond, IRExpr* iftrue, IRExpr* iffalse );
extern IRExpr* IRExpr_VECRET ( void );
extern IRExpr* IRExpr_GSPTR  ( void );

/* Deep-copy an IRExpr. */
extern IRExpr* deepCopyIRExpr ( const IRExpr* );

/* Pretty-print an IRExpr. */
extern void ppIRExpr ( const IRExpr* );

/* NULL-terminated IRExpr vector constructors, suitable for
   use as arg lists in clean/dirty helper calls. */
extern IRExpr** mkIRExprVec_0 ( void );
extern IRExpr** mkIRExprVec_1 ( IRExpr* );
extern IRExpr** mkIRExprVec_2 ( IRExpr*, IRExpr* );
extern IRExpr** mkIRExprVec_3 ( IRExpr*, IRExpr*, IRExpr* );
extern IRExpr** mkIRExprVec_4 ( IRExpr*, IRExpr*, IRExpr*, IRExpr* );
extern IRExpr** mkIRExprVec_5 ( IRExpr*, IRExpr*, IRExpr*, IRExpr*,
                                IRExpr* );
extern IRExpr** mkIRExprVec_6 ( IRExpr*, IRExpr*, IRExpr*, IRExpr*,
                                IRExpr*, IRExpr* );
extern IRExpr** mkIRExprVec_7 ( IRExpr*, IRExpr*, IRExpr*, IRExpr*,
                                IRExpr*, IRExpr*, IRExpr* );
extern IRExpr** mkIRExprVec_8 ( IRExpr*, IRExpr*, IRExpr*, IRExpr*,
                                IRExpr*, IRExpr*, IRExpr*, IRExpr* );
extern IRExpr** mkIRExprVec_9 ( IRExpr*, IRExpr*, IRExpr*, IRExpr*,
                                IRExpr*, IRExpr*, IRExpr*, IRExpr*, IRExpr* );
extern IRExpr** mkIRExprVec_13 ( IRExpr*, IRExpr*, IRExpr*, IRExpr*,
                                 IRExpr*, IRExpr*, IRExpr*, IRExpr*,
                                 IRExpr*, IRExpr*, IRExpr*, IRExpr*, IRExpr* );

/* IRExpr copiers:
   - shallowCopy: shallow-copy (ie. create a new vector that shares the
     elements with the original).
   - deepCopy: deep-copy (ie. create a completely new vector). */
extern IRExpr** shallowCopyIRExprVec ( IRExpr** );
extern IRExpr** deepCopyIRExprVec ( IRExpr *const * );

/* Make a constant expression from the given host word taking into
   account (of course) the host word size. */
extern IRExpr* mkIRExpr_HWord ( HWord );

/* Convenience function for constructing clean helper calls. */
extern 
IRExpr* mkIRExprCCall ( IRType retty,
                        Int regparms, const HChar* name, void* addr, 
                        IRExpr** args );


/* Convenience functions for atoms (IRExprs which are either Iex_Tmp or
 * Iex_Const). */
static inline Bool isIRAtom ( const IRExpr* e ) {
   return e->tag == Iex_RdTmp || e->tag == Iex_Const;
}

/* Are these two IR atoms identical?  Causes an assertion
   failure if they are passed non-atoms. */
extern Bool eqIRAtom ( const IRExpr*, const IRExpr* );


/* ------------------ Jump kinds ------------------ */

/* This describes hints which can be passed to the dispatcher at guest
   control-flow transfer points.

   Re Ijk_InvalICache and Ijk_FlushDCache: the guest state _must_ have
   two pseudo-registers, guest_CMSTART and guest_CMLEN, which specify
   the start and length of the region to be invalidated.  CM stands
   for "Cache Management".  These are both the size of a guest word.
   It is the responsibility of the relevant toIR.c to ensure that
   these are filled in with suitable values before issuing a jump of
   kind Ijk_InvalICache or Ijk_FlushDCache.

   Ijk_InvalICache requests invalidation of translations taken from
   the requested range.  Ijk_FlushDCache requests flushing of the D
   cache for the specified range.

   Re Ijk_EmWarn and Ijk_EmFail: the guest state must have a
   pseudo-register guest_EMNOTE, which is 32-bits regardless of the
   host or guest word size.  That register should be made to hold a
   VexEmNote value to indicate the reason for the exit.

   In the case of Ijk_EmFail, the exit is fatal (Vex-generated code
   cannot continue) and so the jump destination can be anything.

   Re Ijk_Sys_ (syscall jumps): the guest state must have a
   pseudo-register guest_IP_AT_SYSCALL, which is the size of a guest
   word.  Front ends should set this to be the IP at the most recently
   executed kernel-entering (system call) instruction.  This makes it
   very much easier (viz, actually possible at all) to back up the
   guest to restart a syscall that has been interrupted by a signal.
*/
typedef
   enum {
      Ijk_INVALID=0x1A00, 
      Ijk_Boring,         /* not interesting; just goto next */
      Ijk_Call,           /* guest is doing a call */
      Ijk_Ret,            /* guest is doing a return */
      Ijk_ClientReq,      /* do guest client req before continuing */
      Ijk_Yield,          /* client is yielding to thread scheduler */
      Ijk_EmWarn,         /* report emulation warning before continuing */
      Ijk_EmFail,         /* emulation critical (FATAL) error; give up */
      Ijk_NoDecode,       /* current instruction cannot be decoded */
      Ijk_MapFail,        /* Vex-provided address translation failed */
      Ijk_InvalICache,    /* Inval icache for range [CMSTART, +CMLEN) */
      Ijk_FlushDCache,    /* Flush dcache for range [CMSTART, +CMLEN) */
      Ijk_NoRedir,        /* Jump to un-redirected guest addr */
      Ijk_SigILL,         /* current instruction synths SIGILL */
      Ijk_SigTRAP,        /* current instruction synths SIGTRAP */
      Ijk_SigSEGV,        /* current instruction synths SIGSEGV */
      Ijk_SigBUS,         /* current instruction synths SIGBUS */
      Ijk_SigFPE,         /* current instruction synths generic SIGFPE */
      Ijk_SigFPE_IntDiv,  /* current instruction synths SIGFPE - IntDiv */
      Ijk_SigFPE_IntOvf,  /* current instruction synths SIGFPE - IntOvf */
      /* Unfortunately, various guest-dependent syscall kinds.  They
	 all mean: do a syscall before continuing. */
      Ijk_Sys_syscall,    /* amd64/x86 'syscall', ppc 'sc', arm 'svc #0' */
      Ijk_Sys_int32,      /* amd64/x86 'int $0x20' */
      Ijk_Sys_int128,     /* amd64/x86 'int $0x80' */
      Ijk_Sys_int129,     /* amd64/x86 'int $0x81' */
      Ijk_Sys_int130,     /* amd64/x86 'int $0x82' */
      Ijk_Sys_int145,     /* amd64/x86 'int $0x91' */
      Ijk_Sys_int210,     /* amd64/x86 'int $0xD2' */
      Ijk_Sys_sysenter    /* x86 'sysenter'.  guest_EIP becomes 
                             invalid at the point this happens. */
   }
   IRJumpKind;

extern void ppIRJumpKind ( IRJumpKind );


/* ------------------ Dirty helper calls ------------------ */

/* A dirty call is a flexible mechanism for calling (possibly
   conditionally) a helper function or procedure.  The helper function
   may read, write or modify client memory, and may read, write or
   modify client state.  It can take arguments and optionally return a
   value.  It may return different results and/or do different things
   when called repeatedly with the same arguments, by means of storing
   private state.

   If a value is returned, it is assigned to the nominated return
   temporary.

   Dirty calls are statements rather than expressions for obvious
   reasons.  If a dirty call is marked as writing guest state, any
   pre-existing values derived from the written parts of the guest
   state are invalid.  Similarly, if the dirty call is stated as
   writing memory, any pre-existing loaded values are invalidated by
   it.

   In order that instrumentation is possible, the call must state, and
   state correctly:

   * Whether it reads, writes or modifies memory, and if so where.

   * Whether it reads, writes or modifies guest state, and if so which
     pieces.  Several pieces may be stated, and their extents must be
     known at translation-time.  Each piece is allowed to repeat some
     number of times at a fixed interval, if required.

   Normally, code is generated to pass just the args to the helper.
   However, if IRExpr_GSPTR() is present in the argument list (at most
   one instance is allowed), then the guest state pointer is passed for
   that arg, so that the callee can access the guest state.  It is
   invalid for .nFxState to be zero but IRExpr_GSPTR() to be present,
   since .nFxState==0 is a claim that the call does not access guest
   state.

   IMPORTANT NOTE re GUARDS: Dirty calls are strict, very strict.  The
   arguments and 'mFx' are evaluated REGARDLESS of the guard value.
   The order of argument evaluation is unspecified.  The guard
   expression is evaluated AFTER the arguments and 'mFx' have been
   evaluated.  'mFx' is expected (by Memcheck) to be a defined value
   even if the guard evaluates to false.
*/

#define VEX_N_FXSTATE  7   /* enough for FXSAVE/FXRSTOR on x86 */

/* Effects on resources (eg. registers, memory locations) */
typedef
   enum {
      Ifx_None=0x1B00,      /* no effect */
      Ifx_Read,             /* reads the resource */
      Ifx_Write,            /* writes the resource */
      Ifx_Modify,           /* modifies the resource */
   }
   IREffect;

/* Pretty-print an IREffect */
extern void ppIREffect ( IREffect );

typedef
   struct _IRDirty {
      /* What to call, and details of args/results.  .guard must be
         non-NULL.  If .tmp is not IRTemp_INVALID, then the call
         returns a result which is placed in .tmp.  If at runtime the
         guard evaluates to false, .tmp has an 0x555..555 bit pattern
         written to it.  Hence conditional calls that assign .tmp are
         allowed. */
      IRCallee* cee;    /* where to call */
      IRExpr*   guard;  /* :: Ity_Bit.  Controls whether call happens */
      /* The args vector may contain IRExpr_GSPTR() and/or
         IRExpr_VECRET(), in both cases, at most once. */
      IRExpr**  args;   /* arg vector, ends in NULL. */
      IRTemp    tmp;    /* to assign result to, or IRTemp_INVALID if none */

      /* Mem effects; we allow only one R/W/M region to be stated */
      IREffect  mFx;    /* indicates memory effects, if any */
      IRExpr*   mAddr;  /* of access, or NULL if mFx==Ifx_None */
      Int       mSize;  /* of access, or zero if mFx==Ifx_None */

      /* Guest state effects; up to N allowed */
      Int  nFxState; /* must be 0 .. VEX_N_FXSTATE */
      struct {
         IREffect fx:16;   /* read, write or modify?  Ifx_None is invalid. */
         UShort   offset;
         UShort   size;
         UChar    nRepeats;
         UChar    repeatLen;
      } fxState[VEX_N_FXSTATE];
      /* The access can be repeated, as specified by nRepeats and
         repeatLen.  To describe only a single access, nRepeats and
         repeatLen should be zero.  Otherwise, repeatLen must be a
         multiple of size and greater than size. */
      /* Overall, the parts of the guest state denoted by (offset,
         size, nRepeats, repeatLen) is
               [offset, +size)
            and, if nRepeats > 0,
               for (i = 1; i <= nRepeats; i++)
                  [offset + i * repeatLen, +size)
         A convenient way to enumerate all segments is therefore
            for (i = 0; i < 1 + nRepeats; i++)
               [offset + i * repeatLen, +size)
      */
   }
   IRDirty;

/* Pretty-print a dirty call */
extern void     ppIRDirty ( const IRDirty* );

/* Allocate an uninitialised dirty call */
extern IRDirty* emptyIRDirty ( void );

/* Deep-copy a dirty call */
extern IRDirty* deepCopyIRDirty ( const IRDirty* );

/* A handy function which takes some of the tedium out of constructing
   dirty helper calls.  The called function impliedly does not return
   any value and has a constant-True guard.  The call is marked as
   accessing neither guest state nor memory (hence the "unsafe"
   designation) -- you can change this marking later if need be.  A
   suitable IRCallee is constructed from the supplied bits. */
extern 
IRDirty* unsafeIRDirty_0_N ( Int regparms, const HChar* name, void* addr, 
                             IRExpr** args );

/* Similarly, make a zero-annotation dirty call which returns a value,
   and assign that to the given temp. */
extern 
IRDirty* unsafeIRDirty_1_N ( IRTemp dst, 
                             Int regparms, const HChar* name, void* addr, 
                             IRExpr** args );


/* --------------- Memory Bus Events --------------- */

typedef
   enum { 
      Imbe_Fence=0x1C00, 
      /* Needed only on ARM.  It cancels a reservation made by a
         preceding Linked-Load, and needs to be handed through to the
         back end, just as LL and SC themselves are. */
      Imbe_CancelReservation
   }
   IRMBusEvent;

extern void ppIRMBusEvent ( IRMBusEvent );


/* --------------- Compare and Swap --------------- */

/* This denotes an atomic compare and swap operation, either
   a single-element one or a double-element one.

   In the single-element case:

     .addr is the memory address.
     .end  is the endianness with which memory is accessed

     If .addr contains the same value as .expdLo, then .dataLo is
     written there, else there is no write.  In both cases, the
     original value at .addr is copied into .oldLo.

     Types: .expdLo, .dataLo and .oldLo must all have the same type.
     It may be any integral type, viz: I8, I16, I32 or, for 64-bit
     guests, I64.

     .oldHi must be IRTemp_INVALID, and .expdHi and .dataHi must
     be NULL.

   In the double-element case:

     .addr is the memory address.
     .end  is the endianness with which memory is accessed

     The operation is the same:

     If .addr contains the same value as .expdHi:.expdLo, then
     .dataHi:.dataLo is written there, else there is no write.  In
     both cases the original value at .addr is copied into
     .oldHi:.oldLo.

     Types: .expdHi, .expdLo, .dataHi, .dataLo, .oldHi, .oldLo must
     all have the same type, which may be any integral type, viz: I8,
     I16, I32 or, for 64-bit guests, I64.

     The double-element case is complicated by the issue of
     endianness.  In all cases, the two elements are understood to be
     located adjacently in memory, starting at the address .addr.

       If .end is Iend_LE, then the .xxxLo component is at the lower
       address and the .xxxHi component is at the higher address, and
       each component is itself stored little-endianly.

       If .end is Iend_BE, then the .xxxHi component is at the lower
       address and the .xxxLo component is at the higher address, and
       each component is itself stored big-endianly.

   This allows representing more cases than most architectures can
   handle.  For example, x86 cannot do DCAS on 8- or 16-bit elements.

   How to know if the CAS succeeded?

   * if .oldLo == .expdLo (resp. .oldHi:.oldLo == .expdHi:.expdLo),
     then the CAS succeeded, .dataLo (resp. .dataHi:.dataLo) is now
     stored at .addr, and the original value there was .oldLo (resp
     .oldHi:.oldLo).

   * if .oldLo != .expdLo (resp. .oldHi:.oldLo != .expdHi:.expdLo),
     then the CAS failed, and the original value at .addr was .oldLo
     (resp. .oldHi:.oldLo).

   Hence it is easy to know whether or not the CAS succeeded.
*/
typedef
   struct {
      IRTemp    oldHi;  /* old value of *addr is written here */
      IRTemp    oldLo;
      IREndness end;    /* endianness of the data in memory */
      IRExpr*   addr;   /* store address */
      IRExpr*   expdHi; /* expected old value at *addr */
      IRExpr*   expdLo;
      IRExpr*   dataHi; /* new value for *addr */
      IRExpr*   dataLo;
   }
   IRCAS;

extern void ppIRCAS ( const IRCAS* cas );

extern IRCAS* mkIRCAS ( IRTemp oldHi, IRTemp oldLo,
                        IREndness end, IRExpr* addr, 
                        IRExpr* expdHi, IRExpr* expdLo,
                        IRExpr* dataHi, IRExpr* dataLo );

extern IRCAS* deepCopyIRCAS ( const IRCAS* );


/* ------------------ Circular Array Put ------------------ */

typedef
   struct {
      IRRegArray* descr; /* Part of guest state treated as circular */
      IRExpr*     ix;    /* Variable part of index into array */
      Int         bias;  /* Constant offset part of index into array */
      IRExpr*     data;  /* The value to write */
   } IRPutI;

extern void ppIRPutI ( const IRPutI* puti );

extern IRPutI* mkIRPutI ( IRRegArray* descr, IRExpr* ix,
                          Int bias, IRExpr* data );

extern IRPutI* deepCopyIRPutI ( const IRPutI* );


/* --------------- Guarded loads and stores --------------- */

/* Conditional stores are straightforward.  They are the same as
   normal stores, with an extra 'guard' field :: Ity_I1 that
   determines whether or not the store actually happens.  If not,
   memory is unmodified.

   The semantics of this is that 'addr' and 'data' are fully evaluated
   even in the case where 'guard' evaluates to zero (false).
*/
typedef
   struct {
      IREndness end;    /* Endianness of the store */
      IRExpr*   addr;   /* store address */
      IRExpr*   data;   /* value to write */
      IRExpr*   guard;  /* Guarding value */
   }
   IRStoreG;

/* Conditional loads are a little more complex.  'addr' is the
   address, 'guard' is the guarding condition.  If the load takes
   place, the loaded value is placed in 'dst'.  If it does not take
   place, 'alt' is copied to 'dst'.  However, the loaded value is not
   placed directly in 'dst' -- it is first subjected to the conversion
   specified by 'cvt'.

   For example, imagine doing a conditional 8-bit load, in which the
   loaded value is zero extended to 32 bits.  Hence:
   * 'dst' and 'alt' must have type I32
   * 'cvt' must be a unary op which converts I8 to I32.  In this 
     example, it would be ILGop_8Uto32.

   There is no explicit indication of the type at which the load is
   done, since that is inferrable from the arg type of 'cvt'.  Note
   that the types of 'alt' and 'dst' and the result type of 'cvt' must
   all be the same.

   Semantically, 'addr' is evaluated even in the case where 'guard'
   evaluates to zero (false), and 'alt' is evaluated even when 'guard'
   evaluates to one (true).  That is, 'addr' and 'alt' are always
   evaluated.
*/
typedef
   enum {
      ILGop_INVALID=0x1D00,
      ILGop_IdentV128, /* 128 bit vector, no conversion */
      ILGop_Ident64,   /* 64 bit, no conversion */
      ILGop_Ident32,   /* 32 bit, no conversion */
      ILGop_16Uto32,   /* 16 bit load, Z-widen to 32 */
      ILGop_16Sto32,   /* 16 bit load, S-widen to 32 */
      ILGop_8Uto32,    /* 8 bit load, Z-widen to 32 */
      ILGop_8Sto32     /* 8 bit load, S-widen to 32 */
   }
   IRLoadGOp;

typedef
   struct {
      IREndness end;    /* Endianness of the load */
      IRLoadGOp cvt;    /* Conversion to apply to the loaded value */
      IRTemp    dst;    /* Destination (LHS) of assignment */
      IRExpr*   addr;   /* Address being loaded from */
      IRExpr*   alt;    /* Value if load is not done. */
      IRExpr*   guard;  /* Guarding value */
   }
   IRLoadG;

extern void ppIRStoreG ( const IRStoreG* sg );

extern void ppIRLoadGOp ( IRLoadGOp cvt );

extern void ppIRLoadG ( const IRLoadG* lg );

extern IRStoreG* mkIRStoreG ( IREndness end,
                              IRExpr* addr, IRExpr* data,
                              IRExpr* guard );

extern IRLoadG* mkIRLoadG ( IREndness end, IRLoadGOp cvt,
                            IRTemp dst, IRExpr* addr, IRExpr* alt, 
                            IRExpr* guard );


/* ------------------ Statements ------------------ */

/* The different kinds of statements.  Their meaning is explained
   below in the comments for IRStmt.

   Those marked META do not represent code, but rather extra
   information about the code.  These statements can be removed
   without affecting the functional behaviour of the code, however
   they are required by some IR consumers such as tools that
   instrument the code.
*/

typedef 
   enum {
      Ist_NoOp=0x1E00,
      Ist_IMark,     /* META */
      Ist_AbiHint,   /* META */
      Ist_Put,
      Ist_PutI,
      Ist_WrTmp,
      Ist_Store,
      Ist_LoadG,
      Ist_StoreG,
      Ist_CAS,
      Ist_LLSC,
      Ist_Dirty,
      Ist_MBE,
      Ist_Exit
   } 
   IRStmtTag;

/* A statement.  Stored as a tagged union.  'tag' indicates what kind
   of expression this is.  'Ist' is the union that holds the fields.
   If an IRStmt 'st' has st.tag equal to Iex_Store, then it's a store
   statement, and the fields can be accessed with
   'st.Ist.Store.<fieldname>'.

   For each kind of statement, we show what it looks like when
   pretty-printed with ppIRStmt().
*/
typedef
   struct _IRStmt {
      IRStmtTag tag;
      union {
         /* A no-op (usually resulting from IR optimisation).  Can be
            omitted without any effect.

            ppIRStmt output: IR-NoOp
         */
         struct {
	 } NoOp;

         /* META: instruction mark.  Marks the start of the statements
            that represent a single machine instruction (the end of
            those statements is marked by the next IMark or the end of
            the IRSB).  Contains the address and length of the
            instruction.

            It also contains a delta value.  The delta must be
            subtracted from a guest program counter value before
            attempting to establish, by comparison with the address
            and length values, whether or not that program counter
            value refers to this instruction.  For x86, amd64, ppc32,
            ppc64 and arm, the delta value is zero.  For Thumb
            instructions, the delta value is one.  This is because, on
            Thumb, guest PC values (guest_R15T) are encoded using the
            top 31 bits of the instruction address and a 1 in the lsb;
            hence they appear to be (numerically) 1 past the start of
            the instruction they refer to.  IOW, guest_R15T on ARM
            holds a standard ARM interworking address.

            ppIRStmt output: ------ IMark(<addr>, <len>, <delta>) ------,
                         eg. ------ IMark(0x4000792, 5, 0) ------,
         */
         struct {
            Addr   addr;   /* instruction address */
            UInt   len;    /* instruction length */
            UChar  delta;  /* addr = program counter as encoded in guest state
                                     - delta */
         } IMark;

         /* META: An ABI hint, which says something about this
            platform's ABI.

            At the moment, the only AbiHint is one which indicates
            that a given chunk of address space, [base .. base+len-1],
            has become undefined.  This is used on amd64-linux and
            some ppc variants to pass stack-redzoning hints to whoever
            wants to see them.  It also indicates the address of the
            next (dynamic) instruction that will be executed.  This is
            to help Memcheck to origin tracking.

            ppIRStmt output: ====== AbiHint(<base>, <len>, <nia>) ======
                         eg. ====== AbiHint(t1, 16, t2) ======
         */
         struct {
            IRExpr* base;     /* Start  of undefined chunk */
            Int     len;      /* Length of undefined chunk */
            IRExpr* nia;      /* Address of next (guest) insn */
         } AbiHint;

         /* Write a guest register, at a fixed offset in the guest state.
            ppIRStmt output: PUT(<offset>) = <data>, eg. PUT(60) = t1
         */
         struct {
            Int     offset;   /* Offset into the guest state */
            IRExpr* data;     /* The value to write */
         } Put;

         /* Write a guest register, at a non-fixed offset in the guest
            state.  See the comment for GetI expressions for more
            information.

            ppIRStmt output: PUTI<descr>[<ix>,<bias>] = <data>,
                         eg. PUTI(64:8xF64)[t5,0] = t1
         */
         struct {
            IRPutI* details;
         } PutI;

         /* Assign a value to a temporary.  Note that SSA rules require
            each tmp is only assigned to once.  IR sanity checking will
            reject any block containing a temporary which is not assigned
            to exactly once.

            ppIRStmt output: t<tmp> = <data>, eg. t1 = 3
         */
         struct {
            IRTemp  tmp;   /* Temporary  (LHS of assignment) */
            IRExpr* data;  /* Expression (RHS of assignment) */
         } WrTmp;

         /* Write a value to memory.  This is a normal store, not a
            Store-Conditional.  To represent a Store-Conditional,
            instead use IRStmt.LLSC.
            ppIRStmt output: ST<end>(<addr>) = <data>, eg. STle(t1) = t2
         */
         struct {
            IREndness end;    /* Endianness of the store */
            IRExpr*   addr;   /* store address */
            IRExpr*   data;   /* value to write */
         } Store;

         /* Guarded store.  Note that this is defined to evaluate all
            expression fields (addr, data) even if the guard evaluates
            to false.
            ppIRStmt output:
              if (<guard>) ST<end>(<addr>) = <data> */
         struct {
            IRStoreG* details;
         } StoreG;

         /* Guarded load.  Note that this is defined to evaluate all
            expression fields (addr, alt) even if the guard evaluates
            to false.
            ppIRStmt output:
              t<tmp> = if (<guard>) <cvt>(LD<end>(<addr>)) else <alt> */
         struct {
            IRLoadG* details;
         } LoadG;

         /* Do an atomic compare-and-swap operation.  Semantics are
            described above on a comment at the definition of IRCAS.

            ppIRStmt output:
               t<tmp> = CAS<end>(<addr> :: <expected> -> <new>)
            eg
               t1 = CASle(t2 :: t3->Add32(t3,1))
               which denotes a 32-bit atomic increment 
               of a value at address t2

            A double-element CAS may also be denoted, in which case <tmp>,
            <expected> and <new> are all pairs of items, separated by
            commas.
         */
         struct {
            IRCAS* details;
         } CAS;

         /* Either Load-Linked or Store-Conditional, depending on
            STOREDATA.

            If STOREDATA is NULL then this is a Load-Linked, meaning
            that data is loaded from memory as normal, but a
            'reservation' for the address is also lodged in the
            hardware.

               result = Load-Linked(addr, end)

            The data transfer type is the type of RESULT (I32, I64,
            etc).  ppIRStmt output:

               result = LD<end>-Linked(<addr>), eg. LDbe-Linked(t1)

            If STOREDATA is not NULL then this is a Store-Conditional,
            hence:

               result = Store-Conditional(addr, storedata, end)

            The data transfer type is the type of STOREDATA and RESULT
            has type Ity_I1. The store may fail or succeed depending
            on the state of a previously lodged reservation on this
            address.  RESULT is written 1 if the store succeeds and 0
            if it fails.  eg ppIRStmt output:

               result = ( ST<end>-Cond(<addr>) = <storedata> )
               eg t3 = ( STbe-Cond(t1, t2) )

            In all cases, the address must be naturally aligned for
            the transfer type -- any misaligned addresses should be
            caught by a dominating IR check and side exit.  This
            alignment restriction exists because on at least some
            LL/SC platforms (ppc), stwcx. etc will trap w/ SIGBUS on
            misaligned addresses, and we have to actually generate
            stwcx. on the host, and we don't want it trapping on the
            host.

            Summary of rules for transfer type:
              STOREDATA == NULL (LL):
                transfer type = type of RESULT
              STOREDATA != NULL (SC):
                transfer type = type of STOREDATA, and RESULT :: Ity_I1
         */
         struct {
            IREndness end;
            IRTemp    result;
            IRExpr*   addr;
            IRExpr*   storedata; /* NULL => LL, non-NULL => SC */
         } LLSC;

         /* Call (possibly conditionally) a C function that has side
            effects (ie. is "dirty").  See the comments above the
            IRDirty type declaration for more information.

            ppIRStmt output:
               t<tmp> = DIRTY <guard> <effects> 
                  ::: <callee>(<args>)
            eg.
               t1 = DIRTY t27 RdFX-gst(16,4) RdFX-gst(60,4)
                     ::: foo{0x380035f4}(t2)
         */       
         struct {
            IRDirty* details;
         } Dirty;

         /* A memory bus event - a fence, or acquisition/release of the
            hardware bus lock.  IR optimisation treats all these as fences
            across which no memory references may be moved.
            ppIRStmt output: MBusEvent-Fence,
                             MBusEvent-BusLock, MBusEvent-BusUnlock.
         */
         struct {
            IRMBusEvent event;
         } MBE;

         /* Conditional exit from the middle of an IRSB.
            ppIRStmt output: if (<guard>) goto {<jk>} <dst>
                         eg. if (t69) goto {Boring} 0x4000AAA:I32
            If <guard> is true, the guest state is also updated by
            PUT-ing <dst> at <offsIP>.  This is done because a
            taken exit must update the guest program counter.
         */
         struct {
            IRExpr*    guard;    /* Conditional expression */
            IRConst*   dst;      /* Jump target (constant only) */
            IRJumpKind jk;       /* Jump kind */
            Int        offsIP;   /* Guest state offset for IP */
         } Exit;
      } Ist;
   }
   IRStmt;

/* Statement constructors. */
extern IRStmt* IRStmt_NoOp    ( void );
extern IRStmt* IRStmt_IMark   ( Addr addr, UInt len, UChar delta );
extern IRStmt* IRStmt_AbiHint ( IRExpr* base, Int len, IRExpr* nia );
extern IRStmt* IRStmt_Put     ( Int off, IRExpr* data );
extern IRStmt* IRStmt_PutI    ( IRPutI* details );
extern IRStmt* IRStmt_WrTmp   ( IRTemp tmp, IRExpr* data );
extern IRStmt* IRStmt_Store   ( IREndness end, IRExpr* addr, IRExpr* data );
extern IRStmt* IRStmt_StoreG  ( IREndness end, IRExpr* addr, IRExpr* data,
                                IRExpr* guard );
extern IRStmt* IRStmt_LoadG   ( IREndness end, IRLoadGOp cvt, IRTemp dst,
                                IRExpr* addr, IRExpr* alt, IRExpr* guard );
extern IRStmt* IRStmt_CAS     ( IRCAS* details );
extern IRStmt* IRStmt_LLSC    ( IREndness end, IRTemp result,
                                IRExpr* addr, IRExpr* storedata );
extern IRStmt* IRStmt_Dirty   ( IRDirty* details );
extern IRStmt* IRStmt_MBE     ( IRMBusEvent event );
extern IRStmt* IRStmt_Exit    ( IRExpr* guard, IRJumpKind jk, IRConst* dst,
                                Int offsIP );

/* Deep-copy an IRStmt. */
extern IRStmt* deepCopyIRStmt ( const IRStmt* );

/* Pretty-print an IRStmt. */
extern void ppIRStmt ( const IRStmt* );


/* ------------------ Basic Blocks ------------------ */

/* Type environments: a bunch of statements, expressions, etc, are
   incomplete without an environment indicating the type of each
   IRTemp.  So this provides one.  IR temporaries are really just
   unsigned ints and so this provides an array, 0 .. n_types_used-1 of
   them.
*/
typedef
   struct {
      IRType* types;
      Int     types_size;
      Int     types_used;
   }
   IRTypeEnv;

/* Obtain a new IRTemp */
extern IRTemp newIRTemp ( IRTypeEnv*, IRType );

/* Deep-copy a type environment */
extern IRTypeEnv* deepCopyIRTypeEnv ( const IRTypeEnv* );

/* Pretty-print a type environment */
extern void ppIRTypeEnv ( const IRTypeEnv* );


/* Code blocks, which in proper compiler terminology are superblocks
   (single entry, multiple exit code sequences) contain:

   - A table giving a type for each temp (the "type environment")
   - An expandable array of statements
   - An expression of type 32 or 64 bits, depending on the
     guest's word size, indicating the next destination if the block 
     executes all the way to the end, without a side exit
   - An indication of any special actions (JumpKind) needed
     for this final jump.
   - Offset of the IP field in the guest state.  This will be
     updated before the final jump is done.
   
   "IRSB" stands for "IR Super Block".
*/
typedef
   struct {
      IRTypeEnv* tyenv;
      IRStmt**   stmts;
      Int        stmts_size;
      Int        stmts_used;
      IRExpr*    next;
      IRJumpKind jumpkind;
      Int        offsIP;
   }
   IRSB;

/* Allocate a new, uninitialised IRSB */
extern IRSB* emptyIRSB ( void );

/* Deep-copy an IRSB */
extern IRSB* deepCopyIRSB ( const IRSB* );

/* Deep-copy an IRSB, except for the statements list, which set to be
   a new, empty, list of statements. */
extern IRSB* deepCopyIRSBExceptStmts ( const IRSB* );

/* Pretty-print an IRSB */
extern void ppIRSB ( const IRSB* );

/* Append an IRStmt to an IRSB */
extern void addStmtToIRSB ( IRSB*, IRStmt* );


/*---------------------------------------------------------------*/
/*--- Helper functions for the IR                             ---*/
/*---------------------------------------------------------------*/

/* For messing with IR type environments */
extern IRTypeEnv* emptyIRTypeEnv  ( void );

/* What is the type of this expression? */
extern IRType typeOfIRConst ( const IRConst* );
extern IRType typeOfIRTemp  ( const IRTypeEnv*, IRTemp );
extern IRType typeOfIRExpr  ( const IRTypeEnv*, const IRExpr* );

/* What are the arg and result type for this IRLoadGOp? */
extern void typeOfIRLoadGOp ( IRLoadGOp cvt,
                              /*OUT*/IRType* t_res,
                              /*OUT*/IRType* t_arg );

/* Sanity check a BB of IR */
extern void sanityCheckIRSB ( const  IRSB*  bb, 
                              const  HChar* caller,
                              Bool   require_flatness, 
                              IRType guest_word_size );
extern Bool isFlatIRStmt ( const IRStmt* );
extern Bool isFlatIRSB ( const IRSB* );

/* Is this any value actually in the enumeration 'IRType' ? */
extern Bool isPlausibleIRType ( IRType ty );


/*---------------------------------------------------------------*/
/*--- IR injection                                            ---*/
/*---------------------------------------------------------------*/

void vex_inject_ir(IRSB *, IREndness);


#endif /* ndef __LIBVEX_IR_H */

/*---------------------------------------------------------------*/
/*---                                             libvex_ir.h ---*/
/*---------------------------------------------------------------*/
