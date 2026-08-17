
/*---------------------------------------------------------------*/
/*--- begin                              libvex_guest_ppc32.h ---*/
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

#ifndef __LIBVEX_PUB_GUEST_PPC32_H
#define __LIBVEX_PUB_GUEST_PPC32_H

#include "libvex_basictypes.h"


/*---------------------------------------------------------------*/
/*--- Vex's representation of the PPC32 CPU state             ---*/
/*---------------------------------------------------------------*/

#define VEX_GUEST_PPC32_REDIR_STACK_SIZE (16/*entries*/ * 2/*words per entry*/)

typedef
   struct {
      /* Event check fail addr and counter. */
      /*   0 */ UInt host_EvC_FAILADDR;
      /*   4 */ UInt host_EvC_COUNTER;
      /*   8 */ UInt pad3;
      /*  12 */ UInt pad4; 
      /* Add 16 to all the numbers below.  Sigh. */
      /* General Purpose Registers */
      /*   0 */ UInt guest_GPR0;
      /*   4 */ UInt guest_GPR1;
      /*   8 */ UInt guest_GPR2;
      /*  12 */ UInt guest_GPR3;
      /*  16 */ UInt guest_GPR4;
      /*  20 */ UInt guest_GPR5;
      /*  24 */ UInt guest_GPR6;
      /*  28 */ UInt guest_GPR7;
      /*  32 */ UInt guest_GPR8;
      /*  36 */ UInt guest_GPR9;
      /*  40 */ UInt guest_GPR10;
      /*  44 */ UInt guest_GPR11;
      /*  48 */ UInt guest_GPR12;
      /*  52 */ UInt guest_GPR13;
      /*  56 */ UInt guest_GPR14;
      /*  60 */ UInt guest_GPR15;
      /*  64 */ UInt guest_GPR16;
      /*  68 */ UInt guest_GPR17;
      /*  72 */ UInt guest_GPR18;
      /*  76 */ UInt guest_GPR19;
      /*  80 */ UInt guest_GPR20;
      /*  84 */ UInt guest_GPR21;
      /*  88 */ UInt guest_GPR22;
      /*  92 */ UInt guest_GPR23;
      /*  96 */ UInt guest_GPR24;
      /* 100 */ UInt guest_GPR25;
      /* 104 */ UInt guest_GPR26;
      /* 108 */ UInt guest_GPR27;
      /* 112 */ UInt guest_GPR28;
      /* 116 */ UInt guest_GPR29;
      /* 120 */ UInt guest_GPR30;
      /* 124 */ UInt guest_GPR31;

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

      /*  128 */ U128 guest_VSR0;
      /*  144 */ U128 guest_VSR1;
      /*  160 */ U128 guest_VSR2;
      /*  176 */ U128 guest_VSR3;
      /*  192 */ U128 guest_VSR4;
      /*  208 */ U128 guest_VSR5;
      /*  224 */ U128 guest_VSR6;
      /*  240 */ U128 guest_VSR7;
      /*  256 */ U128 guest_VSR8;
      /*  272 */ U128 guest_VSR9;
      /*  288 */ U128 guest_VSR10;
      /*  304 */ U128 guest_VSR11;
      /*  320 */ U128 guest_VSR12;
      /*  336 */ U128 guest_VSR13;
      /*  352 */ U128 guest_VSR14;
      /*  368 */ U128 guest_VSR15;
      /*  384 */ U128 guest_VSR16;
      /*  400 */ U128 guest_VSR17;
      /*  416 */ U128 guest_VSR18;
      /*  432 */ U128 guest_VSR19;
      /*  448 */ U128 guest_VSR20;
      /*  464 */ U128 guest_VSR21;
      /*  480 */ U128 guest_VSR22;
      /*  496 */ U128 guest_VSR23;
      /*  512 */ U128 guest_VSR24;
      /*  528 */ U128 guest_VSR25;
      /*  544 */ U128 guest_VSR26;
      /*  560 */ U128 guest_VSR27;
      /*  576 */ U128 guest_VSR28;
      /*  592 */ U128 guest_VSR29;
      /*  608 */ U128 guest_VSR30;
      /*  624 */ U128 guest_VSR31;
      /*  640 */ U128 guest_VSR32;
      /*  656 */ U128 guest_VSR33;
      /*  672 */ U128 guest_VSR34;
      /*  688 */ U128 guest_VSR35;
      /*  704 */ U128 guest_VSR36;
      /*  720 */ U128 guest_VSR37;
      /*  736 */ U128 guest_VSR38;
      /*  752 */ U128 guest_VSR39;
      /*  768 */ U128 guest_VSR40;
      /*  784 */ U128 guest_VSR41;
      /*  800 */ U128 guest_VSR42;
      /*  816 */ U128 guest_VSR43;
      /*  832 */ U128 guest_VSR44;
      /*  848 */ U128 guest_VSR45;
      /*  864 */ U128 guest_VSR46;
      /*  880 */ U128 guest_VSR47;
      /*  896 */ U128 guest_VSR48;
      /*  912 */ U128 guest_VSR49;
      /*  928 */ U128 guest_VSR50;
      /*  944 */ U128 guest_VSR51;
      /*  960 */ U128 guest_VSR52;
      /*  976 */ U128 guest_VSR53;
      /*  992 */ U128 guest_VSR54;
      /* 1008 */ U128 guest_VSR55;
      /* 1024 */ U128 guest_VSR56;
      /* 1040 */ U128 guest_VSR57;
      /* 1056 */ U128 guest_VSR58;
      /* 1072 */ U128 guest_VSR59;
      /* 1088 */ U128 guest_VSR60;
      /* 1104 */ U128 guest_VSR61;
      /* 1120 */ U128 guest_VSR62;
      /* 1136 */ U128 guest_VSR63;

      /* 1152 */ UInt guest_CIA;    // IP (no arch visible register)
      /* 1156 */ UInt guest_LR;     // Link Register
      /* 1160 */ UInt guest_CTR;    // Count Register

      /* XER pieces */
      /* 1164 */ UChar guest_XER_SO; /* in lsb */
      /* 1165 */ UChar guest_XER_OV; /* in lsb */
      /* 1166 */ UChar guest_XER_OV32; /* in lsb */
      /* 1167 */ UChar guest_XER_CA; /* in lsb */
      /* 1168 */ UChar guest_XER_CA32; /* in lsb */
      /* 1169 */ UChar guest_XER_BC; /* all bits */

      /* CR pieces */
      /* 1170 */ UChar guest_CR0_321; /* in [3:1] */
      /* 1171 */ UChar guest_CR0_0;   /* in lsb */
      /* 1172 */ UChar guest_CR1_321; /* in [3:1] */
      /* 1173 */ UChar guest_CR1_0;   /* in lsb */
      /* 1174 */ UChar guest_CR2_321; /* in [3:1] */
      /* 1175 */ UChar guest_CR2_0;   /* in lsb */
      /* 1176 */ UChar guest_CR3_321; /* in [3:1] */
      /* 1177 */ UChar guest_CR3_0;   /* in lsb */
      /* 1178 */ UChar guest_CR4_321; /* in [3:1] */
      /* 1179 */ UChar guest_CR4_0;   /* in lsb */
      /* 1180 */ UChar guest_CR5_321; /* in [3:1] */
      /* 1181 */ UChar guest_CR5_0;   /* in lsb */
      /* 1182 */ UChar guest_CR6_321; /* in [3:1] */
      /* 1183 */ UChar guest_CR6_0;   /* in lsb */
      /* 1184 */ UChar guest_CR7_321; /* in [3:1] */
      /* 1185 */ UChar guest_CR7_0;   /* in lsb */

      /* FP Status and  Control Register fields. Only rounding mode fields
       * and Floating-point Condition Code (FPCC) fields in the FPSCR are
       * supported.
       */
      /* 1186 */ UChar guest_FPROUND; // Binary Floating Point Rounding Mode
      /* 1187 */ UChar guest_DFPROUND; // Decimal Floating Point Rounding Mode
      /* 1188 */ UChar guest_C_FPCC;   // Floating-Point Result Class Descriptor
                                       // and Floating-point Condition Code
      /* 1189 */ UChar pad0;
      /* 1190 */ UChar pad1;
      /* 1191 */ UChar pad2;

      /* Vector Save/Restore Register */
      /* 1192 */ UInt guest_VRSAVE;

      /* Vector Status and Control Register */
      /* 1196 */ UInt guest_VSCR;

      /* Emulation notes */
      /* 1200 */ UInt guest_EMNOTE;

      /* For icbi: record start and length of area to invalidate */
      /* 1204 */ UInt guest_CMSTART;
      /* 1208 */ UInt guest_CMLEN;

      /* Used to record the unredirected guest address at the start of
         a translation whose start has been redirected.  By reading
         this pseudo-register shortly afterwards, the translation can
         find out what the corresponding no-redirection address was.
         Note, this is only set for wrap-style redirects, not for
         replace-style ones. */
      /* 1212 */ UInt guest_NRADDR;
      /* 1216 */ UInt guest_NRADDR_GPR2; /* needed by aix */

     /* A grows-upwards stack for hidden saves/restores of LR and R2
        needed for function interception and wrapping on ppc32-aix5.
        A horrible hack.  REDIR_SP points to the highest live entry,
        and so starts at -1. */
      /* 1220 */ UInt guest_REDIR_SP;
      /* 1224 */ UInt guest_REDIR_STACK[VEX_GUEST_PPC32_REDIR_STACK_SIZE];

      /* Needed for Darwin (but mandated for all guest architectures):
         CIA at the last SC insn.  Used when backing up to restart a
         syscall that has been interrupted by a signal. */
      /* 134C */ UInt guest_IP_AT_SYSCALL;

      /* SPRG3, which AIUI is readonly in user space.  Needed for
         threading on AIX. */
      /* 1356 */ UInt guest_SPRG3_RO;
      /* 1360 */ UInt  padding1;
      /* 1364 */ ULong guest_TFHAR;     // Transaction Failure Handler Address Register
      /* 1372 */ ULong guest_TEXASR;    // Transaction EXception And Summary Register
      /* 1380 */ ULong guest_TFIAR;     // Transaction Failure Instruction Address Register
      /* 1388 */ ULong guest_PPR;       // Program Priority register
      /* 1396 */ UInt  guest_TEXASRU;   // Transaction EXception And Summary Register Upper
      /* 1400 */ UInt  guest_PSPB;      // Problem State Priority Boost register
      /* 1404 */ ULong guest_DSCR;      // Data Stream Control register
      /* Padding to make it have an 16-aligned size */
      /* 1408 */ UInt  padding3;
      /* 1412 */ UInt  padding4;
   }
   VexGuestPPC32State;


