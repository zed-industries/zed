
/*--------------------------------------------------------------------*/
/*--- A minimal setjmp/longjmp facility.     pub_tool_libcsetjmp.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2010-2017 Mozilla Foundation

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

/* Contributed by Julian Seward <jseward@acm.org> */

#ifndef __PUB_TOOL_LIBCSETJMP_H
#define __PUB_TOOL_LIBCSETJMP_H

#include "pub_tool_basics.h"   // UWord

//--------------------------------------------------------------------
// PURPOSE: Provides a minimal setjmp/longjmp facility, that saves/
// restores integer registers, but not necessarily anything more.
//--------------------------------------------------------------------


/* This provides an extremely minimal setjmp/longjmp facility, in
   which only the host's integer registers are saved/restored.  Or at
   least, that is the minimal guaranteed functionality.

   Until Apr 2011 we used __builtin_setjmp and __builtin_longjmp, but
   it appears that that is not always correctly implemented.  See
   https://bugs.kde.org/show_bug.cgi?id=259977.  So this module wraps
   those functions up and facilitates replacing them with our own
   implementations where necessary.
*/

/* --- !!! --- EXTERNAL HEADERS start --- !!! --- */
#include <setjmp.h>
/* --- !!! --- EXTERNAL HEADERS end --- !!! --- */


/* Don't use jmp_buf, __builtin_setjmp or __builtin_longjmp directly.
   They don't always work reliably.  Instead use these macros, which
   provide the opportunity to supply alternative implementations as
   necessary.

   Note that the abstraction is done with macros (ick) rather than
   functions and typedefs, since wrapping __builtin_setjmp up in a
   second function (eg, VG_(minimal_setjmp)) doesn't seem to work for
   whatever reason -- returns via a VG_(minimal_longjmp) go wrong.

   VG_MINIMAL_SETJMP stores the current integer register state in the
   supplied argument, and returns zero.  VG_MINIMAL_LONGJMP resumes
   with the previously saved state, and returns a nonzero, word-sized
   value.  The caller must test all bits of the value in order to make
   a zero/non-zero determination.
*/

#if defined(VGP_ppc32_linux)

#define VG_MINIMAL_JMP_BUF(_name)        UInt _name [32+1+1]
__attribute__((returns_twice))
UWord VG_MINIMAL_SETJMP(VG_MINIMAL_JMP_BUF(_env));
__attribute__((noreturn))
void  VG_MINIMAL_LONGJMP(VG_MINIMAL_JMP_BUF(_env));


#elif defined(VGP_ppc64be_linux) || defined(VGP_ppc64le_linux)

#define VG_MINIMAL_JMP_BUF(_name)        ULong _name [32+1+1]
__attribute__((returns_twice))
UWord VG_MINIMAL_SETJMP(VG_MINIMAL_JMP_BUF(_env));
__attribute__((noreturn))
void  VG_MINIMAL_LONGJMP(VG_MINIMAL_JMP_BUF(_env));


#elif defined(VGP_amd64_linux) || defined(VGP_amd64_darwin) || \
      defined(VGP_amd64_solaris)

#define VG_MINIMAL_JMP_BUF(_name)        ULong _name [16+1]
__attribute__((returns_twice))
UWord VG_MINIMAL_SETJMP(VG_MINIMAL_JMP_BUF(_env));
__attribute__((noreturn))
void  VG_MINIMAL_LONGJMP(VG_MINIMAL_JMP_BUF(_env));


#elif defined(VGP_x86_linux) || defined(VGP_x86_darwin) || \
      defined(VGP_x86_solaris)

#define VG_MINIMAL_JMP_BUF(_name)        UInt _name [8+1]
__attribute__((returns_twice))
__attribute__((regparm(1))) // this is critical; don't delete
UWord VG_MINIMAL_SETJMP(VG_MINIMAL_JMP_BUF(_env));
__attribute__((noreturn))
__attribute__((regparm(1))) // ditto
void  VG_MINIMAL_LONGJMP(VG_MINIMAL_JMP_BUF(_env));

#elif defined(VGP_mips32_linux) || defined(VGP_nanomips_linux)

#define VG_MINIMAL_JMP_BUF(_name)        ULong _name [104 / sizeof(ULong)]
__attribute__((returns_twice))
UWord VG_MINIMAL_SETJMP(VG_MINIMAL_JMP_BUF(_env));
__attribute__((noreturn))
void  VG_MINIMAL_LONGJMP(VG_MINIMAL_JMP_BUF(_env));

#elif defined(VGP_mips64_linux)

#define VG_MINIMAL_JMP_BUF(_name)        ULong _name [168 / sizeof(ULong)]
__attribute__((returns_twice))
UWord VG_MINIMAL_SETJMP(VG_MINIMAL_JMP_BUF(_env));
__attribute__((noreturn))
void  VG_MINIMAL_LONGJMP(VG_MINIMAL_JMP_BUF(_env));

#else

/* The default implementation. */
#define VG_MINIMAL_JMP_BUF(_name) jmp_buf _name
#define VG_MINIMAL_SETJMP(_env)   ((UWord)(__builtin_setjmp((_env))))
#define VG_MINIMAL_LONGJMP(_env)  __builtin_longjmp((_env),1)

#endif

#endif   // __PUB_TOOL_LIBCSETJMP_H

/*--------------------------------------------------------------------*/
/*--- end                                    pub_tool_libcsetjmp.h ---*/
/*--------------------------------------------------------------------*/
