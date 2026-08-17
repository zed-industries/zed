// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2010, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

/**
 * Normalizer basic tests
 */

#ifndef _TSTNORM
#define _TSTNORM

#include "unicode/utypes.h"

#if !UCONFIG_NO_NORMALIZATION

#include "unicode/normlzr.h"
#include "intltest.h"

class BasicNormalizerTest : public IntlTest {
public:
    BasicNormalizerTest();
    virtual ~BasicNormalizerTest();

    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

    void TestHangulCompose();
    void TestHangulDecomp();
    void TestPrevious();
    void TestDecomp();
    void TestCompatDecomp();
    void TestCanonCompose();
    void TestCompatCompose();
    void TestTibetan();
    void TestCompositionExclusion();
    void TestZeroIndex();
    void TestVerisign();
    void TestPreviousNext();
    void TestNormalizerAPI();
    void TestConcatenate();
    void TestCompare();
    void FindFoldFCDExceptions();
    void TestSkippable();
    void TestCustomComp();
    void TestCustomFCC();
    void TestFilteredNormalizer2Coverage();
    void TestComposeUTF8WithEdits();
    void TestDecomposeUTF8WithEdits();
    void TestLowMappingToEmpty_D();
    void TestLowMappingToEmpty_FCD();
    void TestNormalizeIllFormedText();
    void TestComposeJamoTBase();
    void TestComposeBoundaryAfter();
    void TestNFKC_SCF();

private:
    UnicodeString canonTests[24][3];
    UnicodeString compatTests[11][3];
    UnicodeString hangulCanon[2][3];

    void
    TestPreviousNext(const char16_t *src, int32_t srcLength,
                     const UChar32 *expext, int32_t expectLength,
                     const int32_t *expectIndex, // its length=expectLength+1
                     int32_t srcMiddle, int32_t expectMiddle,
                     const char *moves,
                     UNormalizationMode mode,
                     const char *name);

    int32_t countFoldFCDExceptions(uint32_t foldingOptions);

    //------------------------------------------------------------------------
    // Internal utilities
    //
    void backAndForth(Normalizer* iter, const UnicodeString& input);

    void staticTest(UNormalizationMode mode, int options,
                    UnicodeString tests[][3], int length, int outCol);

    void iterateTest(Normalizer* iter, UnicodeString tests[][3], int length, int outCol);

    void assertEqual(const UnicodeString& input,
             const UnicodeString& expected, 
             Normalizer* result,
             const UnicodeString& errPrefix);

    static UnicodeString hex(char16_t ch);
    static UnicodeString hex(const UnicodeString& str);

    void checkLowMappingToEmpty(const Normalizer2 &n2);
};

#endif /* #if !UCONFIG_NO_NORMALIZATION */

#endif // _TSTNORM
