/* elf.c -- Get debug data from a Mach-O file for backtraces.
   Copyright (C) 2020-2021 Free Software Foundation, Inc.
   Written by Ian Lance Taylor, Google.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:

    (1) Redistributions of source code must retain the above copyright
    notice, this list of conditions and the following disclaimer.

    (2) Redistributions in binary form must reproduce the above copyright
    notice, this list of conditions and the following disclaimer in
    the documentation and/or other materials provided with the
    distribution.

    (3) The name of the author may not be used to
    endorse or promote products derived from this software without
    specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT,
INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.  */

#include "config.h"

#include <sys/types.h>
#include <dirent.h>
#include <stdlib.h>
#include <string.h>

#ifdef HAVE_MACH_O_DYLD_H
#include <mach-o/dyld.h>
#endif

#include "backtrace.hpp"
#include "internal.hpp"

namespace tracy
{

/* Mach-O file header for a 32-bit executable.  */

struct macho_header_32
{
  uint32_t magic;	/* Magic number (MACH_O_MAGIC_32) */
  uint32_t cputype;	/* CPU type */
  uint32_t cpusubtype;	/* CPU subtype */
  uint32_t filetype;	/* Type of file (object, executable) */
  uint32_t ncmds;	/* Number of load commands */
  uint32_t sizeofcmds;	/* Total size of load commands */
  uint32_t flags;	/* Flags for special features */
};

/* Mach-O file header for a 64-bit executable.  */

struct macho_header_64
{
  uint32_t magic;	/* Magic number (MACH_O_MAGIC_64) */
  uint32_t cputype;	/* CPU type */
  uint32_t cpusubtype;	/* CPU subtype */
  uint32_t filetype;	/* Type of file (object, executable) */
  uint32_t ncmds;	/* Number of load commands */
  uint32_t sizeofcmds;	/* Total size of load commands */
  uint32_t flags;	/* Flags for special features */
  uint32_t reserved;	/* Reserved */
};

/* Mach-O file header for a fat executable.  */

struct macho_header_fat
{
  uint32_t magic;	/* Magic number (MACH_O_MH_(MAGIC|CIGAM)_FAT(_64)?) */
  uint32_t nfat_arch;   /* Number of components */
};

/* Values for the header magic field.  */

#define MACH_O_MH_MAGIC_32	0xfeedface
#define MACH_O_MH_MAGIC_64	0xfeedfacf
#define MACH_O_MH_MAGIC_FAT	0xcafebabe
#define MACH_O_MH_CIGAM_FAT	0xbebafeca
#define MACH_O_MH_MAGIC_FAT_64	0xcafebabf
#define MACH_O_MH_CIGAM_FAT_64	0xbfbafeca

/* Value for the header filetype field.  */

#define MACH_O_MH_EXECUTE	0x02
#define MACH_O_MH_DYLIB		0x06
#define MACH_O_MH_DSYM		0x0a

/* A component of a fat file.  A fat file starts with a
   macho_header_fat followed by nfat_arch instances of this
   struct.  */

struct macho_fat_arch
{
  uint32_t cputype;	/* CPU type */
  uint32_t cpusubtype;	/* CPU subtype */
  uint32_t offset;	/* File offset of this entry */
  uint32_t size;	/* Size of this entry */
  uint32_t align;	/* Alignment of this entry */
};

/* A component of a 64-bit fat file.  This is used if the magic field
   is MAGIC_FAT_64.  This is only used when some file size or file
   offset is too large to represent in the 32-bit format.  */

struct macho_fat_arch_64
{
  uint32_t cputype;	/* CPU type */
  uint32_t cpusubtype;	/* CPU subtype */
  uint64_t offset;	/* File offset of this entry */
  uint64_t size;	/* Size of this entry */
  uint32_t align;	/* Alignment of this entry */
  uint32_t reserved;	/* Reserved */
};

/* Values for the fat_arch cputype field (and the header cputype
   field).  */

#define MACH_O_CPU_ARCH_ABI64 0x01000000

#define MACH_O_CPU_TYPE_X86 7
#define MACH_O_CPU_TYPE_ARM 12
#define MACH_O_CPU_TYPE_PPC 18

#define MACH_O_CPU_TYPE_X86_64 (MACH_O_CPU_TYPE_X86 | MACH_O_CPU_ARCH_ABI64)
#define MACH_O_CPU_TYPE_ARM64  (MACH_O_CPU_TYPE_ARM | MACH_O_CPU_ARCH_ABI64)
#define MACH_O_CPU_TYPE_PPC64  (MACH_O_CPU_TYPE_PPC | MACH_O_CPU_ARCH_ABI64)

/* The header of a load command.  */

struct macho_load_command
{
  uint32_t cmd;		/* The type of load command */
  uint32_t cmdsize;	/* Size in bytes of the entire command */
};

/* Values for the load_command cmd field.  */

#define MACH_O_LC_SEGMENT	0x01
#define MACH_O_LC_SYMTAB	0x02
#define MACH_O_LC_SEGMENT_64	0x19
#define MACH_O_LC_UUID		0x1b

/* The length of a section of segment name.  */

#define MACH_O_NAMELEN (16)

/* LC_SEGMENT load command.  */

struct macho_segment_command
{
  uint32_t cmd;			/* The type of load command (LC_SEGMENT) */
  uint32_t cmdsize;		/* Size in bytes of the entire command */
  char segname[MACH_O_NAMELEN];	/* Segment name */
  uint32_t vmaddr;		/* Virtual memory address */
  uint32_t vmsize;		/* Virtual memory size */
  uint32_t fileoff;		/* Offset of data to be mapped */
  uint32_t filesize;		/* Size of data in file */
  uint32_t maxprot;		/* Maximum permitted virtual protection */
  uint32_t initprot;		/* Initial virtual memory protection */
  uint32_t nsects;		/* Number of sections in this segment */
  uint32_t flags;		/* Flags */
};

/* LC_SEGMENT_64 load command.  */

struct macho_segment_64_command
{
  uint32_t cmd;			/* The type of load command (LC_SEGMENT) */
  uint32_t cmdsize;		/* Size in bytes of the entire command */
  char segname[MACH_O_NAMELEN];	/* Segment name */
  uint64_t vmaddr;		/* Virtual memory address */
  uint64_t vmsize;		/* Virtual memory size */
  uint64_t fileoff;		/* Offset of data to be mapped */
  uint64_t filesize;		/* Size of data in file */
  uint32_t maxprot;		/* Maximum permitted virtual protection */
  uint32_t initprot;		/* Initial virtual memory protection */
  uint32_t nsects;		/* Number of sections in this segment */
  uint32_t flags;		/* Flags */
};

/* LC_SYMTAB load command.  */

struct macho_symtab_command
{
  uint32_t cmd;		/* The type of load command (LC_SEGMENT) */
  uint32_t cmdsize;	/* Size in bytes of the entire command */
  uint32_t symoff;	/* File offset of symbol table */
  uint32_t nsyms;	/* Number of symbols */
  uint32_t stroff;	/* File offset of string table */
  uint32_t strsize;	/* String table size */
};

/* The length of a Mach-O uuid.  */

#define MACH_O_UUID_LEN (16)

/* LC_UUID load command.  */

struct macho_uuid_command
{
  uint32_t cmd;				/* Type of load command (LC_UUID) */
  uint32_t cmdsize;			/* Size in bytes of command */
  unsigned char uuid[MACH_O_UUID_LEN];	/* UUID */
};

/* 32-bit section header within a LC_SEGMENT segment.  */

struct macho_section
{
  char sectname[MACH_O_NAMELEN];	/* Section name */
  char segment[MACH_O_NAMELEN];		/* Segment of this section */
  uint32_t addr;			/* Address in memory */
  uint32_t size;			/* Section size */
  uint32_t offset;			/* File offset */
  uint32_t align;			/* Log2 of section alignment */
  uint32_t reloff;			/* File offset of relocations */
  uint32_t nreloc;			/* Number of relocs for this section */
  uint32_t flags;			/* Flags */
  uint32_t reserved1;
  uint32_t reserved2;
};

/* 64-bit section header within a LC_SEGMENT_64 segment.   */

struct macho_section_64
{
  char sectname[MACH_O_NAMELEN];	/* Section name */
  char segment[MACH_O_NAMELEN];		/* Segment of this section */
  uint64_t addr;			/* Address in memory */
  uint64_t size;			/* Section size */
  uint32_t offset;			/* File offset */
  uint32_t align;			/* Log2 of section alignment */
  uint32_t reloff;			/* File offset of section relocations */
  uint32_t nreloc;			/* Number of relocs for this section */
  uint32_t flags;			/* Flags */
  uint32_t reserved1;
  uint32_t reserved2;
  uint32_t reserved3;
};

/* 32-bit symbol data.  */

struct macho_nlist
{
  uint32_t n_strx;	/* Index of name in string table */
  uint8_t n_type;	/* Type flag */
  uint8_t n_sect;	/* Section number */
  uint16_t n_desc;	/* Stabs description field */
  uint32_t n_value;	/* Value */
};

/* 64-bit symbol data.  */

struct macho_nlist_64
{
  uint32_t n_strx;	/* Index of name in string table */
  uint8_t n_type;	/* Type flag */
  uint8_t n_sect;	/* Section number */
  uint16_t n_desc;	/* Stabs description field */
  uint64_t n_value;	/* Value */
};

/* Value found in nlist n_type field.  */

#define MACH_O_N_STAB	0xe0	/* Stabs debugging symbol */
#define MACH_O_N_TYPE	0x0e	/* Mask for type bits */

/* Values found after masking with MACH_O_N_TYPE.  */
#define MACH_O_N_UNDF	0x00	/* Undefined symbol */
#define MACH_O_N_ABS	0x02	/* Absolute symbol */
#define MACH_O_N_SECT	0x0e	/* Defined in section from n_sect field */


/* Information we keep for a Mach-O symbol.  */

struct macho_symbol
{
  const char *name;	/* Symbol name */
  uintptr_t address;	/* Symbol address */
};

/* Information to pass to macho_syminfo.  */

struct macho_syminfo_data
{
  struct macho_syminfo_data *next;	/* Next module */
  struct macho_symbol *symbols;		/* Symbols sorted by address */
  size_t count;				/* Number of symbols */
};

/* Names of sections, indexed by enum dwarf_section in internal.h.  */

static const char * const dwarf_section_names[DEBUG_MAX] =
{
  "__debug_info",
  "__debug_line",
  "__debug_abbrev",
  "__debug_ranges",
  "__debug_str",
  "__debug_addr",
  "__debug_str_offs",
  "__debug_line_str",
  "__debug_rnglists"
};

/* Forward declaration.  */

static int macho_add (struct backtrace_state *, const char *, int, off_t,
		      const unsigned char *, struct libbacktrace_base_address,
		      int, backtrace_error_callback, void *, fileline *,
		      int *);

/* A dummy callback function used when we can't find any debug info.  */

static int
macho_nodebug (struct backtrace_state *state ATTRIBUTE_UNUSED,
	       uintptr_t pc ATTRIBUTE_UNUSED,
	       backtrace_full_callback callback ATTRIBUTE_UNUSED,
	       backtrace_error_callback error_callback, void *data)
{
  error_callback (data, "no debug info in Mach-O executable", -1);
  return 0;
}

/* A dummy callback function used when we can't find a symbol
   table.  */

static void
macho_nosyms (struct backtrace_state *state ATTRIBUTE_UNUSED,
	      uintptr_t addr ATTRIBUTE_UNUSED,
	      backtrace_syminfo_callback callback ATTRIBUTE_UNUSED,
	      backtrace_error_callback error_callback, void *data)
{
  error_callback (data, "no symbol table in Mach-O executable", -1);
}

/* Add a single DWARF section to DWARF_SECTIONS, if we need the
   section.  Returns 1 on success, 0 on failure.  */

static int
macho_add_dwarf_section (struct backtrace_state *state, int descriptor,
			 const char *sectname, uint32_t offset, uint64_t size,
			 backtrace_error_callback error_callback, void *data,
			 struct dwarf_sections *dwarf_sections)
{
  int i;

