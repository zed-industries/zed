// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/***********************************************************************
 * Copyright (c) 1997-2016, International Business Machines Corporation
 * and others. All Rights Reserved.
 ***********************************************************************/

#ifndef __CalendarTest__
#define __CalendarTest__

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/calendar.h"
#include "unicode/smpdtfmt.h"
#include "caltztst.h"

class CalendarTest: public CalendarTimeZoneTest {
public:
    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
public:
    /**
     * This test confirms the correct behavior of add when incrementing
     * through subsequent days.
     */
    virtual void TestRog();
    /**
     * Test the handling of the day of the week, checking for correctness and
     * for correct minimum and maximum values.
     */
    virtual void TestDOW943();
    /**
     * test subroutine use by TestDOW943
     */
    void dowTest(UBool lenient);
    /**
     * Confirm that cloned Calendar objects do not inadvertently share substructures.
     */
    virtual void TestClonesUnique908();
    /**
     * Confirm that the Gregorian cutoff value works as advertised.
     */
    virtual void TestGregorianChange768();
    /**
     * Confirm the functioning of the field disambiguation algorithm.
     */
    virtual void TestDisambiguation765();
    /**
     * Test various API methods for API completeness.
     */
    virtual void TestGenericAPI(); // New to C++ -- needs to be back ported to Java

    virtual void TestWOY();

    virtual void TestDebug();

    virtual void TestClearMonth();

public: // package
    /**
     * test subroutine used by TestDisambiguation765
     */
    virtual void verify765(const UnicodeString& msg, Calendar* c, int32_t year, int32_t month, int32_t day);
    /**
     * test subroutine used by TestDisambiguation765
     */
    virtual void verify765(const UnicodeString& msg/*, IllegalArgumentException e*/, UErrorCode status);

public:
    /**
     * Confirm that the offset between local time and GMT behaves as expected.
     */
    virtual void TestGMTvsLocal4064654();

public: // package
    /**
     * test subroutine used by TestGMTvsLocal4064654
     */
    virtual void test4064654(int32_t yr, int32_t mo, int32_t dt, int32_t hr, int32_t mn, int32_t sc);

public:
    /**
     * The operations of adding and setting should not exhibit pathological
     * dependence on the order of operations.  This test checks for this.
     */
    virtual void TestAddSetOrder621();
    /**
     * Confirm that adding to various fields works.
     */
    virtual void TestAdd520();
    /**
     * Execute and test adding and rolling in GregorianCalendar extensively.
     */
    virtual void TestAddRollExtensive();

public: // package
    // internal utility routine for checking date
    virtual void check520(Calendar* c,
                            int32_t y, int32_t m, int32_t d,
                            int32_t hr, int32_t min, int32_t sec,
                            int32_t ms, UCalendarDateFields field);

    virtual void check520(Calendar* c,
                            int32_t y, int32_t m, int32_t d);

public:
    /**
     * Test that setting of fields works.  In particular, make sure that all instances
     * of GregorianCalendar don't share a static instance of the fields array.
     */
    virtual void TestFieldSet4781();
/*    virtual void TestSerialize337();

public: // package
    static UnicodeString& PREFIX;
    static UnicodeString& POSTFIX;
    static UnicodeString& FILENAME;
*/
public:
    /**
     * Verify that the seconds of a Calendar can be zeroed out through the
     * expected sequence of operations.
     */
    virtual void TestSecondsZero121();
    /**
     * Verify that a specific sequence of adding and setting works as expected;
     * it should not vary depending on when and whether the get method is
     * called.
     */
    virtual void TestAddSetGet0610();

public: // package
    // internal routine for checking date
    static UnicodeString value(Calendar* calendar);

public:
    /**
     * Verify that various fields on a known date are set correctly.
     */
    virtual void TestFields060();

public: // package
    static int32_t EXPECTED_FIELDS[];
    static const int32_t EXPECTED_FIELDS_length;

public:
    /**
     * Verify that various fields on a known date are set correctly.  In this
     * case, the start of the epoch (January 1 1970).
     */
    virtual void TestEpochStartFields();

public: // package
    static int32_t EPOCH_FIELDS[];

public:
    /**
     * Test that the days of the week progress properly when add is called repeatedly
     * for increments of 24 days.
     */
    virtual void TestDOWProgression();
    /**
     * Test newly added fields - DOW_LOCAL and YEAR_WOY
     */
    virtual void TestDOW_LOCALandYEAR_WOY();
    // test subroutine used by TestDOW_LOCALandYEAR_WOY
    virtual void doYEAR_WOYLoop(Calendar *cal,
        SimpleDateFormat *sdf, int32_t times, UErrorCode& status);
    // test subroutine used by TestDOW_LOCALandYEAR_WOY
    virtual void loop_addroll(Calendar *cal, /*SimpleDateFormat *sdf, */
        int times, UCalendarDateFields field, UCalendarDateFields field2,
        UErrorCode& errorCode);

