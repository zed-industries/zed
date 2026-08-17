
/*--------------------------------------------------------------------*/
/*--- MallocFree: high-level memory management.                    ---*/
/*---                                        pub_tool_mallocfree.h ---*/
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

#ifndef __PUB_TOOL_MALLOCFREE_H
#define __PUB_TOOL_MALLOCFREE_H

#include "pub_tool_basics.h"   // SizeT

// These can be for allocating memory used by tools.
// Nb: the allocators *always succeed* -- they never return NULL (Valgrind
// will abort if they can't allocate the memory).
// The 'cc' is a string that identifies the allocation point.  It's used when
// --profile-heap=yes is specified.
extern void* VG_(malloc)         ( const HChar* cc, SizeT nbytes );
extern void  VG_(free)           ( void* p );
extern void* VG_(calloc)         ( const HChar* cc, SizeT n, SizeT bytes_per_elem );
extern void*  VG_(realloc)       ( const HChar* cc, void* p, SizeT size );
extern void   VG_(realloc_shrink)( void* ptr, SizeT size );
extern HChar* VG_(strdup)        ( const HChar* cc, const HChar* s );

// TODO: move somewhere else
// Call here to bomb the system when out of memory (mmap anon fails)
__attribute__((noreturn))
extern void VG_(out_of_memory_NORETURN) ( const HChar* who, SizeT szB );

// VG_(perm_malloc) is for allocating small blocks which are
// never released. The overhead for such blocks is minimal.
// VG_(perm_malloc) returns memory which is (at least) aligned
// on a multiple of align.
// Use the macro vg_alignof (type) to get a safe alignment for a type.
// No other function can be used on these permanently allocated blocks.
// In particular, do *not* call VG_(free) or VG_(realloc).
// Technically, these blocks will be returned from big superblocks
// only containing such permanently allocated blocks.
// Note that there is no cc cost centre : all such blocks will be
// regrouped under the "perm_alloc" cost centre.
extern void* VG_(perm_malloc)    ( SizeT nbytes, Int align );

#endif   // __PUB_TOOL_MALLOCFREE_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/

