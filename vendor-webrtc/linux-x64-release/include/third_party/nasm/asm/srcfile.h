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

/*
 * These functions are used to keep track of the source code file and name.
 */
#ifndef ASM_SRCFILE_H
#define ASM_SRCFILE_H

#include "compiler.h"

struct src_location {
    const char *filename;
    int32_t lineno;
};

static inline const_func struct src_location src_nowhere(void)
{
    struct src_location no_where = { NULL, 0 };
    return no_where;
}

/*
 * Comparing the *pointer value* of filenames is valid, because the
 * filename hash system guarantees that each unique filename string is
 * permanently allocated in exactly one location.
 */
static inline bool
src_location_same(struct src_location here, struct src_location there)
{
    return here.filename == there.filename && here.lineno == there.lineno;
}

struct src_location_stack {
    struct src_location l;
    struct src_location_stack *up, *down;
    const void *macro;
};
extern struct src_location_stack _src_top;
extern struct src_location_stack *_src_bottom;
extern struct src_location_stack *_src_error;

void src_init(void);
void src_free(void);
const char *src_set_fname(const char *newname);
static inline const char *src_get_fname(void)
{
    return _src_bottom->l.filename;
}
static inline int32_t src_set_linnum(int32_t newline)
{
    int32_t oldline = _src_bottom->l.lineno;
    _src_bottom->l.lineno = newline;
    return oldline;
}
static inline int32_t src_get_linnum(void)
{
    return _src_bottom->l.lineno;
}

/* Can be used when there is no need for the old information */
void src_set(int32_t line, const char *filename);

/*
 * src_get gets both the source file name and line.
 * It is also used if you maintain private status about the source location
 * It return 0 if the information was the same as the last time you
 * checked, -2 if the name changed and (new-old) if just the line changed.
 *
 * xname must point to a filename string previously returned from any
 * function of this subsystem or be NULL; another string value will
 * not work.
 */
static inline int32_t src_get(int32_t *xline, const char **xname)
{
    const char *xn = *xname;
    int32_t xl = *xline;
    int32_t line = _src_bottom->l.lineno;

    *xline = line;
    *xname = _src_bottom->l.filename;

    /* The return value is expected to be optimized out almost everywhere */
    if (!xn || xn != _src_bottom->l.filename)
        return -2;
    else
        return line - xl;
}

/*
 * Returns the current information as a structure.
 */
static inline struct src_location src_where(void)
{
    return _src_bottom->l;
}

/*
 * Returns the top-level information as a structure. Use this for panic
 * errors, since descent is not possible there.
 */
static inline struct src_location src_where_top(void)
{
    return _src_top.l;
}

/*
 * Returns the appropriate level of the location stack to use for error
 * messages. This is the same as the top level except during the descent
 * through the macro hierarchy for elucidation;
 */
static inline struct src_location src_where_error(void)
{
    return _src_error->l;
}
static inline const void *src_error_down(void)
{
    if (_src_error->down) {
        _src_error = _src_error->down;
        return _src_error->macro;
    } else {
        return NULL;
    }
}
static inline void src_error_reset(void)
{
    _src_error = &_src_top;
}

/*
 * Sets the current information. The filename member of the structure
 * *must* have been previously returned by src_get(), src_where(), or
 * src_get_fname() and therefore be present in the hash.
 */
static inline struct src_location src_update(struct src_location whence)
{
    struct src_location old = _src_bottom->l;
    _src_bottom->l = whence;
    return old;
}

/*
 * Push/pop macro expansion level. "macroname" must remain constant at
 * least until the same macro expansion level is popped.
 */
void src_macro_push(const void *macroname, struct src_location where);
static inline const void *src_macro_current(void)
{
    return _src_bottom->macro;
}
void src_macro_pop(void);

#endif /* ASM_SRCFILE_H */
