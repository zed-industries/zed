// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
**********************************************************************
*   Copyright (C) 1999-2011, International Business Machines
*   Corporation and others.  All Rights Reserved.
**********************************************************************
*   Date        Name        Description
*   11/10/99    aliu        Creation.
**********************************************************************
*/
#ifndef TRANSTST_H
#define TRANSTST_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_TRANSLITERATION

#include "unicode/translit.h"
#include "intltest.h"

/**
 * @test
 * @summary General test of Transliterator
 */
class TransliteratorTest : public IntlTest {

public:
    TransliteratorTest();
    virtual ~TransliteratorTest();

private:
    void runIndexedTest(int32_t index, UBool exec, const char* &name,
                        char* par=nullptr) override;

    void TestInstantiation();
    
    void TestSimpleRules();

    void TestInlineSet();

    void TestAnchors();

    void TestPatternQuoting();

    /**
     * Create some inverses and confirm that they work.  We have to be
     * careful how we do this, since the inverses will not be true
     * inverses -- we can't throw any random string at the composition
     * of the transliterators and expect the identity function.  F x
     * F' != I.  However, if we are careful about the input, we will
     * get the expected results.
     */
    void TestRuleBasedInverse();

    /**
     * Basic test of keyboard.
     */
    void TestKeyboard();

    /**
     * Basic test of keyboard with cursor.
     */
    void TestKeyboard2();

    /**
     * Test keyboard transliteration with back-replacement.
     */
    void TestKeyboard3();
    
    void keyboardAux(const Transliterator& t,
                     const char* DATA[], int32_t DATA_length);
    
    void TestArabic();

    /**
     * Compose the Kana transliterator forward and reverse and try
     * some strings that should come out unchanged.
     */
    void TestCompoundKana();

    /**
     * Compose the hex transliterators forward and reverse.
     */
    void TestCompoundHex();

    /**
     * Do some basic tests of filtering.
     */
    void TestFiltering();

    /**
     * Regression test for bugs found in Greek transliteration.
     */
    void TestJ277();

    /**
     * Prefix, suffix support in hex transliterators.
     */
    void TestJ243();

    /**
     * Parsers need better syntax error messages.
     */
    void TestJ329();

    /**
     * Test segments and segment references.
     */
    void TestSegments();
    
    /**
     * Test cursor positioning outside of the key
     */
    void TestCursorOffset();
    
    /**
     * Test zero length and > 1 char length variable values.  Test
     * use of variable refs in UnicodeSets.
     */
    void TestArbitraryVariableValues();

    /**
     * Confirm that the contextStart, contextLimit, start, and limit
     * behave correctly. J474.
     */
    void TestPositionHandling();

    /**
     * Test the Hiragana-Katakana transliterator.
     */
    void TestHiraganaKatakana();

    /**
     * Test cloning / copy constructor of RBT.
     */
    void TestCopyJ476();

    /**
     * Test inter-Indic transliterators.  These are composed.
     * ICU4C Jitterbug 483.
     */
    void TestInterIndic();

    /**
     * Test filter syntax in IDs. (J918)
     */
    void TestFilterIDs();

    /**
     * Test the case mapping transliterators.
     */
    void TestCaseMap();

    /**
     * Test the name mapping transliterators.
     */
    void TestNameMap();

    /**
     * Test liberalized ID syntax.  1006c
     */
    void TestLiberalizedID();
    /**
     * Test Jitterbug 912
     */
    void TestCreateInstance();

    void TestNormalizationTransliterator();

    void TestCompoundRBT();

    void TestCompoundFilter();

    void TestRemove();

    void TestToRules();

    void TestContext();

    void TestSupplemental();

    void TestQuantifier();

    /**
     * Test Source-Target/Variant.
     */
    void TestSTV();

    void TestCompoundInverse();

    void TestNFDChainRBT();

    /**
     * Inverse of "Null" should be "Null". (J21)
     */
    void TestNullInverse();
    
    /**
     * Check ID of inverse of alias. (J22)
     */
    void TestAliasInverseID();
    
    /**
     * Test IDs of inverses of compound transliterators. (J20)
     */
    void TestCompoundInverseID();
    
    /**
     * Test undefined variable.
     */
    void TestUndefinedVariable();
    
    /**
     * Test empty context.
     */
    void TestEmptyContext();

    /**
     * Test compound filter ID syntax
     */
    void TestCompoundFilterID();

    /**
     * Test new property set syntax
     */
    void TestPropertySet();

    /**
     * Test various failure points of the new 2.0 engine.
     */
    void TestNewEngine();

