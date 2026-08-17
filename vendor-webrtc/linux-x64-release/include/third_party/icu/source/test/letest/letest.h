// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
 *******************************************************************************
 *
 *   Copyright (C) 1999-2014, International Business Machines
 *   Corporation and others.  All Rights Reserved.
 *
 *******************************************************************************
 *   file name:  letest.h
 *
 *   created on: 11/06/2000
 *   created by: Eric R. Mader
 */

#ifndef __LETEST_H
#define __LETEST_H


#ifdef USING_ICULEHB
#include "layout/LETypes.h"
#else
#include "LETypes.h"
#endif
#include "unicode/ctest.h"

#include <stdlib.h>
#include <string.h>

U_NAMESPACE_USE

#define ARRAY_SIZE(array) (sizeof array / sizeof array[0])

#define ARRAY_COPY(dst, src, count) memcpy((void *) (dst), (void *) (src), (count) * sizeof (src)[0])

#define NEW_ARRAY(type,count) (type *) malloc((count) * sizeof(type))

#define DELETE_ARRAY(array) free((void *) (array))

#define GROW_ARRAY(array,newSize) realloc((void *) (array), (newSize) * sizeof (array)[0])

struct TestResult
{
    le_int32   glyphCount;
    LEGlyphID *glyphs;
    le_int32  *indices;
    float     *positions;
};

#ifndef __cplusplus
typedef struct TestResult TestResult;
#endif

U_CFUNC void addCTests(TestNode **root);

#endif
