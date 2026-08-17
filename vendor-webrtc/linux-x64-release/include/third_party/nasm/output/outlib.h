/* ----------------------------------------------------------------------- *
 *   
 *   Copyright 1996-2020 The NASM Authors - All Rights Reserved
 *   See the file AUTHORS included with the NASM distribution for
 *   the specific copyright holders.
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following
 *   conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above
 *     copyright notice, this list of conditions and the following
 *     disclaimer in the documentation and/or other materials provided
 *     with the distribution.
 *     
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
 *     CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *     INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *     MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *     DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 *     CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *     SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *     NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 *     HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 *     CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 *     OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 *     EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * ----------------------------------------------------------------------- */

#ifndef NASM_OUTLIB_H
#define NASM_OUTLIB_H

#include "compiler.h"
#include "nasm.h"
#include "error.h"
#include "hashtbl.h"
#include "saa.h"
#include "rbtree.h"

uint64_t realsize(enum out_type type, uint64_t size);

/* Do-nothing versions of some output routines */
enum directive_result
null_directive(enum directive directive, char *value);
void null_sectalign(int32_t seg, unsigned int value);
void null_reset(void);
int32_t null_segbase(int32_t seg);

/* Do-nothing versions of all the debug routines */
void null_debug_init(void);
void null_debug_linenum(const char *filename, int32_t linenumber,
			int32_t segto);
void null_debug_deflabel(char *name, int32_t segment, int64_t offset,
                         int is_global, char *special);
void null_debug_directive(const char *directive, const char *params);
void null_debug_typevalue(int32_t type);
void null_debug_output(int type, void *param);
void null_debug_cleanup(void);
extern const struct dfmt * const null_debug_arr[2];

/* Wrapper for unported backends */
void nasm_do_legacy_output(const struct out_data *data);

/*
 * Common routines for tasks that really should migrate into the core.
 * This provides a common interface for maintaining sections and symbols,
 * and provide quick lookups as well as declared-order sequential walks.
 *
 * These structures are intended to be embedded at the *top* of a
 * backend-specific structure containing additional information.
 *
 * The tokens O_Section, O_Symbol and O_Reloc are intended to be
 * defined as macros by the backend before including this file!
 */

struct ol_sect;
struct ol_sym;

#ifndef O_Section
typedef struct ol_sect O_Section;
#endif
#ifndef O_Symbol
typedef struct ol_sym O_Symbol;
#endif
#ifndef O_Reloc
typedef void * O_Reloc;
#endif

/* Common section structure */

/*
 * Common flags for sections and symbols; low values reserved for
 * backend.  Note that both ol_sect and ol_sym begin with a flags
 * field, so if a section pointer points to an external symbol instead
 * they can be trivially resolved.
 */
#define OF_SYMBOL 0x80000000
#define OF_GLOBAL 0x40000000
#define OF_IMPSEC 0x20000000
#define OF_COMMON 0x10000000

struct ol_sym;

struct ol_symlist {
    struct ol_symlist *next;
    struct rbtree tree;
};
struct ol_symhead {
    struct ol_symlist *head, **tail;
    struct rbtree *tree;
    uint64_t n;
};

struct ol_sect {
    uint32_t flags;             /* Section/symbol flags */
    struct ol_sect *next;       /* Next section in declared order */
    const char *name;           /* Name of section */
    struct ol_symhead syml;     /* All symbols in this section */
    struct ol_symhead symg;     /* Global symbols in this section */
    struct SAA *data;           /* Contents of section */
    struct SAA *reloc;          /* Section relocations */
    uint32_t index;             /* Primary section index */
    uint32_t subindex;          /* Current subsection index */
};

/* Segment reference */
enum ol_seg_type {
    OS_NOSEG  = 0,                /* Plain number (no segment) */
    OS_SEGREF = 1,                /* It is a segment reference */
    OS_ABS    = 1,                /* Absolute segment reference */
    OS_SECT   = 2,                /* It is a real section */
    OS_OFFS   = OS_SECT,          /* Offset reference in section */
    OS_SEG    = OS_SECT|OS_SEGREF /* Section reference */
};

union ol_segval {
    struct ol_sect *sect;   /* Section structure */
    struct ol_sym  *sym;    /* External symbol structure */
};

struct ol_seg {
    union ol_segval  s;
    enum ol_seg_type t;

    /*
     * For a section:          subsection index
     * For a metasymbol:       virtual segment index
     * For an absolute symbol: absolute value
     */
    uint32_t index;
};

/* seg:offs representing the full location value and type */
struct ol_loc {
    int64_t offs;
    struct ol_seg seg;
};

/* Common symbol structure */
struct ol_sym {
    uint32_t flags;             /* Section/symbol flags */
    uint32_t size;              /* Size value (for backend) */
    struct ol_sym *next;       	/* Next symbol in declared order */
    const char *name;           /* Symbol name */
    struct ol_symlist syml;     /* Section-local symbol list */
    struct ol_symlist symg;     /* Section-local global symbol list */
    struct ol_loc p;            /* Symbol position ("where") */
    struct ol_loc v;            /* Symbol value ("what") */
};

