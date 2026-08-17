// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2003, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef NEW_RESOURCEBUNDLETEST_H
#define NEW_RESOURCEBUNDLETEST_H

#include "intltest.h"

/**
 * Tests for class ResourceBundle
 **/
class NewResourceBundleTest: public IntlTest {
public:
    NewResourceBundleTest();
    virtual ~NewResourceBundleTest();
    
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

    /** 
     * Perform several extensive tests using the subtest routine testTag
     **/
    void TestResourceBundles();
    /** 
     * Test construction of ResourceBundle accessing a custom test resource-file
     **/
    void TestConstruction();

    void TestIteration();

    void TestOtherAPI();

    void TestNewTypes();

    void TestGetByFallback();

    void TestFilter();

    void TestIntervalAliasFallbacks();

#if U_ENABLE_TRACING
    void TestTrace();
#endif

    void TestOpenDirectFillIn();
    void TestStackReuse();

private:
    /**
     * The assignment operator has no real implementation.
     * It is provided to make the compiler happy. Do not call.
     */
    NewResourceBundleTest& operator=(const NewResourceBundleTest&) { return *this; }

    /**
     * extensive subtests called by TestResourceBundles
     **/
    UBool testTag(const char* frag, UBool in_Root, UBool in_te, UBool in_te_IN);

    void record_pass();
    void record_fail();

    int32_t pass;
    int32_t fail;

};

#endif
