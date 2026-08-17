// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
*******************************************************************************
*
*   Copyright (C) 2000-2010, International Business Machines
*   Corporation and others.  All Rights Reserved.
*
*******************************************************************************
*   file name:  makeconv.h
*   encoding:   UTF-8
*   tab size:   8 (not used)
*   indentation:4
*
*   created on: 2000nov01
*   created by: Markus W. Scherer
*/

#ifndef __MAKECONV_H__
#define __MAKECONV_H__

#include "unicode/utypes.h"
#include "ucnv_bld.h"
#include "unewdata.h"
#include "ucm.h"

/* exports from makeconv.c */
U_CFUNC UBool VERBOSE;
U_CFUNC UBool SMALL;
U_CFUNC UBool IGNORE_SISO_CHECK;

/* converter table type for writing */
enum {
    TABLE_NONE,
    TABLE_BASE,
    TABLE_EXT,
    TABLE_BASE_AND_EXT
};

/* abstract converter generator struct, C++ - style */
struct NewConverter;
typedef struct NewConverter NewConverter;

U_CDECL_BEGIN
struct NewConverter {
    void
    (* U_CALLCONV_FPTR close)(NewConverter *cnvData);

    /** is this byte sequence valid? */
    UBool
    (*U_CALLCONV_FPTR isValid)(NewConverter *cnvData,
               const uint8_t *bytes, int32_t length);

    UBool
    (*U_CALLCONV_FPTR addTable)(NewConverter *cnvData, UCMTable *table, UConverterStaticData *staticData);

    uint32_t
    (*U_CALLCONV_FPTR write)(NewConverter *cnvData, const UConverterStaticData *staticData,
             UNewDataMemory *pData, int32_t tableType);
};
U_CDECL_END
#endif /* __MAKECONV_H__ */
