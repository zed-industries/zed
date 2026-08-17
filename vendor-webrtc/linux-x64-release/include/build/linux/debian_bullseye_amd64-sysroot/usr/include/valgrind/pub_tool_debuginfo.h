
/*--------------------------------------------------------------------*/
/*--- DebugInfo.                              pub_tool_debuginfo.h ---*/
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

#ifndef __PUB_TOOL_DEBUGINFO_H
#define __PUB_TOOL_DEBUGINFO_H

#include "pub_tool_basics.h"   // VG_ macro, DiEpoch
#include "pub_tool_xarray.h"   // XArray


/*====================================================================*/
/*=== Debuginfo epochs.                                            ===*/
/*====================================================================*/

// This returns the current epoch.
DiEpoch VG_(current_DiEpoch)(void);


/*====================================================================*/
/*=== Obtaining information pertaining to source artefacts.        ===*/
/*====================================================================*/

/* IMPORTANT COMMENT about memory persistence and ownership.

   Many functions below are returning a string in a HChar** argument.
   This memory must not be freed by the caller : it belongs to the debuginfo
   module. The returned string is *not* guaranteed to be persistent.
   The exact persistence depends on the kind of information returned,
   and of the internal implementation of the debuginfo module.
   In other words: use the memory directly after the call, and if in doubt,
   save it away.

   In general, all returned strings will be invalidated when the
   DebugInfo they correspond to is discarded. This is the case for
   the filename, dirname, fnname and objname.
   An objname might also be invalidated by changes to the address
   space manager segments, e.g. if a segment is merged with another
   segment.

   Retrieving a fnname might imply a call to the c++ demangler.
   A returned fnname is invalidated if any other call to the demangler
   is done. In particular, this means that the memory returned by one of
   the VG_(get_fnname...) functions is invalidated by :
     * another call to any of the functions VG_(get_fnname...).
     * any other call that will directly or indirectly invoke the
       c++ demangler. Such an indirect call to the demangler can a.o. be
       done by calls to pub_tool_errormgr.h functions.
   So, among others, the following is WRONG:
       VG_(get_fnname)(a1, &fnname1);
       VG_(get_fnname)(a2, &fnname2);
       ... it is WRONG to use fnname1 here ....
*/

/* Get the file/function/line number of the instruction at address
   'a'.  For these four, if debug info for the address is found, it
   copies the info into the buffer/UInt and returns True.  If not, it
   returns False.  VG_(get_fnname) always
   demangles C++ function names.  VG_(get_fnname_w_offset) is the
   same, except it appends "+N" to symbol names to indicate offsets.
   NOTE: See IMPORTANT COMMENT above about persistence and ownership. */
extern Bool VG_(get_filename) ( DiEpoch ep, Addr a, const HChar** filename );
extern Bool VG_(get_fnname)   ( DiEpoch ep, Addr a, const HChar** fnname );
extern Bool VG_(get_linenum)  ( DiEpoch ep, Addr a, UInt* linenum );
extern Bool VG_(get_fnname_w_offset)
                              ( DiEpoch ep, Addr a, const HChar** fnname );

/* This one is the most general.  It gives filename, line number and
   optionally directory name.  filename and linenum may not be NULL.
   dirname may be NULL, meaning that the caller does not want
   directory name info.
   If dirname is non-null, directory info is written to *dirname, if
   it is available; if not available, '\0' is written to the first
   byte.

   NOTE: See IMPORTANT COMMENT above about persistence and ownership.

   Returned value indicates whether any filename/line info could be
   found. */
extern Bool VG_(get_filename_linenum)
                              ( DiEpoch ep, Addr a,
                                /*OUT*/const HChar** filename,
                                /*OUT*/const HChar** dirname,
                                /*OUT*/UInt* linenum );

/* Succeeds only if we find from debug info that 'a' is the address of the
   first instruction in a function -- as opposed to VG_(get_fnname) which
   succeeds if we find from debug info that 'a' is the address of any
   instruction in a function.  Use this to instrument the start of
   a particular function.  Nb: if an executable/shared object is stripped
   of its symbols, this function will not be able to recognise function
   entry points within it.
   NOTE: See IMPORTANT COMMENT above about persistence and ownership. */
extern Bool VG_(get_fnname_if_entry) ( DiEpoch ep, Addr a,
                                       const HChar** fnname );

