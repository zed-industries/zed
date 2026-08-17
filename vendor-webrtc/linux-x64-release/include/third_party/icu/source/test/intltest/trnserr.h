// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 2001-2005, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/************************************************************************
*   This test program is intended for testing error conditions of the 
*   transliterator APIs to make sure the exceptions are raised where
*   necessary.
*
*   Date        Name        Description
*   11/14/2001  hshih       Creation.
* 
************************************************************************/


#ifndef TRNSERR_H
#define TRNSERR_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_TRANSLITERATION

#include "unicode/translit.h"
#include "intltest.h"

/**
 * @test
 * @summary Error condition tests of Transliterator
 */
class TransliteratorErrorTest : public IntlTest {
public:
    void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par=nullptr) override;

    /*Tests the returned error codes on all the APIs according to the API documentation. */
    void TestTransliteratorErrors();
    
    void TestUnicodeSetErrors();

    //void TestUniToHexErrors();

    void TestRBTErrors();

    //void TestHexToUniErrors();

    // JitterBug 4452, for coverage.  The reason to put this method here is 
    //  this class is comparable smaller than other Transliterator*Test classes
    void TestCoverage();

};

#endif /* #if !UCONFIG_NO_TRANSLITERATION */

#endif
