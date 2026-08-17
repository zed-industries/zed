
/*--------------------------------------------------------------------*/
/*--- Utilities for inner Valgrind                  libvex_inner.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2017-2017 Philippe Waroquiers
      philippe.waroquiers@skynet.be

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

#ifndef __LIBVEX_INNER_H
#define __LIBVEX_INNER_H

//--------------------------------------------------------------------
// PURPOSE: This header should be imported by every  file in Valgrind
// which needs specific behaviour when running as an "inner" Valgrind.
// Valgrind can self-host itself (i.e. Valgrind can run Valgrind) :
// The outer Valgrind executes the inner Valgrind.
// For more details, see README_DEVELOPPERS.
//--------------------------------------------------------------------

#include "config.h" 

// The code of the inner Valgrind (core or tool code) contains client
// requests (e.g. from helgrind.h, memcheck.h, ...) to help the
// outer Valgrind finding (relevant) errors in the inner Valgrind.
// Such client requests should only be compiled in for an inner Valgrind.
// Use the macro INNER_REQUEST to allow a central enabling/disabling
// of these client requests.
#if defined(ENABLE_INNER)

// By default, the inner Valgrind annotates various actions to help
// the outer tool (memcheck or helgrind).
// Undefine the below to have an inner Valgrind without any annotation.
#define ENABLE_INNER_CLIENT_REQUEST 1

#if defined(ENABLE_INNER_CLIENT_REQUEST)
#define INNER_REQUEST(__zza)  __zza
#else
#define INNER_REQUEST(__zza)  do {} while (0)
#endif

#else

#define INNER_REQUEST(__zza)  do {} while (0)

#endif

#endif   // __LIBVEX_INNER_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
