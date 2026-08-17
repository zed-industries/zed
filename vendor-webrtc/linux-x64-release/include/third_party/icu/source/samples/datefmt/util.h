/********************************************************************
 *   © 2016 and later: Unicode, Inc. and others.
 *   License & terms of use: http://www.unicode.org/copyright.html
 *************************************************************************
 *************************************************************************
 * COPYRIGHT:
 * Copyright (c) 1999-2003, International Business Machines Corporation and
 * others. All Rights Reserved.
 *************************************************************************/

#include "unicode/unistr.h"

using namespace icu;

// Verify that a UErrorCode is successful; exit(1) if not
void check(UErrorCode& status, const char* msg);

// Replace nonprintable characters with unicode escapes
UnicodeString escape(const UnicodeString &source);

// Print the given string to stdout
void uprintf(const UnicodeString &str);