typedef
   enum {
      Vg_FnNameNormal,        // A normal function.
      Vg_FnNameMain,          // "main"
      Vg_FnNameBelowMain      // Something below "main", eg. __libc_start_main.
   } Vg_FnNameKind;           //   Such names are often filtered.

/* Indicates what kind of fnname it is. */
extern Vg_FnNameKind VG_(get_fnname_kind) ( const HChar* name );

/* Like VG_(get_fnname_kind), but takes a code address. */
extern Vg_FnNameKind VG_(get_fnname_kind_from_IP) ( DiEpoch ep, Addr ip );

/* Looks up data_addr in the collection of data symbols, and if found
   puts its name (or as much as will fit) into dname[0 .. n_dname-1],
   which is guaranteed to be zero terminated.  Also data_addr's offset
   from the symbol start is put into *offset. */
extern Bool VG_(get_datasym_and_offset)( DiEpoch ep, Addr data_addr,
                                         /*OUT*/const HChar** dname,
                                         /*OUT*/PtrdiffT* offset );

/* Try to form some description of DATA_ADDR by looking at the DWARF3
   debug info we have.  This considers all global variables, and 8
   frames in the stacks of all threads.  Result is written at the ends
   of DNAME{1,2}V, which are XArray*s of HChar, that have been
   initialised by the caller, and True is returned.  If no description
   is created, False is returned.  Regardless of the return value,
   DNAME{1,2}V are guaranteed to be zero terminated after the call.

   Note that after the call, DNAME{1,2} may have more than one
   trailing zero, so callers should establish the useful text length
   using VG_(strlen) on the contents, rather than VG_(sizeXA) on the
   XArray itself.
*/
Bool VG_(get_data_description)( 
        /*MOD*/ XArray* /* of HChar */ dname1v,
        /*MOD*/ XArray* /* of HChar */ dname2v,
        DiEpoch ep, Addr data_addr
     );

/* True if we have some Call Frame unwindo debuginfo for Addr a */
extern Bool VG_(has_CF_info)(Addr a);

/* Succeeds if the address is within a shared object or the main executable.
   It first searches if Addr a belongs to the text segment of debug info.
   If not found, it asks the address space manager whether it
   knows the name of the file associated with this mapping. */
extern Bool VG_(get_objname) ( DiEpoch ep, Addr a, const HChar** objname );


/* Cursor allowing to describe inlined function calls at an IP,
   by doing successive calls to VG_(describe_IP). */
typedef  struct _InlIPCursor InlIPCursor;

/* Returns info about the code address %eip:  the address, function
   name (if known) and filename/line number (if known), like this:

      0x4001BF05: realloc (vg_replace_malloc.c:339)

   eip can possibly corresponds to inlined function call(s).
   To describe eip and the inlined function calls, the following must
   be done:
       InlIPCursor *iipc = VG_(new_IIPC)(ep, eip);
       do {
          buf = VG_(describe_IP)(eip, iipc);
          ... use buf ...
       } while (VG_(next_IIPC)(iipc));
       VG_(delete_IIPC)(iipc);

   To only describe eip, without the inlined calls at eip, give a NULL iipc:
       buf = VG_(describe_IP)(eip, NULL);   

   Note, that the returned string is allocated in a static buffer local to
   VG_(describe_IP). That buffer will be overwritten with every invocation.
   Therefore, callers need to possibly stash away the string.

   Since this maps a code location to a source artefact (function names),
   new_IIPC requires a DiEpoch argument (ep) too.
*/
extern const HChar* VG_(describe_IP)(DiEpoch ep, Addr eip,
                                     const InlIPCursor* iipc);

/* Builds a IIPC (Inlined IP Cursor) to describe eip and all the inlined calls
   at eip. Such a cursor must be deleted after use using VG_(delete_IIPC). */
extern InlIPCursor* VG_(new_IIPC)(DiEpoch ep, Addr eip);
/* Move the cursor to the next call to describe.
   Returns True if there are still calls to describe.
   False if nothing to describe anymore. */
extern Bool VG_(next_IIPC)(InlIPCursor *iipc);
/* Free all memory associated with iipc. */
extern void VG_(delete_IIPC)(InlIPCursor *iipc);



/* Get an XArray of StackBlock which describe the stack (auto) blocks
   for this ip.  The caller is expected to free the XArray at some
   point.  If 'arrays_only' is True, only array-typed blocks are
   returned; otherwise blocks of all types are returned. */

