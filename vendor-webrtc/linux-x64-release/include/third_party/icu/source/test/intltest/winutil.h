// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
********************************************************************************
*   Copyright (C) 2005-2011, International Business Machines
*   Corporation and others.  All Rights Reserved.
********************************************************************************
*
* File WINUTIL.H
*
********************************************************************************
*/

#ifndef __WINUTIL
#define __WINUTIL

#include "unicode/utypes.h"

#if U_PLATFORM_HAS_WIN32_API

#if !UCONFIG_NO_FORMATTING

/**
 * \file 
 * \brief C++ API: Format dates using Windows API.
 */

class Win32Utilities
{
public:
    struct LCIDRecord
    {
        int32_t lcid;
        char *localeID;
    };

    static LCIDRecord *getLocales(int32_t &localeCount);
    static void freeLocales(LCIDRecord *records);

private:
    Win32Utilities();
};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif // U_PLATFORM_HAS_WIN32_API

#endif // __WINUTIL