    void TestYWOY();
    void TestJD();

    void yearAddTest(Calendar& cal, UErrorCode& status);

public: // package
    // test subroutine use by TestDOWProgression
    virtual void marchByDelta(Calendar* cal, int32_t delta);

 public:
    // for other tests' use
    static UnicodeString fieldName(UCalendarDateFields f);
    static UnicodeString calToStr(const Calendar & cal);

    // List of non-installed locales with interesting calendars

    /**
     * @return the count of 'other' locales to test
     */
    static int32_t testLocaleCount();

    /**
     * @param i index of 'other' locale to return
     * @return locale ID
     */
    static const char* testLocaleID(int32_t i);

    /**
     * Clone the specified calendar, and determine its earliest supported date
     * by setting the extended year to the minimum value.
     * @param cal Calendar (will be cloned)
     * @param isGregorian output: returns 'true' if the calendar's class is GregorianCalendar
     * @param status error code
     */
    static UDate minDateOfCalendar(const Calendar& cal, UBool &isGregorian, UErrorCode& status);

    /**
     * Construct a calendar of the specified locale, and determine its earliest supported date
     * by setting the extended year to the minimum value.
     * @param locale locale of calendar to check
     * @param isGregorian output: returns 'true' if the calendar's class is GregorianCalendar
     * @param status error code
     */
    static UDate minDateOfCalendar(const Locale& locale, UBool &isGregorian, UErrorCode& status);

  // internal - for other test use
 public:
    void Test6703();
    void Test3785();
    void Test1624();
    void TestIslamicUmAlQura();
    void TestIslamicTabularDates();

    /**
     * Test the time stamp array recalculation during heavy Calendar usage
     */
    void TestTimeStamp();
    /**
     * Test the ISO8601 calendar type
     */
    void TestISO8601();

    /**
     * Test cases for [set|get][Repeated|Skipped]WallTimeOption
     */
    void TestAmbiguousWallTimeAPIs();
    void TestRepeatedWallTime();
    void TestSkippedWallTime();

    void TestCloneLocale();

    void TestTimeZoneInLocale();

    void TestHebrewMonthValidation();

    /*
     * utility methods for TestIslamicUmAlQura
     */
    void setAndTestCalendar(Calendar* cal, int32_t initMonth, int32_t initDay, int32_t initYear, UErrorCode& status);
    void setAndTestWholeYear(Calendar* cal, int32_t startYear, UErrorCode& status);

    void TestWeekData();

    void TestAddAcrossZoneTransition();

    void TestChineseCalendarMapping();

