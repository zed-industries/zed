// © 2017 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING
#pragma once

#include "formatted_string_builder.h"
#include "intltest.h"
#include "itformat.h"
#include "number_affixutils.h"
#include "string_segment.h"
#include "numrange_impl.h"
#include "unicode/locid.h"
#include "unicode/numberformatter.h"
#include "unicode/numberrangeformatter.h"

// ICU-20241 Solaris #defines ESP in sys/regset.h
#ifdef ESP
#   undef ESP
#endif

using namespace icu::number;
using namespace icu::number::impl;
using namespace icu::numparse;
using namespace icu::numparse::impl;

////////////////////////////////////////////////////////////////////////////////////////
// INSTRUCTIONS:                                                                      //
// To add new NumberFormat unit test classes, create a new class like the ones below, //
// and then add it as a switch statement in NumberTest at the bottom of this file.    /////////
// To add new methods to existing unit test classes, add the method to the class declaration //
// below, and also add it to the class's implementation of runIndexedTest().                 //
///////////////////////////////////////////////////////////////////////////////////////////////

class AffixUtilsTest : public IntlTest {
  public:
    void testEscape();
    void testUnescape();
    void testContainsReplaceType();
    void testInvalid();
    void testUnescapeWithSymbolProvider();

    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override;

  private:
    UnicodeString unescapeWithDefaults(const SymbolProvider &defaultProvider, UnicodeString input,
                                       UErrorCode &status);
};

class NumberFormatterApiTest : public IntlTestWithFieldPosition {
  public:
    NumberFormatterApiTest();
    NumberFormatterApiTest(UErrorCode &status);

    void notationSimple();
    void notationScientific();
    void notationCompact();
    void unitMeasure();
    void unitCompoundMeasure();
    void unitArbitraryMeasureUnits();
    void unitSkeletons();
    void unitUsage();
    void unitUsageErrorCodes();
    void unitUsageSkeletons();
    void unitCurrency();
    void unitInflections();
    void unitNounClass();
    void unitGender();
    void unitNotConvertible();
    void unitPercent();
    void unitLocaleTags();
    void percentParity();
    void roundingFraction();
    void roundingFigures();
    void roundingFractionFigures();
    void roundingOther();
    void roundingIncrementRegressionTest();
    void roundingPriorityCoverageTest();
    void grouping();
    void padding();
    void integerWidth();
    void symbols();
    // TODO: Add this method if currency symbols override support is added.
    //void symbolsOverride();
    void sign();
    void signNearZero();
    void signCoverage();
    void decimal();
    void scale();
    void locale();
    void skeletonUserGuideExamples();
    void formatTypes();
    void fieldPositionLogic();
    void fieldPositionCoverage();
    void toFormat();
    void errors();
    void validRanges();
    void copyMove();
    void localPointerCAPI();
    void toObject();
    void toDecimalNumber();
    void microPropsInternals();
    void formatUnitsAliases();
    void testIssue22378();
    
    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override;

  private:
    CurrencyUnit USD;
    CurrencyUnit GBP;
    CurrencyUnit CZK;
    CurrencyUnit CAD;
    CurrencyUnit ESP;
    CurrencyUnit PTE;
    CurrencyUnit RON;
    CurrencyUnit TWD;
    CurrencyUnit TRY;
    CurrencyUnit CNY;

    MeasureUnit METER;
    MeasureUnit METER_PER_SECOND;
    MeasureUnit DAY;
    MeasureUnit SQUARE_METER;
    MeasureUnit FAHRENHEIT;
    MeasureUnit SECOND;
    MeasureUnit POUND;
    MeasureUnit POUND_FORCE;
    MeasureUnit SQUARE_MILE;
    MeasureUnit SQUARE_INCH;
    MeasureUnit JOULE;
    MeasureUnit FURLONG;
    MeasureUnit KELVIN;

    NumberingSystem MATHSANB;
    NumberingSystem LATN;

    DecimalFormatSymbols FRENCH_SYMBOLS;
    DecimalFormatSymbols SWISS_SYMBOLS;
    DecimalFormatSymbols MYANMAR_SYMBOLS;