  for (i = 0; i < (int) DEBUG_MAX; ++i)
    {
      if (dwarf_section_names[i][0] != '\0'
	  && strncmp (sectname, dwarf_section_names[i], MACH_O_NAMELEN) == 0)
	{
	  struct backtrace_view section_view;

	  /* FIXME: Perhaps it would be better to try to use a single
	     view to read all the DWARF data, as we try to do for
	     ELF.  */

	  if (!backtrace_get_view (state, descriptor, offset, size,
				   error_callback, data, &section_view))
	    return 0;
	  dwarf_sections->data[i] = (const unsigned char *) section_view.data;
	  dwarf_sections->size[i] = size;
	  break;
	}
    }
  return 1;
}

/* Collect DWARF sections from a DWARF segment.  Returns 1 on success,
   0 on failure.  */

static int
macho_add_dwarf_segment (struct backtrace_state *state, int descriptor,
			 off_t offset, unsigned int cmd, const char *psecs,
			 size_t sizesecs, unsigned int nsects,
			 backtrace_error_callback error_callback, void *data,
			 struct dwarf_sections *dwarf_sections)
{
  size_t sec_header_size;
  size_t secoffset;
  unsigned int i;

  switch (cmd)
    {
    case MACH_O_LC_SEGMENT:
      sec_header_size = sizeof (struct macho_section);
      break;
    case MACH_O_LC_SEGMENT_64:
      sec_header_size = sizeof (struct macho_section_64);
      break;
    default:
      abort ();
    }

