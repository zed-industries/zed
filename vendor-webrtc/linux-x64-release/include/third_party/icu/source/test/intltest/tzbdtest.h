// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2005, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef __TimeZoneBoundaryTest__
#define __TimeZoneBoundaryTest__
 
#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/timezone.h"
#include "unicode/simpletz.h"
#include "caltztst.h"

/**
 * A test which discovers the boundaries of DST programmatically and verifies
 * that they are correct.
 */
class TimeZoneBoundaryTest: public CalendarTimeZoneTest {
    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
public: // package

    TimeZoneBoundaryTest();

    /**
     * Date.toString().substring() Boundary Test
     * Look for a DST changeover to occur within 6 months of the given Date.
     * The initial Date.toString() should yield a string containing the
     * startMode as a SUBSTRING.  The boundary will be tested to be
     * at the expectedBoundary value.
     */

    /**
     * internal routines used by major test routines to perform subtests
     **/
    virtual void findDaylightBoundaryUsingDate(UDate d, const char* startMode, UDate expectedBoundary);
    virtual void findDaylightBoundaryUsingTimeZone(UDate d, UBool startsInDST, UDate expectedBoundary);
    virtual void findDaylightBoundaryUsingTimeZone(UDate d, UBool startsInDST, UDate expectedBoundary, TimeZone* tz);
 
private:
    //static UnicodeString* showDate(long l);
    UnicodeString showDate(UDate d);
    static UnicodeString showNN(int32_t n);
 
public: // package
    /**
     * Given a date, a TimeZone, and expected values for inDaylightTime,
     * useDaylightTime, zone and DST offset, verify that this is the case.
     */
    virtual void verifyDST(UDate d, TimeZone* time_zone, UBool expUseDaylightTime, UBool expInDaylightTime, UDate expZoneOffset, UDate expDSTOffset);
 
    /**
     * Test the behavior of SimpleTimeZone at the transition into and out of DST.
     * Use a binary search to find boundaries.
     */
    virtual void TestBoundaries();
 
    /**
     * internal subroutine used by TestNewRules
     **/
    virtual void testUsingBinarySearch(SimpleTimeZone* tz, UDate d, UDate expectedBoundary);
 
    /**
     * Test the handling of the "new" rules; that is, rules other than nth Day of week.
     */
    virtual void TestNewRules();
 
    /**
     * Find boundaries by stepping.
     */
    virtual void findBoundariesStepwise(int32_t year, UDate interval, TimeZone* z, int32_t expectedChanges);
 
    /**
     * Test the behavior of SimpleTimeZone at the transition into and out of DST.
     * Use a stepwise march to find boundaries.
     */ 
    virtual void TestStepwise();
    void verifyMapping(Calendar& cal, int year, int month, int dom, int hour,
                    double epochHours) ;
private:
    const UDate ONE_SECOND;
    const UDate ONE_MINUTE;
    const UDate ONE_HOUR;
    const UDate ONE_DAY;
    const UDate ONE_YEAR;
    const UDate SIX_MONTHS;
    static const int32_t MONTH_LENGTH[];
    static const UDate PST_1997_BEG;
    static const UDate PST_1997_END;
    static const UDate INTERVAL;
};

#endif /* #if !UCONFIG_NO_FORMATTING */
 
#endif // __TimeZoneBoundaryTest__
