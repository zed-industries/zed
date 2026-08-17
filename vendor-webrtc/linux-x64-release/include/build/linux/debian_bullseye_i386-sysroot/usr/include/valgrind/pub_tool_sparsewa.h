
/*--------------------------------------------------------------------*/
/*--- An sparse array (of words) implementation.                   ---*/
/*---                                          pub_tool_sparsewa.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2008-2017 OpenWorks Ltd
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

#ifndef __PUB_TOOL_SPARSEWA_H
#define __PUB_TOOL_SPARSEWA_H

#include "pub_tool_basics.h"   // UWord

//--------------------------------------------------------------------
// PURPOSE: (see coregrind/pub_core_sparsewa.h for details)
//--------------------------------------------------------------------

/////////////////////////////////////////////////////////
//                                                     //
// SparseWA: Interface                                 //
//                                                     //
/////////////////////////////////////////////////////////

// This interface is a very cut-down version of WordFM.
// If you understand how to use WordFM then it should be
// trivial to use SparseWA.

typedef  struct _SparseWA  SparseWA; /* opaque */

// Create a new one, using the specified allocator/deallocator.
// Never returns NULL.
SparseWA* VG_(newSWA) ( void*(*alloc_nofail)(const HChar* cc, SizeT), 
                        const HChar* cc,
                        void(*dealloc)(void*) );

// Delete one, and free all associated storage
void VG_(deleteSWA) ( SparseWA* swa );

// Add the binding key -> val to this swa.  Any existing binding is
// overwritten.  Returned Bool is True iff a previous binding existed.
Bool VG_(addToSWA) ( SparseWA* swa, UWord key, UWord val );

// Delete key from swa, returning val if found.
// Returned Bool is True iff the key was actually bound in the mapping.
Bool VG_(delFromSWA) ( SparseWA* swa,
                       /*OUT*/UWord* oldV,
                       UWord key );

// Indexes swa at 'key' (or, if you like, looks up 'key' in the
// mapping), and returns the associated value, if any, in *valP.
// Returned Bool is True iff a binding for 'key' actually existed.
Bool VG_(lookupSWA) ( const SparseWA* swa,
                      /*OUT*/UWord* valP,
                      UWord key );

// Set up 'swa' for iteration.
void VG_(initIterSWA) ( SparseWA* swa );

// Get the next key/val pair.  Behaviour undefined (highly likely 
// to segfault) if 'swa' has been modified since initIterSWA was
// called.  Returned Bool is False iff there are no more pairs
// that can be extracted.
Bool VG_(nextIterSWA)( SparseWA* swa,
                       /*OUT*/UWord* keyP, /*OUT*/UWord* valP );

// How many elements are there in 'swa'?  NOTE: dangerous in the
// sense that this is not an O(1) operation but rather O(N),
// since it involves walking the whole tree.
UWord VG_(sizeSWA) ( const SparseWA* swa );

#endif   // __PUB_TOOL_SPARSEWA_H

/*--------------------------------------------------------------------*/
/*--- end                                      pub_tool_sparsewa.h ---*/
/*--------------------------------------------------------------------*/
