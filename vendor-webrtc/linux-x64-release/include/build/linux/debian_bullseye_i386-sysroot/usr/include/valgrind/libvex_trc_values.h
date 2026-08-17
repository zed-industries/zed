
/*---------------------------------------------------------------*/
/*--- begin                               libvex_trc_values.h ---*/
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

#ifndef __LIBVEX_TRC_VALUES_H
#define __LIBVEX_TRC_VALUES_H


/* Magic values that the guest state pointer might be set to when
   returning to the dispatcher.  The only other legitimate value is to
   point to the start of the thread's VEX guest state.

   This file may get included in assembly code, so do not put
   C-specific constructs in it.

   These values should be 61 or above so as not to conflict
   with Valgrind's VG_TRC_ values, which are 60 or below.
*/

#define VEX_TRC_JMP_INVALICACHE 61  /* invalidate icache (translations)
                                       before continuing */
#define VEX_TRC_JMP_FLUSHDCACHE 103 /* flush dcache before continuing */

#define VEX_TRC_JMP_NOREDIR    81  /* jump to undirected guest addr */
#define VEX_TRC_JMP_SIGTRAP    85  /* deliver trap (SIGTRAP) before
                                      continuing */
#define VEX_TRC_JMP_SIGSEGV    87  /* deliver segv (SIGSEGV) before
                                      continuing */
#define VEX_TRC_JMP_SIGBUS     93  /* deliver SIGBUS before continuing */
#define VEX_TRC_JMP_SIGFPE    105  /* deliver SIGFPE before continuing */

#define VEX_TRC_JMP_SIGFPE_INTDIV     97  /* deliver SIGFPE (integer divide
                                             by zero) before continuing */

#define VEX_TRC_JMP_SIGFPE_INTOVF     99  /* deliver SIGFPE (integer overflow)
                                             before continuing */

#define VEX_TRC_JMP_SIGILL     101  /* deliver SIGILL (Illegal instruction)
                                       before continuing */

#define VEX_TRC_JMP_EMWARN     63  /* deliver emulation warning before
                                      continuing */
#define VEX_TRC_JMP_EMFAIL     83  /* emulation fatal error; abort system */

#define VEX_TRC_JMP_CLIENTREQ  65  /* do a client req before continuing */
#define VEX_TRC_JMP_YIELD      67  /* yield to thread sched 
                                      before continuing */
#define VEX_TRC_JMP_NODECODE   69  /* next instruction is not decodable */
#define VEX_TRC_JMP_MAPFAIL    71  /* address translation failed */

#define VEX_TRC_JMP_SYS_SYSCALL  73 /* do syscall before continuing */
#define VEX_TRC_JMP_SYS_INT32    75 /* do syscall before continuing */
#define VEX_TRC_JMP_SYS_INT128   77 /* do syscall before continuing */
#define VEX_TRC_JMP_SYS_INT129   89 /* do syscall before continuing */
#define VEX_TRC_JMP_SYS_INT130   91 /* do syscall before continuing */
#define VEX_TRC_JMP_SYS_INT145  111 /* do syscall before continuing */
#define VEX_TRC_JMP_SYS_INT210  113 /* do syscall before continuing */

#define VEX_TRC_JMP_SYS_SYSENTER 79 /* do syscall before continuing */

#define VEX_TRC_JMP_BORING       95 /* return to sched, but just 
                                       keep going; no special action */

#endif /* ndef __LIBVEX_TRC_VALUES_H */

/*---------------------------------------------------------------*/
/*---                                     libvex_trc_values.h ---*/
/*---------------------------------------------------------------*/
