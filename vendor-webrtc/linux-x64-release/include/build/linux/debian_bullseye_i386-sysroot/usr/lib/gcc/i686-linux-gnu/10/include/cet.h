/* ELF program property for Intel CET.
   Copyright (C) 2017-2020 Free Software Foundation, Inc.

   This file is free software; you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by the
   Free Software Foundation; either version 3, or (at your option) any
   later version.

   This file is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   Under Section 7 of GPL version 3, you are granted additional
   permissions described in the GCC Runtime Library Exception, version
   3.1, as published by the Free Software Foundation.

   You should have received a copy of the GNU General Public License and
   a copy of the GCC Runtime Library Exception along with this program;
   see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
   <http://www.gnu.org/licenses/>.
 */

/* Add x86 feature with IBT and/or SHSTK bits to ELF program property
   if they are enabled.  Otherwise, contents in this header file are
   unused.  Define _CET_ENDBR for assembly codes.  _CET_ENDBR should be
   placed unconditionally at the entrance of a function whose address
   may be taken.  */

#ifndef _CET_H_INCLUDED
#define _CET_H_INCLUDED

#ifdef __ASSEMBLER__

# if defined __CET__ && (__CET__ & 1) != 0
#  ifdef __x86_64__
#   define _CET_ENDBR endbr64
#  else
#   define _CET_ENDBR endbr32
#  endif
# else
#  define _CET_ENDBR
# endif

# ifdef __ELF__
#  ifdef __CET__
#   if (__CET__ & 1) != 0
/* GNU_PROPERTY_X86_FEATURE_1_IBT.  */
#    define __PROPERTY_IBT 0x1
#   else
#    define __PROPERTY_IBT 0x0
#   endif

#   if (__CET__ & 2) != 0
/* GNU_PROPERTY_X86_FEATURE_1_SHSTK.  */
#    define __PROPERTY_SHSTK 0x2
#   else
#    define __PROPERTY_SHSTK 0x0
#   endif

#   define __PROPERTY_BITS (__PROPERTY_IBT | __PROPERTY_SHSTK)

#   ifdef __LP64__
#    define __PROPERTY_ALIGN 3
#   else
#    define __PROPERTY_ALIGN 2
#   endif

	.pushsection ".note.gnu.property", "a"
	.p2align __PROPERTY_ALIGN
	.long 1f - 0f		/* name length.  */
	.long 4f - 1f		/* data length.  */
	/* NT_GNU_PROPERTY_TYPE_0.   */
	.long 5			/* note type.  */
0:
	.asciz "GNU"		/* vendor name.  */
1:
	.p2align __PROPERTY_ALIGN
	/* GNU_PROPERTY_X86_FEATURE_1_AND.  */
	.long 0xc0000002	/* pr_type.  */
	.long 3f - 2f		/* pr_datasz.  */
2:
	/* GNU_PROPERTY_X86_FEATURE_1_XXX.  */
	.long __PROPERTY_BITS
3:
	.p2align __PROPERTY_ALIGN
4:
	.popsection
#  endif /* __CET__ */
# endif /* __ELF__ */
#endif /* __ASSEMBLER__ */

#endif /* _CET_H_INCLUDED */
