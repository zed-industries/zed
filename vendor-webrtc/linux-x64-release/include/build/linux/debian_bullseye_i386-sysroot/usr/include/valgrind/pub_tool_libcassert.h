
/*--------------------------------------------------------------------*/
/*--- Assertions, etc.                       pub_tool_libcassert.h ---*/
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

#ifndef __PUB_TOOL_LIBCBASSERT_H
#define __PUB_TOOL_LIBCBASSERT_H

#include "pub_tool_basics.h"   // VG_ macro

#define tl_assert(expr)                                                 \
  ((void) (LIKELY(expr) ? 0 :                                           \
           (VG_(assert_fail) (/*isCore?*/False, #expr,                  \
                              __FILE__, __LINE__,                       \
                              __PRETTY_FUNCTION__,                      \
                              ""),                                      \
                              0)))

#define tl_assert2(expr, format, args...)                               \
  ((void) (LIKELY(expr) ? 0 :                                           \
           (VG_(assert_fail) (/*isCore?*/False, #expr,                  \
                              __FILE__, __LINE__,                       \
                              __PRETTY_FUNCTION__,                      \
                              format, ##args),                          \
                              0)))

__attribute__ ((__noreturn__))
extern void VG_(exit)( Int status );

/* Prints a panic message, appends newline and bug reporting info, aborts. */
__attribute__ ((__noreturn__))
extern void  VG_(tool_panic) ( const HChar* str );

__attribute__ ((__noreturn__))
extern void VG_(assert_fail) ( Bool isCore, const HChar* expr, const HChar* file, 
                               Int line, const HChar* fn, 
                               const HChar* format, ... );

/* Show the state of various threads related information, such
   as the guest stacktrace for each thread.
   Mostly for debugging V.
   The following activates optional output:
     host_stacktrace : shows the host stacktrace.
     stack_usage True means:
                   shows how much of the valgrind stack was used.
                   shows the client stack range
     exited_thread_slots : show information for thread slots that were used
        but the thread has now exited. */
extern void VG_(show_sched_status) ( Bool host_stacktrace,
                                     Bool stack_usage,
                                     Bool exited_threads);

#endif   // __PUB_TOOL_LIBCBASSERT_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
