
/*---------------------------------------------------------------*/
/*--- begin                              libvex_guest_amd64.h ---*/
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

#ifndef __LIBVEX_PUB_GUEST_AMD64_H
#define __LIBVEX_PUB_GUEST_AMD64_H

#include "libvex_basictypes.h"
#include "libvex_emnote.h"


/*---------------------------------------------------------------*/
/*--- Vex's representation of the AMD64 CPU state.            ---*/
/*---------------------------------------------------------------*/

/* See detailed comments at the top of libvex_guest_x86.h for
   further info.  This representation closely follows the
   x86 representation.
*/


typedef
   struct {
      /* Event check fail addr, counter, and padding to make RAX 16
         aligned. */
      /*   0 */ ULong  host_EvC_FAILADDR;
      /*   8 */ UInt   host_EvC_COUNTER;
      /*  12 */ UInt   pad0;
      /*  16 */ ULong  guest_RAX;
      /*  24 */ ULong  guest_RCX;
      /*  32 */ ULong  guest_RDX;
      /*  40 */ ULong  guest_RBX;
      /*  48 */ ULong  guest_RSP;
      /*  56 */ ULong  guest_RBP;
      /*  64 */ ULong  guest_RSI;
      /*  72 */ ULong  guest_RDI;
      /*  80 */ ULong  guest_R8;
      /*  88 */ ULong  guest_R9;
      /*  96 */ ULong  guest_R10;
      /* 104 */ ULong  guest_R11;
      /* 112 */ ULong  guest_R12;
      /* 120 */ ULong  guest_R13;
      /* 128 */ ULong  guest_R14;
      /* 136 */ ULong  guest_R15;
      /* 4-word thunk used to calculate O S Z A C P flags. */
      /* 144 */ ULong  guest_CC_OP;
      /* 152 */ ULong  guest_CC_DEP1;
      /* 160 */ ULong  guest_CC_DEP2;
      /* 168 */ ULong  guest_CC_NDEP;
      /* The D flag is stored here, encoded as either -1 or +1 */
      /* 176 */ ULong  guest_DFLAG;
      /* 184 */ ULong  guest_RIP;
      /* Bit 18 (AC) of eflags stored here, as either 0 or 1. */
      /* ... */ ULong  guest_ACFLAG;
      /* Bit 21 (ID) of eflags stored here, as either 0 or 1. */
      /* 192 */ ULong guest_IDFLAG;
      /* Probably a lot more stuff too. 
         D,ID flags
         16  128-bit SSE registers
         all the old x87 FPU gunk
         segment registers */

      /* HACK to e.g. make tls on amd64-linux/solaris work.  %fs only ever seems
         to hold a constant value (zero on linux main thread, 0x63 in other
         threads), and so guest_FS_CONST holds
         the 64-bit offset associated with this constant %fs value. */
      /* 200 */ ULong guest_FS_CONST;

      /* YMM registers.  Note that these must be allocated
         consecutively in order that the SSE4.2 PCMP{E,I}STR{I,M}
         helpers can treat them as an array.  YMM16 is a fake reg used
         as an intermediary in handling aforementioned insns. */
      /* 208 */ULong guest_SSEROUND;
      /* 216 */U256  guest_YMM0;
      U256  guest_YMM1;
      U256  guest_YMM2;
      U256  guest_YMM3;
      U256  guest_YMM4;
      U256  guest_YMM5;
      U256  guest_YMM6;
      U256  guest_YMM7;
      U256  guest_YMM8;
      U256  guest_YMM9;
      U256  guest_YMM10;
      U256  guest_YMM11;
      U256  guest_YMM12;
      U256  guest_YMM13;
      U256  guest_YMM14;
      U256  guest_YMM15;
      U256  guest_YMM16;

      /* FPU */
      /* Note.  Setting guest_FTOP to be ULong messes up the
         delicately-balanced PutI/GetI optimisation machinery.
         Therefore best to leave it as a UInt. */
      UInt  guest_FTOP;
      UInt  pad1;
      ULong guest_FPREG[8];
      UChar guest_FPTAG[8];
      ULong guest_FPROUND;
      ULong guest_FC3210;

      /* Emulation notes */
      UInt  guest_EMNOTE;
      UInt  pad2;

      /* Translation-invalidation area description.  Not used on amd64
         (there is no invalidate-icache insn), but needed so as to
         allow users of the library to uniformly assume that the guest
         state contains these two fields -- otherwise there is
         compilation breakage.  On amd64, these two fields are set to
         zero by LibVEX_GuestAMD64_initialise and then should be
         ignored forever thereafter. */
      ULong guest_CMSTART;
      ULong guest_CMLEN;

      /* Used to record the unredirected guest address at the start of
         a translation whose start has been redirected.  By reading
         this pseudo-register shortly afterwards, the translation can
         find out what the corresponding no-redirection address was.
         Note, this is only set for wrap-style redirects, not for
         replace-style ones. */
      ULong guest_NRADDR;

      /* Used for Darwin syscall dispatching. */
      ULong guest_SC_CLASS;

      /* HACK to make e.g. tls on darwin work, wine on linux work, ...
         %gs only ever seems to hold a constant value (e.g. 0x60 on darwin,
         0x6b on linux), and so guest_GS_CONST holds the 64-bit offset
         associated with this constant %gs value.  (A direct analogue
         of the %fs-const hack for amd64-linux/solaris). */
      ULong guest_GS_CONST;

      /* Needed for Darwin (but mandated for all guest architectures):
         RIP at the last syscall insn (int 0x80/81/82, sysenter,
         syscall).  Used when backing up to restart a syscall that has
         been interrupted by a signal. */
      ULong guest_IP_AT_SYSCALL;

      /* Padding to make it have an 16-aligned size */
      ULong pad3;
   }
   VexGuestAMD64State;



