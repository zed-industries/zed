
/*--------------------------------------------------------------------*/
/*--- Process-related libc stuff               pub_tool_libcproc.h ---*/
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

#ifndef __PUB_TOOL_LIBCPROC_H
#define __PUB_TOOL_LIBCPROC_H

#include "pub_tool_basics.h"   // VG_ macro
#include "pub_tool_vki.h"      // vki_rlimit

/* ---------------------------------------------------------------------
   Command-line and environment stuff
   ------------------------------------------------------------------ */

/* Client environment. */
extern HChar** VG_(client_envp);

/* Looks up VG_(client_envp) */
extern HChar* VG_(getenv) ( const HChar* name );

/* Path to all our library/aux files */
extern const HChar *VG_(libdir);

// The name of the LD_PRELOAD-equivalent variable.  It varies across
// platforms.
extern const HChar* VG_(LD_PRELOAD_var_name);

/* Resolves filename of VG_(cl_exec_fd) and copies it to the buffer. 
   Buffer must not be NULL and buf_size must be at least 1.
   If buffer is not large enough it is terminated with '\0' only
   when 'terminate_with_NUL == True'. */
extern void VG_(client_fname)(HChar *buffer, SizeT buf_size,
                              Bool terminate_with_NUL);

/* Concatenates client exename and command line arguments into
   the buffer. Buffer must not be NULL and buf_size must be
   at least 1. Buffer is always terminated with '\0'. */
extern void VG_(client_cmd_and_args)(HChar *buffer, SizeT buf_size);

/* ---------------------------------------------------------------------
   Important syscalls
   ------------------------------------------------------------------ */

extern Int  VG_(waitpid)( Int pid, Int *status, Int options );
extern Int  VG_(system) ( const HChar* cmd );
extern Int  VG_(spawn)  ( const HChar *filename, const HChar **argv );
extern Int  VG_(fork)   ( void);
extern void VG_(execv)  ( const HChar* filename, const HChar** argv );
extern Int  VG_(sysctl) ( Int *name, UInt namelen, void *oldp, SizeT *oldlenp, void *newp, SizeT newlen );

/* ---------------------------------------------------------------------
   Resource limits and capabilities
   ------------------------------------------------------------------ */

extern Int VG_(getrlimit) ( Int resource, struct vki_rlimit *rlim );
extern Int VG_(setrlimit) ( Int resource, const struct vki_rlimit *rlim );
extern Int VG_(prctl) (Int option, 
                       ULong arg2, ULong arg3, ULong arg4, ULong arg5);

/* ---------------------------------------------------------------------
   pids, etc
   ------------------------------------------------------------------ */

extern Int VG_(gettid)  ( void );
extern Int VG_(getpid)  ( void );
extern Int VG_(getppid) ( void );
extern Int VG_(getpgrp) ( void );
extern Int VG_(geteuid) ( void );
extern Int VG_(getegid) ( void );

/* ---------------------------------------------------------------------
   Timing
   ------------------------------------------------------------------ */

// Returns the number of milliseconds passed since the program started
// (roughly;  it gets initialised partway through Valgrind's initialisation
// steps).  This is wallclock time.
extern UInt VG_(read_millisecond_timer) ( void );

extern Int  VG_(gettimeofday)(struct vki_timeval *tv, struct vki_timezone *tz);

#  if defined(VGO_linux) || defined(VGO_solaris)
/* Get the clock value as specified by clk_id.  Asserts if unsuccesful.  */
extern void VG_(clock_gettime)(struct vki_timespec *ts, vki_clockid_t clk_id);
#  elif defined(VGO_darwin)
  /* It seems clock_gettime is only available on recent Darwin versions.
     For the moment, let's assume it is not available.  */
#  else
#    error "Unknown OS"
#  endif

// Returns the number of milliseconds of user cpu time we have used,
// as reported by 'getrusage'.
extern UInt VG_(get_user_milliseconds)(void);

/* ---------------------------------------------------------------------
   atfork
   ------------------------------------------------------------------ */

typedef void (*vg_atfork_t)(ThreadId);
extern void VG_(atfork)(vg_atfork_t pre, vg_atfork_t parent, vg_atfork_t child);


#endif   // __PUB_TOOL_LIBCPROC_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
