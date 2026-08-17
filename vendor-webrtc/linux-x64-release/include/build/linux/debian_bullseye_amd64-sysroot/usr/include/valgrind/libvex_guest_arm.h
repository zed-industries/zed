
/*---------------------------------------------------------------*/
/*--- begin                                libvex_guest_arm.h ---*/
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
*/

#ifndef __LIBVEX_PUB_GUEST_ARM_H
#define __LIBVEX_PUB_GUEST_ARM_H

#include "libvex_basictypes.h"


/*---------------------------------------------------------------*/
/*--- Vex's representation of the ARM CPU state.              ---*/
/*---------------------------------------------------------------*/

typedef
   struct {
      /* 0 */
      /* Event check fail addr and counter. */
      UInt host_EvC_FAILADDR; /* 0 */
      UInt host_EvC_COUNTER;  /* 4 */
      UInt guest_R0;
      UInt guest_R1;
      UInt guest_R2;
      UInt guest_R3;
      UInt guest_R4;
      UInt guest_R5;
      UInt guest_R6;
      UInt guest_R7;
      UInt guest_R8;
      UInt guest_R9;
      UInt guest_R10;
      UInt guest_R11;
      UInt guest_R12;
      UInt guest_R13;     /* stack pointer */
      UInt guest_R14;     /* link register */
      UInt guest_R15T;
      /* program counter[31:1] ++ [T], encoding both the current
         instruction address and the ARM vs Thumb state of the
         machine.  T==1 is Thumb, T==0 is ARM.  Hence values of the
         form X--(31)--X1 denote a Thumb instruction at location
         X--(31)--X0, values of the form X--(30)--X00 denote an ARM
         instruction at precisely that address, and values of the form
         X--(30)--10 are invalid since they would imply an ARM
         instruction at a non-4-aligned address. */

      /* 4-word thunk used to calculate N(sign) Z(zero) C(carry,
         unsigned overflow) and V(signed overflow) flags. */
      /* 72 */
      UInt guest_CC_OP;
      UInt guest_CC_DEP1;
      UInt guest_CC_DEP2;
      UInt guest_CC_NDEP;

      /* A 32-bit value which is used to compute the APSR.Q (sticky
         saturation) flag, when necessary.  If the value stored here
         is zero, APSR.Q is currently zero.  If it is any other value,
         APSR.Q is currently one. */
      UInt guest_QFLAG32;

      /* 32-bit values to represent APSR.GE0 .. GE3.  Same
         zero-vs-nonzero scheme as for QFLAG32. */
      UInt guest_GEFLAG0;
      UInt guest_GEFLAG1;
      UInt guest_GEFLAG2;
      UInt guest_GEFLAG3;

      /* Various pseudo-regs mandated by Vex or Valgrind. */
      /* Emulation notes */
      UInt guest_EMNOTE;

      /* For clinval/clflush: record start and length of area */
      UInt guest_CMSTART;
      UInt guest_CMLEN;

      /* Used to record the unredirected guest address at the start of
         a translation whose start has been redirected.  By reading
         this pseudo-register shortly afterwards, the translation can
         find out what the corresponding no-redirection address was.
         Note, this is only set for wrap-style redirects, not for
         replace-style ones. */
      UInt guest_NRADDR;

      /* Needed for Darwin (but mandated for all guest architectures):
         program counter at the last syscall insn (int 0x80/81/82,
         sysenter, syscall, svc).  Used when backing up to restart a
         syscall that has been interrupted by a signal. */
      /* 124 */
      UInt guest_IP_AT_SYSCALL;

      /* VFP state.  D0 .. D15 must be 8-aligned. */
      /* 128 */
      ULong guest_D0;
      ULong guest_D1;
      ULong guest_D2;
      ULong guest_D3;
      ULong guest_D4;
      ULong guest_D5;
      ULong guest_D6;
      ULong guest_D7;
      ULong guest_D8;
      ULong guest_D9;
      ULong guest_D10;
      ULong guest_D11;
      ULong guest_D12;
      ULong guest_D13;
      ULong guest_D14;
      ULong guest_D15;
      ULong guest_D16;
      ULong guest_D17;
      ULong guest_D18;
      ULong guest_D19;
      ULong guest_D20;
      ULong guest_D21;
      ULong guest_D22;
      ULong guest_D23;
      ULong guest_D24;
      ULong guest_D25;
      ULong guest_D26;
      ULong guest_D27;
      ULong guest_D28;
      ULong guest_D29;
      ULong guest_D30;
      ULong guest_D31;
      UInt  guest_FPSCR;

      /* Not a town in Cornwall, but instead the TPIDRURO, on of the
         Thread ID registers present in CP15 (the system control
         coprocessor), register set "c13", register 3 (the User
         Read-only Thread ID Register).  arm-linux apparently uses it
         to hold the TLS pointer for the thread.  It's read-only in
         user space.  On Linux it is set in user space by various
         thread-related syscalls. */
      UInt guest_TPIDRURO;

      /* TPIDRURW is also apparently used as a thread register, but one
         controlled entirely by, and writable from, user space.  We model
         it as a completely vanilla piece of integer state. */
      UInt guest_TPIDRURW;

      /* Representation of the Thumb IT state.  ITSTATE is a 32-bit
         value with 4 8-bit lanes.  [7:0] pertain to the next insn to
         execute, [15:8] for the one after that, etc.  The per-insn
         update to ITSTATE is to unsignedly shift it right 8 bits,
         hence introducing a zero byte for the furthest ahead
         instruction.  As per the next para, a zero byte denotes the
         condition ALWAYS.

         Each byte lane has one of the two following formats:

         cccc 0001  for an insn which is part of an IT block.  cccc is
                    the guarding condition (standard ARM condition
                    code) XORd with 0xE, so as to cause 'cccc == 0'
                    to encode the condition ALWAYS.

         0000 0000  for an insn which is not part of an IT block.

         If the bottom 4 bits are zero then the top 4 must be too.

         Given the byte lane for an instruction, the guarding
         condition for the instruction is (((lane >> 4) & 0xF) ^ 0xE).
         This is not as stupid as it sounds, because the front end
         elides the shift.  And the am-I-in-an-IT-block check is
         (lane != 0).

         In the case where (by whatever means) we know at JIT time
         that an instruction is not in an IT block, we can prefix its
         IR with assignments ITSTATE = 0 and hence have iropt fold out
         the testing code.

         The condition "is outside or last in IT block" corresponds
         to the top 24 bits of ITSTATE being zero.
      */
      UInt guest_ITSTATE;
   }
   VexGuestARMState;


/*---------------------------------------------------------------*/
/*--- Utility functions for ARM guest stuff.                  ---*/
/*---------------------------------------------------------------*/

/* ALL THE FOLLOWING ARE VISIBLE TO LIBRARY CLIENT */

/* Initialise all guest ARM state. */

extern
void LibVEX_GuestARM_initialise ( /*OUT*/VexGuestARMState* vex_state );

/* Calculate the ARM flag state from the saved data. */

extern
UInt LibVEX_GuestARM_get_cpsr ( /*IN*/const VexGuestARMState* vex_state );


#endif /* ndef __LIBVEX_PUB_GUEST_ARM_H */


/*---------------------------------------------------------------*/
/*---                                      libvex_guest_arm.h ---*/
/*---------------------------------------------------------------*/
