/* -*- mode: C; c-basic-offset: 3; -*- */

/*---------------------------------------------------------------*/
/*--- begin                              libvex_guest_s390x.h ---*/
/*---------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright IBM Corp. 2010-2017

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

#ifndef __LIBVEX_PUB_GUEST_S390X_H
#define __LIBVEX_PUB_GUEST_S390X_H

#include "libvex_basictypes.h"

/*------------------------------------------------------------*/
/*--- Vex's representation of the s390 CPU state.          ---*/
/*------------------------------------------------------------*/

typedef struct {

/*------------------------------------------------------------*/
/*--- ar registers                                         ---*/
/*------------------------------------------------------------*/

   /*    0 */  UInt guest_a0;
   /*    4 */  UInt guest_a1;
   /*    8 */  UInt guest_a2;
   /*   12 */  UInt guest_a3;
   /*   16 */  UInt guest_a4;
   /*   20 */  UInt guest_a5;
   /*   24 */  UInt guest_a6;
   /*   28 */  UInt guest_a7;
   /*   32 */  UInt guest_a8;
   /*   36 */  UInt guest_a9;
   /*   40 */  UInt guest_a10;
   /*   44 */  UInt guest_a11;
   /*   48 */  UInt guest_a12;
   /*   52 */  UInt guest_a13;
   /*   56 */  UInt guest_a14;
   /*   60 */  UInt guest_a15;

/*------------------------------------------------------------*/
/*--- fpr & vr registers                                   ---*/
/*------------------------------------------------------------*/

   /*
      FPRs[0-15] are mapped to the first double words of VR's[0-15].
      According to documentation if we modify fpr1 with FP insn then the content of vr1's 64..128
      bits is unpredictable. If we modify 64..128 of vr1 then fpr1's value is unpredictable too.
      In our implementation writing to one half of vr doesn't affect another part but
      apllications shouldn't rely on it.
   */

   /*   64 */  V128 guest_v0;
   /*   80 */  V128 guest_v1;
   /*   96 */  V128 guest_v2;
   /*  112 */  V128 guest_v3;
   /*  128 */  V128 guest_v4;
   /*  144 */  V128 guest_v5;
   /*  160 */  V128 guest_v6;
   /*  176 */  V128 guest_v7;
   /*  192 */  V128 guest_v8;
   /*  208 */  V128 guest_v9;
   /*  224 */  V128 guest_v10;
   /*  240 */  V128 guest_v11;
   /*  256 */  V128 guest_v12;
   /*  272 */  V128 guest_v13;
   /*  288 */  V128 guest_v14;
   /*  304 */  V128 guest_v15;
   /*  320 */  V128 guest_v16;
   /*  336 */  V128 guest_v17;
   /*  352 */  V128 guest_v18;
   /*  368 */  V128 guest_v19;
   /*  384 */  V128 guest_v20;
   /*  400 */  V128 guest_v21;
   /*  416 */  V128 guest_v22;
   /*  432 */  V128 guest_v23;
   /*  448 */  V128 guest_v24;
   /*  464 */  V128 guest_v25;
   /*  480 */  V128 guest_v26;
   /*  496 */  V128 guest_v27;
   /*  512 */  V128 guest_v28;
   /*  528 */  V128 guest_v29;
   /*  544 */  V128 guest_v30;
   /*  560 */  V128 guest_v31;

/*------------------------------------------------------------*/
/*--- gpr registers                                        ---*/
/*------------------------------------------------------------*/

   /*  576 */  ULong guest_r0;
   /*  584 */  ULong guest_r1;
   /*  592 */  ULong guest_r2;
   /*  600 */  ULong guest_r3;
   /*  608 */  ULong guest_r4;
   /*  616 */  ULong guest_r5;
   /*  624 */  ULong guest_r6;
   /*  632 */  ULong guest_r7;
   /*  640 */  ULong guest_r8;
   /*  648 */  ULong guest_r9;
   /*  656 */  ULong guest_r10;
   /*  664 */  ULong guest_r11;
   /*  672 */  ULong guest_r12;
   /*  680 */  ULong guest_r13;
   /*  688 */  ULong guest_r14;
   /*  696 */  ULong guest_r15;

/*------------------------------------------------------------*/
/*--- S390 miscellaneous registers                         ---*/
/*------------------------------------------------------------*/

   /*  704 */  ULong guest_counter;
   /*  712 */  UInt guest_fpc;
   /*  716 */  UChar unused[4]; /* 4-byte hole to get 8-byte alignment */
   /*  720 */  ULong guest_IA;

/*------------------------------------------------------------*/
/*--- S390 pseudo registers                                ---*/
/*------------------------------------------------------------*/

   /*  728 */  ULong guest_SYSNO;

/*------------------------------------------------------------*/
/*--- 4-word thunk used to calculate the condition code    ---*/
/*------------------------------------------------------------*/

   /*  736 */  ULong guest_CC_OP;
   /*  744 */  ULong guest_CC_DEP1;
   /*  752 */  ULong guest_CC_DEP2;
   /*  760 */  ULong guest_CC_NDEP;

/*------------------------------------------------------------*/
/*--- Pseudo registers. Required by all architectures      ---*/
/*------------------------------------------------------------*/

   /* See comments at bottom of libvex.h */
   /*  768 */  ULong guest_NRADDR;
   /*  776 */  ULong guest_CMSTART;
   /*  784 */  ULong guest_CMLEN;

   /* Used when backing up to restart a syscall that has
      been interrupted by a signal. See also comment in
      libvex_ir.h */
   /*  792 */  ULong guest_IP_AT_SYSCALL;

   /* Emulation notes; see comments in libvex_emnote.h */
   /*  800 */  UInt guest_EMNOTE;

   /* For translation chaining */
   /*  804 */  UInt  host_EvC_COUNTER;
   /*  808 */  ULong host_EvC_FAILADDR;

/*------------------------------------------------------------*/
/*--- Force alignment to 16 bytes                          ---*/
/*------------------------------------------------------------*/
   /*  816 */  UChar padding[0];

   /*  816 */  /* This is the size of the guest state */
} VexGuestS390XState;


/*------------------------------------------------------------*/
/*--- Function prototypes                                  ---*/
/*------------------------------------------------------------*/

void LibVEX_GuestS390X_initialise(VexGuestS390XState *);

/*------------------------------------------------------------*/
/*--- Dedicated registers                                  ---*/
/*------------------------------------------------------------*/

#define guest_LR guest_r14  /* Link register */
#define guest_SP guest_r15  /* Stack pointer */
#define guest_FP guest_r11  /* Frame pointer */

/*---------------------------------------------------------------*/
/*--- end                                libvex_guest_s390x.h ---*/
/*---------------------------------------------------------------*/

#endif /* __LIBVEX_PUB_GUEST_S390X_H */
