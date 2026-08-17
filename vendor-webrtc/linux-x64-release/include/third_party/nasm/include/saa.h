/* ----------------------------------------------------------------------- *
 *   
 *   Copyright 1996-2017 The NASM Authors - All Rights Reserved
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

#ifndef NASM_SAA_H
#define NASM_SAA_H

#include "compiler.h"

/*
 * Routines to manage a dynamic sequential-access array, under the
 * same restriction on maximum mallocable block. This array may be
 * written to in two ways: a contiguous chunk can be reserved of a
 * given size with a pointer returned OR single-byte data may be
 * written. The array can also be read back in the same two ways:
 * as a series of big byte-data blocks or as a list of structures
 * of a given size.
 */

struct SAA {
    /*
     * members `end' and `elem_len' are only valid in first link in
     * list; `rptr' and `rpos' are used for reading
     */
    size_t elem_len;            /* Size of each element */
    size_t blk_len;             /* Size of each allocation block */
    size_t nblks;               /* Total number of allocated blocks */
    size_t nblkptrs;            /* Total number of allocation block pointers */
    size_t length;              /* Total allocated length of the array */
    size_t datalen;             /* Total data length of the array */
    char **wblk;                /* Write block pointer */
    size_t wpos;                /* Write position inside block */
    size_t wptr;                /* Absolute write position */
    char **rblk;                /* Read block pointer */
    size_t rpos;                /* Read position inside block */
    size_t rptr;                /* Absolute read position */
    char **blk_ptrs;            /* Pointer to pointer blocks */
};

struct SAA * never_null saa_init(size_t elem_len);  /* 1 == byte */
void saa_free(struct SAA *);
void *saa_wstruct(struct SAA *);        /* return a structure of elem_len */
void saa_wbytes(struct SAA *, const void *, size_t);    /* write arbitrary bytes */
size_t saa_wcstring(struct SAA *s, const char *str);     /* write a C string */
void saa_rewind(struct SAA *);  /* for reading from beginning */
void *saa_rstruct(struct SAA *);        /* return NULL on EOA */
const void *saa_rbytes(struct SAA *, size_t *); /* return 0 on EOA */
void saa_rnbytes(struct SAA *, void *, size_t); /* read a given no. of bytes */
/* random access */
void saa_fread(struct SAA *, size_t, void *, size_t);
void saa_fwrite(struct SAA *, size_t, const void *, size_t);

/* dump to file */
void saa_fpwrite(struct SAA *, FILE *);

/* Write specific-sized values */
void saa_write8(struct SAA *s, uint8_t v);
void saa_write16(struct SAA *s, uint16_t v);
void saa_write32(struct SAA *s, uint32_t v);
void saa_write64(struct SAA *s, uint64_t v);
void saa_wleb128u(struct SAA *, int);   /* write unsigned LEB128 value */
void saa_wleb128s(struct SAA *, int);   /* write signed LEB128 value */
void saa_writeaddr(struct SAA *, uint64_t, size_t);

#endif                          /* NASM_SAA_H */
