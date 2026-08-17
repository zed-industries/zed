
/*--------------------------------------------------------------------*/
/*--- Malloc replacement.                 pub_tool_replacemalloc.h ---*/
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

#ifndef __PUB_TOOL_REPLACEMALLOC_H
#define __PUB_TOOL_REPLACEMALLOC_H

#include "pub_tool_basics.h"   // Addr

/* If a tool replaces malloc() et al, the easiest way to do so is to
   link libreplacemalloc_toolpreload.o into its vgpreload_*.so file, and
   use the functions declared below.  You can do it from scratch,
   though, if you enjoy that sort of thing. */

/* Can be called from VG_(tdict).malloc_malloc et al to do the actual
 * alloc/freeing. */
extern void* VG_(cli_malloc) ( SizeT align, SizeT nbytes );
extern void  VG_(cli_free)   ( void* p );
// Returns the usable size of a heap-block.  It's the asked-for size plus
// possibly some more due to rounding up.
extern SizeT VG_(cli_malloc_usable_size)( void* p );


/* If a tool uses deferred freeing (e.g. memcheck to catch accesses to
   freed memory) it can maintain number and total size of queued blocks
   in these variable to provide more accurate statistics about client
   memory usage. Currently used by mallinfo(). */
extern Long VG_(free_queue_volume);
extern Long VG_(free_queue_length);

/* Check if an address is within a range, allowing for redzones at edges */
extern Bool VG_(addr_is_in_block)( Addr a, Addr start,
                                   SizeT size, SizeT rz_szB );

/* ------------------------------------------------------------------ */
/* Some options that can be used by a tool if malloc() et al are replaced.
   The tool should call the functions in the appropriate places to give
   control over these aspects of Valgrind's version of malloc(). */

/* DEBUG: print malloc details?  default: NO */
extern Bool VG_(clo_trace_malloc);
/* Minimum alignment in functions that don't specify alignment explicitly.
   default: VG_MIN_MALLOC_SZB */
extern UInt VG_(clo_alignment);

extern Bool VG_(replacement_malloc_process_cmd_line_option) ( const HChar* arg );

// If tool is replacing malloc for the client, the below returns
// the effective client redzone as derived from the default
// provided by the tool, VG_(clo_redzone_size) and the minimum
// redzone required by m_mallocfree.c.
// It is an error to call this before VG_(needs_malloc_replacement) has
// been called.
extern SizeT VG_(malloc_effective_client_redzone_size)(void);

#endif   // __PUB_TOOL_REPLACEMALLOC_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
