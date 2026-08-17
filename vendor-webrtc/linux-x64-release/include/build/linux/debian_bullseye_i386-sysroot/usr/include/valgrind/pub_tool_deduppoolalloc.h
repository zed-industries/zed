
/*--------------------------------------------------------------------*/
/*--- A pool (memory) allocator that avoids duplicated copies.     ---*/
/*---                                    pub_tool_deduppoolalloc.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2014-2017 Philippe Waroquiers philippe.waroquiers@skynet.be

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

#ifndef __PUB_TOOL_DEDUPPOOLALLOC_H
#define __PUB_TOOL_DEDUPPOOLALLOC_H

#include "pub_tool_basics.h"   // UWord

//-----------------------------------------------------------------------------
// PURPOSE: Provides a pool allocator for elements, storing only once identical
// elements. In other words, this can be considered a "dictionary" of elements.
//
// This pool allocator manages elements allocation by allocating "pools" of
// many elements from a lower level allocator (typically pub_tool_mallocfree.h).
// Single elements are allocated from these pools.
// Currently, elements can only be allocated, elements cannot be freed
// individually.
// Once allocated, an element must not be modified anymore.
//
// Elements can be inserted in the pool using VG_(allocEltDedupPA),
// VG_(allocFixedEltDedupPA) or VG_(allocStrDedupPA).
//
// Use VG_(allocFixedEltDedupPA) to allocate elements that are all of
// the same size and that you want to identify with a (small) number:
// VG_(allocFixedEltDedupPA) will assign a sequence number to each
// unique allocated element. This unique number can be translated to
// an address when the element data must be used.
// The idea is that such small numbers can be used as reference instead
// of the element address, to spare memory.
// Elements are numbered starting from 1. The nr 0 can thus be used
// as 'null element'. The address identified by a nr can change
// if new elements are inserted in the pool. Once the pool is frozen,
// an element address does not change.
//
// Use VG_(allocEltDedupPA) for variable size elements or when the
// memory needed to store the element reference is not critical or
// when performance to access elements is critical.
// The address of an element allocated with VG_(allocEltDedupPA) does
// not change, even if new elements are inserted in the pool.
//
// Use VG_(allocStrDedupPA) to create a pool of strings (in other words, a
//  dictionnary of strings). Similarly to VG_(allocFixedEltDedupPA), strings
// inserted in a dedup pool can be identified by an element number.
//
// In the same pool, you can only use one of the allocate element functions.
// 
// A dedup pool allocator has significantly less memory overhead than
// calling directly pub_tool_mallocfree.h if the deduplication factor
// is big. However, allocating an element incurs a cost for searching
// if an identical element is already in the pool.
//
// Note: the elements of the pool cannot be freed (at least currently).
// The only way to free the elements is to delete the dedup pool allocator.
//--------------------------------------------------------------------


typedef  struct _DedupPoolAlloc  DedupPoolAlloc;

/* Create new DedupPoolAlloc, using given allocation and free function.
   alloc_fn must not return NULL (that is, if it returns it must have
   succeeded.)
   poolSzB is the (minimum) size in bytes of the pool of elements allocated
   with alloc. 
   eltAlign is the minimum required alignement for the elements allocated
   from the DedupPoolAlloc.
   This function never returns NULL. */
extern DedupPoolAlloc* VG_(newDedupPA) ( SizeT  poolSzB,
                                         SizeT  eltAlign,
                                         Alloc_Fn_t alloc_fn,
                                         const  HChar* cc,
                                         Free_Fn_t free_fn );

/* Allocates or retrieve element from ddpa with eltSzB bytes to store elt.
   This function never returns NULL.
   If ddpa already contains an element equal to elt, then the address of
   the already existing element is returned.
   Equality between elements is done by comparing all bytes.
   So, if void *elt points to a struct, be sure to initialise all components
   and the holes between components. */
extern const void* VG_(allocEltDedupPA) (DedupPoolAlloc* ddpa,
                                         SizeT eltSzB, const void* elt);

/* Allocates or retrieve a (fixed size) element from ddpa. Returns the
   unique number identifying this element.
   Similarly to VG_(allocEltDedupPA), this will return the unique number
   of an already existing identical element to elt. */
extern UInt VG_(allocFixedEltDedupPA) (DedupPoolAlloc* ddpa,
                                       SizeT eltSzB, const void* elt);

/* Translate an element number to its address. Note that the address
   corresponding to eltNr can change if new elements are inserted
   in the pool. */
extern void* VG_(indexEltNumber) (DedupPoolAlloc* ddpa,
                                  UInt eltNr);

/* Allocates or retrieve a string element from ddpa. Returns the
   unique number identifying this string.
   newStr is set to True if the str is a newly inserted string, False
   if the str was already present in the pool.
   Similarly to VG_(allocEltDedupPA), this will return the unique number
   of an already existing identical string. */
extern UInt VG_(allocStrDedupPA) (DedupPoolAlloc *ddpa,
                                  const HChar* str,
                                  Bool* newStr);
/* Note: Implementing a function to return the string value from its strNr
   implies some overhead, so will be done only if/when needed. */


/* The Dedup Pool Allocator must maintain a data structure to avoid
   duplicates as long as new elements can be allocated from the pool.
   Once no new elements will be allocated, this dedup data structure
   can be released using VG_(freezeDedupPA). Once ddpa has been frozen,
   it is an error to call VG_(allocEltDedupPA) or VG_(allocFixedEltDedupPA).
   If shrink_block is not NULL, the last pool will be shrunk using
   shrink_block. */
extern void VG_(freezeDedupPA) (DedupPoolAlloc* ddpa,
                                void (*shrink_block)(void*, SizeT));

/* How many (unique) elements are there in this ddpa now? */
extern UInt VG_(sizeDedupPA) (DedupPoolAlloc* ddpa);

/* Free all memory associated with a DedupPoolAlloc. */
extern void VG_(deleteDedupPA) ( DedupPoolAlloc* ddpa);

#endif   // __PUB_TOOL_DEDUPPOOLALLOC_

/*--------------------------------------------------------------------*/
/*--- end                               pub_tool_deduppoolalloc.h  ---*/
/*--------------------------------------------------------------------*/
