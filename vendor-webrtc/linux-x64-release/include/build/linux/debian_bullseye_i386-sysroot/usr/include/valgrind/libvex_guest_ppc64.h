
/*---------------------------------------------------------------*/
/*--- begin                              libvex_guest_ppc64.h ---*/
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

#ifndef __LIBVEX_PUB_GUEST_PPC64_H
#define __LIBVEX_PUB_GUEST_PPC64_H

#include "libvex_basictypes.h"

/*
    volatile ==  caller-saved (not preserved across function calls)
non-volatile ==  callee-saved (preserved across function calls)

r0        Volatile register used in function prologs
r1        Stack frame pointer
r2        TOC pointer
r3        Volatile parameter and return value register
r4-r10    Volatile registers used for function parameters
r11       Volatile register used in calls by pointer and as an
          environment pointer for languages which require one
r12       Volatile register used for exception handling and glink code
r13       Reserved for use as system thread ID
r14-r31   Nonvolatile registers used for local variables

f0        Volatile scratch register
f1-f4     Volatile floating point parameter and return value registers
f5-f13    Volatile floating point parameter registers
f14-f31   Nonvolatile registers

LR        Link register (volatile)
CTR       Loop counter register (volatile)
XER       Fixed point exception register (volatile)
FPSCR     Floating point status and control register (volatile)

CR0-CR1   Volatile condition code register fields
CR2-CR4   Nonvolatile condition code register fields
CR5-CR7   Volatile condition code register fields

On processors with the VMX feature.

v0-v1     Volatile scratch registers
v2-v13    Volatile vector parameters registers
v14-v19   Volatile scratch registers
v20-v31   Non-volatile registers
vrsave    Non-volatile 32-bit register
*/


/*---------------------------------------------------------------*/
/*--- Vex's representation of the PPC64 CPU state             ---*/
/*---------------------------------------------------------------*/

#define VEX_GUEST_PPC64_REDIR_STACK_SIZE (16/*entries*/ * 2/*words per entry*/)

