
/*---------------------------------------------------------------*/
/*--- begin                                libvex_guest_x86.h ---*/
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

#ifndef __LIBVEX_PUB_GUEST_X86_H
#define __LIBVEX_PUB_GUEST_X86_H

#include "libvex_basictypes.h"
#include "libvex_emnote.h"


/*---------------------------------------------------------------*/
/*--- Vex's representation of the x86 CPU state.              ---*/
/*---------------------------------------------------------------*/

/* The integer parts should be pretty straightforward. */

/* Hmm, subregisters.  The simulated state is stored in memory in the
   host's byte ordering, so we can't say here what the offsets of %ax,
   %al, %ah etc are since that depends on the host's byte ordering,
   which we don't know. */

/* FPU.  For now, just simulate 8 64-bit registers, their tags, and
   the reg-stack top pointer, of which only the least significant
   three bits are relevant.

   The model is:
     F0 .. F7 are the 8 registers.  FTOP[2:0] contains the 
     index of the current 'stack top' -- pretty meaningless, but
     still.  FTOP is a 32-bit value.  FTOP[31:3] can be anything
     (not guaranteed to be zero).

     When a value is pushed onto the stack, ftop is first replaced by 
     (ftop-1) & 7, and then F[ftop] is assigned the value.

     When a value is popped off the stack, the value is read from
     F[ftop], and then ftop is replaced by (ftop+1) & 7.

     In general, a reference to a register ST(i) actually references
     F[ (ftop+i) & 7 ].

   FTAG0 .. FTAG0+7 are the tags.  Each is a byte, zero means empty,
   non-zero means non-empty.

   The general rule appears to be that a read or modify of a register
   gets a stack underflow fault if the register is empty.  A write of
   a register (only a write, not a modify) gets a stack overflow fault
   if the register is full.  Note that "over" vs "under" is pretty
   meaningless since the FP stack pointer can move around arbitrarily,
   so it's really just two different kinds of exceptions:
   register-empty and register full.

   Naturally Intel (in its infinite wisdom) has seen fit to throw in
   some ad-hoc inconsistencies to the fault-generation rules of the
   above para, just to complicate everything.  Known inconsistencies:

   * fxam can read a register in any state without taking an underflow
     fault.

   * fst from st(0) to st(i) does not take an overflow fault even if the
     destination is already full.

   FPROUND[1:0] is the FPU's notional rounding mode, encoded as per
   the IRRoundingMode type (see libvex_ir.h).  This just happens to be
   the Intel encoding.  Note carefully, the rounding mode is only
   observed on float-to-int conversions, and on float-to-float
   rounding, but not for general float-to-float operations, which are
   always rounded-to-nearest.

   Loads/stores of the FPU control word are faked accordingly -- on
   loads, everything except the rounding mode is ignored, and on
   stores, you get a vanilla control world (0x037F) with the rounding
   mode patched in.  Hence the only values you can get are 0x037F,
   0x077F, 0x0B7F or 0x0F7F.  Vex will emit an emulation warning if
   you try and load a control word which either (1) unmasks FP
   exceptions, or (2) changes the default (80-bit) precision.

   FC3210 contains the C3, C2, C1 and C0 bits in the same place they
   are in the FPU's status word.  (bits 14, 10, 9, 8 respectively).
   All other bits should be zero.  The relevant mask to select just
   those bits is 0x4700.  To select C3, C2 and C0 only, the mask is
   0x4500.  

   SSEROUND[1:0] is the SSE unit's notional rounding mode, encoded as
   per the IRRoundingMode type.  As with the FPU control word, the
   rounding mode is the only part of %MXCSR that Vex observes.  On
   storing %MXCSR, you will get a vanilla word (0x1F80) with the
   rounding mode patched in.  Hence the only values you will get are
   0x1F80, 0x3F80, 0x5F80 or 0x7F80.  Vex will emit an emulation
   warning if you try and load a control word which either (1) unmasks
   any exceptions, (2) sets FZ (flush-to-zero) to 1, or (3) sets DAZ
   (denormals-are-zeroes) to 1. 

   Segments: initial prefixes of local and global segment descriptor
   tables are modelled.  guest_LDT is either zero (NULL) or points in
   the host address space to an array of VEX_GUEST_X86_LDT_NENT
   descriptors, which have the type VexGuestX86SegDescr, defined
   below.  Similarly, guest_GDT is either zero or points in the host
   address space to an array of VEX_GUEST_X86_GDT_NENT descriptors.
   The only place where these are used are in the helper function
   x86g_use_seg().  LibVEX's client is responsible for pointing
   guest_LDT and guest_GDT at suitable tables.  The contents of these
   tables are expected not to change during the execution of any given
   superblock, but they may validly be changed by LibVEX's client in
   between superblock executions.

   Since x86g_use_seg() only expects these tables to have
   VEX_GUEST_X86_{LDT,GDT}_NENT entries, LibVEX's client should not
   attempt to write entries beyond those limits.
*/
typedef
   struct {
      /* Event check fail addr and counter. */
      UInt  host_EvC_FAILADDR; /* 0 */
      UInt  host_EvC_COUNTER;  /* 4 */
      UInt  guest_EAX;         /* 8 */
      UInt  guest_ECX;
      UInt  guest_EDX;
      UInt  guest_EBX;
      UInt  guest_ESP;
      UInt  guest_EBP;
      UInt  guest_ESI;
      UInt  guest_EDI;         /* 36 */

      /* 4-word thunk used to calculate O S Z A C P flags. */
      UInt  guest_CC_OP;       /* 40 */
      UInt  guest_CC_DEP1;
      UInt  guest_CC_DEP2;
      UInt  guest_CC_NDEP;     /* 52 */
      /* The D flag is stored here, encoded as either -1 or +1 */
      UInt  guest_DFLAG;       /* 56 */
      /* Bit 21 (ID) of eflags stored here, as either 0 or 1. */
      UInt  guest_IDFLAG;      /* 60 */
      /* Bit 18 (AC) of eflags stored here, as either 0 or 1. */
      UInt  guest_ACFLAG;      /* 64 */

      /* EIP */
      UInt  guest_EIP;         /* 68 */

      /* FPU */
      ULong guest_FPREG[8];    /* 72 */
      UChar guest_FPTAG[8];   /* 136 */
      UInt  guest_FPROUND;    /* 144 */
      UInt  guest_FC3210;     /* 148 */
      UInt  guest_FTOP;       /* 152 */

      /* SSE */
      UInt  guest_SSEROUND;   /* 156 */
      U128  guest_XMM0;       /* 160 */
      U128  guest_XMM1;
      U128  guest_XMM2;
      U128  guest_XMM3;
      U128  guest_XMM4;
      U128  guest_XMM5;
      U128  guest_XMM6;
      U128  guest_XMM7;

      /* Segment registers. */
      UShort guest_CS;
      UShort guest_DS;
      UShort guest_ES;
      UShort guest_FS;
      UShort guest_GS;
      UShort guest_SS;
      /* LDT/GDT stuff. */
      ULong  guest_LDT; /* host addr, a VexGuestX86SegDescr* */
      ULong  guest_GDT; /* host addr, a VexGuestX86SegDescr* */

      /* Emulation notes */
      UInt   guest_EMNOTE;

      /* For clflush/clinval: record start and length of area */
      UInt guest_CMSTART;
      UInt guest_CMLEN;

      /* Used to record the unredirected guest address at the start of
         a translation whose start has been redirected.  By reading
         this pseudo-register shortly afterwards, the translation can
         find out what the corresponding no-redirection address was.
         Note, this is only set for wrap-style redirects, not for
         replace-style ones. */
      UInt guest_NRADDR;

      /* Used for Darwin syscall dispatching. */
      UInt guest_SC_CLASS;

      /* Needed for Darwin (but mandated for all guest architectures):
         EIP at the last syscall insn (int 0x80/81/82, sysenter,
         syscall).  Used when backing up to restart a syscall that has
         been interrupted by a signal. */
      UInt guest_IP_AT_SYSCALL;

      /* Padding to make it have an 16-aligned size */
      UInt padding1;
      UInt padding2;
      UInt padding3;
   }
   VexGuestX86State;

