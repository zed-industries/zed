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
 * listing.h   header file for listing.c
 */

#ifndef NASM_LISTING_H
#define NASM_LISTING_H

#include "nasm.h"

/*
 * List-file generators should look like this:
 */
struct lfmt {
    /*
     * Called to initialize the listing file generator. Before this
     * is called, the other routines will silently do nothing when
     * called. The `char *' parameter is the file name to write the
     * listing to.
     */
    void (*init)(const char *fname);

    /*
     * Called to clear stuff up and close the listing file.
     */
    void (*cleanup)(void);

    /*
     * Called to output binary data. Parameters are: the offset;
     * the data; the data type. Data types are similar to the
     * output-format interface, only OUT_ADDRESS will _always_ be
     * displayed as if it's relocatable, so ensure that any non-
     * relocatable address has been converted to OUT_RAWDATA by
     * then.
     */
    void (*output)(const struct out_data *data);

    /*
     * Called to send a text line to the listing generator. The
     * `int' parameter is LIST_READ or LIST_MACRO depending on
     * whether the line came directly from an input file or is the
     * result of a multi-line macro expansion.
     *
     * If a line number is provided, print it; if the line number is
     * -1 then use the same line number as the previous call.
     */
    void (*line)(int type, int32_t lineno, const char *line);

    /*
     * Called to change one of the various levelled mechanisms in the
     * listing generator. LIST_INCLUDE and LIST_MACRO can be used to
     * increase the nesting level of include files and macro
     * expansions; LIST_TIMES and LIST_INCBIN switch on the two
     * binary-output-suppression mechanisms for large-scale
     * pseudo-instructions; the size argument prints the size or
     * repetiiton count.
     *
     * LIST_MACRO_NOLIST is synonymous with LIST_MACRO except that
     * it indicates the beginning of the expansion of a `nolist'
     * macro, so anything under that level won't be expanded unless
     * it includes another file.
     */
    void (*uplevel)(int type, int64_t size);

    /*
     * Reverse the effects of uplevel.
     */
    void (*downlevel)(int type);

    /*
     * Called on a warning or error, with the error message.
     */
    void printf_func_ptr(2, 3) (*error)(errflags severity, const char *fmt, ...);

    /*
     * Update the current offset.  Used to give the listing generator
     * an offset to work with when doing things like
     * uplevel(LIST_TIMES) or uplevel(LIST_INCBIN); see
     * list_set_offset();
     */
    void (*set_offset)(uint64_t offset);
};

extern const struct lfmt *lfmt;
extern bool user_nolist;

/*
 * list_options are the requested options; active_list_options gets
 * set when a pass starts.
 *
 * These are simple bitmasks of ASCII-64 mapping directly to option
 * letters.
 */
extern uint64_t list_options, active_list_options;

/*
 * This maps the characters a-z, A-Z and 0-9 onto a 64-bit bitmask.
 * Bit 0 is used to indicate that the listing engine is active, and
 * bit 1 is reserved, so this will only return mask bits 2 and higher;
 * as there are 62 possible characters this fits nicely.
 *
 * The mask returned is 0 for invalid characters, accessing no bits at
 * all.
 *
 * This isn't particularly efficient code, but just about every
 * instance of it should be fed a constant, so the entire function can
 * be precomputed at compile time. The only cases where the full
 * computation is needed is when parsing the -L option or %pragma list
 * options, neither of which is in any way performance critical.
 *
 * The character + represents ALL listing options except -Lw (flush
 * after every line.)
 */
static inline const_func uint64_t list_option_mask_val(unsigned char x)
{
    if (x >= 'a') {
        if (x > 'z')
            return 0;
        x = x - 'a' + 2;
    } else if (x >= 'A') {
        if (x > 'Z')
            return 0;
        x = x - 'A' + 2 + 26;
    } else if (x >= '0') {
        if (x > '9')
            return 0;
        x = x - '0' + 2 + 26*2;
    } else {
        return 0;
    }

    return UINT64_C(1) << x;
}

static inline const_func uint64_t list_option_mask(unsigned char x)
{
    if (x == '+')
        return ~(list_option_mask_val('w') | 3);
    else
        return list_option_mask_val(x);
}

/* Return true if the listing engine is active and a certain option is set. */
static inline pure_func bool list_option(unsigned char x)
{
    return unlikely(active_list_options & list_option_mask(x));
}

/* We can't test this using active_list_options for obvious reasons... */
static inline pure_func bool list_on_every_pass(void)
{
    return unlikely(list_options & list_option_mask('p'));
}

/* Is the listing engine active? */
static inline pure_func bool list_active(void)
{
    return (active_list_options & 1);
}

/* Pragma handler */
enum directive_result list_pragma(const struct pragma *);

#endif
