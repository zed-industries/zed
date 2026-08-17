/* ----------------------------------------------------------------------- *
 *
 *   Copyright 1996-2022 The NASM Authors - All Rights Reserved
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
 * outform.h    header file for binding output format drivers to the
 *              remainder of the code in the Netwide Assembler
 */

/*
 * This header file allows configuration of which output formats
 * get compiled into the NASM binary. You can configure by defining
 * various preprocessor symbols beginning with "OF_", either on the
 * compiler command line or at the top of this file.
 *
 * OF_ONLY                -- only include specified object formats
 * OF_name                -- ensure that output format 'name' is included
 * OF_NO_name             -- remove output format 'name'
 * OF_DOS                 -- ensure that 'obj', 'bin', 'win32' & 'win64' are included.
 * OF_UNIX                -- ensure that 'aout', 'aoutb', 'coff', 'elf32' & 'elf64' are in.
 * OF_OTHERS              -- ensure that 'bin', 'as86', 'rdf' 'macho32' & 'macho64' are in.
 * OF_ALL                 -- ensure that all formats are included.
 *                           note that this doesn't include 'dbg', which is
 *                           only really useful if you're doing development
 *                           work on NASM. Define OF_DBG if you want this.
 *
 * OF_DEFAULT=of_name     -- ensure that 'name' is the default format.
 *
 * eg: -DOF_UNIX -DOF_ELF32 -DOF_DEFAULT=of_elf32 would be a suitable config
 * for an average linux system.
 *
 * Default config = -DOF_ALL -DOF_DEFAULT=of_bin
 *
 * You probably only want to set these options while compiling 'nasm.c'. */

#ifndef NASM_OUTFORM_H
#define NASM_OUTFORM_H

#include "nasm.h"

/* -------------- USER MODIFIABLE PART ---------------- */

/*
 * Insert #defines here in accordance with the configuration
 * instructions above.
 *
 * E.g.
 *
 * #define OF_ONLY
 * #define OF_OBJ
 * #define OF_BIN
 *
 * for a 16-bit DOS assembler with no extraneous formats.
 */

/* ------------ END USER MODIFIABLE PART -------------- */

/* ====configurable info begins here==== */
/* formats configurable:
 * bin,obj,elf32,elf64,aout,aoutb,coff,win32,as86,rdf2,macho32,macho64 */

/* process options... */

#ifndef OF_ONLY
#ifndef OF_ALL
#define OF_ALL                  /* default is to have all formats */
#endif
#endif

#ifdef OF_ALL                   /* set all formats on... */
#ifndef OF_BIN
#define OF_BIN
#endif
#ifndef OF_OBJ
#define OF_OBJ
#endif
#ifndef OF_ELF32
#define OF_ELF32
#endif
#ifndef OF_ELFX32
#define OF_ELFX32
#endif
#ifndef OF_ELF64
#define OF_ELF64
#endif
#ifndef OF_COFF
#define OF_COFF
#endif
#ifndef OF_AOUT
#define OF_AOUT
#endif
#ifndef OF_AOUTB
#define OF_AOUTB
#endif
#ifndef OF_WIN32
#define OF_WIN32
#endif
#ifndef OF_WIN64
#define OF_WIN64
#endif
#ifndef OF_AS86
#define OF_AS86
#endif
#ifndef OF_IEEE
#define OF_IEEE
#endif
#ifndef OF_MACHO32
#define OF_MACHO32
#endif
#ifndef OF_MACHO64
#define OF_MACHO64
#endif
#ifndef OF_DBG
#define OF_DBG
#endif
#endif                          /* OF_ALL */

/* turn on groups of formats specified.... */
#ifdef OF_DOS
#ifndef OF_OBJ
#define OF_OBJ
#endif
#ifndef OF_BIN
#define OF_BIN
#endif
#ifndef OF_COFF
#define OF_COFF                 /* COFF is used by DJGPP */
#endif
#ifndef OF_WIN32
#define OF_WIN32
#endif
#ifndef OF_WIN64
#define OF_WIN64
#endif
#endif

#ifdef OF_UNIX
#ifndef OF_AOUT
#define OF_AOUT
#endif
#ifndef OF_AOUTB
#define OF_AOUTB
#endif
#ifndef OF_COFF
#define OF_COFF
#endif
#ifndef OF_ELF32
#define OF_ELF32
#endif
#ifndef OF_ELF64
#define OF_ELF64
#endif
#ifndef OF_ELFX32
#define OF_ELFX32
#endif
#endif

