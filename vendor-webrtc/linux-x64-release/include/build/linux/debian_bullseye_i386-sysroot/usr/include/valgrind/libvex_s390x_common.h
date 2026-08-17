/* -*- mode: C; c-basic-offset: 3; -*- */

/*--------------------------------------------------------------------*/
/*--- Common defs for s390x                  libvex_s390x_common.h ---*/
/*--------------------------------------------------------------------*/

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

#ifndef __LIBVEX_PUB_S390X_H
#define __LIBVEX_PUB_S390X_H

/* This file includes definitions for s390.

   It must be suitable for inclusion in assembler source files. */


/*--------------------------------------------------------------*/
/*--- Dedicated registers                                    ---*/
/*--------------------------------------------------------------*/

#define S390_REGNO_RETURN_VALUE         2
#define S390_REGNO_TCHAIN_SCRATCH      12
#define S390_REGNO_GUEST_STATE_POINTER 13
#define S390_REGNO_LINK_REGISTER       14
#define S390_REGNO_STACK_POINTER       15


/*--------------------------------------------------------------*/
/*--- Offsets in the stack frame allocated by the dispatcher ---*/
/*--------------------------------------------------------------*/

/* Dispatcher will save 8 FPRs at offsets 160 + 0 ... 160 + 56 */

/* Where the dispatcher saves the r2 contents. */
#define S390_OFFSET_SAVED_R2 160+80

/* Where client's FPC register is saved. */
#define S390_OFFSET_SAVED_FPC_C 160+72

/* Where valgrind's FPC register is saved. */
#define S390_OFFSET_SAVED_FPC_V 160+64

/* Size of frame allocated by VG_(disp_run_translations)
   Need size for
       8 FPRs
     + 1 GPR  (SAVED_R2)
     + 2 FPCs (SAVED_FPC_C and SAVED_FPC_V).

   Additionally, we need a standard frame for helper functions being called
   from client code. (See figure 1-16 in zSeries ABI) */
#define S390_INNERLOOP_FRAME_SIZE ((8+1+2)*8 + 160)


/*--------------------------------------------------------------*/
/*--- Facility bits                                          ---*/
/*--------------------------------------------------------------*/

/* The value of the macro is the number of the facility bit as per POP. */
#define S390_FAC_MSA     17  // message-security-assist
#define S390_FAC_LDISP   18  // long displacement
#define S390_FAC_HFPMAS  20  // HFP multiply-and-add-subtract
#define S390_FAC_EIMM    21  // extended immediate
#define S390_FAC_HFPUNX  23  // HFP unnormalized extension
#define S390_FAC_ETF2    24  // ETF2-enhancement
#define S390_FAC_STCKF   25  // store clock fast insn
#define S390_FAC_PENH    26  // parsing-enhancement
#define S390_FAC_ETF3    30  // ETF3-enhancement
#define S390_FAC_XCPUT   31  // extract-CPU-time
#define S390_FAC_GIE     34  // general insn extension
#define S390_FAC_EXEXT   35  // execute extension
#define S390_FAC_FPEXT   37  // floating-point extension
#define S390_FAC_FPSE    41  // floating-point support enhancement
#define S390_FAC_DFP     42  // decimal floating point
#define S390_FAC_PFPO    44  // perform floating point operation insn
#define S390_FAC_HIGHW   45  // high-word extension
#define S390_FAC_LSC     45  // load/store on condition
#define S390_FAC_DFPZC   48  // DFP zoned-conversion
#define S390_FAC_MISC    49  // miscellaneous insn
#define S390_FAC_CTREXE  50  // constrained transactional execution
#define S390_FAC_LSC2    53  // load/store on condition 2 and load and zero rightmost byte
#define S390_FAC_MSA5    57  // message-security-assist 5
#define S390_FAC_MI2     58  // miscellaneous-instruction-extensions 2
#define S390_FAC_TREXE   73  // transactional execution
#define S390_FAC_MSA4    77  // message-security-assist 4
#define S390_FAC_VX      129 // vector facility
#define S390_FAC_VXE     135 // vector enhancements facility 1
#define S390_FAC_VXE2    148 // vector enhancements facility 2
#define S390_FAC_DFLT    151 // deflate-conversion facility


/*--------------------------------------------------------------*/
/*--- Miscellaneous                                          ---*/
/*--------------------------------------------------------------*/

/* Number of arguments that can be passed in registers */
#define S390_NUM_GPRPARMS 5

/* Number of double words needed to store all facility bits. */
#define S390_NUM_FACILITY_DW 3

#endif /* __LIBVEX_PUB_S390X_H */

/*--------------------------------------------------------------------*/
/*--- end                                    libvex_s390x_common.h ---*/
/*--------------------------------------------------------------------*/
