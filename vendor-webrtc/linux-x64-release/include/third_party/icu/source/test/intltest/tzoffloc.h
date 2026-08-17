// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
*******************************************************************************
* Copyright (C) 2007, International Business Machines Corporation and         *
* others. All Rights Reserved.                                                *
*******************************************************************************
*/

#ifndef _TIMEZONEOFFSETLOCALTEST_
#define _TIMEZONEOFFSETLOCALTEST_

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "intltest.h"

class TimeZoneOffsetLocalTest : public IntlTest {
    // IntlTest override
    void runIndexedTest(int32_t index, UBool exec, const char*& name, char* par) override;

    void TestGetOffsetAroundTransition();
};

#endif /* #if !UCONFIG_NO_FORMATTING */
 
#endif // _TIMEZONEOFFSETLOCALTEST_
