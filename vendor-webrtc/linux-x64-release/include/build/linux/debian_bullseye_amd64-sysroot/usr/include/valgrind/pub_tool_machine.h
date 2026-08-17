
/*--------------------------------------------------------------------*/
/*--- Machine-related stuff.                    pub_tool_machine.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2000-2017 Julian Seward
      jseward@acm.org

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

#ifndef __PUB_TOOL_MACHINE_H
#define __PUB_TOOL_MACHINE_H

#include "pub_tool_basics.h"           // ThreadID
#include "libvex.h"                    // VexArchInfo

#if defined(VGP_x86_linux) || defined(VGP_x86_solaris)
#  define VG_MIN_INSTR_SZB          1  // min length of native instruction
#  define VG_MAX_INSTR_SZB         16  // max length of native instruction
#  define VG_CLREQ_SZB             14  // length of a client request, may
                                       //   be larger than VG_MAX_INSTR_SZB
#  define VG_STACK_REDZONE_SZB      0  // number of addressable bytes below %RSP

#elif defined(VGP_amd64_linux) || defined(VGP_amd64_solaris)
#  define VG_MIN_INSTR_SZB          1
#  define VG_MAX_INSTR_SZB         16
#  define VG_CLREQ_SZB             19
#  define VG_STACK_REDZONE_SZB    128

#elif defined(VGP_ppc32_linux)
#  define VG_MIN_INSTR_SZB          4
#  define VG_MAX_INSTR_SZB          4 
#  define VG_CLREQ_SZB             20
#  define VG_STACK_REDZONE_SZB      0

#elif defined(VGP_ppc64be_linux)  || defined(VGP_ppc64le_linux)
#  define VG_MIN_INSTR_SZB          4
#  define VG_MAX_INSTR_SZB          4 
#  define VG_CLREQ_SZB             20
#  define VG_STACK_REDZONE_SZB    288  // number of addressable bytes below R1
                                       // from 64-bit PowerPC ELF ABI 
                                       // Supplement 1.7

#elif defined(VGP_arm_linux)
#  define VG_MIN_INSTR_SZB          2
#  define VG_MAX_INSTR_SZB          4 
#  define VG_CLREQ_SZB             20
#  define VG_STACK_REDZONE_SZB      0

#elif defined(VGP_arm64_linux)
#  define VG_MIN_INSTR_SZB          4
#  define VG_MAX_INSTR_SZB          4 
#  define VG_CLREQ_SZB             20
#  define VG_STACK_REDZONE_SZB      0

#elif defined(VGP_s390x_linux)
#  define VG_MIN_INSTR_SZB          2
#  define VG_MAX_INSTR_SZB          6
#  define VG_CLREQ_SZB             10
#  define VG_STACK_REDZONE_SZB      0  // s390 has no redzone

#elif defined(VGP_x86_darwin)
#  define VG_MIN_INSTR_SZB          1  // min length of native instruction
#  define VG_MAX_INSTR_SZB         16  // max length of native instruction
#  define VG_CLREQ_SZB             14  // length of a client request, may
                                       //   be larger than VG_MAX_INSTR_SZB
#  define VG_STACK_REDZONE_SZB      0  // number of addressable bytes below %RSP

#elif defined(VGP_amd64_darwin)
#  define VG_MIN_INSTR_SZB          1
#  define VG_MAX_INSTR_SZB         16
#  define VG_CLREQ_SZB             19
#  define VG_STACK_REDZONE_SZB    128

#elif defined(VGP_mips32_linux)
#  define VG_MIN_INSTR_SZB          4
#  define VG_MAX_INSTR_SZB          8
#  define VG_CLREQ_SZB             20
#  define VG_STACK_REDZONE_SZB      0

#elif defined(VGP_mips64_linux)
#  define VG_MIN_INSTR_SZB          4
#  define VG_MAX_INSTR_SZB          8
#  define VG_CLREQ_SZB             20
#  define VG_STACK_REDZONE_SZB      0

#elif defined(VGP_nanomips_linux)
#  define VG_MIN_INSTR_SZB          2
#  define VG_MAX_INSTR_SZB          6
#  define VG_CLREQ_SZB             20
#  define VG_STACK_REDZONE_SZB      0

#else
#  error Unknown platform
#endif

// Guest state accessors
// Are mostly in the core_ header.
//  Only these two are available to tools.
Addr VG_(get_IP) ( ThreadId tid );
Addr VG_(get_SP) ( ThreadId tid );

// Get and set the shadow1 SP register
Addr VG_(get_SP_s1) ( ThreadId tid );
void VG_(set_SP_s1) ( ThreadId tid, Addr sp );

// For get/set, 'area' is where the asked-for guest state will be copied
// into/from.  If shadowNo == 0, the real (non-shadow) guest state is
// accessed.  If shadowNo == 1, the first shadow area is accessed, and
// if shadowNo == 2, the second shadow area is accessed.  This gives a
// completely general way to read/modify a thread's guest register state
// providing you know the offsets you need.
void
VG_(get_shadow_regs_area) ( ThreadId tid, 
                            /*DST*/UChar* dst,
                            /*SRC*/Int shadowNo, PtrdiffT offset, SizeT size );
