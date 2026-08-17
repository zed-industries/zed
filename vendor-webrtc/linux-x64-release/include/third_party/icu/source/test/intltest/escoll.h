// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2003, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

/**
 * CollationSpanishTest is a third level test class. This tests the locale
 * specific primary and tertiary rules. This Spanish sort uses the traditional
 * sorting sequence.  The Spanish modern sorting sequence does not sort
 * ch and ll as unique characters.
 */

#ifndef _ESCOLL
#define _ESCOLL

#include "unicode/utypes.h"

#if !UCONFIG_NO_COLLATION

#include "tscoll.h"

class CollationSpanishTest: public IntlTestCollator {
public:
    // If this is too small for the test data, just increase it.
    // Just don't make it too large, otherwise the executable will get too big
    enum EToken_Len { MAX_TOKEN_LEN = 16 };

    CollationSpanishTest();
    virtual ~CollationSpanishTest();
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

    // performs tests with strength PRIMARY
    void TestPrimary(/* char* par */);

    // prforms test with strength TERTIARY
    void TestTertiary(/* char* par */);

private:
    static const char16_t testSourceCases[][MAX_TOKEN_LEN];
    static const char16_t testTargetCases[][MAX_TOKEN_LEN];
    static const Collator::EComparisonResult results[];

    Collator *myCollation;
};

#endif /* #if !UCONFIG_NO_COLLATION */

#endif
