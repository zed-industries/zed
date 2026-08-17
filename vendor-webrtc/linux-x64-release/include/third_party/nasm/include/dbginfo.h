/* ----------------------------------------------------------------------- *
 *
 *   Copyright 2020 The NASM Authors - All Rights Reserved
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

/*
 * dbginfo.h - debugging info structures
 */

#ifndef NASM_DBGINFO_H
#define NASM_DBGINFO_H

#include "compiler.h"
#include "srcfile.h"
#include "rbtree.h"

struct debug_macro_def;         /* Definition */
struct debug_macro_inv;         /* Invocation */
struct debug_macro_addr;        /* Address range */

/*
 * Definitions structure, one for each non-.nolist macro invoked
 * anywhere in the program; unique for each macro, even if a macro is
 * redefined and/or overloaded.
 */
struct debug_macro_def {
    struct debug_macro_def *next; /* List of definitions */
    const char *name;            /* Macro name */
    struct src_location where;   /* Start of definition */
    size_t ninv;                 /* Call count */
};

/*
 * Invocation structure. One for each invocation of a non-.nolist macro.
 */
struct debug_macro_inv_list {
    struct debug_macro_inv *l;
    size_t n;
};

struct debug_macro_inv {
    struct debug_macro_inv *next; /* List of same-level invocations */
    struct debug_macro_inv_list down;
    struct debug_macro_inv *up;   /* Parent invocation */
    struct debug_macro_def *def;  /* Macro definition */
    struct src_location where;    /* Start of invocation */
    struct {                      /* Address range pointers */
        struct rbtree *tree;           /* rbtree of address ranges */
        struct debug_macro_addr *last; /* Quick lookup for latest section */
    } addr;
    uint32_t naddr;             /* Number of address ranges */
    int32_t  lastseg;           /* lastaddr segment number  */
};

/*
 * Address range structure. An rbtree containing one address range for each
 * section which this particular macro has generated code/data/space into.
 */
struct debug_macro_addr {
    struct rbtree tree;          /* rbtree; key = index, must be first */
    struct debug_macro_addr *up; /* same section in parent invocation */
    uint64_t start;              /* starting offset */
    uint64_t len;                /* length of range */
};

/*
 * Complete information structure */
struct debug_macro_info {
    struct debug_macro_inv_list inv;
    struct debug_macro_def_list {
        struct debug_macro_def *l;
        size_t n;
    } def;
};

static inline int32_t debug_macro_seg(const struct debug_macro_addr *dma)
{
    return dma->tree.key;
}

/* Get/create a addr structure for the macro we are emitting for */
struct debug_macro_addr *debug_macro_get_addr(int32_t seg);

/* The macro we are currently emitting for, if any */
extern struct debug_macro_inv *debug_current_macro;

#endif /* NASM_DBGINFO_H */