typedef
   struct {
      PtrdiffT base;       /* offset from sp or fp */
      SizeT    szB;        /* size in bytes */
      Bool     spRel;      /* True => sp-rel, False => fp-rel */
      Bool     isVec;      /* does block have an array type, or not? */
      HChar    name[16];   /* first 15 chars of name (asciiz) */
   }
   StackBlock;

extern XArray* /* of StackBlock */
VG_(di_get_stack_blocks_at_ip)( Addr ip, Bool arrays_only );


/* Get an array of GlobalBlock which describe the global blocks owned
   by the shared object characterised by the given di_handle.  Asserts
   if the handle is invalid.  The caller is responsible for freeing
   the array at some point.  If 'arrays_only' is True, only
   array-typed blocks are returned; otherwise blocks of all types are
   returned. */

typedef
   struct {
      Addr  addr;
      SizeT szB;
      Bool  isVec;      /* does block have an array type, or not? */
      HChar name[16];   /* first 15 chars of name (asciiz) */
      HChar soname[16]; /* first 15 chars of name (asciiz) */
   }
   GlobalBlock;

extern XArray* /* of GlobalBlock */
VG_(di_get_global_blocks_from_dihandle) ( ULong di_handle,
                                          Bool  arrays_only );


/*====================================================================*/
/*=== Obtaining information pertaining to shared objects.          ===*/
/*====================================================================*/

/* A way to make limited debuginfo queries on a per-mapped-object
   basis. */
typedef  struct _DebugInfo  DebugInfo;

/* Returns NULL if the DebugInfo isn't found.  It doesn't matter if
   debug info is present or not. */
DebugInfo* VG_(find_DebugInfo) ( DiEpoch ep, Addr a );

/* Fish bits out of DebugInfos. */
Addr          VG_(DebugInfo_get_text_avma)   ( const DebugInfo *di );
SizeT         VG_(DebugInfo_get_text_size)   ( const DebugInfo *di );
Addr          VG_(DebugInfo_get_bss_avma)    ( const DebugInfo *di );
SizeT         VG_(DebugInfo_get_bss_size)    ( const DebugInfo *di );
Addr          VG_(DebugInfo_get_plt_avma)    ( const DebugInfo *di );
SizeT         VG_(DebugInfo_get_plt_size)    ( const DebugInfo *di );
Addr          VG_(DebugInfo_get_gotplt_avma) ( const DebugInfo *di );
SizeT         VG_(DebugInfo_get_gotplt_size) ( const DebugInfo *di );
Addr          VG_(DebugInfo_get_got_avma)    ( const DebugInfo *di );
SizeT         VG_(DebugInfo_get_got_size)    ( const DebugInfo *di );
const HChar*  VG_(DebugInfo_get_soname)      ( const DebugInfo *di );
const HChar*  VG_(DebugInfo_get_filename)    ( const DebugInfo *di );
PtrdiffT      VG_(DebugInfo_get_text_bias)   ( const DebugInfo *di );

/* Function for traversing the DebugInfo list.  When called with NULL
   it returns the first element; otherwise it returns the given
   element's successor.  Note that the order of elements in the list
   changes in response to most of the queries listed in this header,
   that explicitly or implicitly have to search the list for a
   particular code address.  So it isn't safe to assume that the order
   of the list stays constant. */
const DebugInfo* VG_(next_DebugInfo)    ( const DebugInfo *di );

/* A simple enumeration to describe the 'kind' of various kinds of
   segments that arise from the mapping of object files. */
typedef
   enum {
      Vg_SectUnknown,
      Vg_SectText,
      Vg_SectData,
      Vg_SectBSS,
      Vg_SectGOT,
      Vg_SectPLT,
      Vg_SectGOTPLT,
      Vg_SectOPD
   }
   VgSectKind;

/* Convert a VgSectKind to a string, which must be copied if you want
   to change it. */
const HChar* VG_(pp_SectKind)( VgSectKind kind );

/* Given an address 'a', make a guess of which section of which object
   it comes from.  If objname is non-NULL, then the object's name is put
   into *objname. This only looks in debug info, it does not examine
   the address space manager mapped files. */
VgSectKind VG_(DebugInfo_sect_kind)( /*OUT*/const HChar** objname, Addr a);


#endif   // __PUB_TOOL_DEBUGINFO_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
