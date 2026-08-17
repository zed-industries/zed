// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2009, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef _PARSEPOSITIONIONTEST_
#define _PARSEPOSITIONIONTEST_
 
#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "intltest.h"


/** 
 * Performs test for ParsePosition
 **/
class ParsePositionTest: public IntlTest {
    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
public:

    void TestParsePosition();
    void TestFieldPosition();
    void TestFieldPosition_example();
    void Test4109023();

protected:
    UBool failure(UErrorCode status, const char* msg, UBool possibleDataError=false);
};

#endif /* #if !UCONFIG_NO_FORMATTING */
 
#endif // _PARSEPOSITIONIONTEST_
//eof
