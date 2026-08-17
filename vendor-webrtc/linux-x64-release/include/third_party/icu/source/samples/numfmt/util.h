/********************************************************************
 *   © 2016 and later: Unicode, Inc. and others.
 *   License & terms of use: http://www.unicode.org/copyright.html
 *************************************************************************
 *************************************************************************
 * COPYRIGHT:
 * Copyright (c) 1999-2002, International Business Machines Corporation and
 * others. All Rights Reserved.
 *************************************************************************/

#include "unicode/unistr.h"
#include "unicode/fmtable.h"

using namespace icu;

#ifndef UPRV_LENGTHOF 
#define UPRV_LENGTHOF(array) (int32_t)(sizeof(array)/sizeof((array)[0])) 
#endif 

// Verify that a UErrorCode is successful; exit(1) if not
void check(UErrorCode& status, const char* msg);

// Replace nonprintable characters with unicode escapes
UnicodeString escape(const UnicodeString &source);

// Print the given string to stdout
void uprintf(const UnicodeString &str);

// Create a display string for a formattable
UnicodeString formattableToString(const Formattable& f);
