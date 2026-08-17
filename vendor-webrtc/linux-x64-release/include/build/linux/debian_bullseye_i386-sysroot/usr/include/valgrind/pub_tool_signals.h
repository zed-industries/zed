
/*--------------------------------------------------------------------*/
/*--- Signals stuff.                            pub_tool_signals.h ---*/
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

#ifndef __PUB_TOOL_SIGNALS_H
#define __PUB_TOOL_SIGNALS_H

#include "pub_tool_basics.h"   // Addr

// Register an interest in apparently internal faults; used code which
// wanders around dangerous memory (ie, leakcheck).  The catcher is
// not expected to return.
// Returns the previously set fault_catcher (NULL if there was no fault
// catcher set)
//
// It's frustrating that we need this header for a single function used
// only by Memcheck during leak checking.  We should find a way to remove
// the need for this file.
typedef void (*fault_catcher_t)(Int sig, Addr addr);
extern fault_catcher_t VG_(set_fault_catcher)(fault_catcher_t catcher);

#endif   // __PUB_TOOL_SIGNALS_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
