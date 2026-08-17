// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
*******************************************************************************
*
*   Copyright (C) 2012-2013, International Business Machines
*   Corporation and others.  All Rights Reserved.
*
*******************************************************************************
*   file name:  listformattertest.cpp
*   encoding:   UTF-8
*   tab size:   8 (not used)
*   indentation:4
*
*   created on: 2012aug27
*   created by: Umesh P. Nair
*/

#ifndef __LISTFORMATTERTEST_H__
#define __LISTFORMATTERTEST_H__

#include "unicode/fpositer.h"
#include "unicode/listformatter.h"
#include "intltest.h"
#include "itformat.h"

#if !UCONFIG_NO_FORMATTING

class ListFormatterTest : public IntlTestWithFieldPosition {
  public:
    ListFormatterTest();
    virtual ~ListFormatterTest() {}

    void runIndexedTest(int32_t index, UBool exec, const char *&name, char *par=0) override;

    void TestRoot();
    void TestBogus();
    void TestEnglish();
    void TestEnglishUS();
    void TestEnglishGB();
    void TestNynorsk();
    void TestChineseTradHK();
    void TestRussian();
    void TestMalayalam();
    void TestZulu();
    void TestOutOfOrderPatterns();
    void Test9946();
    void TestFieldPositionIteratorWith1Item();
    void TestFieldPositionIteratorWith2Items();
    void TestFieldPositionIteratorWith3Items();
    void TestFieldPositionIteratorWith2ItemsPatternShift();
    void TestFieldPositionIteratorWith3ItemsPatternShift();
    void TestFormattedValue();
    void TestDifferentStyles();
    void TestCreateStyled();
    void TestContextual();
    void TestNextPosition();
    void TestInt32Overflow();
    void Test21871();

  private:
    void CheckFormatting(
        const ListFormatter* formatter,
        UnicodeString data[],
        int32_t data_size,
        const UnicodeString& expected_result,
        const char* testName);
    void ExpectPositions(
        const FormattedList& iter,
        int32_t *values,
        int32_t tupleCount,
        UErrorCode& status);
    void RunTestFieldPositionIteratorWithNItems(
        UnicodeString *data,
        int32_t n,
        int32_t *values,
        int32_t tupleCount,
        const char16_t *expectedFormatted,
        const char* testName);
    void RunTestFieldPositionIteratorWithNItemsPatternShift(
        UnicodeString *data,
        int32_t n,
        int32_t *values,
        int32_t tupleCount,
        const char16_t *expectedFormatted,
        const char* testName);
    void RunTestFieldPositionIteratorWithFormatter(
        ListFormatter* formatter,
        UnicodeString *data,
        int32_t n,
        int32_t *values,
        int32_t tupleCount,
        const char16_t *expectedFormatted,
        const char* testName);
    void CheckFourCases(
        const char* locale_string,
        UnicodeString one,
        UnicodeString two,
        UnicodeString three,
        UnicodeString four,
        UnicodeString results[4],
        const char* testName);
    UBool RecordFourCases(
        const Locale& locale,
        UnicodeString one,
        UnicodeString two,
        UnicodeString three,
        UnicodeString four,
        UnicodeString results[4],
        const char* testName);
    void DoTheRealListStyleTesting(
        Locale locale,
        UnicodeString items[],
        int itemCount,
        UListFormatterType type,
        UListFormatterWidth width,
        const char* expected,
        IcuTestErrorCode status);

  private:
    // Reused test data.
    const UnicodeString prefix;
    const UnicodeString one;
    const UnicodeString two;
    const UnicodeString three;
    const UnicodeString four;
};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
