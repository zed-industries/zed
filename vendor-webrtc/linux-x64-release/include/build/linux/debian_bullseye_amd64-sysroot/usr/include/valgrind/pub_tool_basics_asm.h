
/*--------------------------------------------------------------------*/
/*--- Header imported directly by every tool asm file.             ---*/
/*---                                        pub_tool_basics_asm.h ---*/
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

#ifndef __PUB_TOOL_BASICS_ASM_H
#define __PUB_TOOL_BASICS_ASM_H

// See pub_tool_basics.h for the purpose of these macros.
//
// Note that although the macros here (which are used in asm files) have the
// same name as those in pub_tool_basics.h (which are used in C files), they
// have different definitions.  Actually, on Linux the definitions are the
// same, but on Darwin they are different.  The reason is that C names on
// Darwin always get a '_' prepended to them by the compiler.  But in order to
// refer to them from asm code, we have to add the '_' ourselves.  Having two
// versions of these macros makes that difference transparent, so we can use
// VG_/ML_ in both asm and C files.
//
// Note also that the exact prefixes used have to match those used in
// pub_tool_basics.h.

#define VGAPPEND(str1,str2) str1##str2
 
#if defined(VGO_linux) || defined(VGO_solaris)
#  define VG_(str)    VGAPPEND( vgPlain_,          str)
#  define ML_(str)    VGAPPEND( vgModuleLocal_,    str)
#elif defined(VGO_darwin)
#  define VG_(str)    VGAPPEND(_vgPlain_,          str)
#  define ML_(str)    VGAPPEND(_vgModuleLocal_,    str)
#else
#  error Unknown OS
#endif

/* Let the linker know we don't need an executable stack.
   The call to MARK_STACK_NO_EXEC should be put unconditionally
   at the end of all asm source files.
*/
#if defined(VGO_linux)
#  if defined(VGA_arm)
#    define MARK_STACK_NO_EXEC .section .note.GNU-stack,"",%progbits
#  else
#    define MARK_STACK_NO_EXEC .section .note.GNU-stack,"",@progbits
#  endif
#else
#  define MARK_STACK_NO_EXEC
#endif

#endif /* __PUB_TOOL_BASICS_ASM_H */

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
