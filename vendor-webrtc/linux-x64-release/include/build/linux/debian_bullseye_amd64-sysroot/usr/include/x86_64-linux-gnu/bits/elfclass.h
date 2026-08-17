/* This file specifies the native word size of the machine, which indicates
   the ELF file class used for executables and shared objects on this
   machine.  */

#ifndef _LINK_H
# error "Never use <bits/elfclass.h> directly; include <link.h> instead."
#endif

#include <bits/wordsize.h>

#define __ELF_NATIVE_CLASS __WORDSIZE

/* The entries in the .hash table always have a size of 32 bits.  */
typedef uint32_t Elf_Symndx;
