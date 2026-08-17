// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2013, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef _INTLTESTDATEFORMATSYMBOLS
#define _INTLTESTDATEFORMATSYMBOLS

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "intltest.h"

/**
 * Tests for DateFormatSymbols
 **/
class IntlTestDateFormatSymbols: public IntlTest {
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

private:
    /**
     * Test the API of DateFormatSymbols; primarily a simple get/set set.
     */
    void TestSymbols(/* char *par */);
    /**
     * Test getMonths.
     */
    void TestGetMonths();
    void TestGetMonths2();

    void TestGetWeekdays2();
    void TestGetEraNames();
    void TestGetSetSpecificItems();

    UBool UnicodeStringsArePrefixes(int32_t count, int32_t prefixLen, const UnicodeString *prefixArray, const UnicodeString *baseArray);
};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