/*---------------------------------------------------------------*/
/*--- Utility functions for amd64 guest stuff.                ---*/
/*---------------------------------------------------------------*/

/* ALL THE FOLLOWING ARE VISIBLE TO LIBRARY CLIENT */

/* Initialise all guest amd64 state.  The FPU is put in default
   mode. */
extern
void LibVEX_GuestAMD64_initialise ( /*OUT*/VexGuestAMD64State* vex_state );


/* Extract from the supplied VexGuestAMD64State structure the
   corresponding native %rflags value. */
extern 
ULong LibVEX_GuestAMD64_get_rflags ( /*IN*/const VexGuestAMD64State* vex_state );

/* Put rflags into the given state. */
extern
void LibVEX_GuestAMD64_put_rflags ( ULong rflags,
                                    /*MOD*/VexGuestAMD64State* vex_state );

/* Set the carry flag in the given state to 'new_carry_flag', which
   should be zero or one. */
extern
void
LibVEX_GuestAMD64_put_rflag_c ( ULong new_carry_flag,
                                /*MOD*/VexGuestAMD64State* vex_state );

/* Do FXSAVE from the supplied VexGuestAMD64tate structure and store the
   result at the given address which represents a buffer of at least 416
   bytes. */
extern
void LibVEX_GuestAMD64_fxsave ( /*IN*/VexGuestAMD64State* gst,
                                /*OUT*/HWord fp_state );

/* Do FXRSTOR from the supplied address and store read values to the given
   VexGuestAMD64State structure. */
extern
VexEmNote LibVEX_GuestAMD64_fxrstor ( /*IN*/HWord fp_state,
                                      /*MOD*/VexGuestAMD64State* gst );

#endif /* ndef __LIBVEX_PUB_GUEST_AMD64_H */

/*---------------------------------------------------------------*/
/*---                                    libvex_guest_amd64.h ---*/
/*---------------------------------------------------------------*/
