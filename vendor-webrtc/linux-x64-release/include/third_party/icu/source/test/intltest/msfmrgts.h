// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT:
 * Copyright (c) 1997-2009, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef _MESSAGEFORMATREGRESSIONTEST_
#define _MESSAGEFORMATREGRESSIONTEST_

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "intltest.h"

/**
 * Performs regression test for MessageFormat
 **/
class MessageFormatRegressionTest: public IntlTest {

    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
public:

    void Test4074764();
    void Test4058973();
    void Test4031438();
    void Test4052223();
    void Test4104976();
    void Test4106659();
    void Test4106660();
    void Test4111739();
    void Test4114743();
    void Test4116444();
    void Test4114739();
    void Test4113018();
    void Test4106661();
    void Test4094906();
    void Test4118592();
    void Test4118594();
    void Test4105380();
    void Test4120552();
    void Test4142938();
    void TestChoicePatternQuote();
    void Test4112104();
    void TestICU12584();
    void TestAPI();

protected:
    UBool failure(UErrorCode status, const char* msg, UBool possibleDataError=false);

};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif // _MESSAGEFORMATREGRESSIONTEST_
//eof
