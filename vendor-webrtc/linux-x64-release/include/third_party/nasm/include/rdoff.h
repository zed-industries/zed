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

/*
 * rdoff.h	RDOFF Object File manipulation routines header file
 */

#ifndef RDOFF_H
#define RDOFF_H 1

/*
 * RDOFF definitions. They are used by RDOFF utilities and by NASM's
 * 'outrdf2.c' output module.
 */

/* RDOFF format revision (currently used only when printing the version) */
#define RDOFF2_REVISION		"0.6.1"

/* RDOFF2 file signature */
#define RDOFF2_SIGNATURE	"RDOFF2"

/* Maximum size of an import/export label (including trailing zero) */
#define EXIM_LABEL_MAX		256

/* Maximum size of library or module name (including trailing zero) */
#define MODLIB_NAME_MAX		128

/* Maximum number of segments that we can handle in one file */
#define RDF_MAXSEGS		64

/* Record types that may present the RDOFF header */
#define RDFREC_GENERIC		0
#define RDFREC_RELOC		1
#define RDFREC_IMPORT		2
#define RDFREC_GLOBAL		3
#define RDFREC_DLL		4
#define RDFREC_BSS		5
#define RDFREC_SEGRELOC		6
#define RDFREC_FARIMPORT	7
#define RDFREC_MODNAME		8
#define RDFREC_COMMON		10

/*
 * Generic record - contains the type and length field, plus a 128 byte
 * array 'data'
 */
struct GenericRec {
    uint8_t type;
    uint8_t reclen;
    char data[128];
};

/*
 * Relocation record
 */
struct RelocRec {
    uint8_t type;                  /* must be 1 */
    uint8_t reclen;                /* content length */
    uint8_t segment;               /* only 0 for code, or 1 for data supported,
                                   but add 64 for relative refs (ie do not require
                                   reloc @ loadtime, only linkage) */
    int32_t offset;                /* from start of segment in which reference is loc'd */
    uint8_t length;                /* 1 2 or 4 bytes */
    uint16_t refseg;              /* segment to which reference refers to */
};

/*
 * Extern/import record
 */
struct ImportRec {
    uint8_t type;                  /* must be 2 */
    uint8_t reclen;                /* content length */
    uint8_t flags;                 /* SYM_* flags (see below) */
    uint16_t segment;             /* segment number allocated to the label for reloc
                                   records - label is assumed to be at offset zero
                                   in this segment, so linker must fix up with offset
                                   of segment and of offset within segment */
    char label[EXIM_LABEL_MAX]; /* zero terminated, should be written to file
                                   until the zero, but not after it */
};

/*
 * Public/export record
 */
struct ExportRec {
    uint8_t type;                  /* must be 3 */
    uint8_t reclen;                /* content length */
    uint8_t flags;                 /* SYM_* flags (see below) */
    uint8_t segment;               /* segment referred to (0/1/2) */
    int32_t offset;                /* offset within segment */
    char label[EXIM_LABEL_MAX]; /* zero terminated as in import */
};

/*
 * DLL record
 */
struct DLLRec {
    uint8_t type;                  /* must be 4 */
    uint8_t reclen;                /* content length */
    char libname[MODLIB_NAME_MAX];      /* name of library to link with at load time */
};

/*
 * BSS record
 */
struct BSSRec {
    uint8_t type;                  /* must be 5 */
    uint8_t reclen;                /* content length */
    int32_t amount;                /* number of bytes BSS to reserve */
};

/*
 * Module name record
 */
struct ModRec {
    uint8_t type;                  /* must be 8 */
    uint8_t reclen;                /* content length */
    char modname[MODLIB_NAME_MAX];      /* module name */
};

/*
 * Common variable record
 */
struct CommonRec {
    uint8_t type;                  /* must be 10 */
    uint8_t reclen;                /* equals 7+label length */
    uint16_t segment;             /* segment number */
    int32_t size;                  /* size of common variable */
    uint16_t align;               /* alignment (power of two) */
    char label[EXIM_LABEL_MAX]; /* zero terminated as in import */
};

/* Flags for ExportRec */
#define SYM_DATA	1
#define SYM_FUNCTION	2
#define SYM_GLOBAL	4
#define SYM_IMPORT	8

#endif                          /* RDOFF_H */
