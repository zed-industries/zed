
/*--------------------------------------------------------------------*/
/*--- Obtaining information about an address.  pub_tool_addrinfo.h ---*/
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

#ifndef __PUB_TOOL_ADDRINFO_H
#define __PUB_TOOL_ADDRINFO_H

#include "pub_tool_basics.h"   // VG_ macro
#include "pub_tool_aspacemgr.h"  // SegKind

/*====================================================================*/
/*=== Obtaining information about an address                       ===*/
/*====================================================================*/

// Different kinds of blocks.
// Block_Mallocd is used by tools that maintain detailed information about
//   Client allocated heap blocks.
// Block_Freed is used by tools such as memcheck that maintain a 'quarantine' 
//   list of blocks freed by the Client but not yet physically freed.
// Block_MempoolChunk and Block_UserG are used for mempool or user defined heap
//   blocks.
// Block_ClientArenaMallocd and Block_ClientArenaFree are used when the tool
//   replaces the malloc/free/... functions but does not maintain detailed
//   information about Client allocated heap blocks.
// Block_ValgrindArenaMallocd and Block_ValgrindArenaFree are used for heap
//   blocks of Valgrind internal heap.
typedef enum {
   Block_Mallocd = 111,
   Block_Freed,
   Block_MempoolChunk,
   Block_UserG,
   Block_ClientArenaMallocd,
   Block_ClientArenaFree,
   Block_ValgrindArenaMallocd,
   Block_ValgrindArenaFree,
} BlockKind;

/* ------------------ Addresses -------------------- */

/* The classification of a faulting address. */
typedef 
   enum { 
      Addr_Undescribed, // as-yet unclassified
      Addr_Unknown,     // classification yielded nothing useful
      Addr_Block,       // in malloc'd/free'd block
      Addr_Stack,       // on a thread's stack       
      Addr_DataSym,     // in a global data sym
      Addr_Variable,    // variable described by the debug info
      Addr_SectKind,    // Section from a mmap-ed object file
      Addr_BrkSegment,  // address in brk data segment
      Addr_SegmentKind  // Client segment (mapped memory)
   }
   AddrTag;

/* For an address in a stack, gives the address position in this stack. */
typedef 
   enum {
      StackPos_stacked,         // Addressable and 'active' stack zone.
      StackPos_below_stack_ptr, // Below stack ptr
      StackPos_guard_page       // In the guard page
   }
   StackPos;


/* Note about ThreadInfo tid and tnr in various parts of _Addrinfo:
   A tid is an index in the VG_(threads)[] array. The entries
   in  VG_(threads) array are re-used, so the tid in an 'old' _Addrinfo
   might be misleading: if the thread that was using tid has been terminated
   and the tid was re-used by another thread, the tid will very probably
   be wrongly interpreted by the user.
   So, such an _Addrinfo should be printed just after it has been produced,
   before the tid could possibly be re-used by another thread.

   A tool such as helgrind is uniquely/unambiguously identifying each thread
   by a number. If the tool sets tnr between the call to
   VG_(describe_addr) and the call to VG_(pp_addrinfo), then VG_(pp_addrinfo)
   will by preference print tnr instead of tid.
   Visually, a tid will be printed as   thread %d
   while a tnr will be printed as       thread #%d
*/

typedef
   struct _ThreadInfo {
      ThreadId tid;   // 0 means thread not known.
      UInt     tnr;   // 0 means no tool specific thread nr, or not known.
   } ThreadInfo;

/* Zeroes/clear all the fields of *tinfo. */
extern void VG_(initThreadInfo) (ThreadInfo *tinfo);

typedef
   struct _AddrInfo
   AddrInfo;
   
struct _AddrInfo {
   AddrTag tag;
   union {
      // As-yet unclassified.
      struct { } Undescribed;

      // On a stack. tinfo indicates which thread's stack?
      // IP is the address of an instruction of the function where the
      // stack address was. 0 if not found. IP can be symbolised using epoch.
      // frameNo is the frame nr of the call where the stack address was.
      // -1 if not found.
      // stackPos describes the address 'position' in the stack.
      // If stackPos is StackPos_below_stack_ptr or StackPos_guard_page,
      // spoffset gives the offset from the thread stack pointer.
      // (spoffset will be negative, as stacks are assumed growing down).
      struct {
         ThreadInfo tinfo;
         DiEpoch  epoch;
         Addr     IP;
         Int      frameNo;
         StackPos stackPos;
         PtrdiffT spoffset;
      } Stack;

      // This covers heap blocks (normal and from mempools), user-defined
      // blocks and Arena blocks.
      // alloc_tinfo identifies the thread that has allocated the block.
      // This is used by tools such as helgrind that maintain
      // more detailed information about client blocks.
      struct {
         BlockKind   block_kind;
         const HChar* block_desc;   // "block","mempool","user-defined",arena
         SizeT       block_szB;
         PtrdiffT    rwoffset;
         ExeContext* allocated_at; // might contain null_ExeContext.
         ThreadInfo  alloc_tinfo;  // which thread alloc'd this block.
         ExeContext* freed_at;     // might contain null_ExeContext.
      } Block;

      // In a global .data symbol.  This holds
      // the variable's name (zero terminated), plus a (memory) offset.
      struct {
         HChar   *name;
         PtrdiffT offset;
      } DataSym;

      // Is described by Dwarf debug info.  XArray*s of HChar.
      struct {
         XArray* /* of HChar */ descr1;
         XArray* /* of HChar */ descr2;
      } Variable;

      // Could only narrow it down to be the PLT/GOT/etc of a given
      // object.  Better than nothing, perhaps.
      struct {
         HChar      *objname;
         VgSectKind kind;
      } SectKind;

      // Described address is or was in the brk data segment.
      // brk_limit is the limit that was in force
      // at the time address was described. 
      // If address is >= brk_limit, it means address is in a zone
      // of the data segment that was shrinked.
      struct {
         Addr brk_limit; // limit in force when address was described.
      } BrkSegment;

      struct {
         SegKind segkind;   // SkAnonC, SkFileC or SkShmC.
         HChar   *filename; // NULL if segkind != SkFileC
         Bool    hasR;
         Bool    hasW;
         Bool    hasX;
      } SegmentKind;

      // Classification yielded nothing useful.
      struct { } Unknown;

   } Addr;
};


/* Describe an address as best you can, putting the result in ai.
   On entry, ai->tag must be equal to Addr_Undescribed.
   This might allocate some memory, that can be cleared with
   VG_(clear_addrinfo). */
extern void VG_(describe_addr) ( DiEpoch ep, Addr a, /*OUT*/AddrInfo* ai );

extern void VG_(clear_addrinfo) ( AddrInfo* ai);

/* Prints the AddrInfo ai describing a.
   Note that an ai with tag Addr_Undescribed will cause an assert.*/
extern void VG_(pp_addrinfo) ( Addr a, const AddrInfo* ai );

/* Same as VG_(pp_addrinfo) but provides some memcheck specific behaviour:
   * maybe_gcc indicates Addr a was just below the stack ptr when the error
     with a was encountered.
   * the message for Unknown tag is slightly different, as memcheck
     has a recently freed list. */
extern void VG_(pp_addrinfo_mc) ( Addr a, const AddrInfo* ai, Bool maybe_gcc );

#endif   // __PUB_TOOL_ADDRINFO_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