    /**
     * skeleton is the full length skeleton, which must round-trip.
     *
     * conciseSkeleton should be the shortest available skeleton.
     * The concise skeleton can be read but not printed.
     */
    void assertFormatDescending(
      const char16_t* message,
      const char16_t* skeleton,
      const char16_t* conciseSkeleton,
      const UnlocalizedNumberFormatter& f,
      Locale locale,
      ...);

    /** See notes above regarding skeleton vs conciseSkeleton */
    void assertFormatDescendingBig(
      const char16_t* message,
      const char16_t* skeleton,
      const char16_t* conciseSkeleton,
      const UnlocalizedNumberFormatter& f,
      Locale locale,
      ...);

    /** See notes above regarding skeleton vs conciseSkeleton */
    FormattedNumber assertFormatSingle(
      const char16_t* message,
      const char16_t* skeleton,
      const char16_t* conciseSkeleton,
      const UnlocalizedNumberFormatter& f,
      Locale locale,
      double input,
      const UnicodeString& expected);

    void assertUndefinedSkeleton(const UnlocalizedNumberFormatter& f);

    void assertNumberFieldPositions(
      const char16_t* message,
      const FormattedNumber& formattedNumber,
      const UFieldPosition* expectedFieldPositions,
      int32_t length);

    struct UnitInflectionTestCase {
        const char *unitIdentifier;
        const char *locale;
        const char *unitDisplayCase;
        double value;
        const char16_t *expected;
    };

    void runUnitInflectionsTestCases(UnlocalizedNumberFormatter unf,
                                     UnicodeString skeleton,
                                     const UnitInflectionTestCase *cases,
                                     int32_t numCases,
                                     IcuTestErrorCode &status);
};

class DecimalQuantityTest : public IntlTest {
  public:
    void testDecimalQuantityBehaviorStandalone();
    void testSwitchStorage();
    void testCopyMove();
    void testAppend();
    void testConvertToAccurateDouble();
    void testUseApproximateDoubleWhenAble();
    void testHardDoubleConversion();
    void testFitsInLong();
    void testToDouble();
    void testMaxDigits();
    void testNickelRounding();
    void testScientificAndCompactSuppressedExponent();
    void testSuppressedExponentUnchangedByInitialScaling();
    void testDecimalQuantityParseFormatRoundTrip();

    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override;

  private:
    void assertDoubleEquals(UnicodeString message, double a, double b);
    void assertHealth(const DecimalQuantity &fq);
    void assertToStringAndHealth(const DecimalQuantity &fq, const UnicodeString &expected);
    void checkDoubleBehavior(double d, bool explicitRequired);
};

class DoubleConversionTest : public IntlTest {
  public:
    void testDoubleConversionApi();

    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override;
};

class ModifiersTest : public IntlTest {
  public:
    void testConstantAffixModifier();
    void testConstantMultiFieldModifier();
    void testSimpleModifier();
    void testCurrencySpacingEnabledModifier();

    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override;

  private:
    void assertModifierEquals(const Modifier &mod, int32_t expectedPrefixLength, bool expectedStrong,
                              UnicodeString expectedChars, UnicodeString expectedFields,
                              UErrorCode &status);

    void assertModifierEquals(const Modifier &mod, FormattedStringBuilder &sb, int32_t expectedPrefixLength,
                              bool expectedStrong, UnicodeString expectedChars,
                              UnicodeString expectedFields, UErrorCode &status);
};

class PatternModifierTest : public IntlTest {
  public:
    void testBasic();
    void testPatternWithNoPlaceholder();
    void testMutableEqualsImmutable();

    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override;

  private:
    UnicodeString getPrefix(const MutablePatternModifier &mod, UErrorCode &status);
    UnicodeString getSuffix(const MutablePatternModifier &mod, UErrorCode &status);
};

class PatternStringTest : public IntlTestWithFieldPosition {
  public:
    void testLocalized();
    void testToPatternSimple();
    void testExceptionOnInvalid();
    void testBug13117();
    void testCurrencyDecimal();

    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override;

  private:
};

