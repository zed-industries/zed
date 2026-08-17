/* siginfo constants.  Linux version.
   Copyright (C) 1997-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

#ifndef _BITS_SIGINFO_CONSTS_H
#define _BITS_SIGINFO_CONSTS_H 1

#ifndef _SIGNAL_H
#error "Don't include <bits/siginfo-consts.h> directly; use <signal.h> instead."
#endif

/* Most of these constants are uniform across all architectures, but there
   is one exception.  */
#include <bits/siginfo-arch.h>
#ifndef __SI_ASYNCIO_AFTER_SIGIO
# define __SI_ASYNCIO_AFTER_SIGIO 1
#endif

/* Values for `si_code'.  Positive values are reserved for kernel-generated
   signals.  */
enum
{
  SI_ASYNCNL = -60,		/* Sent by asynch name lookup completion.  */
  SI_DETHREAD = -7,		/* Sent by execve killing subsidiary
				   threads.  */
  SI_TKILL,			/* Sent by tkill.  */
  SI_SIGIO,			/* Sent by queued SIGIO. */
#if __SI_ASYNCIO_AFTER_SIGIO
  SI_ASYNCIO,			/* Sent by AIO completion.  */
  SI_MESGQ,			/* Sent by real time mesq state change.  */
  SI_TIMER,			/* Sent by timer expiration.  */
#else
  SI_MESGQ,
  SI_TIMER,
  SI_ASYNCIO,
#endif
  SI_QUEUE,			/* Sent by sigqueue.  */
  SI_USER,			/* Sent by kill, sigsend.  */
  SI_KERNEL = 0x80		/* Send by kernel.  */

#define SI_ASYNCNL	SI_ASYNCNL
#define SI_DETHREAD	SI_DETHREAD
#define SI_TKILL	SI_TKILL
#define SI_SIGIO	SI_SIGIO
#define SI_ASYNCIO	SI_ASYNCIO
#define SI_MESGQ	SI_MESGQ
#define SI_TIMER	SI_TIMER
#define SI_ASYNCIO	SI_ASYNCIO
#define SI_QUEUE	SI_QUEUE
#define SI_USER		SI_USER
#define SI_KERNEL	SI_KERNEL
};


# if defined __USE_XOPEN_EXTENDED || defined __USE_XOPEN2K8
/* `si_code' values for SIGILL signal.  */
enum
{
  ILL_ILLOPC = 1,		/* Illegal opcode.  */
#  define ILL_ILLOPC	ILL_ILLOPC
  ILL_ILLOPN,			/* Illegal operand.  */
#  define ILL_ILLOPN	ILL_ILLOPN
  ILL_ILLADR,			/* Illegal addressing mode.  */
#  define ILL_ILLADR	ILL_ILLADR
  ILL_ILLTRP,			/* Illegal trap. */
#  define ILL_ILLTRP	ILL_ILLTRP
  ILL_PRVOPC,			/* Privileged opcode.  */
#  define ILL_PRVOPC	ILL_PRVOPC
  ILL_PRVREG,			/* Privileged register.  */
#  define ILL_PRVREG	ILL_PRVREG
  ILL_COPROC,			/* Coprocessor error.  */
#  define ILL_COPROC	ILL_COPROC
  ILL_BADSTK,			/* Internal stack error.  */
#  define ILL_BADSTK	ILL_BADSTK
  ILL_BADIADDR			/* Unimplemented instruction address.  */
#  define ILL_BADIADDR ILL_BADIADDR
};

/* `si_code' values for SIGFPE signal.  */
enum
{
  FPE_INTDIV = 1,		/* Integer divide by zero.  */
#  define FPE_INTDIV	FPE_INTDIV
  FPE_INTOVF,			/* Integer overflow.  */
#  define FPE_INTOVF	FPE_INTOVF
  FPE_FLTDIV,			/* Floating point divide by zero.  */
#  define FPE_FLTDIV	FPE_FLTDIV
  FPE_FLTOVF,			/* Floating point overflow.  */
#  define FPE_FLTOVF	FPE_FLTOVF
  FPE_FLTUND,			/* Floating point underflow.  */
#  define FPE_FLTUND	FPE_FLTUND
  FPE_FLTRES,			/* Floating point inexact result.  */
#  define FPE_FLTRES	FPE_FLTRES
  FPE_FLTINV,			/* Floating point invalid operation.  */
#  define FPE_FLTINV	FPE_FLTINV
  FPE_FLTSUB,			/* Subscript out of range.  */
#  define FPE_FLTSUB	FPE_FLTSUB
  FPE_FLTUNK = 14,		/* Undiagnosed floating-point exception.  */
#  define FPE_FLTUNK	FPE_FLTUNK
  FPE_CONDTRAP			/* Trap on condition.  */
#  define FPE_CONDTRAP	FPE_CONDTRAP
};