#define VEX_GUEST_X86_LDT_NENT /*64*/ 8192 /* use complete LDT */
#define VEX_GUEST_X86_GDT_NENT /*16*/ 8192 /* use complete GDT */


/*---------------------------------------------------------------*/
/*--- Types for x86 guest stuff.                              ---*/
/*---------------------------------------------------------------*/

/* VISIBLE TO LIBRARY CLIENT */

/* This is the hardware-format for a segment descriptor, ie what the
   x86 actually deals with.  It is 8 bytes long.  It's ugly. */

typedef struct {
    union {
       struct {
          UShort  LimitLow;
          UShort  BaseLow;
          UInt    BaseMid         : 8;
          UInt    Type            : 5;
          UInt    Dpl             : 2;
          UInt    Pres            : 1;
          UInt    LimitHi         : 4;
          UInt    Sys             : 1;
          UInt    Reserved_0      : 1;
          UInt    Default_Big     : 1;
          UInt    Granularity     : 1;
          UInt    BaseHi          : 8;
       } Bits;
       struct {
          UInt word1;
          UInt word2;
       } Words;
    }
    LdtEnt;
} VexGuestX86SegDescr;


/*---------------------------------------------------------------*/
/*--- Utility functions for x86 guest stuff.                  ---*/
/*---------------------------------------------------------------*/

