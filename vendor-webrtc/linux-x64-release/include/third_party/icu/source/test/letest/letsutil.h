// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
 *******************************************************************************
 *
 *   Copyright (C) 1999-2014, International Business Machines
 *   Corporation and others.  All Rights Reserved.
 *
 *******************************************************************************
 *   file name:  letsutil.h
 *
 *   created on: 04/25/2006
 *   created by: Eric R. Mader
 */

#ifndef __LETSUTIL_H
#define __LETSUTIL_H

#include "unicode/utypes.h"
#include "unicode/unistr.h"
#include "unicode/ubidi.h"

#include "layout/LETypes.h"
#include "layout/LEScripts.h"
#include "layout/LayoutEngine.h"
#include "layout/LELanguages.h"

#ifndef USING_ICULEHB
#include "OpenTypeLayoutEngine.h"
#endif

#include "letest.h"

char *getCString(const UnicodeString *uString);
char *getCString(const LEUnicode16 *uChars);
char *getUTF8String(const UnicodeString *uString);
void freeCString(char *cString);
le_bool getRTL(const UnicodeString &text);
le_int32 getLanguageCode(const char *lang);

#endif
