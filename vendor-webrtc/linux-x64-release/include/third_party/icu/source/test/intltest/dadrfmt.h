// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 2007, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

/**
 * DataDrivenFormatTest is a test class that uses data stored in resource
 * bundles to perform testing. For more details on data structure, see
 * source/test/testdata/calendar.txt
 */

#ifndef _INTLTESTDATADRIVENFORMAT
#define _INTLTESTDATADRIVENFORMAT

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "tsdate.h"
#include "uvector.h"
#include "unicode/format.h"
//#include "fldset.h"

class TestDataModule;
class TestData;
class DataMap;
//class DateTimeStyle;

class DataDrivenFormatTest : public IntlTest {
    void runIndexedTest(int32_t index, UBool exec, const char* &name,
            char* par = nullptr) override;
public:
    DataDrivenFormatTest();
    virtual ~DataDrivenFormatTest();
protected:

    void DataDrivenTest(char *par);
    void processTest(TestData *testData);
private:
    void testConvertDate(TestData *testData, const DataMap *settings, UBool fmt);
//    void testOps(TestData *testData, const DataMap *settings);
//    void testConvert(int32_t n, const FormatFieldsSet &fromSet,
//            Format *fromCal, const FormatFieldsSet &toSet, Format *toCal,
//            UBool fwd);
private:
    TestDataModule *driver;
};

#endif /* #if !UCONFIG_NO_COLLATION */

#endif