/*---------------------------------------------------------------*/
/*--- Utility functions for PPC32 guest stuff.                ---*/
/*---------------------------------------------------------------*/

/* ALL THE FOLLOWING ARE VISIBLE TO LIBRARY CLIENT */

/* Initialise all guest PPC32 state. */

extern
void LibVEX_GuestPPC32_initialise ( /*OUT*/VexGuestPPC32State* vex_state );


/* Write the given native %CR value to the supplied VexGuestPPC32State
   structure. */
extern
void LibVEX_GuestPPC32_put_CR ( UInt cr_native,
                                /*OUT*/VexGuestPPC32State* vex_state );

/* Extract from the supplied VexGuestPPC32State structure the
   corresponding native %CR value. */
extern
UInt LibVEX_GuestPPC32_get_CR ( /*IN*/const VexGuestPPC32State* vex_state );


/* Write the given native %XER value to the supplied VexGuestPPC32State
   structure. */
extern
void LibVEX_GuestPPC32_put_XER ( UInt xer_native,
                                 /*OUT*/VexGuestPPC32State* vex_state );

/* Extract from the supplied VexGuestPPC32State structure the
   corresponding native %XER value. */
extern
UInt LibVEX_GuestPPC32_get_XER ( /*IN*/const VexGuestPPC32State* vex_state );

#endif /* ndef __LIBVEX_PUB_GUEST_PPC32_H */


/*---------------------------------------------------------------*/
/*---                                    libvex_guest_ppc32.h ---*/
/*---------------------------------------------------------------*/
