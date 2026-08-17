// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
********************************************************************************
*   Copyright (C) 2005-2011, International Business Machines
*   Corporation and others.  All Rights Reserved.
********************************************************************************
*
* File WINNMTST.H
*
********************************************************************************
*/

#ifndef __WINNMTST
#define __WINNMTST

#include "unicode/utypes.h"

#if U_PLATFORM_USES_ONLY_WIN32_API

#if !UCONFIG_NO_FORMATTING

/**
 * \file 
 * \brief C++ API: Format dates using Windows API.
 */

class NumberFormatTest;

class Win32NumberTest
{
public:
    static void testLocales(NumberFormatTest *log);

private:
    Win32NumberTest();
};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif // U_PLATFORM_USES_ONLY_WIN32_API

#endif // __WINNMTST
