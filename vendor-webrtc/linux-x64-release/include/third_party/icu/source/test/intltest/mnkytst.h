// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2003, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

/**
 * CollationMonkeyTest is a third level test class.  This tests the random 
 * substrings of the default test strings to verify if the compare and 
 * sort key algorithm works correctly.  For example, any string is always
 * less than the string itself appended with any character.
 */

#ifndef _MNKYTST
#define _MNKYTST

#include "unicode/utypes.h"

#if !UCONFIG_NO_COLLATION

#include "tscoll.h"

class CollationMonkeyTest: public IntlTestCollator {
public:
    // If this is too small for the test data, just increase it.
    // Just don't make it too large, otherwise the executable will get too big
    enum EToken_Len { MAX_TOKEN_LEN = 16 };

    CollationMonkeyTest();
    virtual ~CollationMonkeyTest();
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

    // utility function used in tests, returns absolute value
    int32_t checkValue(int32_t value);

    // perform monkey tests using Collator::compare
    void TestCompare(/* char* par */);

    // perform monkey tests using CollationKey::compareTo
    void TestCollationKey(/* char* par */);

    void TestRules(/* char* par */);

private:
    void report(UnicodeString& s, UnicodeString& t, int32_t result, int32_t revResult);

    const UnicodeString source;

    Collator *myCollator;
};

#endif /* #if !UCONFIG_NO_COLLATION */

#endif
