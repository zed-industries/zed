// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT:
 * Copyright (c) 1997-2016, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef _DATEFORMATTEST_
#define _DATEFORMATTEST_

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/datefmt.h"
#include "unicode/smpdtfmt.h"
#include "caltztst.h"

/**
 * Performs many different tests for DateFormat and SimpleDateFormat
 **/
class DateFormatTest: public CalendarTimeZoneTest {
    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
public:
    /**
     * Verify that patterns have the correct values and could produce
     * the DateFormat instances that contain the correct localized patterns.
     */
    void TestPatterns();
    /**
     *  "Test written by Wally Wedel and emailed to me."
     *  Test handling of timezone offsets
     **/
    virtual void TestWallyWedel();
    /**
     * Test operator==
     */
    virtual void TestEquals();
    /**
     * Test the parsing of 2-digit years.
     */
    virtual void TestTwoDigitYearDSTParse();

public: // package
    // internal utility routine (generates escape sequences for characters)
    static UnicodeString& escape(UnicodeString& s);

public:
    /**
     * Verify that returned field position indices are correct.
     */
    void TestFieldPosition();

    void TestGeneral();

public: // package
    // internal utility function
    static void getFieldText(DateFormat* df, int32_t field, UDate date, UnicodeString& str);

public:
    /**
     * Verify that strings which contain incomplete specifications are parsed
     * correctly.  In some instances, this means not being parsed at all, and
     * returning an appropriate error.
     */
    virtual void TestPartialParse994();

public: // package
    // internal test subroutine, used by TestPartialParse994
    virtual void tryPat994(SimpleDateFormat* format, const char* pat, const char* str, UDate expected);

public:
    /**
     * Verify the behavior of patterns in which digits for different fields run together
     * without intervening separators.
     */
    virtual void TestRunTogetherPattern985();
    /**
     * Verify the behavior of patterns in which digits for different fields run together
     * without intervening separators.
     */
    virtual void TestRunTogetherPattern917();

public: // package
    // internal test subroutine, used by TestRunTogetherPattern917
    virtual void testIt917(SimpleDateFormat* fmt, UnicodeString& str, UDate expected);

public:
    /**
     * Verify the handling of Czech June and July, which have the unique attribute that
     * one is a proper prefix substring of the other.
     */
    virtual void TestCzechMonths459();
    /**
     * Test the handling of 'D' in patterns.
     */
    virtual void TestLetterDPattern212();
    /**
     * Test the day of year pattern.
     */
    virtual void TestDayOfYearPattern195();

public: // package
    // interl test subroutine, used by TestDayOfYearPattern195
    virtual void tryPattern(SimpleDateFormat& sdf, UDate d, const char* pattern, UDate expected);

public:
    /**
     * Test the handling of single quotes in patterns.
     */
    virtual void TestQuotePattern161();
    /**
     * Verify the correct behavior when handling invalid input strings.
     */
    virtual void TestBadInput135();

public:
    /**
     * Verify the correct behavior when parsing an array of inputs against an
     * array of patterns, with known results.  The results are encoded after
     * the input strings in each row.
     */
    virtual void TestBadInput135a();
    /**
     * Test the parsing of two-digit years.
     */
    virtual void TestTwoDigitYear();

public: // package
    // internal test subroutine, used by TestTwoDigitYear
    virtual void parse2DigitYear(DateFormat& fmt, const char* str, UDate expected);

public:
    /**
     * Test the formatting of time zones.
     */
    virtual void TestDateFormatZone061();
    /**
     * Further test the formatting of time zones.
     */
    virtual void TestDateFormatZone146();

    void TestTimeZoneStringsAPI();

    void TestGMTParsing();

public: // package
    /**
     * Test the formatting of dates in different locales.
     */
    virtual void TestLocaleDateFormat();

    virtual void TestFormattingLocaleTimeSeparator();

    virtual void TestDateFormatCalendar();

    virtual void TestSpaceParsing();

    void TestExactCountFormat();

    void TestWhiteSpaceParsing();

    void TestInvalidPattern();

    void TestGreekMay();

    void TestGenericTime();

    void TestGenericTimeZoneOrder();

    void Test6338();

    void Test6726();

    void Test6880();

    void TestISOEra();

    void TestFormalChineseDate();

    void TestStandAloneGMTParse();

    void TestParsePosition();

    void TestMonthPatterns();

    void TestContext();

    void TestNonGregoFmtParse();

    void TestFormatsWithNumberSystems();

public:
    /**
     * Test host-specific formatting.
     */
    void TestHost();

public:
    /**
     * Test patterns added in CLDR 1.4, CLDR 23
     */
    void TestEras();

    void TestNarrowNames();

    void TestShortDays();

    void TestStandAloneDays();

    void TestStandAloneMonths();

    void TestQuarters();

    void TestZTimeZoneParsing();

    void TestRelativeClone();

    void TestHostClone();

    void TestHebrewClone();

    void TestDateFormatSymbolsClone();

    void TestTimeZoneDisplayName();

    void TestTimeZoneInLocale();

    void TestRoundtripWithCalendar();

public:
    /***
     * Test Relative Dates
     */
     void TestRelative();
/*   void TestRelativeError();
     void TestRelativeOther();
*/

    void TestDotAndAtLeniency();

    void TestDateFormatLeniency();

    void TestParseMultiPatternMatch();

    void TestParseLeniencyAPIs();

    // test override NumberFormat
    void TestNumberFormatOverride();
    void TestCreateInstanceForSkeleton();
    void TestCreateInstanceForSkeletonDefault();
    void TestCreateInstanceForSkeletonWithCalendar();
    void TestDFSCreateForLocaleNonGregorianLocale();
    void TestDFSCreateForLocaleWithCalendarInLocale();
    void TestChangeCalendar();

    void TestPatternFromSkeleton();

    void TestAmPmMidnightNoon();
    void TestFlexibleDayPeriod();
    void TestDayPeriodWithLocales();
    void TestMinuteSecondFieldsInOddPlaces();
    void TestDayPeriodParsing();
    void TestParseRegression13744();
    void TestAdoptCalendarLeak();
    void Test20741_ABFields();
    void Test22023_UTCWithMinusZero();
    void TestNumericFieldStrictParse();
    void TestHourCycle();
    void TestHCInLocale();

private:
    UBool showParse(DateFormat &format, const UnicodeString &formattedString);

public:
    /**
     * Test parsing a number as a string
     */
    void TestNumberAsStringParsing();

 private:
      void TestRelative(int daysdelta,
                                  const Locale& loc,
                                  const char *expectChars);

 private:
    void expectParse(const char** data, int32_t data_length,
                     const Locale& locale);

    void expect(const char** data, int32_t data_length,
                const Locale& loc);

    void expectFormat(const char **data, int32_t data_length,
                      const Locale &locale);

};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif // _DATEFORMATTEST_
//eof
