/* Accumulation of various pieces of knowledge about ELF.
   Copyright (C) 2000-2012, 2014, 2016 Red Hat, Inc.
   This file is part of elfutils.
   Written by Ulrich Drepper <drepper@redhat.com>, 2000.

   This file is free software; you can redistribute it and/or modify
   it under the terms of either

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at
       your option) any later version

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at
       your option) any later version

   or both in parallel, as here.

   elfutils is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see <http://www.gnu.org/licenses/>.  */

#ifndef _ELF_KNOWLEDGE_H
#define _ELF_KNOWLEDGE_H	1

#include <stdbool.h>


/* Test whether a section can be stripped or not.  */
#define SECTION_STRIP_P(shdr, name, remove_comment) \
  /* Sections which are allocated are not removed.  */			      \
  (((shdr)->sh_flags & SHF_ALLOC) == 0					      \
   /* We never remove .note sections.  */				      \
   && (shdr)->sh_type != SHT_NOTE					      \
   && (((shdr)->sh_type) != SHT_PROGBITS				      \
       /* Never remove .gnu.warning.* sections.  */			      \
       || (name != NULL							      \
	   && strncmp (name, ".gnu.warning.", sizeof ".gnu.warning." - 1) != 0\
	   /* We remove .comment sections only if explicitly told to do so. */\
	   && (remove_comment						      \
	       || strcmp (name, ".comment") != 0))))


/* Test whether `sh_info' field in section header contains a section
   index.  There are two kinds of sections doing this:

   - the sections containing relocation information reference in this
     field the section to which the relocations apply;

   - section with the SHF_INFO_LINK flag set to signal that `sh_info'
     references a section.  This allows correct handling of unknown
     sections.  */
#define SH_INFO_LINK_P(Shdr) \
  ((Shdr)->sh_type == SHT_REL || (Shdr)->sh_type == SHT_RELA		      \
   || ((Shdr)->sh_flags & SHF_INFO_LINK) != 0)


/* Size of an entry in the hash table.  The ELF specification says all
   entries are regardless of platform 32-bits in size.  Early 64-bit
   ports (namely Alpha for Linux) got this wrong.  The wording was not
   clear.

   Several years later the ABI for the 64-bit S390s was developed.
   Many things were copied from the IA-64 ABI (which uses the correct
   32-bit entry size) but it does get the SHT_HASH entry size wrong by
   using a 64-bit entry size.  So now we need this macro to special
   case both the alpha and s390x ABIs.  */
#define SH_ENTSIZE_HASH(Ehdr) \
  ((Ehdr)->e_machine == EM_ALPHA					      \
   || ((Ehdr)->e_machine == EM_S390					      \
       && (Ehdr)->e_ident[EI_CLASS] == ELFCLASS64) ? 8 : 4)

/* GNU Annobin notes are not fully standardized and abuses the owner name.  */

#define ELF_NOTE_GNU_BUILD_ATTRIBUTE_PREFIX "GA"

#define NT_GNU_BUILD_ATTRIBUTE_OPEN 0x100
#define NT_GNU_BUILD_ATTRIBUTE_FUNC 0x101

#define GNU_BUILD_ATTRIBUTE_TYPE_NUMERIC	'*'
#define GNU_BUILD_ATTRIBUTE_TYPE_STRING		'$'
#define GNU_BUILD_ATTRIBUTE_TYPE_BOOL_TRUE	'+'
#define GNU_BUILD_ATTRIBUTE_TYPE_BOOL_FALSE	'!'

#define GNU_BUILD_ATTRIBUTE_VERSION	1
#define GNU_BUILD_ATTRIBUTE_STACK_PROT	2
#define GNU_BUILD_ATTRIBUTE_RELRO	3
#define GNU_BUILD_ATTRIBUTE_STACK_SIZE	4
#define GNU_BUILD_ATTRIBUTE_TOOL	5
#define GNU_BUILD_ATTRIBUTE_ABI		6
#define GNU_BUILD_ATTRIBUTE_PIC		7
#define GNU_BUILD_ATTRIBUTE_SHORT_ENUM	8

#endif	/* elf-knowledge.h */
