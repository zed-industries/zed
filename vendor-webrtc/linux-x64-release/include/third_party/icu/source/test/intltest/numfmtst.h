// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/************************************************************************
 * COPYRIGHT:
 * Copyright (c) 1997-2016, International Business Machines Corporation
 * and others. All Rights Reserved.
 ************************************************************************/

#ifndef _NUMBERFORMATTEST_
#define _NUMBERFORMATTEST_

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/numfmt.h"
#include "unicode/decimfmt.h"
#include "caltztst.h"
#include "datadrivennumberformattestsuite.h"

/**
 * Expected field positions from field position iterator. Tests should
 * stack allocate an array of these making sure that the last element is
 * {0, -1, 0} (The sentinel element indicating end of iterator). Then test
 * should call verifyFieldPositionIterator() passing both this array of
 * expected results and the field position iterator from the format method.
 */
struct NumberFormatTest_Attributes {
    int32_t id;
    int32_t spos;
    int32_t epos;
};


/**
 * Header for the data-driven test, powered by numberformattestspecification.txt
 */
class NumberFormatDataDrivenTest : public DataDrivenNumberFormatTestSuite {
  public:
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
    void TestNumberFormatTestTuple();
    void TestDataDrivenICU4C();

  protected:
    UBool isFormatPass(
            const NumberFormatTestTuple &tuple,
            UnicodeString &appendErrorMessage,
            UErrorCode &status) override;
    UBool isToPatternPass(
            const NumberFormatTestTuple &tuple,
            UnicodeString &appendErrorMessage,
            UErrorCode &status) override;
    UBool isParsePass(
            const NumberFormatTestTuple &tuple,
            UnicodeString &appendErrorMessage,
            UErrorCode &status) override;
    UBool isParseCurrencyPass(
            const NumberFormatTestTuple &tuple,
            UnicodeString &appendErrorMessage,
            UErrorCode &status) override;
};

/**
 * Performs various in-depth test on NumberFormat
 **/
class NumberFormatTest: public CalendarTimeZoneTest {

    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
 public:

    /**
     * Test APIs (to increase code coverage)
     */
    void TestAPI();

    void TestCoverage();
    void TestLocalizedPatternSymbolCoverage();

    /**
     * Test the handling of quotes
     **/
    void TestQuotes();
    /**
     * Test patterns with exponential representation
     **/
    void TestExponential();
    /**
     * Test handling of patterns with currency symbols
     **/
    void TestCurrencySign();
    /**
     * Test different format patterns
     **/
    void TestPatterns();
    /**
     * API coverage for DigitList
     **/
    //void TestDigitList();

    void Test20186_SpacesAroundSemicolon();

    /**
     * Test localized currency patterns.
     */
    void TestCurrency();

    /**
     * Test the Currency object handling, new as of ICU 2.2.
     */
    void TestCurrencyObject();

    void TestCurrencyPatterns();

    /**
     * Do rudimentary testing of parsing.
     */
    void TestParse();
    /**
     * Test proper rounding by the format method.
     */
    void TestRounding487();

    // New tests for alphaWorks upgrade
    void TestExponent();

    void TestScientific();

    void TestScientific2();

    void TestScientificGrouping();

    void TestInt64();

    void TestSurrogateSupport();

    /**
     * Test the functioning of the secondary grouping value.
     */
    void TestSecondaryGrouping();

    void TestWhiteSpaceParsing();

    void TestComplexCurrency();

    void TestPad();
    void TestPatterns2();

    /**
     * Test currency registration.
     */
    void TestRegCurrency();

    void TestCurrencyNames();

    void TestCurrencyVariants();

    void TestCurrencyAmount();

    void TestCurrencyUnit();

    void TestSymbolsWithBadLocale();

    void TestAdoptDecimalFormatSymbols();

    void TestPerMill();

    void TestIllegalPatterns();

    void TestCases();

    void TestJB3832();

    void TestHost();

    void TestHostClone();

    void TestCurrencyFormat();

    /* Port of ICU4J rounding test. */
    void TestRounding();

    void TestRoundingPattern();

    void TestNonpositiveMultiplier();

    void TestNumberingSystems();


    void TestSpaceParsing();
    void TestMultiCurrencySign();
    void TestCurrencyFormatForMixParsing();
    void TestMismatchedCurrencyFormatFail();
    void TestDecimalFormatCurrencyParse();
    void TestCurrencyIsoPluralFormat();
    void TestCurrencyParsing();
    void TestParseCurrencyInUCurr();
    void TestFormatAttributes();
    void TestFieldPositionIterator();

