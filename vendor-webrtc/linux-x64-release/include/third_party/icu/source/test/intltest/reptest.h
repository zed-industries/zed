// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 2001, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/************************************************************************
*   This test program is intended for testing Replaceable class.
*
*   Date        Name        Description
*   11/28/2001  hshih       Creation.
* 
************************************************************************/


#ifndef REPTEST_H
#define REPTEST_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_TRANSLITERATION

#include "unicode/translit.h"
#include "intltest.h"

/**
 * @test
 * @summary Testing the Replaceable class
 */
class ReplaceableTest : public IntlTest {
public:
    void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par=nullptr) override;

    /*Tests the Replaceable class according to the API documentation. */
    void TestReplaceableClass();
private:
    void check( const UnicodeString& transliteratorName, 
                const UnicodeString& test, 
                const UnicodeString& shouldProduceStyles);
};

#endif /* #if !UCONFIG_NO_TRANSLITERATION */

#endif
