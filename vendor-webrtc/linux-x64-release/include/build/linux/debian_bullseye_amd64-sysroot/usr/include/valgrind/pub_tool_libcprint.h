
/*--------------------------------------------------------------------*/
/*--- Printing libc stuff.                    pub_tool_libcprint.h ---*/
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

#ifndef __PUB_TOOL_LIBCPRINT_H
#define __PUB_TOOL_LIBCPRINT_H

#include "pub_tool_basics.h"      // VG_ macro

/* ---------------------------------------------------------------------
   Formatting functions
   ------------------------------------------------------------------ */

/* The formatting functions supports a subset (and 2 extensions) of
   the 'printf' format.
   The extensions are:
     %pS : print a string (like %s) but escaping chars for XML safety.
     %ps : with --xml=no, synonym for %s, with --xml=yes, synonym of %pS.

   Note: these extensions do not cause the compiler to barf with PRINTF_CHECK
   as for the classical printf, %p requires a pointer, which must also
   be provided for the %ps and %pS extensions. The s/S following %p
   are understood by PRINTF_CHECK as characters to output.
*/

extern UInt VG_(sprintf)  ( HChar* buf, const HChar* format, ... )
                          PRINTF_CHECK(2, 3);

extern UInt VG_(vsprintf) ( HChar* buf, const HChar* format, va_list vargs )
                          PRINTF_CHECK(2, 0);

extern UInt VG_(snprintf) ( HChar* buf, Int size, 
                                       const HChar *format, ... )
                          PRINTF_CHECK(3, 4);

extern UInt VG_(vsnprintf)( HChar* buf, Int size, 
                                       const HChar *format, va_list vargs )
                          PRINTF_CHECK(3, 0);

/* ---------------------------------------------------------------------
   Output-printing functions
   ------------------------------------------------------------------ */

// Note that almost all output goes to the file descriptor given by the
// --log-fd/--log-file/--log-socket argument, which defaults to 2 (stderr).
// (Except that some text always goes to stdout/stderr at startup, and
// debugging messages always go to stderr.)  Hence no need for
// VG_(fprintf)().

/* No, really.  I _am_ that strange. */
#define OINK(nnn) VG_(message)(Vg_DebugMsg, "OINK %d\n",nnn)

/* Print a message with a prefix that depends on the VgMsgKind.
   Should be used for all user output. */

typedef
   enum {                 // Prefix
      Vg_FailMsg,         // "valgrind:"
      Vg_UserMsg,         // "==pid=="
      Vg_DebugMsg,        // "--pid--"
      Vg_ClientMsg        // "**pid**"
   }
   VgMsgKind;

// These print output that isn't prefixed with anything, and should be
// used in very few cases, such as printing usage messages.
extern UInt VG_(printf)   ( const HChar *format, ... )
                          PRINTF_CHECK(1, 2);
extern UInt VG_(vprintf)  ( const HChar *format, va_list vargs )
                          PRINTF_CHECK(1, 0);

extern UInt VG_(printf_xml)  ( const HChar *format, ... )
                             PRINTF_CHECK(1, 2);

extern UInt VG_(vprintf_xml) ( const HChar *format, va_list vargs )
                             PRINTF_CHECK(1, 0);

typedef struct _VgFile VgFile;

extern VgFile *VG_(fopen)    ( const HChar *name, Int flags, Int mode );
extern void    VG_(fclose)   ( VgFile *fp );
extern UInt    VG_(fprintf)  ( VgFile *fp, const HChar *format, ... )
                               PRINTF_CHECK(2, 3);
extern UInt    VG_(vfprintf) ( VgFile *fp, const HChar *format, va_list vargs )
                               PRINTF_CHECK(2, 0);

/* Do a printf-style operation on either the XML 
   or normal output channel
   or gdb output channel, depending on the setting of VG_(clo_xml)
   and the state of VG_(log_output_sink). */
extern UInt VG_(emit) ( const HChar* format, ... ) PRINTF_CHECK(1, 2);

/* Yet another, totally general, version of vprintf, which hands all
   output bytes to CHAR_SINK, passing it OPAQUE as the second arg. */
extern void VG_(vcbprintf)( void(*char_sink)(HChar, void* opaque),
                            void* opaque,
                            const HChar* format, va_list vargs );

extern UInt VG_(message)( VgMsgKind kind, const HChar* format, ... )
   PRINTF_CHECK(2, 3);

extern UInt VG_(vmessage)( VgMsgKind kind, const HChar* format, va_list vargs )
   PRINTF_CHECK(2, 0);

// Short-cuts for VG_(message)().

// This is used for messages printed due to start-up failures that occur
// before the preamble is printed, eg. due a bad executable.
extern UInt VG_(fmsg)( const HChar* format, ... ) PRINTF_CHECK(1, 2);

// This is used if an option was bad for some reason.  Note: don't use it just
// because an option was unrecognised -- return 'False' from
// VG_(tdict).tool_process_cmd_line_option) to indicate that -- use it if eg.
// an option was given an inappropriate argument.  This function prints an
// error message. It shuts down the entire system if the current parsing mode
// is cloE or cloP.
extern void VG_(fmsg_bad_option) ( const HChar* opt, const HChar* format, ... )
   PRINTF_CHECK(2, 3);

// This is used for messages that are interesting to the user:  info about
// their program (eg. preamble, tool error messages, postamble) or stuff they
// requested.
extern UInt VG_(umsg)( const HChar* format, ... ) PRINTF_CHECK(1, 2);

// This is used for debugging messages that are only of use to developers.
extern UInt VG_(dmsg)( const HChar* format, ... ) PRINTF_CHECK(1, 2);

/* Flush any output cached by previous calls to VG_(message) et al. */
extern void VG_(message_flush) ( void );

/* Return a textual representation of a SysRes value in a statically
   allocated buffer. The buffer will be overwritten with the next 
   invocation. */
extern const HChar *VG_(sr_as_string) ( SysRes sr );

#endif   // __PUB_TOOL_LIBCPRINT_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