    void TestLenientParse();

    void TestDecimal();
    void TestCurrencyFractionDigits();

    void TestExponentParse();
    void TestExplicitParents();
    void TestAvailableNumberingSystems();
    void Test9087();
    void TestFormatFastpaths();

    void TestFormattableSize();

    void TestUFormattable();

    void TestEnumSet();

    void TestSignificantDigits();
    void TestShowZero();

    void TestCompatibleCurrencies();
    void TestBug9936();
    void TestParseNegativeWithFaLocale();
    void TestParseNegativeWithAlternateMinusSign();

    void TestCustomCurrencySignAndSeparator();

    void TestParseSignsAndMarks();
    void Test10419RoundingWith0FractionDigits();
    void Test10468ApplyPattern();
    void TestRoundingScientific10542();
    void TestZeroScientific10547();
    void TestAccountingCurrency();
    void TestCurrencyFormatForMissingLocale();
    void TestEquality();

    void TestCurrencyUsage();

    void TestDoubleLimit11439();
    void TestFastPathConsistent11524();
    void TestGetAffixes();
    void TestToPatternScientific11648();
    void TestBenchmark();
    void TestCtorApplyPatternDifference();
    void TestFractionalDigitsForCurrency();
    void TestFormatCurrencyPlural();
    void Test11868();
    void Test11739_ParseLongCurrency();
    void Test13035_MultiCodePointPaddingInPattern();
    void Test13737_ParseScientificStrict();
    void Test10727_RoundingZero();
    void Test11376_getAndSetPositivePrefix();
    void Test11475_signRecognition();
    void Test11640_getAffixes();
    void Test11649_toPatternWithMultiCurrency();
    void Test13327_numberingSystemBufferOverflow();
    void Test13391_chakmaParsing();

    void Test11735_ExceptionIssue();
    void Test11035_FormatCurrencyAmount();
    void Test11318_DoubleConversion();
    void TestParsePercentRegression();
    void TestMultiplierWithScale();
    void TestFastFormatInt32();
    void Test11646_Equality();
    void TestParseNaN();
    void TestFormatFailIfMoreThanMaxDigits();
    void TestParseCaseSensitive();
    void TestParseNoExponent();
    void TestSignAlwaysShown();
    void TestMinimumGroupingDigits();
    void Test11897_LocalizedPatternSeparator();
    void Test13055_PercentageRounding();
    void Test11839();
    void Test10354();
    void Test11645_ApplyPatternEquality();
    void Test12567();
    void Test11626_CustomizeCurrencyPluralInfo();
    void Test20073_StrictPercentParseErrorIndex();
    void Test13056_GroupingSize();
    void Test11025_CurrencyPadding();
    void Test11648_ExpDecFormatMalPattern();
    void Test11649_DecFmtCurrencies();
    void Test13148_ParseGroupingSeparators();
    void Test12753_PatternDecimalPoint();
    void Test11647_PatternCurrencySymbols();
    void Test11913_BigDecimal();
    void Test11020_RoundingInScientificNotation();
    void Test11640_TripleCurrencySymbol();
    void Test13763_FieldPositionIteratorOffset();
    void Test13777_ParseLongNameNonCurrencyMode();
    void Test13804_EmptyStringsWhenParsing();
    void Test20037_ScientificIntegerOverflow();
    void Test13840_ParseLongStringCrash();
    void Test13850_EmptyStringCurrency();
    void Test20348_CurrencyPrefixOverride();
    void Test20956_MonetarySymbolGetters();
    void Test20358_GroupingInPattern();
    void Test13731_DefaultCurrency();
    void Test20499_CurrencyVisibleDigitsPlural();
    void Test13735_GroupingSizeGetter();
    void Test13734_StrictFlexibleWhitespace();
    void Test20961_CurrencyPluralPattern();
    void Test21134_ToNumberFormatter();
    void Test13733_StrictAndLenient();
    void Test20425_IntegerIncrement();
    void Test20425_FractionWithIntegerIncrement();
    void Test21232_ParseTimeout();
    void Test10997_FormatCurrency();
    void Test21556_CurrencyAsDecimal();
    void Test22088_Ethiopic();

 private:
    UBool testFormattableAsUFormattable(const char *file, int line, Formattable &f);

    void expectParseCurrency(const NumberFormat &fmt, const char16_t* currency, double amount, const char *text);

    static UBool equalValue(const Formattable& a, const Formattable& b);

