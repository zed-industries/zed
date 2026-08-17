
/*--------------------------------------------------------------------*/
/*--- OSet: a fast data structure with no dups.    pub_tool_oset.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2005-2017 Nicholas Nethercote
      njn@valgrind.org

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

#ifndef __PUB_TOOL_OSET_H
#define __PUB_TOOL_OSET_H

#include "pub_tool_basics.h"   // Word

// This module implements an ordered set, a data structure with fast
// (eg. amortised log(n) or better) insertion, lookup and deletion of
// elements.  It does not allow duplicates, and will assert if you insert a
// duplicate to an OSet.
//
// It has two interfaces.  
//
// - The "OSetWord_" interface provides an easier-to-use interface for the
//   case where you just want to store UWord-sized values.  The user
//   provides the allocation and deallocation functions, and possibly a 
//   comparison function.
//
// - The "OSetGen_" interface provides a totally generic interface, which
//   allows any kind of structure to be put into the set.  The user provides
//   the allocation and deallocation functions.  Also, each element has a
//   key, which the lookup is done with.  The key may be the whole element
//   (eg. in an OSet of integers, each integer serves both as an element and
//   a key), or it may be only part of it (eg. if the key is a single field
//   in a struct).  The user can provide a function that compares an element
//   with a key;  this is very flexible, and with the right comparison
//   function even a (non-overlapping) interval list can be created.  But
//   the cost of calling a function for every comparison can be high during
//   lookup.  If no comparison function is provided, we assume that keys are
//   unsigned words, and that the key is the first word in each
//   element.  This fast comparison is suitable for an OSet containing
//   structs where the first element is an Addr, for example.
//   Do not assume fast comparison works properly with signed words.
//   A.o. iterating over the values will not return them in the correct
//   order.
//
// Each OSet interface also has an iterator, which makes it simple to
// traverse all the nodes in order.  Note that the iterator maintains state
// and so is non-reentrant.
//
// Note that once you insert an element into an OSet, if you modify any part
// of it looked at by your cmp() function, this may cause incorrect
// behaviour as the sorted order maintained will be wrong.

/*--------------------------------------------------------------------*/
/*--- Types                                                        ---*/
/*--------------------------------------------------------------------*/

typedef struct _OSet     OSet;

// - OSetCmp_t:   returns -1, 0 or 1 if key is <, == or > elem.
typedef Word  (*OSetCmp_t)         ( const void* key, const void* elem );

/*--------------------------------------------------------------------*/
/*--- Creating and destroying OSets (UWord)                        ---*/
/*--------------------------------------------------------------------*/

// * Create: allocates and initialises the OSet.  Never returns NULL.
//   Parameters:
//   - alloc_fn  The allocation function used internally for allocating the
//               OSet and all its nodes. It must not return NULL (that is,
//               if it returns it must have succeeded.)
//   - cc        Cost centre string used by 'alloc'.
//   - free_fn   The deallocation function used internally for freeing nodes
//               called by VG_(OSetWord_Destroy)().
//
// * Destroy: frees all nodes in the table, plus the memory used by
//   the table itself.  The passed-in function is called on each node first
//   to allow the destruction of any attached resources;  if NULL it is not
//   called.

extern OSet* VG_(OSetWord_Create) ( Alloc_Fn_t alloc_fn, const HChar* cc,
                                    Free_Fn_t free_fn );
extern void  VG_(OSetWord_Destroy) ( OSet* os );

/*--------------------------------------------------------------------*/
/*--- Operations on OSets (UWord)                                  ---*/
/*--------------------------------------------------------------------*/

// In everything that follows, the parameter 'key' is always the *address*
// of the key, and 'elem' is *address* of the elem, as are the return values
// of the functions that return elems.
//
// * Size: The number of elements in the set.
//
// * Contains: Determines if the value is in the set.
//
// * Insert: Inserts a new element into the set.  Duplicates are forbidden,
//   and will cause assertion failures.
//
// * Remove: Removes the value from the set, if present.  Returns a Bool
//   indicating if the value was removed.
//
// * ResetIter: Each OSet has an iterator.  This resets it to point to the
//   first element in the OSet.
// 
// * Next: Copies the next value according to the OSet's iterator into &val,
//   advances the iterator by one, and returns True;  the elements are
//   visited in increasing order of unsigned words (UWord).  Or, returns
//   False if the iterator has reached the set's end.
//   
//   You can thus iterate in order through a set like this:
//
//     Word val;
//     VG_(OSetWord_ResetIter)(oset);
//     while ( VG_(OSetWord_Next)(oset, &val) ) {
//        ... do stuff with 'val' ...
//     }
//
//   Note that iterators are cleared any time an element is inserted or
//   removed from the OSet, to avoid possible mayhem caused by the iterator
//   getting out of sync with the OSet's contents.  "Cleared" means that
//   they will return False if VG_(OSetWord_Next)() is called without an
//   intervening call to VG_(OSetWord_ResetIter)().

