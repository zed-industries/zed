
/*--------------------------------------------------------------------*/
/*--- The translation table and cache.                             ---*/
/*---                                          pub_tool_transtab.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2014-2017 Florian Krohm   (florian@eich-krohm.de)

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

#ifndef __PUB_TOOL_TRANSTAB_H
#define __PUB_TOOL_TRANSTAB_H

#include "pub_tool_basics.h"   // VG_ macro and primitive types

void VG_(discard_translations_safely) ( Addr  start, SizeT len,
                                        const HChar* who );

#endif   // __PUB_TOOL_TRANSTAB_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