    void expectPositions(FieldPositionIterator& iter, int32_t *values, int32_t tupleCount,
                         const UnicodeString& str);

    void expectPosition(FieldPosition& pos, int32_t id, int32_t start, int32_t limit,
                        const UnicodeString& str);

    void expect2(NumberFormat& fmt, const Formattable& n, const UnicodeString& str);

    void expect3(NumberFormat& fmt, const Formattable& n, const UnicodeString& str);

    void expect2(NumberFormat& fmt, const Formattable& n, const char* str) {
        expect2(fmt, n, UnicodeString(str, ""));
    }

    void expect2(NumberFormat* fmt, const Formattable& n, const UnicodeString& str, UErrorCode ec);

    void expect2(NumberFormat* fmt, const Formattable& n, const char* str, UErrorCode ec) {
        expect2(fmt, n, UnicodeString(str, ""), ec);
    }

    void expect(NumberFormat& fmt, const UnicodeString& str, const Formattable& n);

    void expect(NumberFormat& fmt, const char *str, const Formattable& n) {
        expect(fmt, UnicodeString(str, ""), n);
    }

    void expect(NumberFormat& fmt, const Formattable& n,
                const UnicodeString& exp, UBool rt=true);

    void expect(NumberFormat& fmt, const Formattable& n,
                const char *exp, UBool rt=true) {
        expect(fmt, n, UnicodeString(exp, ""), rt);
    }

    void expect(NumberFormat* fmt, const Formattable& n,
                const UnicodeString& exp, UBool rt, UErrorCode errorCode);

    void expect(NumberFormat* fmt, const Formattable& n,
                const char *exp, UBool rt, UErrorCode errorCode) {
        expect(fmt, n, UnicodeString(exp, ""), rt, errorCode);
    }

    void expect(NumberFormat* fmt, const Formattable& n,
                const UnicodeString& exp, UErrorCode errorCode) {
        expect(fmt, n, exp, true, errorCode);
    }

    void expect(NumberFormat* fmt, const Formattable& n,
                const char *exp, UErrorCode errorCode) {
        expect(fmt, n, UnicodeString(exp, ""), true, errorCode);
    }

    void expectCurrency(NumberFormat& nf, const Locale& locale,
                        double value, const UnicodeString& string);

    void expectPad(DecimalFormat& fmt, const UnicodeString& pat,
                   int32_t pos, int32_t width, char16_t pad);

    void expectPad(DecimalFormat& fmt, const char *pat,
                   int32_t pos, int32_t width, char16_t pad) {
        expectPad(fmt, UnicodeString(pat, ""), pos, width, pad);
    }

    void expectPad(DecimalFormat& fmt, const UnicodeString& pat,
                   int32_t pos, int32_t width, const UnicodeString& pad);

    void expectPad(DecimalFormat& fmt, const char *pat,
                   int32_t pos, int32_t width, const UnicodeString& pad) {
        expectPad(fmt, UnicodeString(pat, ""), pos, width, pad);
    }

    void expectPat(DecimalFormat& fmt, const UnicodeString& exp);

    void expectPat(DecimalFormat& fmt, const char *exp) {
        expectPat(fmt, UnicodeString(exp, ""));
    }

    void expectPad(DecimalFormat& fmt, const UnicodeString& pat,
                   int32_t pos);

    void expectPad(DecimalFormat& fmt, const char *pat,
                   int32_t pos) {
        expectPad(fmt, pat, pos, 0, (char16_t)0);
    }

    void expect_rbnf(NumberFormat& fmt, const UnicodeString& str, const Formattable& n);

    void expect_rbnf(NumberFormat& fmt, const Formattable& n,
                const UnicodeString& exp, UBool rt=true);

    // internal utility routine
    static UnicodeString& escape(UnicodeString& s);

    enum { ILLEGAL = -1 };

    // internal subtest used by TestRounding487
    void roundingTest(NumberFormat& nf, double x, int32_t maxFractionDigits, const char* expected);

    // internal rounding checking for TestRounding
    void checkRounding(DecimalFormat* df, double base, int iterations, double increment);

    double checkRound(DecimalFormat* df, double iValue, double lastParsed);

    void verifyRounding(
        DecimalFormat& format,
        const double *values,
        const char * const *expected,
        const DecimalFormat::ERoundingMode *roundingModes,
        const char * const *descriptions,
        int32_t valueSize,
        int32_t roundingModeSize);

    void verifyFieldPositionIterator(
            NumberFormatTest_Attributes *expected,
            FieldPositionIterator &iter);

};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif // _NUMBERFORMATTEST_