/*
 * Operations
 */
void ol_init(void);
void ol_cleanup(void);

/* Convert offs:seg to a location structure */
extern void
ol_mkloc(struct ol_loc *loc, int64_t offs, int32_t seg);

/* Get the section or external symbol from a struct ol_seg */
static inline O_Section *seg_sect(struct ol_seg *seg)
{
    return (O_Section *)seg->s.sect;
}
static inline O_Symbol *seg_xsym(struct ol_seg *seg)
{
    return (O_Symbol *)seg->s.sym;
}

/*
 * Return a pointer to the symbol structure if and only if a section is
 * really a symbol of some kind (extern, common...)
 */
static inline struct ol_sym *_seg_extsym(struct ol_sect *sect)
{
    return (sect->flags & OF_SYMBOL) ? (struct ol_sym *)sect : NULL;
}
static inline O_Symbol *seg_extsym(O_Section *sect)
{
    return (O_Symbol *)_seg_extsym((struct ol_sect *)sect);
}

/*
 * Find a section or create a new section structure if it does not exist
 * and allocate it an index value via seg_alloc().
 */
extern struct ol_sect *
_ol_get_sect(const char *name, size_t ssize, size_t rsize);
static inline O_Section *ol_get_sect(const char *name)
{
    return (O_Section *)_ol_get_sect(name, sizeof(O_Section), sizeof(O_Reloc));
}

/* Find a section by name without creating one */
extern struct ol_sect *_ol_sect_by_name(const char *);
static inline O_Section *ol_sect_by_name(const char *name)
{
    return (O_Section *)_ol_sect_by_name(name);
}

/* Find a section or external symbol by index; NULL if not valid */
extern struct ol_sect *_ol_sect_by_index(int32_t index);
static inline O_Section *ol_sect_by_index(int32_t index)
{
    return (O_Section *)_ol_sect_by_index(index);
}

/* Global list of sections (not including external symbols) */
extern struct ol_sect *_ol_sect_list;
static inline O_Section *ol_sect_list(void)
{
    return (O_Section *)_ol_sect_list;
}

/* Count of sections (not including external symbols) */
extern uint64_t _ol_nsects;
static inline uint64_t ol_nsects(void)
{
    return _ol_nsects;
}

/*
 * Start a new subsection for the given section. At the moment, once a
 * subsection has been created, it is not possible to revert to an
 * earlier subsection. ol_sect_by_index() will return the main section
 * structure. Returns the new section index.  This is used to prevent
 * the front end from optimizing across subsection boundaries.
 */
extern int32_t _ol_new_subsection(struct ol_sect *sect);
static inline int32_t ol_new_subsection(O_Section *sect)
{
    return _ol_new_subsection((struct ol_sect *)sect);
}

/*
 * Create a new symbol. If this symbol is OS_OFFS, add it to the relevant
 * section, too. If the symbol already exists, return NULL; this is
 * different from ol_get_section() as a single section may be invoked
 * many times. On the contrary, the front end will prevent a single symbol
 * from being defined more than once.
 *
 * If flags has OF_GLOBAL set, add it to the global symbol hash for the
 * containing section. If flags has OF_IMPSEC set, allocate a segment
 * index for it via seg_alloc() and add it to the section by index list.
 */
extern struct ol_sym *_ol_new_sym(const char *name, const struct ol_loc *v,
                                  uint32_t flags, size_t size);
static inline O_Symbol *ol_new_sym(const char *name, const struct ol_loc *v,
                                   uint32_t flags)
{
    return (O_Symbol *)_ol_new_sym(name, v, flags, sizeof(O_Symbol));
}

/* Find a symbol by name in the global namespace */
extern struct ol_sym *_ol_sym_by_name(const char *name);
static inline O_Symbol *ol_sym_by_name(const char *name)
{
    return (O_Symbol *)_ol_sym_by_name(name);
}

/*
 * Find a symbol by address in a specific section. If no symbol is defined
 * at that exact address, return the immediately previously defined one.
 * If global is set, then only return global symbols.
 */
extern struct ol_sym *_ol_sym_by_address(struct ol_sect *sect, int64_t addr,
                                         bool global);
static inline O_Symbol *ol_sym_by_address(O_Section *sect, int64_t addr,
                                          bool global)
{
    return (O_Symbol *)_ol_sym_by_address((struct ol_sect *)sect, addr, global);
}

/* Global list of symbols */
extern struct ol_sym *_ol_sym_list;
static inline O_Symbol *ol_sym_list(void)
{
    return (O_Symbol *)_ol_sym_list;
}

/* Global count of symbols */
extern uint64_t _ol_nsyms;
static inline uint64_t ol_nsyms(void)
{
    return _ol_nsyms;
}

#endif /* NASM_OUTLIB_H */
