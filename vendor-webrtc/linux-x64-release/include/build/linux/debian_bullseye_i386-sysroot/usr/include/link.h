/* Data structure for communication from the run-time dynamic linker for
   loaded ELF shared objects.
   Copyright (C) 1995-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

#ifndef	_LINK_H
#define	_LINK_H	1

#include <features.h>
#include <elf.h>
#include <dlfcn.h>
#include <sys/types.h>

/* We use this macro to refer to ELF types independent of the native wordsize.
   `ElfW(TYPE)' is used in place of `Elf32_TYPE' or `Elf64_TYPE'.  */
#define ElfW(type)	_ElfW (Elf, __ELF_NATIVE_CLASS, type)
#define _ElfW(e,w,t)	_ElfW_1 (e, w, _##t)
#define _ElfW_1(e,w,t)	e##w##t

#include <bits/elfclass.h>		/* Defines __ELF_NATIVE_CLASS.  */
#include <bits/link.h>

/* Rendezvous structure used by the run-time dynamic linker to communicate
   details of shared object loading to the debugger.  If the executable's
   dynamic section has a DT_DEBUG element, the run-time linker sets that
   element's value to the address where this structure can be found.  */

struct r_debug
  {
    int r_version;		/* Version number for this protocol.  */

    struct link_map *r_map;	/* Head of the chain of loaded objects.  */

    /* This is the address of a function internal to the run-time linker,
       that will always be called when the linker begins to map in a
       library or unmap it, and again when the mapping change is complete.
       The debugger can set a breakpoint at this address if it wants to
       notice shared object mapping changes.  */
    ElfW(Addr) r_brk;
    enum
      {
	/* This state value describes the mapping change taking place when
	   the `r_brk' address is called.  */
	RT_CONSISTENT,		/* Mapping change is complete.  */
	RT_ADD,			/* Beginning to add a new object.  */
	RT_DELETE		/* Beginning to remove an object mapping.  */
      } r_state;

    ElfW(Addr) r_ldbase;	/* Base address the linker is loaded at.  */
  };

/* This is the instance of that structure used by the dynamic linker.  */
extern struct r_debug _r_debug;

/* This symbol refers to the "dynamic structure" in the `.dynamic' section
   of whatever module refers to `_DYNAMIC'.  So, to find its own
   `struct r_debug', a program could do:
     for (dyn = _DYNAMIC; dyn->d_tag != DT_NULL; ++dyn)
       if (dyn->d_tag == DT_DEBUG)
	 r_debug = (struct r_debug *) dyn->d_un.d_ptr;
   */
extern ElfW(Dyn) _DYNAMIC[];

/* Structure describing a loaded shared object.  The `l_next' and `l_prev'
   members form a chain of all the shared objects loaded at startup.

   These data structures exist in space used by the run-time dynamic linker;
   modifying them may have disastrous results.  */

struct link_map
  {
    /* These first few members are part of the protocol with the debugger.
       This is the same format used in SVR4.  */

    ElfW(Addr) l_addr;		/* Difference between the address in the ELF
				   file and the addresses in memory.  */
    char *l_name;		/* Absolute file name object was found in.  */
    ElfW(Dyn) *l_ld;		/* Dynamic section of the shared object.  */
    struct link_map *l_next, *l_prev; /* Chain of loaded objects.  */
  };

#ifdef __USE_GNU

/* Version numbers for la_version handshake interface.  */
#define LAV_CURRENT	1

/* Activity types signaled through la_activity.  */
enum
  {
    LA_ACT_CONSISTENT,		/* Link map consistent again.  */
    LA_ACT_ADD,			/* New object will be added.  */
    LA_ACT_DELETE		/* Objects will be removed.  */
  };

/* Values representing origin of name for dynamic loading.  */
enum
  {
    LA_SER_ORIG = 0x01,		/* Original name.  */
    LA_SER_LIBPATH = 0x02,	/* Directory from LD_LIBRARY_PATH.  */
    LA_SER_RUNPATH = 0x04,	/* Directory from RPATH/RUNPATH.  */
    LA_SER_CONFIG = 0x08,	/* Found through ldconfig.  */
    LA_SER_DEFAULT = 0x40,	/* Default directory.  */
    LA_SER_SECURE = 0x80	/* Unused.  */
  };

/* Values for la_objopen return value.  */
enum
  {
    LA_FLG_BINDTO = 0x01,	/* Audit symbols bound to this object.  */
    LA_FLG_BINDFROM = 0x02	/* Audit symbols bound from this object.  */
  };

/* Values for la_symbind flags parameter.  */
enum
  {
    LA_SYMB_NOPLTENTER = 0x01,	/* la_pltenter will not be called.  */
    LA_SYMB_NOPLTEXIT = 0x02,	/* la_pltexit will not be called.  */
    LA_SYMB_STRUCTCALL = 0x04,	/* Return value is a structure.  */
    LA_SYMB_DLSYM = 0x08,	/* Binding due to dlsym call.  */
    LA_SYMB_ALTVALUE = 0x10	/* Value has been changed by a previous
				   la_symbind call.  */
  };

struct dl_phdr_info
  {
    ElfW(Addr) dlpi_addr;
    const char *dlpi_name;
    const ElfW(Phdr) *dlpi_phdr;
    ElfW(Half) dlpi_phnum;

    /* Note: Following members were introduced after the first
       version of this structure was available.  Check the SIZE
       argument passed to the dl_iterate_phdr callback to determine
       whether or not each later member is available.  */

    /* Incremented when a new object may have been added.  */
    __extension__ unsigned long long int dlpi_adds;
    /* Incremented when an object may have been removed.  */
    __extension__ unsigned long long int dlpi_subs;

    /* If there is a PT_TLS segment, its module ID as used in
       TLS relocations, else zero.  */
    size_t dlpi_tls_modid;

    /* The address of the calling thread's instance of this module's
       PT_TLS segment, if it has one and it has been allocated
       in the calling thread, otherwise a null pointer.  */
    void *dlpi_tls_data;
  };

__BEGIN_DECLS

extern int dl_iterate_phdr (int (*__callback) (struct dl_phdr_info *,
					       size_t, void *),
			    void *__data);


/* Prototypes for the ld.so auditing interfaces.  These are not
   defined anywhere in ld.so but instead have to be provided by the
   auditing DSO.  */
extern unsigned int la_version (unsigned int __version);
extern void la_activity (uintptr_t *__cookie, unsigned int __flag);
extern char *la_objsearch (const char *__name, uintptr_t *__cookie,
			   unsigned int __flag);
extern unsigned int la_objopen (struct link_map *__map, Lmid_t __lmid,
				uintptr_t *__cookie);
extern void la_preinit (uintptr_t *__cookie);
extern uintptr_t la_symbind32 (Elf32_Sym *__sym, unsigned int __ndx,
			       uintptr_t *__refcook, uintptr_t *__defcook,
			       unsigned int *__flags, const char *__symname);
extern uintptr_t la_symbind64 (Elf64_Sym *__sym, unsigned int __ndx,
			       uintptr_t *__refcook, uintptr_t *__defcook,
			       unsigned int *__flags, const char *__symname);
extern unsigned int la_objclose (uintptr_t *__cookie);

__END_DECLS

#endif

#endif /* link.h */