/* ALL THE FOLLOWING ARE VISIBLE TO LIBRARY CLIENT */

/* Initialise all guest x86 state.  The FPU is put in default mode. */
extern
void LibVEX_GuestX86_initialise ( /*OUT*/VexGuestX86State* vex_state );


/* Extract from the supplied VexGuestX86State structure the
   corresponding native %eflags value. */
extern 
UInt LibVEX_GuestX86_get_eflags ( /*IN*/const VexGuestX86State* vex_state );

/* Put eflags into the given state. */
extern
void LibVEX_GuestX86_put_eflags ( UInt eflags,
                                  /*MOD*/VexGuestX86State* vex_state );

/* Set the carry flag in the given state to 'new_carry_flag', which
   should be zero or one. */
extern
void
LibVEX_GuestX86_put_eflag_c ( UInt new_carry_flag,
                              /*MOD*/VexGuestX86State* vex_state );

/* Do x87 save from the supplied VexGuestX86State structure and store the
   result at the given address which represents a buffer of at least 108
   bytes. */
extern
void LibVEX_GuestX86_get_x87 ( /*IN*/VexGuestX86State* vex_state,
                               /*OUT*/UChar* x87_state );

/* Do x87 restore from the supplied address and store read values to the given
   VexGuestX86State structure. */
extern
VexEmNote LibVEX_GuestX86_put_x87 ( /*IN*/UChar* x87_state,
                                    /*MOD*/VexGuestX86State* vex_state);

/* Return mxcsr from the supplied VexGuestX86State structure. */
extern
UInt LibVEX_GuestX86_get_mxcsr ( /*IN*/VexGuestX86State* vex_state );

/* Modify the given VexGuestX86State structure according to the passed mxcsr
   value. */
extern
VexEmNote LibVEX_GuestX86_put_mxcsr ( /*IN*/UInt mxcsr,
                                      /*MOD*/VexGuestX86State* vex_state);

#endif /* ndef __LIBVEX_PUB_GUEST_X86_H */

/*---------------------------------------------------------------*/
/*---                                      libvex_guest_x86.h ---*/
/*---------------------------------------------------------------*/