typedef
   struct {
     /* Event check fail addr, counter, and padding to make GPR0 16
        aligned. */
      /*   0 */ ULong  host_EvC_FAILADDR;
      /*   8 */ UInt   host_EvC_COUNTER;
      /*  12 */ UInt   pad0;
      /* Add 16 to all of the offsets below .. */
      /* General Purpose Registers */
      /*   0 */ ULong guest_GPR0;
      /*   8 */ ULong guest_GPR1;
      /*  16 */ ULong guest_GPR2;
      /*  24 */ ULong guest_GPR3;
      /*  32 */ ULong guest_GPR4;
      /*  40 */ ULong guest_GPR5;
      /*  48 */ ULong guest_GPR6;
      /*  56 */ ULong guest_GPR7;
      /*  64 */ ULong guest_GPR8;
      /*  72 */ ULong guest_GPR9;
      /*  80 */ ULong guest_GPR10;
      /*  88 */ ULong guest_GPR11;
      /*  96 */ ULong guest_GPR12;
      /* 104 */ ULong guest_GPR13;
      /* 112 */ ULong guest_GPR14;
      /* 120 */ ULong guest_GPR15;
      /* 128 */ ULong guest_GPR16;
      /* 136 */ ULong guest_GPR17;
      /* 144 */ ULong guest_GPR18;
      /* 152 */ ULong guest_GPR19;
      /* 160 */ ULong guest_GPR20;
      /* 168 */ ULong guest_GPR21;
      /* 176 */ ULong guest_GPR22;
      /* 184 */ ULong guest_GPR23;
      /* 192 */ ULong guest_GPR24;
      /* 200 */ ULong guest_GPR25;
      /* 208 */ ULong guest_GPR26;
      /* 216 */ ULong guest_GPR27;
      /* 224 */ ULong guest_GPR28;
      /* 232 */ ULong guest_GPR29;
      /* 240 */ ULong guest_GPR30;
      /* 248 */ ULong guest_GPR31;

      // Vector Registers, Floating Point Registers, and VSX Registers
      // With ISA 2.06, the "Vector-Scalar Floating-point" category
      // provides facilities to support vector and scalar binary floating-
      // point operations.  A unified register file is an integral part
      // of this new facility, combining floating point and vector registers
      // using a 64x128-bit vector.  These are referred to as VSR[0..63].
      // The floating point registers are now mapped into double word element 0
      // of VSR[0..31]. The 32x128-bit vector registers defined by the "Vector
      // Facility [Category: Vector]" are now mapped to VSR[32..63].

      // IMPORTANT: the user of libvex must place the guest state so as
      // to ensure that guest_VSR{0..63}, and any shadows thereof, are
      // 16-aligned.

      /*  256 */ U128 guest_VSR0;
      /*  272 */ U128 guest_VSR1;
      /*  288 */ U128 guest_VSR2;
      /*  304 */ U128 guest_VSR3;
      /*  320 */ U128 guest_VSR4;
      /*  336 */ U128 guest_VSR5;
      /*  352 */ U128 guest_VSR6;
      /*  368 */ U128 guest_VSR7;
      /*  384 */ U128 guest_VSR8;
      /*  400 */ U128 guest_VSR9;
      /*  416 */ U128 guest_VSR10;
      /*  432 */ U128 guest_VSR11;
      /*  448 */ U128 guest_VSR12;
      /*  464 */ U128 guest_VSR13;
      /*  480 */ U128 guest_VSR14;
      /*  496 */ U128 guest_VSR15;
      /*  512 */ U128 guest_VSR16;
      /*  528 */ U128 guest_VSR17;
      /*  544 */ U128 guest_VSR18;
      /*  560 */ U128 guest_VSR19;
      /*  576 */ U128 guest_VSR20;
      /*  592 */ U128 guest_VSR21;
      /*  608 */ U128 guest_VSR22;
      /*  624 */ U128 guest_VSR23;
      /*  640 */ U128 guest_VSR24;
      /*  656 */ U128 guest_VSR25;
      /*  672 */ U128 guest_VSR26;
      /*  688 */ U128 guest_VSR27;
      /*  704 */ U128 guest_VSR28;
      /*  720 */ U128 guest_VSR29;
      /*  736 */ U128 guest_VSR30;
      /*  752 */ U128 guest_VSR31;
      /*  768 */ U128 guest_VSR32;
      /*  784 */ U128 guest_VSR33;
      /*  800 */ U128 guest_VSR34;
      /*  816 */ U128 guest_VSR35;
      /*  832 */ U128 guest_VSR36;
      /*  848 */ U128 guest_VSR37;
      /*  864 */ U128 guest_VSR38;
      /*  880 */ U128 guest_VSR39;
      /*  896 */ U128 guest_VSR40;
      /*  912 */ U128 guest_VSR41;
      /*  928 */ U128 guest_VSR42;
      /*  944 */ U128 guest_VSR43;
      /*  960 */ U128 guest_VSR44;
      /*  976 */ U128 guest_VSR45;
      /*  992 */ U128 guest_VSR46;
      /* 1008 */ U128 guest_VSR47;
      /* 1024 */ U128 guest_VSR48;
      /* 1040 */ U128 guest_VSR49;
      /* 1056 */ U128 guest_VSR50;
      /* 1072 */ U128 guest_VSR51;
      /* 1088 */ U128 guest_VSR52;
      /* 1104 */ U128 guest_VSR53;
      /* 1120 */ U128 guest_VSR54;
      /* 1136 */ U128 guest_VSR55;
      /* 1152 */ U128 guest_VSR56;
      /* 1168 */ U128 guest_VSR57;
      /* 1184 */ U128 guest_VSR58;
      /* 1200 */ U128 guest_VSR59;
      /* 1216 */ U128 guest_VSR60;
      /* 1232 */ U128 guest_VSR61;
      /* 1248 */ U128 guest_VSR62;
      /* 1264 */ U128 guest_VSR63;

      /* 1280 */ ULong guest_CIA;    // IP (no arch visible register)
      /* 1288 */ ULong guest_LR;     // Link Register
      /* 1296 */ ULong guest_CTR;    // Count Register

      /* XER pieces */
      /* 1304 */ UChar guest_XER_SO; /* in lsb */
      /* 1305 */ UChar guest_XER_OV; /* in lsb */
      /* 1306 */ UChar guest_XER_OV32; /* in lsb */
      /* 1307 */ UChar guest_XER_CA; /* in lsb */
      /* 1308 */ UChar guest_XER_CA32; /* in lsb */
      /* 1309 */ UChar guest_XER_BC; /* all bits */

      /* CR pieces */
      /* 1310 */ UChar guest_CR0_321; /* in [3:1] */
      /* 1311 */ UChar guest_CR0_0;   /* in lsb */
      /* 1312 */ UChar guest_CR1_321; /* in [3:1] */
      /* 1313 */ UChar guest_CR1_0;   /* in lsb */
      /* 1314 */ UChar guest_CR2_321; /* in [3:1] */
      /* 1315 */ UChar guest_CR2_0;   /* in lsb */
      /* 1316 */ UChar guest_CR3_321; /* in [3:1] */
      /* 1317 */ UChar guest_CR3_0;   /* in lsb */
      /* 1318 */ UChar guest_CR4_321; /* in [3:1] */
      /* 1319 */ UChar guest_CR4_0;   /* in lsb */
      /* 1320 */ UChar guest_CR5_321; /* in [3:1] */
      /* 1321 */ UChar guest_CR5_0;   /* in lsb */
      /* 1322 */ UChar guest_CR6_321; /* in [3:1] */
      /* 1323 */ UChar guest_CR6_0;   /* in lsb */
      /* 1324 */ UChar guest_CR7_321; /* in [3:1] */
      /* 1325 */ UChar guest_CR7_0;   /* in lsb */

      /* FP Status and  Control Register fields. Only rounding mode fields
       * and Floating-point Condition Code (FPCC) fields are supported.
       */
      /* 1326 */ UChar guest_FPROUND; // Binary Floating Point Rounding Mode
      /* 1327 */ UChar guest_DFPROUND; // Decimal Floating Point Rounding Mode
      /* 1328 */ UChar guest_C_FPCC;   // Floating-point Condition Code
                                       // and Floating-point Condition Code

      /* 1329 */ UChar pad2;
      /* 1330 */ UChar pad3;
      /* 1331 */ UChar pad4;

      /* Vector Save/Restore Register */
      /* 1332 */ UInt guest_VRSAVE;

      /* Vector Status and Control Register */
      /* 1336 */ UInt guest_VSCR;

      /* Emulation notes */
      /* 1340 */ UInt guest_EMNOTE;

      /* gcc adds 4 bytes padding here: pre-empt it. */
      /* 1344 */ UInt  padding;

      /* For icbi: record start and length of area to invalidate */
      /* 1348 */ ULong guest_CMSTART;
      /* 1356 */ ULong guest_CMLEN;

      /* Used to record the unredirected guest address at the start of
         a translation whose start has been redirected.  By reading
         this pseudo-register shortly afterwards, the translation can
         find out what the corresponding no-redirection address was.
         Note, this is only set for wrap-style redirects, not for
         replace-style ones. */
      /* 1364 */ ULong guest_NRADDR;
      /* 1372 */ ULong guest_NRADDR_GPR2;

     /* A grows-upwards stack for hidden saves/restores of LR and R2
        needed for function interception and wrapping on ppc64-linux.
        A horrible hack.  REDIR_SP points to the highest live entry,
        and so starts at -1. */
      /* 1380 */ ULong guest_REDIR_SP;
      /* 1388 */ ULong guest_REDIR_STACK[VEX_GUEST_PPC64_REDIR_STACK_SIZE];

      /* Needed for Darwin: CIA at the last SC insn.  Used when backing up
         to restart a syscall that has been interrupted by a signal. */
      /* 1646 */ ULong guest_IP_AT_SYSCALL;

      /* SPRG3, which AIUI is readonly in user space.  Needed for
         threading on AIX. */
      /* 1654 */ ULong guest_SPRG3_RO;

      /* 1662 */ ULong guest_TFHAR;     // Transaction Failure Handler Address Register
      /* 1670 */ ULong guest_TEXASR;    // Transaction EXception And Summary Register
      /* 1678 */ ULong guest_TFIAR;     // Transaction Failure Instruction Address Register
      /* 1686 */ ULong guest_PPR;       // Program Priority register
      /* 1694 */ UInt  guest_TEXASRU;   // Transaction EXception And Summary Register Upper
      /* 1698 */ UInt  guest_PSPB;      // Problem State Priority Boost register
      /* 1702 */ ULong guest_DSCR;      // Data Stream Control register

      /* Padding to make it have an 16-aligned size */
      /* 1710 */   UInt  padding1;
      /* 1714 */   UInt  padding2;
      /* 1718 */   UInt  padding3;

   }
   VexGuestPPC64State;


