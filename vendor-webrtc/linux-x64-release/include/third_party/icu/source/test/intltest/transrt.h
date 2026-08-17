// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
**********************************************************************
*   Copyright (C) 2001-2004, International Business Machines
*   Corporation and others.  All Rights Reserved.
**********************************************************************
*   Date        Name        Description
*   05/23/00    aliu        Creation.
**********************************************************************
*/
#ifndef TRANSRT_H
#define TRANSRT_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_TRANSLITERATION

#include "unicode/translit.h"
#include "intltest.h"

/**
 * @test
 * @summary Round trip test of Transliterator
 */
class TransliteratorRoundTripTest : public IntlTest {

    void runIndexedTest(int32_t index, UBool exec, const char* &name,
                        char* par=nullptr) override;

    void TestKana();
    void TestHiragana();
    void TestKatakana();
    void TestJamo();
    void TestHangul();
    void TestHan();
    void TestGreek();
    void TestGreekUNGEGN();
    void Testel();
    void TestCyrillic();
    void TestDevanagariLatin();
    void TestInterIndic();
    void TestHebrew();
    void TestArabic();
    void TestDebug(const char* name,const char fromSet[],
                   const char* toSet,const char* exclusions);
};

#endif /* #if !UCONFIG_NO_TRANSLITERATION */

#endif
