// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2001, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

/**
 * Collation currency tests.
 * (It's important to stay current!)
 */

#ifndef _CURRCOLL
#define _CURRCOLL

#include "unicode/utypes.h"

#if !UCONFIG_NO_COLLATION

#include "unicode/coleitr.h"
#include "tscoll.h"

class CollationCurrencyTest: public IntlTestCollator
{
public:

    // If this is too small for the test data, just increase it.
    // Just don't make it too large, otherwise the executable will get too big
    enum EToken_Len { MAX_TOKEN_LEN = 16 };

    CollationCurrencyTest();
    virtual ~CollationCurrencyTest();
    void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par = nullptr) override;

    void currencyTest(/*char *par*/);
};

#endif /* #if !UCONFIG_NO_COLLATION */

#endif
