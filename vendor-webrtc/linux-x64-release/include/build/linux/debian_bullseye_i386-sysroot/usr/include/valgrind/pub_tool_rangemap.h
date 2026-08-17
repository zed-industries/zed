
/*--------------------------------------------------------------------*/
/*--- A mapping where the keys exactly cover the address space.    ---*/
/*---                                          pub_tool_rangemap.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2014-2017 Mozilla Foundation

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

/* Contributed by Julian Seward <jseward@acm.org> */

#ifndef __PUB_TOOL_RANGEMAP_H
#define __PUB_TOOL_RANGEMAP_H

//--------------------------------------------------------------------
// PURPOSE: a mapping from the host machine word (UWord) ranges to
// arbitrary other UWord values.  The set of ranges exactly covers all
// possible UWord values.
// --------------------------------------------------------------------

/* It's an abstract type. */
typedef  struct _RangeMap  RangeMap;

/* Create a new RangeMap, using given allocation and free functions.
   alloc_fn must not return NULL (that is, if it returns it must have
   succeeded.)  The new array will contain a single range covering the
   entire key space, which will be bound to the value |initialVal|.
   This function never returns NULL. */
RangeMap* VG_(newRangeMap) ( Alloc_Fn_t alloc_fn,
                             const HChar* cc,
                             Free_Fn_t free_fn,
                             UWord initialVal );

/* Free all memory associated with a RangeMap. */
void VG_(deleteRangeMap) ( RangeMap* );

/* Bind the range [key_min, key_max] to val, overwriting any other
   bindings existing in the range.  Asserts if key_min > key_max.  If
   as a result of this addition, there come to be multiple adjacent
   ranges with the same value, these ranges are merged together.  Note
   that this is slow: O(N) in the number of existing ranges. */
void VG_(bindRangeMap) ( RangeMap* rm,
                         UWord key_min, UWord key_max, UWord val );

/* Looks up |key| in the array and returns the associated value and
   the key bounds.  Can never fail since the RangeMap covers the
   entire key space.  This is fast: O(log N) in the number of
   ranges. */
void VG_(lookupRangeMap) ( /*OUT*/UWord* key_min, /*OUT*/UWord* key_max,
                           /*OUT*/UWord* val, const RangeMap* rm, UWord key );

/* How many elements are there in the map? */
UInt VG_(sizeRangeMap) ( const RangeMap* rm );

/* Get the i'th component */
void VG_(indexRangeMap) ( /*OUT*/UWord* key_min, /*OUT*/UWord* key_max,
                          /*OUT*/UWord* val, const RangeMap* rm, Word ix );

#endif   // __PUB_TOOL_RANGEMAP_H

/*--------------------------------------------------------------------*/
/*--- end                                      pub_tool_rangemap.h ---*/
/*--------------------------------------------------------------------*/