    /**
     * Test quantified segment behavior.  We want:
     * ([abc])+ > x $1 x; applied to "cba" produces "xax"
     */
    void TestQuantifiedSegment();

    /* Devanagari-Latin rules Test */
    void TestDevanagariLatinRT();

    /* Telugu-Latin rules Test */
    void TestTeluguLatinRT();
    
    /* Gujarati-Latin rules Test */
    void TestGujaratiLatinRT();
    
    /* Sanskrit-Latin rules Test */
    void TestSanskritLatinRT();
    
    /* Test Compound Indic-Latin transliterators*/
    void TestCompoundLatinRT();

    /* Test bindi and tippi for Gurmukhi */
    void TestGurmukhiDevanagari();
    /**
     * Test instantiation from a locale.
     */
    void TestLocaleInstantiation();        
    
    /**
     * Test title case handling of accent (should ignore accents)
     */
    void TestTitleAccents();

    /**
     * Basic test of a locale resource based rule.
     */
    void TestLocaleResource();

    /**
     * Make sure parse errors reference the right line.
     */
    void TestParseError();

    /**
     * Make sure sets on output are disallowed.
     */
    void TestOutputSet();

    /**
     * Test the use variable range pragma, making sure that use of
     * variable range characters is detected and flagged as an error.
     */
    void TestVariableRange();

    /**
     * Test invalid post context error handling
     */
    void TestInvalidPostContext();

    /**
     * Test ID form variants
     */
    void TestIDForms();

    /**
     * Mark's toRules test.
     */
    void TestToRulesMark();

    /**
     * Test Escape and Unescape transliterators.
     */
    void TestEscape();

    void TestAnchorMasking();

    /**
     * Make sure display names of variants look reasonable.
     */
    void TestDisplayName();
    
    /** 
     * Check to see if case mapping works correctly.
     */
    void TestSpecialCases();
    /**
     * Check to see that incremental gets at least part way through a reasonable string.
     */
    void TestIncrementalProgress();

    /** 
     * Check that casing handles surrogates.
     */
    void TestSurrogateCasing();

    void TestFunction();

    void TestInvalidBackRef();

    void TestMulticharStringSet();

    void TestUserFunction();

    void TestAnyX();

    void TestAny();

    void TestSourceTargetSet();

    void TestPatternWhiteSpace();

    void TestAllCodepoints();

    void TestBoilerplate();

    void TestAlternateSyntax();

    void TestRuleStripping();

    void TestHalfwidthFullwidth();

    void TestThai();

    /**
     * Tests the multiple-pass syntax
     */
    void TestBeginEnd();

    /**
     * Tests that toRules() works right with the multiple-pass syntax
     */
    void TestBeginEndToRules();

    /**
     * Tests the registerAlias() function
     */
    void TestRegisterAlias();

    void TestBasicTransliteratorEvenWithoutData();
    //======================================================================
    // Support methods
    //======================================================================
 protected:
    void expectT(const UnicodeString& id,
                 const UnicodeString& source,
                 const UnicodeString& expectedResult);

    void expect(const UnicodeString& rules,
                const UnicodeString& source,
                const UnicodeString& expectedResult,
                UTransPosition *pos=0);

    void expect(const UnicodeString& id,
                const UnicodeString& rules,
                const UnicodeString& source,
                const UnicodeString& expectedResult,
                UTransPosition *pos=0);

    void expect(const Transliterator& t,
                const UnicodeString& source,
                const UnicodeString& expectedResult,
                const Transliterator& reverseTransliterator);
    
    void expect(const Transliterator& t,
                const UnicodeString& source,
                const UnicodeString& expectedResult,
                UTransPosition *pos=0);
    
    void expectAux(const UnicodeString& tag,
                   const UnicodeString& source,
                   const UnicodeString& result,
                   const UnicodeString& expectedResult);
    
    virtual void expectAux(const UnicodeString& tag,
                   const UnicodeString& summary, UBool pass,
                   const UnicodeString& expectedResult);

    static UnicodeString& formatInput(UnicodeString &appendTo,
                                      const UnicodeString& input,
                                      const UTransPosition& pos);

    void checkRules(const UnicodeString& label, Transliterator& t2,
                    const UnicodeString& testRulesForward);
    void CheckIncrementalAux(const Transliterator* t, 
                             const UnicodeString& input);

    void reportParseError(const UnicodeString& message, const UParseError& parseError, const UErrorCode& status);


    const UnicodeString DESERET_DEE;
    const UnicodeString DESERET_dee;

};

#endif /* #if !UCONFIG_NO_TRANSLITERATION */

#endif
