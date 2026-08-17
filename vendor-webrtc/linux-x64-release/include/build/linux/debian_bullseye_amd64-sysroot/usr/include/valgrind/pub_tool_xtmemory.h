
/*-----------------------------------------------------------------------*/
/*--- Support functions for xtree memory reports. pub_tool_xtmemory.h ---*/
/*-----------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2016-2017 Philippe Waroquiers

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

#ifndef __PUB_TOOL_XTMEMORY_H
#define __PUB_TOOL_XTMEMORY_H

/* Type to profile allocated size and nr of blocks, typically used for
   --xtree-memory=allocs. */
typedef
   struct _XT_Allocs {
      SizeT nbytes;
      SizeT nblocks;
   } XT_Allocs;

/* Support functions to produce a full xtree memory profiling. */
/* tool must call VG_(XTMemory_Full_init) to ini full xtree memory profiling. */
extern void VG_(XTMemory_Full_init) (XT_filter_IPs_t filter_IPs_Fn);
/* Then each time a certain nr of blocks are allocated or freed, the below
   functions must be called. The arguments are:
      szB: nr of bytes for the allocated/freed block(s)
      ec_alloc : ExeContext of the allocation (original allocation for
                 free and resize_in_place).
      ec_free  : ExeContext of the free.
   The tool is responsible to properly provide the ExeContext for
   the allocation and free. For VG_(XTMemory_Full_free), ec_alloc
   must be the one that was used for the allocation of the just released
   block. */
extern void VG_(XTMemory_Full_alloc)(SizeT szB,
                                     ExeContext* ec_alloc);
extern void VG_(XTMemory_Full_free)(SizeT szB,
                                    ExeContext* ec_alloc,
                                    ExeContext* ec_free);
extern void VG_(XTMemory_Full_resize_in_place)(SizeT oldSzB, SizeT newSzB,
                                               ExeContext* ec_alloc);

/* Handle the production of a xtree memory report, either during run (fini False
   e.g. via a gdb monitor command), or at the end of execution (fini True).

   VG_(XTMemory_report) behaviour depends on the value of the command line
   options --xtree-memory=none|allocs|full and --xtree-memory-file=<filename> :
     If --xtree-memory=full, the report will be produced from the data
       provided via the calls to void VG_(XTMemory_Full_*).
     Otherwise, for --xtree-memory=allocs or for --xtree-memory=none (if fini
       is False), next_block is used to get the data for the report:
   next_block is called repetitively to get information about all allocated
   blocks, till xta->nblocks is 0.
   If filename is NULL, --xtree-memory-file is used to produce the name.
   filter_IPs_fn : used for --xtree-memory=allocs/none filtering (see
   VG_(XT_create) and XT_filter_IPs_t typdef for more information). */
extern void VG_(XTMemory_report)
     (const HChar* filename, Bool fini,
      void (*next_block)(XT_Allocs* xta, ExeContext** ec_alloc),
      XT_filter_IPs_t filter_IPs_fn);

#endif   // __PUB_TOOL_XTMEMORY_H


/*-----------------------------------------------------------------------*/
/*--- end                                         pub_tool_xtmemory.h ---*/
/*-----------------------------------------------------------------------*/