/* `si_code' values for SIGSEGV signal.  */
enum
{
  SEGV_MAPERR = 1,		/* Address not mapped to object.  */
#  define SEGV_MAPERR	SEGV_MAPERR
  SEGV_ACCERR,			/* Invalid permissions for mapped object.  */
#  define SEGV_ACCERR	SEGV_ACCERR
  SEGV_BNDERR,			/* Bounds checking failure.  */
#  define SEGV_BNDERR	SEGV_BNDERR
  SEGV_PKUERR,			/* Protection key checking failure.  */
#  define SEGV_PKUERR	SEGV_PKUERR
  SEGV_ACCADI,			/* ADI not enabled for mapped object.  */
#  define SEGV_ACCADI	SEGV_ACCADI
  SEGV_ADIDERR,			/* Disrupting MCD error.  */
#  define SEGV_ADIDERR	SEGV_ADIDERR
  SEGV_ADIPERR			/* Precise MCD exception.  */
#  define SEGV_ADIPERR	SEGV_ADIPERR
};

/* `si_code' values for SIGBUS signal.  */
enum
{
  BUS_ADRALN = 1,		/* Invalid address alignment.  */
#  define BUS_ADRALN	BUS_ADRALN
  BUS_ADRERR,			/* Non-existant physical address.  */
#  define BUS_ADRERR	BUS_ADRERR
  BUS_OBJERR,			/* Object specific hardware error.  */
#  define BUS_OBJERR	BUS_OBJERR
  BUS_MCEERR_AR,		/* Hardware memory error: action required.  */
#  define BUS_MCEERR_AR	BUS_MCEERR_AR
  BUS_MCEERR_AO			/* Hardware memory error: action optional.  */
#  define BUS_MCEERR_AO	BUS_MCEERR_AO
};
# endif

# ifdef __USE_XOPEN_EXTENDED
/* `si_code' values for SIGTRAP signal.  */
enum
{
  TRAP_BRKPT = 1,		/* Process breakpoint.  */
#  define TRAP_BRKPT	TRAP_BRKPT
  TRAP_TRACE,			/* Process trace trap.  */
#  define TRAP_TRACE	TRAP_TRACE
  TRAP_BRANCH,			/* Process taken branch trap.  */
#  define TRAP_BRANCH	TRAP_BRANCH
  TRAP_HWBKPT,			/* Hardware breakpoint/watchpoint.  */
#  define TRAP_HWBKPT	TRAP_HWBKPT
  TRAP_UNK			/* Undiagnosed trap.  */
#  define TRAP_UNK	TRAP_UNK
};
# endif

# if defined __USE_XOPEN_EXTENDED || defined __USE_XOPEN2K8
/* `si_code' values for SIGCHLD signal.  */
enum
{
  CLD_EXITED = 1,		/* Child has exited.  */
#  define CLD_EXITED	CLD_EXITED
  CLD_KILLED,			/* Child was killed.  */
#  define CLD_KILLED	CLD_KILLED
  CLD_DUMPED,			/* Child terminated abnormally.  */
#  define CLD_DUMPED	CLD_DUMPED
  CLD_TRAPPED,			/* Traced child has trapped.  */
#  define CLD_TRAPPED	CLD_TRAPPED
  CLD_STOPPED,			/* Child has stopped.  */
#  define CLD_STOPPED	CLD_STOPPED
  CLD_CONTINUED			/* Stopped child has continued.  */
#  define CLD_CONTINUED	CLD_CONTINUED
};

/* `si_code' values for SIGPOLL signal.  */
enum
{
  POLL_IN = 1,			/* Data input available.  */
#  define POLL_IN	POLL_IN
  POLL_OUT,			/* Output buffers available.  */
#  define POLL_OUT	POLL_OUT
  POLL_MSG,			/* Input message available.   */
#  define POLL_MSG	POLL_MSG
  POLL_ERR,			/* I/O error.  */
#  define POLL_ERR	POLL_ERR
  POLL_PRI,			/* High priority input available.  */
#  define POLL_PRI	POLL_PRI
  POLL_HUP			/* Device disconnected.  */
#  define POLL_HUP	POLL_HUP
};
# endif

/* Architectures might also add architecture-specific constants.
   These are all considered GNU extensions.  */
#ifdef __USE_GNU
# include <bits/siginfo-consts-arch.h>
#endif

#endif
