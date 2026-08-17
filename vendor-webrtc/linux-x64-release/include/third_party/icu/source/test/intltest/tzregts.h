// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1998-2005, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef _TIMEZONEREGRESSIONTEST_
#define _TIMEZONEREGRESSIONTEST_
 
#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/timezone.h"
#include "unicode/gregocal.h"
#include "unicode/simpletz.h"
#include "intltest.h"



/** 
 * Performs regression test for Calendar
 **/
class TimeZoneRegressionTest: public IntlTest {    
    
    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
public:
    
    void Test4052967();
    void Test4073209();
    void Test4073215();
    void Test4084933();
    void Test4096952();
    void Test4109314();
    void Test4126678();
    void Test4151406();
    void Test4151429();
    void Test4154537();
    void Test4154542();
    void Test4154650();
    void Test4154525();
    void Test4162593();
    void Test4176686();
    void TestJ186();
    void TestJ449();
    void TestJDK12API();
    void Test4184229();
    UBool checkCalendar314(GregorianCalendar *testCal, TimeZone *testTZ);
    void TestNegativeDaylightSaving();


protected:
    UDate findTransitionBinary(const SimpleTimeZone& tz, UDate min, UDate max);
    UDate findTransitionStepwise(const SimpleTimeZone& tz, UDate min, UDate max);
    UBool failure(UErrorCode status, const char* msg);
};

#endif /* #if !UCONFIG_NO_FORMATTING */
 
#endif // _CALENDARREGRESSIONTEST_
//eof
