// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2013, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef _PLURALFORMATTEST
#define _PLURALFORMATTEST

#include "unicode/utypes.h"
#include "unicode/plurrule.h"
#include "unicode/plurfmt.h"


#if !UCONFIG_NO_FORMATTING

#include "intltest.h"

/**
 * Test basic functionality of various API functions
 **/
class PluralFormatTest : public IntlTest {
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

private:
    /**
     * Performs tests on many API functions, see detailed comments in source code
     **/
    void pluralFormatBasicTest(/* char* par */);
    void pluralFormatUnitTest(/* char* par */);
    void pluralFormatLocaleTest(/* char* par */);
    void pluralFormatExtendedTest();
    void pluralFormatExtendedParseTest();
    void ordinalFormatTest();
    void TestDecimals();
    void numberFormatTest(PluralFormat* plFmt, 
                          NumberFormat *numFmt, 
                          int32_t start, 
                          int32_t end, 
                          UnicodeString* numOddAppendStr,
                          UnicodeString* numEvenAppendStr, 
                          UBool overwrite, // overwrite the numberFormat.format result
                          UnicodeString *message);
    void helperTestResults(const char** localeArray, 
                           int32_t capacityOfArray, 
                           UnicodeString& testPattern, 
                           int8_t *expectingResults);
};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
