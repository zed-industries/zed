// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT:
 * Copyright (c) 1997-2012, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef _CALENDARREGRESSIONTEST_
#define _CALENDARREGRESSIONTEST_

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/calendar.h"
#include "unicode/gregocal.h"
#include "intltest.h"

/**
 * Performs regression test for Calendar
 **/
class CalendarRegressionTest: public IntlTest {

    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
public:
    void test4100311();
    void test4074758();
    void test4028518();
    void test4031502() ;
    void test4035301() ;
    void test4040996() ;
    void test4051765() ;
    void test4059654() ;
    void test4061476() ;
    void test4070502() ;
    void test4071197() ;
    void test4071385() ;
    void test4073929() ;
    void test4083167() ;
    void test4086724() ;
    void test4092362() ;
    void test4095407() ;
    void test4096231() ;
    void test4096539() ;
    void test41003112() ;
    void test4103271() ;
    void test4106136() ;
    void test4108764() ;
    void test4114578() ;
    void test4118384() ;
    void test4125881() ;
    void test4125892() ;
    void test4141665() ;
    void test4142933() ;
    void test4145158() ;
    void test4145983() ;
    void test4147269() ;

    void Test4149677() ;
    void Test4162587() ;
    void Test4165343() ;
    void Test4166109() ;
    void Test4167060() ;
    void Test4197699();
    void TestJ81();
    void TestJ438();
    void TestT5555();
    void TestT6745();
	void TestT8057();
    void TestLeapFieldDifference();
    void TestMalaysianInstance();
    void TestWeekShift();
    void TestTimeZoneTransitionAdd();
    void TestDeprecates();
    void TestT8596();
    void Test9019();
    void TestT9452();
    void TestT11632();
    void TestPersianCalOverflow();
    void TestIslamicCalOverflow();
    void TestWeekOfYear13548();
    void TestUTCWrongAMPM22023();
    void TestAsiaManilaAfterSetGregorianChange22043();

    void Test13745();
    void TestRespectUExtensionFw();

    void printdate(GregorianCalendar *cal, const char *string);
    void dowTest(UBool lenient) ;

    void VerifyGetStayInBound(double test_value);
    void VerifyNoAssertWithSetGregorianChange(const char* timezone);

    static UDate getAssociatedDate(UDate d, UErrorCode& status);
    static UDate makeDate(int32_t y, int32_t m = 0, int32_t d = 0, int32_t hr = 0, int32_t min = 0, int32_t sec = 0);

    static const UDate EARLIEST_SUPPORTED_MILLIS;
    static const UDate LATEST_SUPPORTED_MILLIS;
    static const char* FIELD_NAME[];

protected:
    UBool failure(UErrorCode status, const char* msg);
};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif // _CALENDARREGRESSIONTEST_
//eof
