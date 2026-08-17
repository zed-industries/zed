
/*--------------------------------------------------------------------*/
/*--- Address space manager.                  pub_tool_aspacemgr.h ---*/
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

#ifndef __PUB_TOOL_ASPACEMGR_H
#define __PUB_TOOL_ASPACEMGR_H

#include "pub_tool_basics.h"   // VG_ macro

//--------------------------------------------------------------
// Definition of address-space segments

/* Describes segment kinds. Enumerators are one-hot encoded so they
   can be or'ed together. */
typedef
   enum {
      SkFree  = 0x01,  // unmapped space
      SkAnonC = 0x02,  // anonymous mapping belonging to the client
      SkAnonV = 0x04,  // anonymous mapping belonging to valgrind
      SkFileC = 0x08,  // file mapping belonging to the client
      SkFileV = 0x10,  // file mapping belonging to valgrind
      SkShmC  = 0x20,  // shared memory segment belonging to the client
      SkResvn = 0x40   // reservation
   }
   SegKind;

/* Describes how a reservation segment can be resized. */
typedef
   enum {
      SmLower,  // lower end can move up
      SmFixed,  // cannot be shrunk
      SmUpper   // upper end can move down
   }
   ShrinkMode;

/* Describes a segment.  Invariants:

     kind == SkFree:
        // the only meaningful fields are .start and .end

     kind == SkAnon{C,V}:
        // smode==SmFixed
        // there's no associated file:
        dev==ino==foff = 0, fnidx == -1
        // segment may have permissions

     kind == SkFile{C,V}:
        // smode==SmFixed
        // there is an associated file
        // segment may have permissions

     kind == SkShmC:
        // smode==SmFixed
        // there's no associated file:
        dev==ino==foff = 0, fnidx == -1
        // segment may have permissions

     kind == SkResvn
        // the segment may be resized if required
        // there's no associated file:
        dev==ino==foff = 0, fnidx == -1
        // segment has no permissions
        hasR==hasW==hasX == False

     Also: hasT==True is only allowed in SkFileC, SkAnonC, and SkShmC
           (viz, not allowed to make translations from non-client areas)
*/
typedef
   struct {
      SegKind kind;
      /* Extent */
      Addr    start;    // lowest address in range
      Addr    end;      // highest address in range
      /* Shrinkable? (SkResvn only) */
      ShrinkMode smode;
      /* Associated file (SkFile{C,V} only) */
      ULong   dev;
      ULong   ino;
      Off64T  offset;
      UInt    mode;
      Int     fnIdx;    // file name table index, if name is known
      /* Permissions (SkAnon{C,V}, SkFile{C,V} only) */
      Bool    hasR;
      Bool    hasW;
      Bool    hasX;
      Bool    hasT;     // True --> translations have (or MAY have)
                        // been taken from this segment
      Bool    isCH;     // True --> is client heap (SkAnonC ONLY)
   }
   NSegment;


/* Collect up the start addresses of segments whose kind matches one of
   the kinds specified in kind_mask.
   The interface is a bit strange in order to avoid potential
   segment-creation races caused by dynamic allocation of the result
   buffer *starts.

   The function first computes how many entries in the result
   buffer *starts will be needed.  If this number <= nStarts,
   they are placed in starts[0..], and the number is returned.
   If nStarts is not large enough, nothing is written to
   starts[0..], and the negation of the size is returned.

   Correct use of this function may mean calling it multiple times in
   order to establish a suitably-sized buffer. */
extern Int VG_(am_get_segment_starts)( UInt kind_mask, Addr* starts,
                                       Int nStarts );

/* Finds the segment containing 'a'.  Only returns file/anon/resvn
   segments.  This returns a 'NSegment const *' - a pointer to
   readonly data. */
extern NSegment const * VG_(am_find_nsegment) ( Addr a ); 

/* Get the filename corresponding to this segment, if known and if it
   has one. The function may return NULL if the file name is not known. */
extern const HChar* VG_(am_get_filename)( NSegment const * );

/* Is the area [start .. start+len-1] validly accessible by the 
   client with at least the permissions 'prot' ?  To find out
   simply if said area merely belongs to the client, pass 
   VKI_PROT_NONE as 'prot'.  Will return False if any part of the
   area does not belong to the client or does not have at least
   the stated permissions. */
extern Bool VG_(am_is_valid_for_client) ( Addr start, SizeT len, 
                                          UInt prot );

/* Really just a wrapper around VG_(am_mmap_anon_float_valgrind). */
extern void* VG_(am_shadow_alloc)(SizeT size);

/* Unmap the given address range and update the segment array
   accordingly.  This fails if the range isn't valid for valgrind. */
extern SysRes VG_(am_munmap_valgrind)( Addr start, SizeT length );

#endif   // __PUB_TOOL_ASPACEMGR_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