    void TestBasicConversionGregorian();
    void TestBasicConversionISO8601();
    void TestBasicConversionJapanese();
    void TestBasicConversionBuddhist();
    void TestBasicConversionTaiwan();
    void TestBasicConversionPersian();
    void TestBasicConversionIslamic();
    void TestBasicConversionIslamicTBLA();
    void TestBasicConversionIslamicCivil();
    void TestBasicConversionIslamicRGSA();
    void TestBasicConversionIslamicUmalqura();
    void TestBasicConversionHebrew();
    void TestBasicConversionChinese();
    void TestBasicConversionDangi();
    void TestBasicConversionIndian();
    void TestBasicConversionCoptic();
    void TestBasicConversionEthiopic();
    void TestBasicConversionEthiopicAmeteAlem();

    void AsssertCalendarFieldValue(
        Calendar* cal, double time, const char* type,
        int32_t era, int32_t year, int32_t month, int32_t week_of_year,
        int32_t week_of_month, int32_t date, int32_t day_of_year, int32_t day_of_week,
        int32_t day_of_week_in_month, int32_t am_pm, int32_t hour, int32_t hour_of_day,
        int32_t minute, int32_t second, int32_t millisecond, int32_t zone_offset,
        int32_t dst_offset, int32_t year_woy, int32_t dow_local, int32_t extended_year,
        int32_t julian_day, int32_t milliseconds_in_day, int32_t is_leap_month);

    void TestChineseCalendarMonthInSpecialYear();
    void TestGregorianCalendarInTemporalLeapYear();
    void TestChineseCalendarInTemporalLeapYear();
    void TestDangiCalendarInTemporalLeapYear();
    void TestHebrewCalendarInTemporalLeapYear();
    void TestIslamicCalendarInTemporalLeapYear();
    void TestIslamicCivilCalendarInTemporalLeapYear();
    void TestIslamicUmalquraCalendarInTemporalLeapYear();
    void TestIslamicRGSACalendarInTemporalLeapYear();
    void TestIslamicTBLACalendarInTemporalLeapYear();
    void TestPersianCalendarInTemporalLeapYear();
    void TestIndianCalendarInTemporalLeapYear();
    void TestTaiwanCalendarInTemporalLeapYear();
    void TestJapaneseCalendarInTemporalLeapYear();
    void TestBuddhistCalendarInTemporalLeapYear();
    void TestCopticCalendarInTemporalLeapYear();
    void TestEthiopicCalendarInTemporalLeapYear();
    void TestEthiopicAmeteAlemCalendarInTemporalLeapYear();

    void TestChineseCalendarGetTemporalMonthCode();
    void TestDangiCalendarGetTemporalMonthCode();
    void TestHebrewCalendarGetTemporalMonthCode();
    void TestCopticCalendarGetTemporalMonthCode();
    void TestEthiopicCalendarGetTemporalMonthCode();
    void TestEthiopicAmeteAlemCalendarGetTemporalMonthCode();

    void TestGregorianCalendarSetTemporalMonthCode();
    void TestChineseCalendarSetTemporalMonthCode();
    void TestHebrewCalendarSetTemporalMonthCode();
    void TestCopticCalendarSetTemporalMonthCode();
    void TestEthiopicCalendarSetTemporalMonthCode();

    void TestMostCalendarsOrdinalMonthSet();
    void TestChineseCalendarOrdinalMonthSet();
    void TestDangiCalendarOrdinalMonthSet();
    void TestHebrewCalendarOrdinalMonthSet();

    void TestCalendarAddOrdinalMonth();
    void TestCalendarRollOrdinalMonth();
    void TestLimitsOrdinalMonth();
    void TestActualLimitsOrdinalMonth();
    void TestDangiOverflowIsLeapMonthBetween22507();

    void TestFWWithISO8601();
    void TestRollWeekOfYear();

    void RunChineseCalendarInTemporalLeapYearTest(Calendar* cal);
    void RunIslamicCalendarInTemporalLeapYearTest(Calendar* cal);
    void Run366DaysIsLeapYearCalendarInTemporalLeapYearTest(Calendar* cal);
    void RunChineseCalendarGetTemporalMonthCode(Calendar* cal);
    void RunCECalendarGetTemporalMonthCode(Calendar* cal);
};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif // __CalendarTest__
