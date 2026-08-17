/*
*******************************************************************************
*
*   © 2016 and later: Unicode, Inc. and others.
*   License & terms of use: http://www.unicode.org/copyright.html
*
*******************************************************************************
*******************************************************************************
*
*   Copyright (C) 2003, International Business Machines
*   Corporation and others.  All Rights Reserved.
*
*******************************************************************************
*   file name:  uit_len8.h
*   encoding:   UTF-8
*   tab size:   8 (not used)
*   indentation:4
*
*   created on: 2003feb10
*   created by: Markus W. Scherer
*
*   This file contains the declaration for a "lenient UTF-8" UCharIterator
*   as used in the uciter8 sample code.
*/

#ifndef __UIT_LEN8_H__
#define __UIT_LEN8_H__

#include "unicode/utypes.h"
#include "unicode/uiter.h"

U_CAPI void U_EXPORT2
uiter_setLenient8(UCharIterator *iter, const char *s, int32_t length);

#endif
