// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2015, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
 
#ifndef __CalendarLimitTest__
#define __CalendarLimitTest__
 
#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "caltztst.h"

/**
 * This test verifies the behavior of Calendar around the very earliest limits
 * which it can handle.  It also verifies the behavior for large values of millis.
 *
 * Bug ID 4033662.
 */
class CalendarLimitTest: public CalendarTimeZoneTest {
    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
public: // package
    //test routine used by TestCalendarLimit
    virtual void test(UDate millis, Calendar *cal, DateFormat *fmt);

    // bug 986c: deprecate nextDouble/previousDouble
    //static double nextDouble(double a);
    //static double previousDouble(double a);
    static UBool withinErr(double a, double b, double err);

public:
    // test behaviour and error reporting at boundaries of defined range
    virtual void TestCalendarExtremeLimit();

    void TestLimits();
    void TestLimitsThread(int32_t threadNumber);

private:
    /*
     * Test the functions getMaximum/getGeratestMinimum logically correct.
     * This method assumes day of week cycle is consistent.
     * @param cal The calendar instance to be tested.
     * @param leapMonth true if the calendar system has leap months
     */
    void doTheoreticalLimitsTest(Calendar& cal, UBool leapMonth);

    /*
     * Test the functions getXxxMinimum() and getXxxMaximum() by marching a
     * test calendar 'cal' through 'numberOfDays' sequential days starting
     * with 'startDate'.  For each date, read a field value along with its
     * reported actual minimum and actual maximum.  These values are
     * checked against one another as well as against getMinimum(),
     * getGreatestMinimum(), getLeastMaximum(), and getMaximum().  We
     * expect to see:
     *
     * 1. minimum <= actualMinimum <= greatestMinimum <=
     *    leastMaximum <= actualMaximum <= maximum
     *
     * 2. actualMinimum <= value <= actualMaximum
     *
     * Note: In addition to outright failures, this test reports some
     * results as warnings.  These are not generally of concern, but they
     * should be evaluated by a human.  To see these, run this test in
     * verbose mode.
     * @param cal the calendar to be tested
     * @param fieldsToTest an array of field values to be tested, e.g., new
     * int[] { UCAL_MONTH, UCAL_DAY_OF_MONTH }.  It only makes
     * sense to test the day fields; the time fields are not tested by this
     * method.  If null, then test all standard fields.
     * @param startDate the first date to test
     * @param testDuration if positive, the number of days to be tested.
     * If negative, the number of seconds to run the test.
     */
    void doLimitsTest(Calendar& cal, const int32_t* fieldsToTest, UDate startDate, int32_t testDuration);

    /**
     * doLimitsTest with default test duration and fields
     */
    void doLimitsTest(Calendar& cal, UDate startDate, int32_t endTime);

    UnicodeString& ymdToString(const Calendar& cal, UnicodeString& str);
};

#endif /* #if !UCONFIG_NO_FORMATTING */
 
#endif // __CalendarLimitTest__
