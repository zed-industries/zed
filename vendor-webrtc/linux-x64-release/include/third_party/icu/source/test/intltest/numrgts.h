// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/***********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2013, International Business Machines Corporation
 * and others. All Rights Reserved.
 ***********************************************************************/

#ifndef _NUMBERFORMATREGRESSIONTEST_
#define _NUMBERFORMATREGRESSIONTEST_
 
#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/unistr.h"
#include "unicode/numfmt.h"
#include "unicode/decimfmt.h"
#include "intltest.h"

/** 
 * Performs regression test for MessageFormat
 **/
class NumberFormatRegressionTest: public IntlTest {

    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
public:

    void Test4075713();
    void Test4074620() ;
    void Test4088161();
    void Test4087245();
    void Test4087535();
    void Test4088503();
    void Test4066646();
    float assignFloatValue(float returnfloat);
    void Test4059870();
    void Test4083018();
    void Test4071492();
    void Test4086575();
    void Test4068693();
    void Test4069754();
    void Test4087251();
    void Test4090489();
    void Test4090504();
    void Test4095713();
    void Test4092561();
    void Test4092480();
    void Test4087244();
    void Test4070798();
    void Test4071005();
    void Test4071014();
    void Test4071859();
    void Test4093610();
    void roundingTest(DecimalFormat *df, double x, UnicodeString& expected);
    void Test4098741();
    void Test4074454();
    void Test4099404();
    void Test4101481();
    void Test4052223();
    void Test4061302();
    void Test4062486();
    void Test4108738();
    void Test4106658();
    void Test4106662();
    void Test4114639();
    void Test4106664();
    void Test4106667();
    void Test4110936();
    void Test4122840();
    void Test4125885();
    void Test4134034();
    void Test4134300();
    void Test4140009();
    void Test4141750();
    void Test4145457();
    void Test4147295();
    void Test4147706();

    void Test4162198();
    void Test4162852();

    void Test4167494();
    void Test4170798();
    void Test4176114();
    void Test4179818();
    void Test4212072();
    void Test4216742();
    void Test4217661();
    void Test4161100();
    void Test4243011();
    void Test4243108();
    void TestJ691();
    void Test8199();
    void Test9109();
    void Test9780();
    void Test9677();
    void Test10361();
protected:
    UBool failure(UErrorCode status, const UnicodeString& msg, UBool possibleDataError=false);
    UBool failure(UErrorCode status, const UnicodeString& msg, const char *l, UBool possibleDataError=false);
    UBool failure(UErrorCode status, const UnicodeString& msg, const Locale& l, UBool possibleDataError=false);
};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif // _NUMBERFORMATREGRESSIONTEST_
//eof