extern Word  VG_(OSetWord_Size)         ( const OSet* os );
extern void  VG_(OSetWord_Insert)       ( OSet* os, UWord val );
extern Bool  VG_(OSetWord_Contains)     ( const OSet* os, UWord val );
extern Bool  VG_(OSetWord_Remove)       ( OSet* os, UWord val );
extern void  VG_(OSetWord_ResetIter)    ( OSet* os );
extern Bool  VG_(OSetWord_Next)         ( OSet* os, /*OUT*/UWord* val );


/*--------------------------------------------------------------------*/
/*--- Creating and destroying OSets and OSet members (Gen)         ---*/
/*--------------------------------------------------------------------*/

// * Create: allocates and initialises the OSet. Never returns NULL.
//   Parameters:
//   - keyOff    The offset of the key within the element.
//   - cmp       The comparison function between keys and elements, or NULL
//               if the OSet should use fast comparisons.
//   - alloc_fn  The allocation function used for allocating the OSet itself;
//               It must not return NULL (that is, if it returns it must
//               have succeeded.)
//               If a pool allocator is used, it's called to allocate pool of
//               nodes.
//               If no pool allocator is used, it's called for each
//               invocation of VG_(OSetGen_AllocNode)().
//   - cc        Cost centre string used by 'alloc'.
//   - free_fn   If no pool allocator is used, this is the deallocation
//               function used by VG_(OSetGen_FreeNode)() and
//               VG_(OSetGen_Destroy)().
//               If a pool allocator is used, the memory used by the nodes is
//               deallocated when the pool is deleted.
//   (for more details about pool allocators, see pub_tool_poolalloc.h).
//   
//
//   If cmp is NULL, keyOff must be zero.  This is checked.
//
// * Destroy: frees all nodes in the table, plus the memory used by
//   the table itself.  The passed-in function is called on each node first
//   to allow the destruction of any attached resources;  if NULL it is not
//   called.
//
// * AllocNode: Allocate and zero memory for a node to go into the OSet.
//   If a pool allocator is used, it uses the pool allocator to allocate a node.
//   Otherwise, uses the alloc function given to VG_(OSetGen_Create)() to
//   allocate a node which is big enough for both an element and the OSet
//   metadata.
//   Not all elements in one OSet have to be the same size.
//   However, if a pool allocator is used, elements will all have a size equal
//   to the max user data size given at creation + the node meta data size.
//
//   Note that the element allocated will be at most word-aligned, which may
//   be less aligned than the element type would normally be.
//
// * FreeNode: Deallocate a node allocated with OSetGen_AllocNode().  Using
//   a deallocation function (such as VG_(free)()) directly will likely
//   lead to assertions in Valgrind's allocator.

extern OSet* VG_(OSetGen_Create)    ( PtrdiffT keyOff, OSetCmp_t cmp,
                                      Alloc_Fn_t alloc_fn, const HChar* cc,
                                      Free_Fn_t free_fn);


extern OSet* VG_(OSetGen_Create_With_Pool)    ( PtrdiffT keyOff, OSetCmp_t cmp,
                                                Alloc_Fn_t alloc_fn,
                                                const HChar* cc,
                                                Free_Fn_t free_fn,
                                                SizeT poolSize,
                                                SizeT maxEltSize);
