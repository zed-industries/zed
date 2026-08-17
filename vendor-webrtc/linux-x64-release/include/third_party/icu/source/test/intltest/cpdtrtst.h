// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html

/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2003, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/***********************************************************************
************************************************************************
*   Date        Name        Description
*   03/09/2000   Madhu        Creation.
************************************************************************/

#ifndef CPDTRTST_H
#define CPDTRTST_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_TRANSLITERATION

#include "unicode/translit.h"
#include "cpdtrans.h"
#include "intltest.h"

/**
 * @test
 * @summary General test of Compound Transliterator
 */
class CompoundTransliteratorTest : public IntlTest {
public:
    void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par=nullptr) override;

    /*Tests the constructors */
    void TestConstruction();
    /*Tests the function clone, and operator==()*/
    void TestCloneEqual();
    /*Tests the function getCount()*/
    void TestGetCount();
    /*Tests the function getTransliterator() and setTransliterators() and adoptTransliterators()*/
    void TestGetSetAdoptTransliterator();
    /*Tests the function handleTransliterate()*/
    void TestTransliterate();

    //======================================================================
    // Support methods
    //======================================================================

    /**
     * Splits a UnicodeString
     */
    UnicodeString* split(const UnicodeString& str, char16_t seperator, int32_t& count);

    void expect(const CompoundTransliterator& t,
                const UnicodeString& source,
                const UnicodeString& expectedResult);

    void expectAux(const UnicodeString& tag,
                   const UnicodeString& summary, UBool pass,
                   const UnicodeString& expectedResult);


};

#endif /* #if !UCONFIG_NO_TRANSLITERATION */

#endif
