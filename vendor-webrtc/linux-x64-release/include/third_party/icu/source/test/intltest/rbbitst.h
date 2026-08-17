// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*************************************************************************
 * Copyright (c) 1999-2016, International Business Machines
 * Corporation and others. All Rights Reserved.
 *************************************************************************
 *   Date        Name        Description
 *   12/15/99    Madhu        Creation.
 *   01/12/2000  Madhu        Updated for changed API and added new tests
 ************************************************************************/


#ifndef RBBITEST_H
#define RBBITEST_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_BREAK_ITERATION

#include <memory>

#include "intltest.h"
#include "unicode/brkiter.h"
#include "unicode/rbbi.h"
#include "unicode/uscript.h"

class  Enumeration;
class  BITestData;
struct TestParams;
class  RBBIMonkeyKind;

U_NAMESPACE_BEGIN
class  UVector32;
U_NAMESPACE_END

/**
 * Test the RuleBasedBreakIterator class giving different rules
 */
class RBBITest: public IntlTest {
public:

    RBBITest();
    virtual ~RBBITest();

    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

    void TestGetAvailableLocales();
    void TestGetDisplayName();
    void TestEndBehaviour();
    void TestBug4153072();
    void TestJapaneseLineBreak();
    void TestThaiLineBreak();
    void TestMixedThaiLineBreak();
    void TestMaiyamok();
    void TestMonkey();

    void TestExtended();
    void executeTest(TestParams *, UErrorCode &status);

    void TestWordBreaks();
    void TestWordBoundary();
    void TestLineBreaks();
    void TestSentBreaks();
    void TestBug3818();
    void TestJapaneseWordBreak();
    void TestTrieDict();
    void TestUnicodeFiles();
    void TestBug5775();
    void TestTailoredBreaks();
    void TestDictRules();
    void TestBug5532();
    void TestBug9983();
    void TestBug7547();
    void TestBug12797();
    void TestBug12918();
    void TestBug12932();
    void TestEmoji();
    void TestBug12519();
    void TestBug12677();
    void TestTableRedundancies();
    void TestBug13447();
    void TestReverse();
    void TestReverse(std::unique_ptr<RuleBasedBreakIterator>bi);
    void TestBug13692();
    void TestDebugRules();
    void TestUnpairedSurrogate();

    void TestDebug();
    void TestProperties();
    void Test8BitsTrieWith8BitStateTable();
    void Test8BitsTrieWith16BitStateTable();
    void Test16BitsTrieWith8BitStateTable();
    void Test16BitsTrieWith16BitStateTable();
    void TestTable_8_16_Bits();
    void TestBug13590();
    void TestLSTMThai();
    void TestLSTMBurmese();
    void TestRandomAccess();
    void TestExternalBreakEngineWithFakeTaiLe();
    void TestExternalBreakEngineWithFakeYue();

#if U_ENABLE_TRACING
    void TestTraceCreateCharacter();
    void TestTraceCreateWord();
    void TestTraceCreateSentence();
    void TestTraceCreateTitle();
    void TestTraceCreateLine();
    void TestTraceCreateLineNormal();
    void TestTraceCreateLineStrict();
    void TestTraceCreateLineLoose();
    void TestTraceCreateLineNormalPhrase();
    void TestTraceCreateLineLoosePhrase();
    void TestTraceCreateLineStrictPhrase();
    void TestTraceCreateLinePhrase();
    void TestTraceCreateBreakEngine();
#endif

/***********************/
private:
    /**
     * internal methods to prepare test data
     **/

    void RunMonkey(BreakIterator *bi, RBBIMonkeyKind &mk, const char *name, uint32_t  seed,
        int32_t loopCount, UBool useUText);

    // Run one of the Unicode Consortium boundary test data files.
    void runUnicodeTestData(const char *fileName, RuleBasedBreakIterator *bi);

    // Run tests from one of the LSTM test files.
    void runLSTMTestFromFile(const char* filename, UScriptCode script);

    // Run a single test case from one of the Unicode Consortium test files.
    void checkUnicodeTestCase(const char *testFileName, int lineNumber,
                         const UnicodeString &testString,
                         UVector32 *breakPositions,
                         RuleBasedBreakIterator *bi);

    // Run the actual tests for TestTailoredBreaks()
    void TBTest(BreakIterator* brkitr, int type, const char *locale, const char* escapedText,
                const int32_t *expectOffsets, int32_t expectOffsetsCount);

    /** Filter for test cases from the Unicode test data files.
     *  Some need to be skipped because ICU is unable to fully implement the
     *  Unicode boundary specifications.
     *  @param testCase the test data string.
     *  @param fileName the Unicode test data file name.
     *  @return false if the test case should be run, true if it should be skipped.
     */
    UBool testCaseIsKnownIssue(const UnicodeString &testCase, const char *fileName);

    // Test parameters, from the test framework and test invocation.
    const char* fTestParams;

    // Helper functions to test different trie bit sizes and state table bit sizes.
    void testTrieStateTable(int32_t numChar, bool expectedTrieWidthIn8Bits, bool expectedStateRowIn8Bits);

#if U_ENABLE_TRACING
    void assertTestTraceResult(int32_t fnNumber, const char* expectedData);
#endif

};

#endif /* #if !UCONFIG_NO_BREAK_ITERATION */

#endif
