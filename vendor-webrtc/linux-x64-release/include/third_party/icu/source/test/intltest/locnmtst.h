// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT:
 * Copyright (c) 2010-2016, International Business Machines Corporation
 * and others. All Rights Reserved.
 ********************************************************************/

#include "intltest.h"
#include "unicode/locdspnm.h"

/**
 * Tests for the LocaleDisplayNames class
 **/
class LocaleDisplayNamesTest: public IntlTest {
public:
    LocaleDisplayNamesTest();
    virtual ~LocaleDisplayNamesTest();

    void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par = nullptr) override;

#if !UCONFIG_NO_FORMATTING
    /**
     * Test methods to set and get data fields
     **/
    void TestCreate();
    void TestCreateDialect();
    void TestWithKeywordsAndEverything();
    void TestUldnOpen();
    void TestUldnOpenDialect();
    void TestUldnWithKeywordsAndEverything();
    void TestUldnComponents();
    void TestRootEtc();
    void TestCurrencyKeyword();
    void TestUnknownCurrencyKeyword();
    void TestUntranslatedKeywords();
    void TestPrivateUse();
    void TestUldnDisplayContext();
    void TestUldnWithGarbage();
    void TestSubstituteHandling();
    void TestNumericRegionID();

    void VerifySubstitute(LocaleDisplayNames* ldn);
    void VerifyNoSubstitute(LocaleDisplayNames* ldn);
#endif

};