void
VG_(set_shadow_regs_area) ( ThreadId tid, 
                            /*DST*/Int shadowNo, PtrdiffT offset, SizeT size,
                            /*SRC*/const UChar* src );

// Apply a function 'f' to all the general purpose registers in all the
// current threads. This is all live threads, or (when the process is exiting)
// all threads that were instructed to die by the thread calling exit.
// This is very Memcheck-specific -- it's used to find the roots when
// doing leak checking.
extern void VG_(apply_to_GP_regs)(void (*f)(ThreadId tid,
                                            const HChar* regname, UWord val));

// This iterator lets you inspect each live thread's stack bounds.
// Returns False at the end.  'tid' is the iterator and you can only
// safely change it by making calls to these functions.
extern void VG_(thread_stack_reset_iter) ( /*OUT*/ThreadId* tid );
// stack_min is the address of the lowest stack byte,
// stack_max is the address of the highest stack byte.
// In other words, the live stack is [stack_min, stack_max].
extern Bool VG_(thread_stack_next)       ( /*MOD*/ThreadId* tid,
                                           /*OUT*/Addr* stack_min, 
                                           /*OUT*/Addr* stack_max );

// Returns .client_stack_highest_byte for the given thread
// i.e. the highest addressable byte of the stack.
extern Addr VG_(thread_get_stack_max) ( ThreadId tid );

// Returns how many bytes have been allocated for the stack of the given thread
extern SizeT VG_(thread_get_stack_size) ( ThreadId tid );

// Returns the lowest address of the alternate signal stack.
// See also the man page of sigaltstack().
extern Addr VG_(thread_get_altstack_min) ( ThreadId tid );

// Returns how many bytes have been allocated for the alternate signal stack.
// See also the man page of sigaltstack().
extern SizeT VG_(thread_get_altstack_size) ( ThreadId tid );

// Given a pointer to a function as obtained by "& functionname" in C,
// produce a pointer to the actual entry point for the function.  For
// most platforms it's the identity function.  Unfortunately, on
// ppc64-linux it isn't (sigh).
extern void* VG_(fnptr_to_fnentry)( void* );

/* Returns the size of the largest guest register that we will
   simulate in this run.  This depends on both the guest architecture
   and on the specific capabilities we are simulating for that guest
   (eg, AVX or non-AVX ?, for amd64). */
extern Int VG_(machine_get_size_of_largest_guest_register) ( void );

/* Return host cpu info. */
extern void VG_(machine_get_VexArchInfo)( /*OUT*/VexArch*,
                                          /*OUT*/VexArchInfo* );

#endif   // __PUB_TOOL_MACHINE_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
