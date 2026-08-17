// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
*******************************************************************************
*
*   Copyright (C) 2000-2008, International Business Machines
*   Corporation and others.  All Rights Reserved.
*
*******************************************************************************
*   file name:  genmbcs.h
*   encoding:   UTF-8
*   tab size:   8 (not used)
*   indentation:4
*
*   created on: 2000jul10
*   created by: Markus W. Scherer
*/

#ifndef __GENMBCS_H__
#define __GENMBCS_H__

#include "makeconv.h"

enum {
    /*
     * TODO: Consider using ucnvmbcs.h constants.
     * However, not all values need to be exactly the same, for example
     * the xxx_UTF8_MAX values may be different. (Especially SBCS_UTF8_MAX
     * may be higher in makeconv than in the runtime code because that
     * affects only a small number of .cnv files [if any] but all
     * runtime UConverterSharedData objects.
     */
    MBCS_STAGE_2_SHIFT=4,
    MBCS_STAGE_2_BLOCK_SIZE=0x40,       /* =64=1<<6 for 6 bits in stage 2 */
    MBCS_STAGE_2_BLOCK_SIZE_SHIFT=6,    /* log2(MBCS_STAGE_2_BLOCK_SIZE) */
    MBCS_STAGE_2_BLOCK_MASK=0x3f,       /* for after shifting by MBCS_STAGE_2_SHIFT */
    MBCS_STAGE_1_SHIFT=10,
    MBCS_STAGE_1_BMP_SIZE=0x40, /* 0x10000>>MBCS_STAGE_1_SHIFT, or 16 for one entry per 1k code points on the BMP */
    MBCS_STAGE_1_SIZE=0x440,    /* 0x110000>>MBCS_STAGE_1_SHIFT, or 17*64 for one entry per 1k code points */
    MBCS_STAGE_2_SIZE=0xfbc0,   /* 0x10000-MBCS_STAGE_1_SIZE: stages 1 & 2 share a 16-bit-indexed array */
    MBCS_MAX_STAGE_2_TOP=MBCS_STAGE_2_SIZE,
    MBCS_STAGE_2_MAX_BLOCKS=MBCS_STAGE_2_SIZE>>MBCS_STAGE_2_BLOCK_SIZE_SHIFT,

    MBCS_STAGE_2_ALL_UNASSIGNED_INDEX=0, /* stage 1 entry for the all-unassigned stage 2 block */
    MBCS_STAGE_2_FIRST_ASSIGNED=MBCS_STAGE_2_BLOCK_SIZE, /* start of the first stage 2 block after the all-unassigned one */

    MBCS_STAGE_3_BLOCK_SIZE=16,         /* =16=1<<4 for 4 bits in stage 3 */
    MBCS_STAGE_3_BLOCK_MASK=0xf,
    MBCS_STAGE_3_FIRST_ASSIGNED=MBCS_STAGE_3_BLOCK_SIZE, /* start of the first stage 3 block after the all-unassigned one */

    MBCS_STAGE_3_GRANULARITY=16,        /* =1<<4: MBCS stage 2 indexes are shifted left 4 */
    MBCS_STAGE_3_SBCS_SIZE=0x10000,     /* max 64k mappings for SBCS */
    MBCS_STAGE_3_MBCS_SIZE=0x10000*MBCS_STAGE_3_GRANULARITY, /* max mappings for MBCS */

    /*
     * SBCS_UTF8_MAX: Maximum code point with UTF-8-friendly SBCS data structures.
     * Possible values are 0x01ff..0xffff, in steps of 0x100.
     *
     * Unlike for MBCS, this constant only affects the stage 3 block allocation size;
     * there is no additional stage 1/2 table stored in the .cnv file.
     * The max value should be at least 0x7ff to cover 2-byte UTF-8.
     * 0xfff also covers a number other small scripts which have legacy charsets
     * (like Thai).
     * Higher values up to 0x1fff are harmless and potentially useful because
     * that covers small-script blocks which usually have either dense mappings
     * or no mappings at all.
     * Starting at U+2000, there are mostly symbols and format characters
     * with a low density of SBCS mappings, which would result in more wasted
     * stage 3 entries with the larger block size.
     */
    SBCS_UTF8_MAX=0x1fff,

    /*
     * MBCS_UTF8_MAX: Maximum code point with UTF-8-friendly MBCS data structures.
     * Possible values are 0x01ff..0xffff, in steps of 0x100.
     *
     * Note that with 0xffff, MBCSAddFromUnicode() may overflow the additional UTF-8 stage table
     * with extreme input data. The function checks for this overflow.
     *
     * 0xd7ff is chosen for the majority of common characters including Unihan and Hangul.
     * At U+d800 there are mostly surrogates, private use codes, compatibility characters, etc.
     * Larger values cause slightly larger MBCS .cnv files.
     */
    MBCS_UTF8_MAX=0xd7ff,
    MBCS_UTF8_LIMIT=MBCS_UTF8_MAX+1,    /* =0xd800 */

    MBCS_UTF8_STAGE_SHIFT=6,
    MBCS_UTF8_STAGE_3_BLOCK_SIZE=0x40,  /* =64=1<<6 for 6 bits from last trail byte */
    MBCS_UTF8_STAGE_3_BLOCK_MASK=0x3f,

    /* size of the single-stage table for up to U+d7ff (used instead of stage1/2) */
    MBCS_UTF8_STAGE_SIZE=MBCS_UTF8_LIMIT>>MBCS_UTF8_STAGE_SHIFT, /* =0x360 */

    MBCS_FROM_U_EXT_FLAG=0x10,          /* UCMapping.f bit for base table mappings that fit into the base toU table */
    MBCS_FROM_U_EXT_MASK=0x0f,          /* but need to go into the extension fromU table */

    /* =4 number of regular stage 3 blocks for final UTF-8 trail byte */
    MBCS_UTF8_STAGE_3_BLOCKS=MBCS_UTF8_STAGE_3_BLOCK_SIZE/MBCS_STAGE_3_BLOCK_SIZE,

    MBCS_MAX_FALLBACK_COUNT=8192
};

U_CFUNC NewConverter *
MBCSOpen(UCMFile *ucm);

struct MBCSData;
typedef struct MBCSData MBCSData;

/*
 * Get a dummy MBCSData for use with MBCSOkForBaseFromUnicode()
 * for creating an extension-only file.
 * Assume maxCharLength>1.
 */
U_CFUNC const MBCSData *
MBCSGetDummy(void);

/* Test if a 1:1 mapping fits into the MBCS base table's fromUnicode structure. */
U_CFUNC UBool
MBCSOkForBaseFromUnicode(const MBCSData *mbcsData,
                         const uint8_t *bytes, int32_t length,
                         UChar32 c, int8_t flag);

U_CFUNC NewConverter *
CnvExtOpen(UCMFile *ucm);

#endif /* __GENMBCS_H__ */
