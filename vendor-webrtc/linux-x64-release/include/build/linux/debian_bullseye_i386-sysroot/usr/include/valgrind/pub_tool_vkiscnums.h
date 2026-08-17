
/*--------------------------------------------------------------------*/
/*--- Syscall numbers and related operations. pub_tool_vkiscnums.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2005-2017 Nicholas Nethercote
      njn@valgrind.org
   Copyright (C) 2006-2017 OpenWorks LLP
      info@open-works.co.uk

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

#ifndef __PUB_TOOL_VKISCNUMS_H
#define __PUB_TOOL_VKISCNUMS_H

#include "pub_tool_vkiscnums_asm.h"
#include "pub_tool_basics.h"    // Word


// This converts a syscall number into a string, suitable for printing.  It is
// needed because some platforms (Darwin) munge sysnums in various ways.
// The string is allocated in a static buffer and will be overwritten in the
// next invocation.
extern const HChar *VG_(sysnum_string) (Word sysnum);

// Macro provided for backward compatibility purposes.
#define VG_SYSNUM_STRING(sysnum) VG_(sysnum_string)(sysnum)


#endif   // __PUB_TOOL_VKISCNUMS_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