#ifdef OF_OTHERS
#ifndef OF_BIN
#define OF_BIN
#endif
#ifndef OF_AS86
#define OF_AS86
#endif
#ifndef OF_IEEE
#define OF_IEEE
#endif
#ifndef OF_MACHO32
#define OF_MACHO32
#endif
#ifndef OF_MACHO64
#define OF_MACHO64
#endif
#endif

/* finally... override any format specifically specified to be off */
#ifdef OF_NO_BIN
#undef OF_BIN
#endif
#ifdef OF_NO_OBJ
#undef OF_OBJ
#endif
#ifdef OF_NO_ELF32
#undef OF_ELF32
#endif
#ifdef OF_NO_ELF64
#undef OF_ELF64
#endif
#ifdef OF_NO_ELFX32
#undef OF_ELFX32
#endif
#ifdef OF_NO_AOUT
#undef OF_AOUT
#endif
#ifdef OF_NO_AOUTB
#undef OF_AOUTB
#endif
#ifdef OF_NO_COFF
#undef OF_COFF
#endif
#ifdef OF_NO_WIN32
#undef OF_WIN32
#endif
#ifdef OF_NO_WIN64
#undef OF_WIN64
#endif
#ifdef OF_NO_AS86
#undef OF_AS86
#endif
#ifdef OF_NO_IEEE
#undef OF_IEEE
#endif
#ifdef OF_NO_MACHO32
#undef OF_MACHO32
#endif
#ifdef OF_NO_MACHO64
#undef OF_MACHO64
#endif
#ifdef OF_NO_DBG
#undef OF_DBG
#endif

#ifndef OF_DEFAULT
#define OF_DEFAULT of_bin
#endif

extern const struct ofmt of_bin;
extern const struct ofmt of_ith;
extern const struct ofmt of_srec;
extern const struct ofmt of_aout;
extern const struct ofmt of_aoutb;
extern const struct ofmt of_coff;
extern const struct ofmt of_elf32;
extern const struct ofmt of_elfx32;
extern const struct ofmt of_elf64;
extern const struct ofmt of_as86;
extern const struct ofmt of_obj;
extern const struct ofmt of_win32;
extern const struct ofmt of_win64;
extern const struct ofmt of_ieee;
extern const struct ofmt of_macho32;
extern const struct ofmt of_macho64;
extern const struct ofmt of_dbg;

#ifdef BUILD_DRIVERS_ARRAY      /* only if included from outform.c */

/*
 * pull in the externs for the different formats, then make the
 * drivers array based on the above defines
 */

static const struct ofmt * const drivers[] = {
#ifdef OF_BIN
    &of_bin,
    &of_ith,
    &of_srec,
#endif
#ifdef OF_AOUT
    &of_aout,
#endif
#ifdef OF_AOUTB
    &of_aoutb,
#endif
#ifdef OF_COFF
    &of_coff,
#endif
#ifdef OF_ELF32
    &of_elf32,
#endif
#ifdef OF_ELF64
    &of_elf64,
#endif
#ifdef OF_ELFX32
    &of_elfx32,
#endif
#ifdef OF_AS86
    &of_as86,
#endif
#ifdef OF_OBJ
    &of_obj,
#endif
#ifdef OF_WIN32
    &of_win32,
#endif
#ifdef OF_WIN64
    &of_win64,
#endif
#ifdef OF_IEEE
    &of_ieee,
#endif
#ifdef OF_MACHO32
    &of_macho32,
#endif
#ifdef OF_MACHO64
    &of_macho64,
#endif
#ifdef OF_DBG
    &of_dbg,
#endif

    NULL
};

static const struct ofmt_alias ofmt_aliases[] = {
#ifdef OF_ELF32
    { "elf",  &of_elf32 },
#endif
#ifdef OF_MACHO32
    { "macho", &of_macho32 },
#endif
#ifdef OF_WIN32
    { "win",   &of_win32 },
#endif
    { NULL, NULL }
};

#endif /* BUILD_DRIVERS_ARRAY */

const struct ofmt *ofmt_find(const char *name, const struct ofmt_alias **ofmt_alias);
const struct dfmt *dfmt_find(const struct ofmt *, const char *);
void ofmt_list(const struct ofmt *, FILE *);
void dfmt_list(FILE *);
extern const struct dfmt null_debug_form;

#endif /* NASM_OUTFORM_H */
