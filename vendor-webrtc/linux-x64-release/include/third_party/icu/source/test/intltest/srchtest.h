// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/****************************************************************************
 * COPYRIGHT: 
 * Copyright (c) 2001-2008, International Business Machines Corporation and others
 * All Rights Reserved.
 ***************************************************************************/

#ifndef _STRSRCH_H
#define _STRSRCH_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_COLLATION 

#include "unicode/tblcoll.h"
#include "unicode/brkiter.h"
#include "intltest.h"
#include "unicode/usearch.h"

struct SearchData;
typedef struct SearchData SearchData;

class StringSearchTest: public IntlTest 
{
public:
    StringSearchTest();
    virtual ~StringSearchTest();

    void runIndexedTest(int32_t index, UBool exec, const char* &name, 
                        char* par = nullptr) override;
#if !UCONFIG_NO_BREAK_ITERATION
private:
    RuleBasedCollator *m_en_us_; 
    RuleBasedCollator *m_fr_fr_;
    RuleBasedCollator *m_de_;
    RuleBasedCollator *m_es_;
    BreakIterator     *m_en_wordbreaker_;
    BreakIterator     *m_en_characterbreaker_;

    RuleBasedCollator * getCollator(const char *collator);
    BreakIterator     * getBreakIterator(const char *breaker);
    char              * toCharString(const UnicodeString &text);
    Collator::ECollationStrength getECollationStrength(
                                   const UCollationStrength &strength) const;
    UBool           assertEqualWithStringSearch(      StringSearch *strsrch,
                                                const SearchData   *search);
    UBool           assertEqual(const SearchData *search);
    UBool           assertCanonicalEqual(const SearchData *search);
    UBool           assertEqualWithAttribute(const SearchData *search, 
                                            USearchAttributeValue canonical,
                                            USearchAttributeValue overlap);
    void TestOpenClose();
    void TestInitialization();
    void TestBasic();
    void TestNormExact();
    void TestStrength();
#if !UCONFIG_NO_BREAK_ITERATION
    void TestBreakIterator();
#endif
    void TestVariable();
    void TestOverlap();
    void TestCollator();
    void TestPattern();
    void TestText();
    void TestCompositeBoundaries();
    void TestGetSetOffset();
    void TestGetSetAttribute();
    void TestGetMatch();
    void TestSetMatch();
    void TestReset();
    void TestSupplementary();
    void TestContraction();
    void TestIgnorable();
    void TestCanonical();
    void TestNormCanonical();
    void TestStrengthCanonical();
#if !UCONFIG_NO_BREAK_ITERATION
    void TestBreakIteratorCanonical();
#endif
    void TestVariableCanonical();
    void TestOverlapCanonical();
    void TestCollatorCanonical();
    void TestPatternCanonical();
    void TestTextCanonical();
    void TestCompositeBoundariesCanonical();
    void TestGetSetOffsetCanonical();
    void TestSupplementaryCanonical();
    void TestContractionCanonical();
    void TestUClassID();
    void TestSubclass();
    void TestCoverage();
    void TestDiacriticMatch();
#endif
};

#endif /* #if !UCONFIG_NO_COLLATION */

#endif
