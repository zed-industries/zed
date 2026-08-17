// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT:
 * Copyright (c) 1997-2013, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef _PluralRulesTest
#define _PluralRulesTest

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "intltest.h"
#include "number_decimalquantity.h"
#include "unicode/localpointer.h"
#include "unicode/plurrule.h"

/**
 * Test basic functionality of various API functions
 **/
class PluralRulesTest : public IntlTest {
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

private:
    /**
     * Performs tests on many API functions, see detailed comments in source code
     **/
    void testAPI();
    void testGetUniqueKeywordValue();
    void testGetSamples();
    void testGetDecimalQuantitySamples();
    void testGetOrAddSamplesFromString();
    void testGetOrAddSamplesFromStringCompactNotation();
    void testSamplesWithExponent();
    void testSamplesWithCompactNotation();
    void testWithin();
    void testGetAllKeywordValues();
    void testCompactDecimalPluralKeyword();
    void testDoubleValue();
    void testLongValue();
    void testScientificPluralKeyword();
    void testOrdinal();
    void testSelect();
    void testSelectRange();
    void testAvailableLocales();
    void testParseErrors();
    void testFixedDecimal();
    void testSelectTrailingZeros();
    void testLocaleExtension();

    void assertRuleValue(const UnicodeString& rule, double expected);
    void assertRuleKeyValue(const UnicodeString& rule, const UnicodeString& key,
                            double expected);
    void checkNewSamples(UnicodeString description, 
                         const LocalPointer<PluralRules> &test,
                         UnicodeString keyword,
                         UnicodeString samplesString,
                         ::icu::number::impl::DecimalQuantity firstInRange);
    UnicodeString getPluralKeyword(const LocalPointer<PluralRules> &rules,
                                   Locale locale, double number, const char16_t* skeleton);
    void checkSelect(const LocalPointer<PluralRules> &rules, UErrorCode &status, 
                                  int32_t line, const char *keyword, ...);
    void compareLocaleResults(const char* loc1, const char* loc2, const char* loc3);
};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