class NumberParserTest : public IntlTest {
  public:
    void testBasic();
    void testLocaleFi();
    void testSeriesMatcher();
    void testCombinedCurrencyMatcher();
    void testAffixPatternMatcher();
    void testGroupingDisabled();
    void testCaseFolding();
    void test20360_BidiOverflow();
    void testInfiniteRecursion();

    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override;
};

class NumberSkeletonTest : public IntlTest {
  public:
    void validTokens();
    void invalidTokens();
    void unknownTokens();
    void unexpectedTokens();
    void duplicateValues();
    void stemsRequiringOption();
    void defaultTokens();
    void flexibleSeparators();
    void wildcardCharacters();
    void perUnitInArabic();
    void perUnitToSkeleton();
    void measurementSystemOverride();

    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override;

  private:
    void expectedErrorSkeleton(const char16_t** cases, int32_t casesLen);
};

class NumberRangeFormatterTest : public IntlTestWithFieldPosition {
  public:
    NumberRangeFormatterTest();
    NumberRangeFormatterTest(UErrorCode &status);

    void testSanity();
    void testBasic();
    void testCollapse();
    void testIdentity();
    void testDifferentFormatters();
    void testNaNInfinity();
    void testPlurals();
    void testFieldPositions();
    void testCopyMove();
    void toObject();
    void testGetDecimalNumbers();
    void test21684_Performance();
    void test21358_SignPosition();
    void test21683_StateLeak();
    void testCreateLNRFFromNumberingSystemInSkeleton();

    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override;

  private:
    CurrencyUnit USD;
    CurrencyUnit CHF;
    CurrencyUnit GBP;
    CurrencyUnit PTE;

    MeasureUnit METER;
    MeasureUnit KILOMETER;
    MeasureUnit FAHRENHEIT;
    MeasureUnit KELVIN;

    void assertFormatRange(
      const char16_t* message,
      const UnlocalizedNumberRangeFormatter& f,
      Locale locale,
      const char16_t* expected_10_50,
      const char16_t* expected_49_51,
      const char16_t* expected_50_50,
      const char16_t* expected_00_30,
      const char16_t* expected_00_00,
      const char16_t* expected_30_3K,
      const char16_t* expected_30K_50K,
      const char16_t* expected_49K_51K,
      const char16_t* expected_50K_50K,
      const char16_t* expected_50K_50M);
    
    FormattedNumberRange assertFormattedRangeEquals(
      const char16_t* message,
      const LocalizedNumberRangeFormatter& l,
      double first,
      double second,
      const char16_t* expected);
};

class SimpleNumberFormatterTest : public IntlTestWithFieldPosition {
  public:
    void testBasic();
    void testWithOptions();
    void testSymbols();
    void testSign();
    void testCopyMove();
    void testCAPI();

    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override;
};

class NumberPermutationTest : public IntlTest {
  public:
    void testPermutations();

    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override;
};


// NOTE: This macro is identical to the one in itformat.cpp
#define TESTCLASS(id, TestClass)          \
    case id:                              \
        name = #TestClass;                \
        if (exec) {                       \
            logln(#TestClass " test---"); \
            logln((UnicodeString)"");     \
            TestClass test;               \
            callTest(test, par);          \
        }                                 \
        break

class NumberTest : public IntlTest {
  public:
    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par = 0) override {
        if (exec) {
            logln("TestSuite NumberTest: ");
        }

        switch (index) {
        TESTCLASS(0, AffixUtilsTest);
        TESTCLASS(1, NumberFormatterApiTest);
        TESTCLASS(2, DecimalQuantityTest);
        TESTCLASS(3, ModifiersTest);
        TESTCLASS(4, PatternModifierTest);
        TESTCLASS(5, PatternStringTest);
        TESTCLASS(6, DoubleConversionTest);
        TESTCLASS(7, NumberParserTest);
        TESTCLASS(8, NumberSkeletonTest);
        TESTCLASS(9, NumberRangeFormatterTest);
        TESTCLASS(10, SimpleNumberFormatterTest);
        TESTCLASS(11, NumberPermutationTest);
        default: name = ""; break; // needed to end loop
        }
    }
};

#endif /* #if !UCONFIG_NO_FORMATTING */
