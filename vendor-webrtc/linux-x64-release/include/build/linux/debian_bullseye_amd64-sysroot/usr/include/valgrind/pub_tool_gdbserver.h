
/*--------------------------------------------------------------------*/
/*--- Handle remote gdb protocol.             pub_tool_gdbserver.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2011-2017 Philippe Waroquiers

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

#ifndef __PUB_TOOL_GDBSERVER_H
#define __PUB_TOOL_GDBSERVER_H

#include "pub_tool_basics.h"   // VG_ macro

//--------------------------------------------------------------------
// PURPOSE: This module provides the support to have a gdb
// connecting to a valgrind process using remote gdb protocol. It provides
//  * A function to allow a tool (or the valgrind core) to
//    wait for a gdb to connect and then handle gdb commands.
//    Typically, this can be used to let the user debug the process
//    when valgrind reports an error.
//  * A function allowing to instrument the code to support gdb breakpoints.
//  * A function allowing the tool to support watchpoints.
//  * A utility function to help implementing the processing of the
//    gdb_monitor_command strings.


// Function to be used by tool or coregrind to allow a gdb to connect
// to this process. 
// Calling VG_(gdbserver) with tid > 0 means to let a debugger attach
// to the valgrind process. gdbserver will report to gdb that the
// process stopped in thread tid.
// Calling VG_(gdbserver) with tid == 0 indicates to close
// the connection with GDB (if still open) and stop gdbserver.
//--------------------------------------------------------------------
extern void VG_(gdbserver) ( ThreadId tid );

/* defines the various kinds of breakpoints that gdbserver
   might ask to insert/remove. Note that the below matches
   the gdbserver protocol definition. The level of support
   of the various breakpoint kinds depends on the tool.

   For the moment, it is unclear how a tool would implement
   hardware_breakpoint in valgrind  :).

   software_breakpoint implies some (small) specific
   instrumentation to be done for gdbserver. This instrumentation
   is implemented for all tools in m_translate.c.

   write/read/access watchpoints can only be done by tools
   which are maintaining some notion of address accessibility
   as part of their instrumentation. watchpoints can then
   be done by marking the watched address(es) as not accessible.
   But instead of giving back an error (or whatever the tool
   wants to do with unaccessible mechanism), the tool must then
   just call gdbserver. See memcheck for an example of reusing
   accessibility for watchpoint support.
*/
typedef
   enum {
      software_breakpoint,
      hardware_breakpoint,
      write_watchpoint,
      read_watchpoint,
      access_watchpoint } PointKind;
extern const HChar* VG_(ppPointKind) (PointKind kind);


/* watchpoint support --------------------------------------*/
/* True if one or more bytes in [addr, addr+len[ are being watched by
   gdbserver for write or read or access.
   In addition, VG_(is_watched) will invoke gdbserver if
   the access provided by the tool matches the watchpoint kind.
   For this, the tool must pass the kind of access it has detected:
      write_watchpoint indicates the tool has detected a write
      read_watchpoint indicates the tool has detected a read
      access_watchpoint indicates the tool has detected an access but does
      not know if this is a read or a write
*/
extern Bool VG_(is_watched)(PointKind kind, Addr addr, Int szB);

extern void VG_(needs_watchpoint) (
   // indicates the given Addr/len is being watched (insert)
   // or not watched anymore (! insert).
   // gdbserver will maintain the list of watched addresses.
   // The tool can use VG_(is_watched) to verify if an
   // access to an Addr is in one of the watched intervals.
   // Must return True if the watchpoint has been properly inserted or
   // removed. False if not supported.
   // Note that an address can only be watched for a single kind.
   // The tool must be ready to be called successively with
   // multiple kinds for the same addr and len and with
   // different kinds. The last kind must replace the previous values.
   // Behaviour with multiple watches having overlapping addr+len
   // is undefined.
   Bool (*watchpoint) (PointKind kind, Bool insert, Addr addr, SizeT len)
);


// can be used during the processing of the VG_USERREQ__GDB_MONITOR_COMMAND 
// tool client request to output information to gdb or vgdb.
// The output of VG_(gdb_printf) is not subject to 'output control'
// by the user: e.g. the monitor command 'v.set log_output' has no effect.
// The output of VG_(gdb_printf) is given to gdb/vgdb. The only case
// in which this output is not given to gdb/vgdb is when the connection
// with gdb/vgdb has been lost : in such a case, output is written
// to the valgrind log output.
// To produce some output which is subject to user output control via
// monitor command v.set gdb_output or mixed output, use VG_(printf)
// or VG_(umsg) or similar.
// Typically, VG_(gdb_printf) has to be used when there is no point
// having this output in the output log of Valgrind. Examples
// is the monitor help output, or if vgdb is used to implement
// 'tool control scripts' such as callgrind_control.
extern UInt VG_(gdb_printf) ( const HChar *format, ... ) PRINTF_CHECK(1, 2);

/* Utility functions to (e.g.) parse gdb monitor commands.

   keywords is a set of keywords separated by a space
   keyword_id will search for the keyword starting with the string input_word
   and return its position.
   It returns -1 if no keyword matches.
   It returns -2 if two or more keywords are starting with input_word
                 and none of these matches exactly input_word
   Example with keywords = "hello world here is hell" :
   input_word    result
   ----------    ------
   paradise   => -1
   i          =>  3
   hell       =>  4
   hel        => -2
   ishtar     => -1

   report indicates when to output an error msg with VG_(gdb_printf).
   kwd_report_none : no error is reported.
   kwd_report_all : the error msg will show all possible keywords
   kwd_report_duplicated_matches : the error msg will show only the
     ambiguous matches.
*/
typedef
   enum {
      kwd_report_none,
      kwd_report_all,
      kwd_report_duplicated_matches } kwd_report_error;
extern Int VG_(keyword_id) (const HChar* keywords, const HChar* input_word, 
                            kwd_report_error report);

/* Extract an address and (optionally) a size from the string
   currently being parsed by strtok_r (see pub_tool_libcbase.h).
   If no size in the string, keeps the current value of szB.
   If parsing is ok,
     returns True.
   If parsing is not ok;
     set *address and *szB to 0,
     reports problem to the user using VG_(gdb_printf)
     returns False. */
extern Bool VG_(strtok_get_address_and_size) (Addr* address, 
                                              SizeT* szB, 
                                              HChar **ssaveptr);

/* Print various statistics about Valgrind core,
   and optionally tool and memory statistics. */
extern void VG_(print_all_stats) (Bool memory_stats, Bool tool_stats);

#endif   // __PUB_TOOL_GDBSERVER_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
