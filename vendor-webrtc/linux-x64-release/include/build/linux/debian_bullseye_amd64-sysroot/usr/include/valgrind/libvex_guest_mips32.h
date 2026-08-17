
/*---------------------------------------------------------------*/
/*--- begin                             libvex_guest_mips32.h ---*/
/*---------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2010-2017 RT-RK
      mips-valgrind@rt-rk.com

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

#ifndef __LIBVEX_PUB_GUEST_MIPS32_H
#define __LIBVEX_PUB_GUEST_MIPS32_H

#include "libvex_basictypes.h"


/*---------------------------------------------------------------*/
/*--- Vex's representation of the MIPS32 CPU state.           ---*/
/*---------------------------------------------------------------*/

typedef
   struct {
      /*    0 */ UInt host_EvC_FAILADDR;
      /*    4 */ UInt host_EvC_COUNTER;

      /* CPU Registers */
      /*    8 */ UInt guest_r0;   /* Hardwired to 0. */
      /*   12 */ UInt guest_r1;   /* Assembler temporary */
      /*   16 */ UInt guest_r2;   /* Values for function returns ...*/
      /*   20 */ UInt guest_r3;   /* ... and expression evaluation */
      /*   24 */ UInt guest_r4;   /* Function arguments */
      /*   28 */ UInt guest_r5;
      /*   32 */ UInt guest_r6;
      /*   36 */ UInt guest_r7;
      /*   40 */ UInt guest_r8;   /* Temporaries */
      /*   44 */ UInt guest_r9;
      /*   48 */ UInt guest_r10;
      /*   52 */ UInt guest_r11;
      /*   56 */ UInt guest_r12;
      /*   60 */ UInt guest_r13;
      /*   64 */ UInt guest_r14;
      /*   68 */ UInt guest_r15;
      /*   72 */ UInt guest_r16;  /* Saved temporaries */
      /*   76 */ UInt guest_r17;
      /*   80 */ UInt guest_r18;
      /*   84 */ UInt guest_r19;
      /*   88 */ UInt guest_r20;
      /*   92 */ UInt guest_r21;
      /*   96 */ UInt guest_r22;
      /*  100 */ UInt guest_r23;
      /*  104 */ UInt guest_r24;  /* Temporaries */
      /*  108 */ UInt guest_r25;
      /*  112 */ UInt guest_r26;  /* Reserved for OS kernel */
      /*  116 */ UInt guest_r27;
      /*  120 */ UInt guest_r28;  /* Global pointer */
      /*  124 */ UInt guest_r29;  /* Stack pointer */
      /*  128 */ UInt guest_r30;  /* Frame pointer */
      /*  132 */ UInt guest_r31;  /* Return address */
      /*  136 */ UInt guest_PC;   /* Program counter */
      /*  140 */ UInt guest_HI;   /* Multiply and divide reg higher result */
      /*  144 */ UInt guest_LO;   /* Multiply and divide reg lower result */
      /*  148 */ UInt _padding1;

      /* FPU Registers */
      /*  152 */ ULong guest_f0;  /* Floating point general purpose registers */
      /*  160 */ ULong guest_f1;
      /*  168 */ ULong guest_f2;
      /*  176 */ ULong guest_f3;
      /*  184 */ ULong guest_f4;
      /*  192 */ ULong guest_f5;
      /*  200 */ ULong guest_f6;
      /*  208 */ ULong guest_f7;
      /*  216 */ ULong guest_f8;
      /*  224 */ ULong guest_f9;
      /*  232 */ ULong guest_f10;
      /*  240 */ ULong guest_f11;
      /*  248 */ ULong guest_f12;
      /*  256 */ ULong guest_f13;
      /*  264 */ ULong guest_f14;
      /*  272 */ ULong guest_f15;
      /*  280 */ ULong guest_f16;
      /*  288 */ ULong guest_f17;
      /*  296 */ ULong guest_f18;
      /*  304 */ ULong guest_f19;
      /*  312 */ ULong guest_f20;
      /*  320 */ ULong guest_f21;
      /*  328 */ ULong guest_f22;
      /*  336 */ ULong guest_f23;
      /*  344 */ ULong guest_f24;
      /*  352 */ ULong guest_f25;
      /*  360 */ ULong guest_f26;
      /*  368 */ ULong guest_f27;
      /*  376 */ ULong guest_f28;
      /*  384 */ ULong guest_f29;
      /*  392 */ ULong guest_f30;
      /*  400 */ ULong guest_f31;

      /*  408 */ UInt guest_FIR;
      /*  412 */ UInt guest_FCCR;
      /*  416 */ UInt guest_FEXR;
      /*  420 */ UInt guest_FENR;
      /*  424 */ UInt guest_FCSR;

      /* TLS pointer for the thread. It's read-only in user space.
         On Linux it is set in user space by various thread-related
         syscalls.
         User Local Register.
         This register provides read access to the coprocessor 0
         UserLocal register, if it is implemented. In some operating
         environments, the UserLocal register is a pointer to a
         thread-specific storage block.
      */
      /*  428 */ UInt guest_ULR;

      /* Emulation notes */
      /*  432 */ UInt guest_EMNOTE;

      /* For clflush: record start and length of area to invalidate. */
      /*  436 */ UInt guest_CMSTART;
      /*  440 */ UInt guest_CMLEN;
      /*  444 */ UInt guest_NRADDR;

      /*  448 */ UInt guest_COND;

      /* MIPS32 DSP ASE(r2) specific registers. */
      /*  452 */ UInt guest_DSPControl;
      /*  456 */ ULong guest_ac0;
      /*  464 */ ULong guest_ac1;
      /*  472 */ ULong guest_ac2;
      /*  480 */ ULong guest_ac3;

      /*  488 */ UInt guest_CP0_status;
      /*  492 */ UInt guest_CP0_Config5;

      /*  496 */ UInt guest_LLaddr;
      /*  500 */ UInt guest_LLdata;

      /* MIPS32 MSA 128-bit vector registers */
      /*  504 */ V128 guest_w0;
      /*  520 */ V128 guest_w1;
      /*  536 */ V128 guest_w2;
      /*  552 */ V128 guest_w3;
      /*  568 */ V128 guest_w4;
      /*  584 */ V128 guest_w5;
      /*  600 */ V128 guest_w6;
      /*  616 */ V128 guest_w7;
      /*  632 */ V128 guest_w8;
      /*  648 */ V128 guest_w9;
      /*  664 */ V128 guest_w10;
      /*  680 */ V128 guest_w11;
      /*  696 */ V128 guest_w12;
      /*  712 */ V128 guest_w13;
      /*  728 */ V128 guest_w14;
      /*  744 */ V128 guest_w15;
      /*  760 */ V128 guest_w16;
      /*  776 */ V128 guest_w17;
      /*  792 */ V128 guest_w18;
      /*  808 */ V128 guest_w19;
      /*  824 */ V128 guest_w20;
      /*  840 */ V128 guest_w21;
      /*  856 */ V128 guest_w22;
      /*  872 */ V128 guest_w23;
      /*  888 */ V128 guest_w24;
      /*  904 */ V128 guest_w25;
      /*  920 */ V128 guest_w26;
      /*  936 */ V128 guest_w27;
      /*  952 */ V128 guest_w28;
      /*  968 */ V128 guest_w29;
      /*  984 */ V128 guest_w30;
      /*  1000 */ V128 guest_w31;

      /*  1016 */ UInt guest_MSACSR;

      /*  1020 */ UInt _padding3;

      /*  1020 */ ULong guest_LLdata64;
      /*  1028 */ ULong _padding4;
} VexGuestMIPS32State;
/*---------------------------------------------------------------*/
/*--- Utility functions for MIPS32 guest stuff.               ---*/
/*---------------------------------------------------------------*/

/* ALL THE FOLLOWING ARE VISIBLE TO LIBRARY CLIENT. */

/* Initialise all guest MIPS32 state. */

extern
void LibVEX_GuestMIPS32_initialise ( /*OUT*/VexGuestMIPS32State* vex_state );

/* FR bit of CP0_STATUS_FR register */
#define MIPS_CP0_STATUS_FR (1ul << 26)

/* FRE bit of CP0_Config5 register */
#define MIPS_CONF5_FRE     (1ul << 8)

#endif /* ndef __LIBVEX_PUB_GUEST_MIPS32_H */


/*---------------------------------------------------------------*/
/*---                                   libvex_guest_mips32.h ---*/
/*---------------------------------------------------------------*/
