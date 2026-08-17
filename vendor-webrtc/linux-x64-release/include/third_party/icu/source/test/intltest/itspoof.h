// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
**********************************************************************
* Copyright (C) 2011-2013, International Business Machines Corporation 
* and others.  All Rights Reserved.
**********************************************************************
*/

/**
 * IntlTestSpoof is the top level test class for the Unicode Spoof detection tests
 */

#ifndef INTLTESTSPOOF_H
#define INTLTESTSPOOF_H

#include "unicode/utypes.h"
#if !UCONFIG_NO_REGULAR_EXPRESSIONS && !UCONFIG_NO_NORMALIZATION && !UCONFIG_NO_FILE_IO
#include "unicode/uspoof.h"
#include "intltest.h"


class IntlTestSpoof: public IntlTest {
public:
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;
    
    // Test the USpoofDetector API functions that require C++
    // The pure C part of the API, which is most of it, is tested in cintltst
    void  testSpoofAPI();

    void  testSkeleton();

    void testBidiSkeleton();

    void testAreConfusable();
    
    void testAreBidiConfusable();

    void testInvisible();

    void testConfData();

    void testBug8654();

    void testScriptSet();

    void testRestrictionLevel();

    void testMixedNumbers();

    void testBug12153();

    void testBug12825();

    void testBug12815();

    void testBug13314_MixedNumbers();

    void testBug13328_MixedCombiningMarks();

    void testCombiningDot();

    // Internal functions to run a single skeleton test case.
    void checkSkeleton(const USpoofChecker *sc, uint32_t flags, const char *input, const char *expected,
                       int32_t lineNum);
    void checkBidiSkeleton(const USpoofChecker *sc, const UnicodeString &input, UBiDiDirection direction,
                           const UnicodeString  &expected, int32_t lineNum);
};

#endif  // !UCONFIG_NO_REGULAR_EXPRESSIONS && !UCONFIG_NO_NORMALIZATION && !UCONFIG_NO_FILE_IO
#endif