// Same as VG_(OSetGen_Create) but created OSet will use a pool allocator to
// allocate the nodes.
// The node size is the sum of a fixed small meta data size needed for OSet
// + the size of the user data element.
// The maximum size for the user data element is specified by maxEltSize.
// (if poolSize is 0, maxEltSize is not relevant for the OSet).
// It is interesting to use a pool allocator when an OSet has many elements,
// and these elements have a small fixed size, or have a variable size, but
// always <= than a (small) maximum value.
// In such a case, allocating the nodes in pools reduces significantly
// the memory overhead needed by each node.
// When a node is freed (i.e. OSetGen_Freenode is called), the node is
// put back in the pool allocator free list (for sub-sequent re-use by
// OSetGen_AllocNode). Note that the pool memory is only released when
// the pool is destroyed : calls to VG_(OSetGen_Free) do not cause
// any calls to OSetFree_t _free function.
// If there are several OSet managing similar such elements, it might be
// interesting to use a shared pool for these OSet.
// To have multiple OSets sharing a pool allocator, create the first OSet
// with VG_(OSetGen_Create_With_Pool). Create subsequent OSet with
// VG_(OSetGen_EmptyClone).

extern void  VG_(OSetGen_Destroy)   ( OSet* os );
extern void* VG_(OSetGen_AllocNode) ( const OSet* os, SizeT elemSize );
extern void  VG_(OSetGen_FreeNode)  ( const OSet* os, void* elem );

extern OSet* VG_(OSetGen_EmptyClone) (const OSet* os);
// Creates a new empty OSet.
// The new OSet will have the same characteristics as os.
// If os uses a pool allocator, this pool allocator will be shared with
// the new OSet. A shared pool allocator is only deleted (and its memory is
// released) when the last OSet using the shared pool is destroyed.

/*-------------------------------------------------------------------*/
/*--- Operations on OSets (Gen)                                    ---*/
/*--------------------------------------------------------------------*/

// In everything that follows, the parameter 'key' is always the *address*
// of the key, and 'elem' is *address* of the elem, as are the return values
// of the functions that return elems.
//
// * Size: The number of elements in the set.
//
// * Insert: Inserts a new element into the set.  Note that 'elem' must
//   have been allocated using VG_(OSetGen_AllocNode)(), otherwise you will
//   get assertion failures about "bad magic".  Duplicates are forbidden,
//   and will also cause assertion failures.
//
// * Contains: Determines if any element in the OSet matches the key.
//
// * Lookup: Returns a pointer to the element matching the key, if there is
//   one, otherwise returns NULL.
//
// * LookupWithCmp: Like Lookup, but you specify the comparison function,
//   which overrides the OSet's normal one.
//
// * Remove: Removes the element matching the key, if there is one.  Returns
//   NULL if no element matches the key.
//
// * ResetIter: Each OSet has an iterator.  This resets it to point to the
//   first element in the OSet.
// 
// * ResetIterAt: Like ResetIter, but instead of resetting the iterator to the
//   smallest element, it resets the iterator to point to the smallest element
//   in the set whose key is greater-than-or-equal to the given key.  (In many
//   cases this will be the element whose key equals that of the given key.)
//
// * Next: Returns a pointer to the element pointed to by the OSet's
//   iterator, and advances the iterator by one;  the elements are visited
//   in order.  Or, returns NULL if the iterator has reached the OSet's end.
//   
//   You can thus iterate in order through a set like this:
//
//     VG_(OSetGen_ResetIter)(oset);
//     while ( (elem = VG_(OSetGen_Next)(oset)) ) {
//        ... do stuff with 'elem' ...
//     }
//
//   Note that iterators are cleared any time an element is inserted or
//   removed from the OSet, to avoid possible mayhem caused by the iterator
//   getting out of sync with the OSet's contents.  "Cleared" means that
//   they will return NULL if VG_(OSetGen_Next)() is called without an
//   intervening call to VG_(OSetGen_ResetIter)().

extern UInt  VG_(OSetGen_Size)         ( const OSet* os );
extern void  VG_(OSetGen_Insert)       ( OSet* os, void* elem );
extern Bool  VG_(OSetGen_Contains)     ( const OSet* os, const void* key );
extern void* VG_(OSetGen_Lookup)       ( const OSet* os, const void* key );
extern void* VG_(OSetGen_LookupWithCmp)( OSet* os,
                                         const void* key, OSetCmp_t cmp );
extern void* VG_(OSetGen_Remove)       ( OSet* os, const void* key );
extern void  VG_(OSetGen_ResetIter)    ( OSet* os );
extern void  VG_(OSetGen_ResetIterAt)  ( OSet* os, const void* key );
extern void* VG_(OSetGen_Next)         ( OSet* os );


#endif   // __PUB_TOOL_OSET_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