/*---------------------------------------------------------------*/
/*--- Utility functions for PPC64 guest stuff.                ---*/
/*---------------------------------------------------------------*/

/* ALL THE FOLLOWING ARE VISIBLE TO LIBRARY CLIENT */

/* Initialise all guest PPC64 state. */
extern
void LibVEX_GuestPPC64_initialise ( /*OUT*/VexGuestPPC64State* vex_state );


/* Write the given native %CR value to the supplied VexGuestPPC64State
   structure.  Note, %CR is 32-bits even for ppc64. */
extern
void LibVEX_GuestPPC64_put_CR ( UInt cr_native,
                                /*OUT*/VexGuestPPC64State* vex_state );

/* Extract from the supplied VexGuestPPC64State structure the
   corresponding native %CR value.  Note, %CR is 32-bits even for
   ppc64. */
extern
UInt LibVEX_GuestPPC64_get_CR ( /*IN*/const VexGuestPPC64State* vex_state );


/* Write the given native %XER value to the supplied
   VexGuestPPC64State structure.  Note, %XER is 32-bits even for
   ppc64. */
extern
void LibVEX_GuestPPC64_put_XER ( UInt xer_native,
                                 /*OUT*/VexGuestPPC64State* vex_state );

/* Extract from the supplied VexGuestPPC64State structure the
   corresponding native %XER value.  Note, %CR is 32-bits even for
   ppc64. */
extern
UInt LibVEX_GuestPPC64_get_XER ( /*IN*/const VexGuestPPC64State* vex_state );

#endif /* ndef __LIBVEX_PUB_GUEST_PPC64_H */


/*---------------------------------------------------------------*/
/*---                                    libvex_guest_ppc64.h ---*/
/*---------------------------------------------------------------*/