  secoffset = 0;
  for (i = 0; i < nsects; ++i)
    {
      if (secoffset + sec_header_size > sizesecs)
	{
	  error_callback (data, "section overflow withing segment", 0);
	  return 0;
	}

      switch (cmd)
	{
	case MACH_O_LC_SEGMENT:
	  {
	    struct macho_section section;

	    memcpy (&section, psecs + secoffset, sizeof section);
	    macho_add_dwarf_section (state, descriptor, section.sectname,
				     offset + section.offset, section.size,
				     error_callback, data, dwarf_sections);
	  }
	  break;

	case MACH_O_LC_SEGMENT_64:
	  {
	    struct macho_section_64 section;

	    memcpy (&section, psecs + secoffset, sizeof section);
	    macho_add_dwarf_section (state, descriptor, section.sectname,
				     offset + section.offset, section.size,
				     error_callback, data, dwarf_sections);
	  }
	  break;

	default:
	  abort ();
	}

      secoffset += sec_header_size;
    }

  return 1;
}

/* Compare struct macho_symbol for qsort.  */

static int
macho_symbol_compare (const void *v1, const void *v2)
{
  const struct macho_symbol *m1 = (const struct macho_symbol *) v1;
  const struct macho_symbol *m2 = (const struct macho_symbol *) v2;

  if (m1->address < m2->address)
    return -1;
  else if (m1->address > m2->address)
    return 1;
  else
    return 0;
}

/* Compare an address against a macho_symbol for bsearch.  We allocate
   one extra entry in the array so that this can safely look at the
   next entry.  */

static int
macho_symbol_search (const void *vkey, const void *ventry)
{
  const uintptr_t *key = (const uintptr_t *) vkey;
  const struct macho_symbol *entry = (const struct macho_symbol *) ventry;
  uintptr_t addr;

  addr = *key;
  if (addr < entry->address)
    return -1;
  else if (entry->name[0] == '\0'
	   && entry->address == ~(uintptr_t) 0)
    return -1;
  else if ((entry + 1)->name[0] == '\0'
	   && (entry + 1)->address == ~(uintptr_t) 0)
    return -1;
  else if (addr >= (entry + 1)->address)
    return 1;
  else
    return 0;
}

/* Return whether the symbol type field indicates a symbol table entry
   that we care about: a function or data symbol.  */

static int
macho_defined_symbol (uint8_t type)
{
  if ((type & MACH_O_N_STAB) != 0)
    return 0;
  switch (type & MACH_O_N_TYPE)
    {
    case MACH_O_N_UNDF:
      return 0;
    case MACH_O_N_ABS:
      return 1;
    case MACH_O_N_SECT:
      return 1;
    default:
      return 0;
    }
}

/* Add symbol table information for a Mach-O file.  */

static int
macho_add_symtab (struct backtrace_state *state, int descriptor,
		  struct libbacktrace_base_address base_address, int is_64,
		  off_t symoff, unsigned int nsyms, off_t stroff,
		  unsigned int strsize,
		  backtrace_error_callback error_callback, void *data)
{
  size_t symsize;
  struct backtrace_view sym_view;
  int sym_view_valid;
  struct backtrace_view str_view;
  int str_view_valid;
  size_t ndefs;
  size_t symtaboff;
  unsigned int i;
  size_t macho_symbol_size;
  struct macho_symbol *macho_symbols;
  unsigned int j;
  struct macho_syminfo_data *sdata;

  sym_view_valid = 0;
  str_view_valid = 0;
  macho_symbol_size = 0;
  macho_symbols = NULL;

  if (is_64)
    symsize = sizeof (struct macho_nlist_64);
  else
    symsize = sizeof (struct macho_nlist);

  if (!backtrace_get_view (state, descriptor, symoff, nsyms * symsize,
			   error_callback, data, &sym_view))
    goto fail;
  sym_view_valid = 1;

  if (!backtrace_get_view (state, descriptor, stroff, strsize,
			   error_callback, data, &str_view))
    return 0;
  str_view_valid = 1;

  ndefs = 0;
  symtaboff = 0;
  for (i = 0; i < nsyms; ++i, symtaboff += symsize)
    {
      if (is_64)
	{
	  struct macho_nlist_64 nlist;

	  memcpy (&nlist, (const char *) sym_view.data + symtaboff,
		  sizeof nlist);
	  if (macho_defined_symbol (nlist.n_type))
	    ++ndefs;
	}
      else
	{
	  struct macho_nlist nlist;

	  memcpy (&nlist, (const char *) sym_view.data + symtaboff,
		  sizeof nlist);
	  if (macho_defined_symbol (nlist.n_type))
	    ++ndefs;
	}
    }

  /* Add 1 to ndefs to make room for a sentinel.  */
  macho_symbol_size = (ndefs + 1) * sizeof (struct macho_symbol);
  macho_symbols = ((struct macho_symbol *)
		   backtrace_alloc (state, macho_symbol_size, error_callback,
				    data));
  if (macho_symbols == NULL)
    goto fail;

  j = 0;
  symtaboff = 0;
  for (i = 0; i < nsyms; ++i, symtaboff += symsize)
    {
      uint32_t strx;
      uint64_t value;
      const char *name;

      strx = 0;
      value = 0;
      if (is_64)
	{
	  struct macho_nlist_64 nlist;

	  memcpy (&nlist, (const char *) sym_view.data + symtaboff,
		  sizeof nlist);
	  if (!macho_defined_symbol (nlist.n_type))
	    continue;

	  strx = nlist.n_strx;
	  value = nlist.n_value;
	}
      else
	{
	  struct macho_nlist nlist;

	  memcpy (&nlist, (const char *) sym_view.data + symtaboff,
		  sizeof nlist);
	  if (!macho_defined_symbol (nlist.n_type))
	    continue;

	  strx = nlist.n_strx;
	  value = nlist.n_value;
	}

      if (strx >= strsize)
	{
	  error_callback (data, "symbol string index out of range", 0);
	  goto fail;
	}

      name = (const char *) str_view.data + strx;
      if (name[0] == '_')
	++name;
      macho_symbols[j].name = name;
      macho_symbols[j].address = libbacktrace_add_base (value, base_address);
      ++j;
    }

  sdata = ((struct macho_syminfo_data *)
	   backtrace_alloc (state, sizeof *sdata, error_callback, data));
  if (sdata == NULL)
    goto fail;

  /* We need to keep the string table since it holds the names, but we
     can release the symbol table.  */

  backtrace_release_view (state, &sym_view, error_callback, data);
  sym_view_valid = 0;
  str_view_valid = 0;

  /* Add a trailing sentinel symbol.  */
  macho_symbols[j].name = "";
  macho_symbols[j].address = ~(uintptr_t) 0;

  backtrace_qsort (macho_symbols, ndefs + 1, sizeof (struct macho_symbol),
		   macho_symbol_compare);

  sdata->next = NULL;
  sdata->symbols = macho_symbols;
  sdata->count = ndefs;

  if (!state->threaded)
    {
      struct macho_syminfo_data **pp;

      for (pp = (struct macho_syminfo_data **) (void *) &state->syminfo_data;
	   *pp != NULL;
	   pp = &(*pp)->next)
	;
      *pp = sdata;
    }
  else
    {
      while (1)
	{
	  struct macho_syminfo_data **pp;

	  pp = (struct macho_syminfo_data **) (void *) &state->syminfo_data;

	  while (1)
	    {
	      struct macho_syminfo_data *p;

	      p = backtrace_atomic_load_pointer (pp);
	      
	      if (p == NULL)
		break;

	      pp = &p->next;
	    }

	  if (__sync_bool_compare_and_swap (pp, NULL, sdata))
	    break;
	}
    }

  return 1;

 fail:
  if (macho_symbols != NULL)
    backtrace_free (state, macho_symbols, macho_symbol_size,
		    error_callback, data);
  if (sym_view_valid)
    backtrace_release_view (state, &sym_view, error_callback, data);
  if (str_view_valid)
    backtrace_release_view (state, &str_view, error_callback, data);
  return 0;
}

/* Return the symbol name and value for an ADDR.  */

static void
macho_syminfo (struct backtrace_state *state, uintptr_t addr,
	       backtrace_syminfo_callback callback,
	       backtrace_error_callback error_callback ATTRIBUTE_UNUSED,
	       void *data)
{
  struct macho_syminfo_data *sdata;
  struct macho_symbol *sym;

  sym = NULL;
  if (!state->threaded)
    {
      for (sdata = (struct macho_syminfo_data *) state->syminfo_data;
	   sdata != NULL;
	   sdata = sdata->next)
	{
	  sym = ((struct macho_symbol *)
		 bsearch (&addr, sdata->symbols, sdata->count,
			  sizeof (struct macho_symbol), macho_symbol_search));
	  if (sym != NULL)
	    break;
	}
    }
  else
    {
      struct macho_syminfo_data **pp;

      pp = (struct macho_syminfo_data **) (void *) &state->syminfo_data;
      while (1)
	{
	  sdata = backtrace_atomic_load_pointer (pp);
	  if (sdata == NULL)
	    break;

	  sym = ((struct macho_symbol *)
		 bsearch (&addr, sdata->symbols, sdata->count,
			  sizeof (struct macho_symbol), macho_symbol_search));
	  if (sym != NULL)
	    break;

	  pp = &sdata->next;
	}
    }

  if (sym == NULL)
    callback (data, addr, NULL, 0, 0);
  else
    callback (data, addr, sym->name, sym->address, 0);
}

/* Look through a fat file to find the relevant executable.  Returns 1
   on success, 0 on failure (in both cases descriptor is closed).  */

static int
macho_add_fat (struct backtrace_state *state, const char *filename,
	       int descriptor, int swapped, off_t offset,
	       const unsigned char *match_uuid,
	       struct libbacktrace_base_address base_address,
	       int skip_symtab, uint32_t nfat_arch, int is_64,
	       backtrace_error_callback error_callback, void *data,
	       fileline *fileline_fn, int *found_sym)
{
  int arch_view_valid;
  unsigned int cputype;
  size_t arch_size;
  struct backtrace_view arch_view;
  unsigned int i;

  arch_view_valid = 0;

#if defined (__x86_64__)
  cputype = MACH_O_CPU_TYPE_X86_64;
#elif defined (__i386__)
  cputype = MACH_O_CPU_TYPE_X86;
#elif defined (__aarch64__)
  cputype = MACH_O_CPU_TYPE_ARM64;
#elif defined (__arm__)
  cputype = MACH_O_CPU_TYPE_ARM;
#elif defined (__ppc__)
  cputype = MACH_O_CPU_TYPE_PPC;
#elif defined (__ppc64__)
  cputype = MACH_O_CPU_TYPE_PPC64;
#else
  error_callback (data, "unknown Mach-O architecture", 0);
  goto fail;
#endif

  if (is_64)
    arch_size = sizeof (struct macho_fat_arch_64);
  else
    arch_size = sizeof (struct macho_fat_arch);

  if (!backtrace_get_view (state, descriptor, offset,
			   nfat_arch * arch_size,
			   error_callback, data, &arch_view))
    goto fail;

  for (i = 0; i < nfat_arch; ++i)
    {
      uint32_t fcputype;
      uint64_t foffset;

      if (is_64)
	{
	  struct macho_fat_arch_64 fat_arch_64;

	  memcpy (&fat_arch_64,
		  (const char *) arch_view.data + i * arch_size,
		  arch_size);
	  fcputype = fat_arch_64.cputype;
	  foffset = fat_arch_64.offset;
	  if (swapped)
	    {
	      fcputype = __builtin_bswap32 (fcputype);
	      foffset = __builtin_bswap64 (foffset);
	    }
	}
      else
	{
	  struct macho_fat_arch fat_arch_32;

	  memcpy (&fat_arch_32,
		  (const char *) arch_view.data + i * arch_size,
		  arch_size);
	  fcputype = fat_arch_32.cputype;
	  foffset = (uint64_t) fat_arch_32.offset;
	  if (swapped)
	    {
	      fcputype = __builtin_bswap32 (fcputype);
	      foffset = (uint64_t) __builtin_bswap32 ((uint32_t) foffset);
	    }
	}

      if (fcputype == cputype)
	{
	  /* FIXME: What about cpusubtype?  */
	  backtrace_release_view (state, &arch_view, error_callback, data);
	  return macho_add (state, filename, descriptor, foffset, match_uuid,
			    base_address, skip_symtab, error_callback, data,
			    fileline_fn, found_sym);
	}
    }

  error_callback (data, "could not find executable in fat file", 0);

 fail:
  if (arch_view_valid)
    backtrace_release_view (state, &arch_view, error_callback, data);
  if (descriptor != -1)
    backtrace_close (descriptor, error_callback, data);
  return 0;
}

/* Look for the dsym file for FILENAME.  This is called if FILENAME
   does not have debug info or a symbol table.  Returns 1 on success,
   0 on failure.  */

static int
macho_add_dsym (struct backtrace_state *state, const char *filename,
		struct libbacktrace_base_address base_address,
		const unsigned char *uuid,
		backtrace_error_callback error_callback, void *data,
		fileline* fileline_fn)
{
  const char *p;
  const char *dirname;
  char *diralc;
  size_t dirnamelen;
  const char *basename;
  size_t basenamelen;
  const char *dsymsuffixdir;
  size_t dsymsuffixdirlen;
  size_t dsymlen;
  char *dsym;
  char *ps;
  int d;
  int does_not_exist;
  int dummy_found_sym;

  diralc = NULL;
  dirnamelen = 0;
  dsym = NULL;
  dsymlen = 0;

  p = strrchr (filename, '/');
  if (p == NULL)
    {
      dirname = ".";
      dirnamelen = 1;
      basename = filename;
      basenamelen = strlen (basename);
      diralc = NULL;
    }
  else
    {
      dirnamelen = p - filename;
      diralc = (char*)backtrace_alloc (state, dirnamelen + 1, error_callback, data);
      if (diralc == NULL)
	goto fail;
      memcpy (diralc, filename, dirnamelen);
      diralc[dirnamelen] = '\0';
      dirname = diralc;
      basename = p + 1;
      basenamelen = strlen (basename);
    }

  dsymsuffixdir = ".dSYM/Contents/Resources/DWARF/";
  dsymsuffixdirlen = strlen (dsymsuffixdir);

  dsymlen = (dirnamelen
	     + 1
	     + basenamelen
	     + dsymsuffixdirlen
	     + basenamelen
	     + 1);
  dsym = (char*)backtrace_alloc (state, dsymlen, error_callback, data);
  if (dsym == NULL)
    goto fail;

  ps = dsym;
  memcpy (ps, dirname, dirnamelen);
  ps += dirnamelen;
  *ps++ = '/';
  memcpy (ps, basename, basenamelen);
  ps += basenamelen;
  memcpy (ps, dsymsuffixdir, dsymsuffixdirlen);
  ps += dsymsuffixdirlen;
  memcpy (ps, basename, basenamelen);
  ps += basenamelen;
  *ps = '\0';

  if (diralc != NULL)
    {
      backtrace_free (state, diralc, dirnamelen + 1, error_callback, data);
      diralc = NULL;
    }

  d = backtrace_open (dsym, error_callback, data, &does_not_exist);
  if (d < 0)
    {
      /* The file does not exist, so we can't read the debug info.
	 Just return success.  */
      backtrace_free (state, dsym, dsymlen, error_callback, data);
      return 1;
    }

  if (!macho_add (state, dsym, d, 0, uuid, base_address, 1,
		  error_callback, data, fileline_fn, &dummy_found_sym))
    goto fail;

  backtrace_free (state, dsym, dsymlen, error_callback, data);

  return 1;

 fail:
  if (dsym != NULL)
    backtrace_free (state, dsym, dsymlen, error_callback, data);
  if (diralc != NULL)
    backtrace_free (state, diralc, dirnamelen, error_callback, data);
  return 0;
}

/* Add the backtrace data for a Macho-O file.  Returns 1 on success, 0
   on failure (in both cases descriptor is closed).

   FILENAME: the name of the executable.
   DESCRIPTOR: an open descriptor for the executable, closed here.
   OFFSET: the offset within the file of this executable, for fat files.
   MATCH_UUID: if not NULL, UUID that must match.
   BASE_ADDRESS: the load address of the executable.
   SKIP_SYMTAB: if non-zero, ignore the symbol table; used for dSYM files.
   FILELINE_FN: set to the fileline function, by backtrace_dwarf_add.
   FOUND_SYM: set to non-zero if we found the symbol table.
*/

static int
macho_add (struct backtrace_state *state, const char *filename, int descriptor,
	   off_t offset, const unsigned char *match_uuid,
	   struct libbacktrace_base_address base_address, int skip_symtab,
	   backtrace_error_callback error_callback, void *data,
	   fileline *fileline_fn, int *found_sym)
{
  struct backtrace_view header_view;
  struct macho_header_32 header;
  off_t hdroffset;
  int is_64;
  struct backtrace_view cmds_view;
  int cmds_view_valid;
  struct dwarf_sections dwarf_sections;
  int have_dwarf;
  unsigned char uuid[MACH_O_UUID_LEN];
  int have_uuid;
  size_t cmdoffset;
  unsigned int i;

  *found_sym = 0;

  cmds_view_valid = 0;

  /* The 32-bit and 64-bit file headers start out the same, so we can
     just always read the 32-bit version.  A fat header is shorter but
     it will always be followed by data, so it's OK to read extra.  */

  if (!backtrace_get_view (state, descriptor, offset,
			   sizeof (struct macho_header_32),
			   error_callback, data, &header_view))
    goto fail;

  memcpy (&header, header_view.data, sizeof header);

  backtrace_release_view (state, &header_view, error_callback, data);

  switch (header.magic)
    {
    case MACH_O_MH_MAGIC_32:
      is_64 = 0;
      hdroffset = offset + sizeof (struct macho_header_32);
      break;
    case MACH_O_MH_MAGIC_64:
      is_64 = 1;
      hdroffset = offset + sizeof (struct macho_header_64);
      break;
    case MACH_O_MH_MAGIC_FAT:
    case MACH_O_MH_MAGIC_FAT_64:
      {
	struct macho_header_fat fat_header;

	hdroffset = offset + sizeof (struct macho_header_fat);
	memcpy (&fat_header, &header, sizeof fat_header);
	return macho_add_fat (state, filename, descriptor, 0, hdroffset,
			      match_uuid, base_address, skip_symtab,
			      fat_header.nfat_arch,
			      header.magic == MACH_O_MH_MAGIC_FAT_64,
			      error_callback, data, fileline_fn, found_sym);
      }
    case MACH_O_MH_CIGAM_FAT:
    case MACH_O_MH_CIGAM_FAT_64:
      {
	struct macho_header_fat fat_header;
	uint32_t nfat_arch;

	hdroffset = offset + sizeof (struct macho_header_fat);
	memcpy (&fat_header, &header, sizeof fat_header);
	nfat_arch = __builtin_bswap32 (fat_header.nfat_arch);
	return macho_add_fat (state, filename, descriptor, 1, hdroffset,
			      match_uuid, base_address, skip_symtab,
			      nfat_arch,
			      header.magic == MACH_O_MH_CIGAM_FAT_64,
			      error_callback, data, fileline_fn, found_sym);
      }
    default:
      error_callback (data, "executable file is not in Mach-O format", 0);
      goto fail;
    }

  switch (header.filetype)
    {
    case MACH_O_MH_EXECUTE:
    case MACH_O_MH_DYLIB:
    case MACH_O_MH_DSYM:
      break;
    default:
      error_callback (data, "executable file is not an executable", 0);
      goto fail;
    }

  if (!backtrace_get_view (state, descriptor, hdroffset, header.sizeofcmds,
			   error_callback, data, &cmds_view))
    goto fail;
  cmds_view_valid = 1;

  memset (&dwarf_sections, 0, sizeof dwarf_sections);
  have_dwarf = 0;
  memset (&uuid, 0, sizeof uuid);
  have_uuid = 0;

  cmdoffset = 0;
  for (i = 0; i < header.ncmds; ++i)
    {
      const char *pcmd;
      struct macho_load_command load_command;

      if (cmdoffset + sizeof load_command > header.sizeofcmds)
	break;

      pcmd = (const char *) cmds_view.data + cmdoffset;
      memcpy (&load_command, pcmd, sizeof load_command);

      switch (load_command.cmd)
	{
	case MACH_O_LC_SEGMENT:
	  {
	    struct macho_segment_command segcmd;

	    memcpy (&segcmd, pcmd, sizeof segcmd);
	    if (memcmp (segcmd.segname,
			"__DWARF\0\0\0\0\0\0\0\0\0",
			MACH_O_NAMELEN) == 0)
	      {
		if (!macho_add_dwarf_segment (state, descriptor, offset,
					      load_command.cmd,
					      pcmd + sizeof segcmd,
					      (load_command.cmdsize
					       - sizeof segcmd),
					      segcmd.nsects, error_callback,
					      data, &dwarf_sections))
		  goto fail;
		have_dwarf = 1;
	      }
	  }
	  break;

	case MACH_O_LC_SEGMENT_64:
	  {
	    struct macho_segment_64_command segcmd;

	    memcpy (&segcmd, pcmd, sizeof segcmd);
	    if (memcmp (segcmd.segname,
			"__DWARF\0\0\0\0\0\0\0\0\0",
			MACH_O_NAMELEN) == 0)
	      {
		if (!macho_add_dwarf_segment (state, descriptor, offset,
					      load_command.cmd,
					      pcmd + sizeof segcmd,
					      (load_command.cmdsize
					       - sizeof segcmd),
					      segcmd.nsects, error_callback,
					      data, &dwarf_sections))
		  goto fail;
		have_dwarf = 1;
	      }
	  }
	  break;

	case MACH_O_LC_SYMTAB:
	  if (!skip_symtab)
	    {
	      struct macho_symtab_command symcmd;

	      memcpy (&symcmd, pcmd, sizeof symcmd);
	      if (!macho_add_symtab (state, descriptor, base_address, is_64,
				     offset + symcmd.symoff, symcmd.nsyms,
				     offset + symcmd.stroff, symcmd.strsize,
				     error_callback, data))
		goto fail;

	      *found_sym = 1;
	    }
	  break;

	case MACH_O_LC_UUID:
	  {
	    struct macho_uuid_command uuidcmd;

	    memcpy (&uuidcmd, pcmd, sizeof uuidcmd);
	    memcpy (&uuid[0], &uuidcmd.uuid[0], MACH_O_UUID_LEN);
	    have_uuid = 1;
	  }
	  break;

	default:
	  break;
	}

      cmdoffset += load_command.cmdsize;
    }

  if (!backtrace_close (descriptor, error_callback, data))
    goto fail;
  descriptor = -1;

  backtrace_release_view (state, &cmds_view, error_callback, data);
  cmds_view_valid = 0;

  if (match_uuid != NULL)
    {
      /* If we don't have a UUID, or it doesn't match, just ignore
	 this file.  */
      if (!have_uuid
	  || memcmp (match_uuid, &uuid[0], MACH_O_UUID_LEN) != 0)
	return 1;
    }

  if (have_dwarf)
    {
      int is_big_endian;

      is_big_endian = 0;
#if defined(__BYTE_ORDER__) && defined(__ORDER_BIG_ENDIAN__)
#if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
      is_big_endian = 1;
#endif
#endif

      if (!backtrace_dwarf_add (state, base_address, &dwarf_sections,
				is_big_endian, NULL, error_callback, data,
				fileline_fn, NULL))
	goto fail;
    }

  if (!have_dwarf && have_uuid)
    {
      if (!macho_add_dsym (state, filename, base_address, &uuid[0],
			   error_callback, data, fileline_fn))
	goto fail;
    }

  return 1;

 fail:
  if (cmds_view_valid)
    backtrace_release_view (state, &cmds_view, error_callback, data);
  if (descriptor != -1)
    backtrace_close (descriptor, error_callback, data);
  return 0;
}

#ifdef HAVE_MACH_O_DYLD_H

/* Initialize the backtrace data we need from a Mach-O executable
   using the dyld support functions.  This closes descriptor.  */

int
backtrace_initialize (struct backtrace_state *state, const char *filename,
		      int descriptor, backtrace_error_callback error_callback,
		      void *data, fileline *fileline_fn)
{
  uint32_t c;
  uint32_t i;
  int closed_descriptor;
  int found_sym;
  fileline macho_fileline_fn;

  closed_descriptor = 0;
  found_sym = 0;
  macho_fileline_fn = macho_nodebug;

  c = _dyld_image_count ();
  for (i = 0; i < c; ++i)
    {
      struct libbacktrace_base_address base_address;
      const char *name;
      int d;
      fileline mff;
      int mfs;

      name = _dyld_get_image_name (i);
      if (name == NULL)
	continue;

      if (strcmp (name, filename) == 0 && !closed_descriptor)
	{
	  d = descriptor;
	  closed_descriptor = 1;
	}
      else
	{
	  int does_not_exist;

	  d = backtrace_open (name, error_callback, data, &does_not_exist);
	  if (d < 0)
	    continue;
	}

      base_address.m = _dyld_get_image_vmaddr_slide (i);

      mff = macho_nodebug;
      if (!macho_add (state, name, d, 0, NULL, base_address, 0,
		      error_callback, data, &mff, &mfs))
	continue;

      if (mff != macho_nodebug)
	macho_fileline_fn = mff;
      if (mfs)
	found_sym = 1;
    }

  if (!closed_descriptor)
    backtrace_close (descriptor, error_callback, data);

  if (!state->threaded)
    {
      if (found_sym)
	state->syminfo_fn = macho_syminfo;
      else if (state->syminfo_fn == NULL)
	state->syminfo_fn = macho_nosyms;
    }
  else
    {
      if (found_sym)
	backtrace_atomic_store_pointer (&state->syminfo_fn, &macho_syminfo);
      else
	(void) __sync_bool_compare_and_swap (&state->syminfo_fn, NULL,
					     macho_nosyms);
    }

  if (!state->threaded)
    *fileline_fn = state->fileline_fn;
  else
    *fileline_fn = backtrace_atomic_load_pointer (&state->fileline_fn);

  if (*fileline_fn == NULL || *fileline_fn == macho_nodebug)
    *fileline_fn = macho_fileline_fn;

  return 1;
}

#else /* !defined (HAVE_MACH_O_DYLD_H) */

/* Initialize the backtrace data we need from a Mach-O executable
   without using the dyld support functions.  This closes
   descriptor.  */

int
backtrace_initialize (struct backtrace_state *state, const char *filename,
		      int descriptor, backtrace_error_callback error_callback,
		      void *data, fileline *fileline_fn)
{
  fileline macho_fileline_fn;
  struct libbacktrace_base_address zero_base_address;
  int found_sym;

  macho_fileline_fn = macho_nodebug;
  memset (&zero_base_address, 0, sizeof zero_base_address);
  if (!macho_add (state, filename, descriptor, 0, NULL, zero_base_address, 0,
		  error_callback, data, &macho_fileline_fn, &found_sym))
    return 0;

  if (!state->threaded)
    {
      if (found_sym)
	state->syminfo_fn = macho_syminfo;
      else if (state->syminfo_fn == NULL)
	state->syminfo_fn = macho_nosyms;
    }
  else
    {
      if (found_sym)
	backtrace_atomic_store_pointer (&state->syminfo_fn, &macho_syminfo);
      else
	(void) __sync_bool_compare_and_swap (&state->syminfo_fn, NULL,
					     macho_nosyms);
    }

  if (!state->threaded)
    *fileline_fn = state->fileline_fn;
  else
    *fileline_fn = backtrace_atomic_load_pointer (&state->fileline_fn);

  if (*fileline_fn == NULL || *fileline_fn == macho_nodebug)
    *fileline_fn = macho_fileline_fn;

  return 1;
}

#endif /* !defined (HAVE_MACH_O_DYLD_H) */

}
